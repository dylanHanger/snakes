pub mod collisions;
pub mod death;
pub mod food;
pub mod grid;
pub mod input;
pub mod movement;
pub mod players;
pub mod replays;
pub mod snakes;
pub mod turns;

pub mod prelude {}

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    path::PathBuf,
};

use bevy::{
    app::AppExit,
    diagnostic::{DiagnosticsPlugin, LogDiagnosticsPlugin},
    log::LogPlugin,
    prelude::{
        apply_system_buffers, in_state, not, on_event, resource_exists, App, Component, Condition,
        CoreSet, Deref, In, IntoPipeSystem, IntoSystemConfig, IntoSystemConfigs,
        IntoSystemSetConfigs, Plugin, Resource, StartupSet, States,
    },
};
use bevy_turborand::RngPlugin;

use collisions::prelude::*;
use death::prelude::*;
use food::prelude::*;
use input::prelude::*;
use movement::prelude::*;
use players::prelude::*;
use snakes::prelude::*;
use turns::prelude::*;

use crate::config::read_config_from_file;

use self::{
    death::config::DeathConfig,
    food::config::FoodConfig,
    replays::prelude::{create_replay, record_replay, ReplayWriter},
    turns::config::TurnConfig,
};

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

        // Replay stuff
        app.insert_resource(config.replays);
        app.add_startup_system(
            create_replay.pipe(|result: In<Result<(), std::io::Error>>| result.0.unwrap()),
        );
        app.add_system(
            record_replay
                .pipe(|result: In<Result<(), std::io::Error>>| result.0.unwrap())
                .run_if(turn_ready)
                .run_if(resource_exists::<ReplayWriter>())
                .in_base_set(TurnSet::Request),
        );

        // Add core gameplay mechanics
        add_players(app, config.players);
        add_death(app, config.death);
        add_turns(app, config.turns);
        add_food(app, config.food);
        add_simulation(app);
        add_input(app);

        // Add a cleanup system to prevent zombie processes
        app.add_system(
            kill_external_agents
                .run_if(on_event::<AppExit>().or_else(turn_ready.and_then(turns_finished)))
                .in_base_set(CoreSet::PostUpdate),
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Running,
    Paused,
    Step,
    GameOver,
}

fn add_stages(app: &mut App) {
    // FIXME: TurnStages are spaghetti and I am sure they can be optimised
    app.configure_sets(
        (
            CoreSet::UpdateFlush,
            TurnSet::PreTurn,
            TurnSet::PreTurnFlush,
            TurnSet::Request,
            TurnSet::RequestFlush,
            TurnSet::PostRequest,
            TurnSet::PostRequestFlush,
            TurnSet::Simulate,
            TurnSet::SimulateFlush,
            TurnSet::PostSimulate,
            TurnSet::PostSimulateFlush,
            CoreSet::PostUpdate,
        )
            .chain(),
    )
    .add_system(apply_system_buffers.in_base_set(TurnSet::PreTurnFlush))
    .add_system(apply_system_buffers.in_base_set(TurnSet::RequestFlush))
    .add_system(apply_system_buffers.in_base_set(TurnSet::PostRequestFlush))
    .add_system(apply_system_buffers.in_base_set(TurnSet::SimulateFlush))
    .add_system(apply_system_buffers.in_base_set(TurnSet::PostSimulateFlush));
}

fn add_players(app: &mut App, player_details: Players) {
    app.insert_resource(player_details)
        // Create the players and set them to spawn immediately
        .add_startup_system(setup_players)
        .add_system(scoreboard_system.in_base_set(CoreSet::PostUpdate))
        .add_system(external_error_system);
}

fn add_turns(app: &mut App, turn_config: TurnConfig) {
    let turn = Turn::from(turn_config);
    app.add_state::<GameState>()
        .insert_resource(turn_config)
        .insert_resource(turn)
        .add_systems(
            (
                // Increment the turn timer
                turn_timer_system.run_if(not(turns_finished)),
                // Check if the turn is ready to be simulated
                turn_ready_system.run_if(not(in_state(GameState::Paused))),
            )
                .in_base_set(CoreSet::PreUpdate),
        )
        .add_system(
            pause_after_step
                .run_if(in_state(GameState::Step).and_then(turn_ready))
                .in_base_set(TurnSet::PostSimulate),
        )
        .add_system(
            request_turn_system
                .run_if(not(turn_requested))
                .in_base_set(TurnSet::Request),
        )
        .add_system(
            end_turn_system
                .run_if(turn_ready)
                .in_base_set(CoreSet::PostUpdate),
        );
}

fn add_food(app: &mut App, food_config: FoodConfig) {
    app.insert_resource(DespawnedFoods::new())
        .insert_resource(food_config)
        .add_system(
            spawn_food_system
                .run_if(not(turn_requested).and_then(can_spawn_food))
                .in_base_set(TurnSet::PreTurn),
        )
        .add_system(
            rotting_system
                .run_if(turn_ready)
                .after(eat_food_system)
                .in_base_set(TurnSet::PostSimulate),
        );
}

fn add_simulation(app: &mut App) {
    app.add_system(
        slither_system
            .pipe(movement_system)
            .run_if(turn_ready)
            .in_base_set(TurnSet::Simulate),
    )
    .add_systems((collision_system, eat_food_system).in_base_set(TurnSet::PostSimulate));
}

fn add_death(app: &mut App, death_config: DeathConfig) {
    app.add_event::<DeathEvent>()
        .insert_resource(death_config)
        .add_system(
            respawn_system
                .run_if(not(turn_requested))
                .in_base_set(TurnSet::PreTurn),
        )
        .add_system(
            death_system
                .after(collision_system)
                .after(eat_food_system)
                .in_base_set(TurnSet::PostSimulate),
        )
        .add_system(
            death_timer_system
                .run_if(turn_ready)
                .after(death_system)
                .in_base_set(TurnSet::PostSimulate),
        );
}

fn add_input(app: &mut App) {
    app.add_startup_system(init_external_agents.in_base_set(StartupSet::PostStartup))
        .add_systems(
            (
                ai_moves_system.run_if(not(turn_requested)),
                random_moves_system.run_if(not(turn_requested)),
                external_update_system.run_if(not(turn_requested)),
            )
                .before(request_turn_system)
                .in_base_set(TurnSet::Request),
        )
        // Read input from the external agents
        .add_system(external_moves_system.in_base_set(TurnSet::Request))
        .add_systems(
            (
                limit_snake_moves.run_if(turn_ready),
                default_snake_moves.run_if(turn_ready),
            )
                .in_base_set(TurnSet::PostRequest),
        );
}
