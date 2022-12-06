pub mod collisions;
pub mod death;
pub mod food;
pub mod grid;
pub mod input;
pub mod movement;
pub mod players;
pub mod snakes;
pub mod turns;

pub mod prelude {}

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use bevy::{
    diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::{
        App, Commands, Component, CoreStage, Deref, IntoPipeSystem, Plugin, Resource, StartupStage,
        SystemSet, SystemStage,
    },
};
use bevy_turborand::RngPlugin;
use iyes_loopless::{
    prelude::{AppLooplessStateExt, ConditionSet, IntoConditionalSystem},
    state::NextState,
};

use collisions::prelude::*;
use death::prelude::*;
use food::prelude::*;
use input::prelude::*;
use movement::prelude::*;
use players::prelude::*;
use snakes::prelude::*;
use turns::prelude::*;

use crate::config::read_config_from_file;

use self::{death::config::DeathConfig, food::config::FoodConfig, turns::config::TurnConfig};

#[derive(Deref, Resource)]
pub struct RngSeed(pub String);

#[derive(Component)]
pub struct Actor;
pub struct SnakesPlugin {
    pub config_file: PathBuf,
}
impl Plugin for SnakesPlugin {
    fn build(&self, app: &mut App) {
        let config = read_config_from_file(&self.config_file).unwrap_or_else(|e| {
            panic!(
                "Failed to read config file {}: {:?}",
                self.config_file.display(),
                e
            )
        });

        let mut h = DefaultHasher::new();
        config.seed.hash(&mut h);
        app.add_plugin(DiagnosticsPlugin::default())
            .add_plugin(LogPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_plugin(RngPlugin::new().with_rng_seed(h.finish()));

        // Add stages for more fine grained control over when entities are added or removed
        add_stages(app);

        app.insert_resource(RngSeed(config.seed));
        // Let the world know the size of the arena
        app.insert_resource(config.grid);

        // Add core gameplay mechanics
        add_players(app, config.players);
        add_death(app, config.death);
        add_turns(app, config.turns);
        add_food(app, config.food);
        add_simulation(app);
        add_input(app);

        // Add a cleanup system to prevent zombie processes
        // TODO: This system should also run when before the game closes
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            ConditionSet::new()
                .run_if(turn_ready)
                .run_if(turns_finished)
                .with_system(kill_external_agents)
                .into(),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Paused,
    Step,
    Running,
}

fn add_stages(app: &mut App) {
    app.add_stage_after(
        CoreStage::Update,
        TurnStage::PreTurn,
        SystemStage::parallel(),
    )
    .add_stage_after(
        TurnStage::PreTurn,
        TurnStage::Request,
        SystemStage::parallel(),
    )
    .add_stage_after(
        TurnStage::Request,
        TurnStage::PostRequest,
        SystemStage::parallel(),
    )
    .add_stage_after(
        TurnStage::PostRequest,
        TurnStage::Simulate,
        SystemStage::parallel(),
    )
    .add_stage_after(
        TurnStage::Simulate,
        TurnStage::PostSimulate,
        SystemStage::parallel(),
    );
}

fn add_players(app: &mut App, player_details: Players) {
    app.insert_resource(player_details)
        .add_startup_system_set(
            SystemSet::new()
                .label("setup")
                // Create the players and set them to spawn immediately
                .with_system(setup_players),
        )
        .add_system_to_stage(CoreStage::PostUpdate, scoreboard_system)
        .add_system(external_error_system);
}

fn add_turns(app: &mut App, turn_config: TurnConfig) {
    app.add_loopless_state(GameState::Running)
        .insert_resource(Turn::from(turn_config))
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            ConditionSet::new()
                .label("wait")
                .run_if_not(turns_finished)
                // Increment the turn timer
                .with_system(turn_timer_system)
                // Check if the turn is ready to be simulated
                .with_system(turn_ready_system.run_not_in_state(GameState::Paused))
                .into(),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            ConditionSet::new()
                .label("step")
                .run_in_state(GameState::Step)
                .run_if(turn_ready)
                .with_system(|mut commands: Commands| {
                    commands.insert_resource(NextState(GameState::Paused));
                })
                .into(),
        )
        .add_system_set_to_stage(
            TurnStage::Request,
            ConditionSet::new()
                .label("request")
                .run_if_not(turn_requested)
                // Mark the turn as requested
                .with_system(request_turn_system)
                .into(),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            ConditionSet::new()
                .label("end")
                .run_if(turn_ready)
                .with_system(end_turn_system)
                .into(),
        );
}

fn add_food(app: &mut App, food_config: FoodConfig) {
    app.insert_resource(DespawnedFoods::new())
        .insert_resource(food_config)
        .add_system_set_to_stage(
            TurnStage::PreTurn,
            ConditionSet::new()
                .label("spawn food")
                .run_if_not(turn_requested)
                .run_if(can_spawn_food)
                .with_system(spawn_food_system)
                .into(),
        )
        .add_system_set_to_stage(
            TurnStage::PostSimulate,
            ConditionSet::new()
                .after("collisions")
                .run_if(turn_ready)
                .with_system(rotting_system)
                .into(),
        );
}

fn add_simulation(app: &mut App) {
    app.add_system_set_to_stage(
        TurnStage::Simulate,
        ConditionSet::new()
            .label("simulate")
            .run_if(turn_ready)
            .with_system(slither_system.pipe(movement_system))
            .into(),
    )
    .add_system_set_to_stage(
        TurnStage::PostSimulate,
        SystemSet::new()
            .label("collisions")
            .with_system(collision_system)
            .with_system(eat_food_system),
    );
}

fn add_death(app: &mut App, death_config: DeathConfig) {
    app.add_event::<DeathEvent>()
        .insert_resource(death_config)
        .add_system_set_to_stage(
            TurnStage::PreTurn,
            ConditionSet::new()
                .label("spawn snakes")
                .run_if_not(turn_requested)
                .with_system(respawn_system)
                .into(),
        )
        .add_system_set_to_stage(
            TurnStage::PostSimulate,
            SystemSet::new()
                .label("deaths")
                .after("collisions")
                .with_system(death_system),
        )
        .add_system_set_to_stage(
            TurnStage::PostSimulate,
            ConditionSet::new()
                .label("respawn")
                .after("deaths")
                .run_if(turn_ready)
                .with_system(death_timer_system)
                .into(),
        );
}

fn add_input(app: &mut App) {
    app.add_startup_system_to_stage(
        StartupStage::PostStartup,
        // Send the initialization input to the external agents
        init_external_agents,
    )
    .add_system_set_to_stage(
        TurnStage::Request,
        ConditionSet::new()
            .label("input")
            .before("request")
            .run_if_not(turn_requested)
            // Compute move for AI agents
            .with_system(ai_moves_system)
            // Choose a random move
            .with_system(random_moves_system)
            // Send out the input to the external agents so they can begin computing a move
            .with_system(external_update_system)
            .into(),
    )
    .add_system_set_to_stage(
        TurnStage::Request,
        SystemSet::new()
            .label("input")
            // Read input from the external agents
            .with_system(external_moves_system),
    )
    .add_system_set_to_stage(
        TurnStage::PostRequest,
        ConditionSet::new()
            .label("fix input")
            .run_if(turn_ready)
            // Make sure snakes don't try to make illegal moves
            .with_system(limit_snake_moves)
            // Make sure snakes have a default move if they don't have one
            .with_system(default_snake_moves)
            .into(),
    );
}
