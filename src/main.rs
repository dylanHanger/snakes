mod collisions;
mod death;
mod food;
mod grid;
mod input;
mod movement;
mod players;
mod snakes;
mod turns;

use std::time::Duration;

use bevy::{
    prelude::{
        default, App, ClearColor, Color, Commands, Component, CoreStage, DefaultPlugins, Deref,
        DerefMut, Entity, IntoChainSystem, OrthographicCameraBundle, Plugin, ResMut, StartupStage,
        SystemSet, SystemStage, WindowDescriptor,
    },
    utils::HashSet,
};
use collisions::prelude::*;
use death::prelude::*;
use food::prelude::*;
use grid::prelude::*;
use input::prelude::*;
use iyes_loopless::prelude::ConditionSet;
use movement::prelude::*;
use players::prelude::{color_players, scoreboard_system, Player, PlayerColors, Scoreboard};
use snakes::prelude::*;
use turns::prelude::*;

#[derive(Component)]
pub struct Actor;

#[derive(Deref, DerefMut)]
pub struct ProcessedEntities(HashSet<Entity>);
impl ProcessedEntities {
    pub fn new() -> Self {
        Self(HashSet::new())
    }
}
impl Default for ProcessedEntities {
    fn default() -> Self {
        Self::new()
    }
}

struct SnakesPlugin;
impl Plugin for SnakesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .insert_resource(GameGrid::new(32, 32))
            .insert_resource(Turn::new(Duration::from_millis(100), false))
            .insert_resource(PlayerColors::default())
            .insert_resource(Scoreboard::new())
            .insert_resource(ProcessedEntities::new())
            .insert_resource(DeathConfig { respawn_time: 10 })
            .insert_resource(FoodConfig {
                last_for_turns: 75,
                growth_amount: 5,
            })
            .add_stage_after(
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
            )
            .add_event::<DeathEvent>()
            .add_startup_system(setup)
            .add_startup_system(setup_camera)
            .add_startup_system_to_stage(StartupStage::PostStartup, init_external_agents)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .label("wait")
                    .with_system(turn_timer_system)
                    .with_system(turn_ready_system),
            )
            .add_system_set_to_stage(
                TurnStage::PreTurn,
                ConditionSet::new()
                    .label("spawn")
                    .run_if_not(turn_requested)
                    .run_if(can_spawn_food)
                    .with_system(spawn_food_system)
                    .into(),
            )
            .add_system_set_to_stage(
                TurnStage::Request,
                ConditionSet::new()
                    .label("request")
                    .run_if_not(turn_requested)
                    .with_system(external_update_system)
                    .into(),
            )
            .add_system_set_to_stage(
                TurnStage::Request,
                ConditionSet::new()
                    .label("input")
                    .before("request")
                    .run_if_not(turn_requested)
                    .with_system(ai_moves_system)
                    .with_system(random_moves_system)
                    .into(),
            )
            .add_system_set_to_stage(
                TurnStage::Request,
                SystemSet::new()
                    .label("input")
                    .with_system(external_moves_system)
                    .with_system(keyboard_moves_system),
            )
            .add_system_set_to_stage(
                TurnStage::PostRequest,
                ConditionSet::new()
                    .label("fix input")
                    .run_if(turn_ready)
                    .with_system(limit_snake_moves)
                    .with_system(default_snake_moves)
                    .into(),
            )
            .add_system_set_to_stage(
                TurnStage::Simulate,
                ConditionSet::new()
                    .label("simulate")
                    .run_if(turn_ready)
                    .with_system(slither_system.chain(movement_system))
                    .into(),
            )
            .add_system_set_to_stage(
                TurnStage::PostSimulate,
                SystemSet::new()
                    .label("collisions")
                    .with_system(collision_system)
                    .with_system(eat_food_system),
            )
            .add_system_set_to_stage(
                TurnStage::PostSimulate,
                ConditionSet::new()
                    .after("collisions")
                    .run_if(turn_ready)
                    .with_system(rotting_system)
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
                    .label("spawn")
                    .after("deaths")
                    .run_if(turn_ready)
                    .with_system(spawn_snakes_system)
                    .into(),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                ConditionSet::new()
                    .label("end")
                    .run_if(turn_ready)
                    .with_system(reset_turn_system)
                    .into(),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .label("rendering")
                    .with_system(scoreboard_system)
                    .with_system(color_players)
                    .with_system(color_food)
                    .with_system(draw_grid_objects),
            );
    }
}

fn setup(mut commands: Commands, mut scoreboard: ResMut<Scoreboard>) {
    // commands
    //     .spawn()
    //     .insert(Player { id: 0 })
    //     .insert(Respawning { time: 0 })
    //     .insert(KeyboardMoves::wasd());
    // scoreboard.insert_new(Player { id: 0 });
    // commands
    //     .spawn()
    //     .insert(Player { id: 1 })
    //     .insert(Respawning { time: 0 })
    //     .insert(KeyboardMoves::arrows());
    // scoreboard.insert_new(Player { id: 1 });
    commands
        .spawn()
        .insert(Player { id: 0 })
        .insert(Respawning { time: 0 })
        .insert(BuiltinAi::Medium);
    scoreboard.insert_new(Player { id: 0 });
    commands
        .spawn()
        .insert(Player { id: 2 })
        .insert(Respawning { time: 0 })
        .insert(BuiltinAi::Hard);
    scoreboard.insert_new(Player { id: 2 });
    commands
        .spawn()
        .insert(Player { id: 3 })
        .insert(Respawning { time: 0 })
        .insert(CustomAi::new(
            "python".to_string(),
            vec!["monty.py".to_string()],
        ));
    scoreboard.insert_new(Player { id: 3 });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: 600.,
            height: 600.,
            title: "Snakes!".to_string(),
            ..default()
        })
        .add_plugin(SnakesPlugin)
        .run();
}
