mod collisions;
mod death;
mod food;
mod grid;
mod movement;
mod snakes;
mod turns;

use std::time::Duration;

use bevy::prelude::{
    default, App, ClearColor, Color, Commands, Component, CoreStage, DefaultPlugins,
    IntoChainSystem, OrthographicCameraBundle, Plugin, SystemSet, SystemStage,
    WindowDescriptor,
};
use collisions::prelude::*;
use death::prelude::*;
use food::prelude::*;
use grid::prelude::*;
use movement::prelude::*;
use snakes::prelude::*;
use turns::prelude::*;

#[derive(Component)]
pub struct Actor;

struct SnakesPlugin;
impl Plugin for SnakesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .insert_resource(GameGrid::new(32, 32))
            .insert_resource(Turn::new(Duration::from_millis(100), false))
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
            .add_event::<DeathEvent>()
            .add_startup_system(setup)
            .add_startup_system(setup_camera)
            .add_system_set_to_stage(
                TurnStage::PreTurn,
                SystemSet::new()
                    .label("spawn")
                    // .with_system(spawn_snakes_system)
                    .with_run_criteria(turn_ready)
                    .with_run_criteria(can_spawn_food)
                    .with_system(spawn_food_system),
            )
            .add_system_set_to_stage(
                TurnStage::Request,
                SystemSet::new()
                    .label("wait")
                    .with_system(turn_timer_system)
                    .with_system(turn_ready_system),
            )
            .add_system_set_to_stage(
                TurnStage::Request,
                SystemSet::new()
                    .label("input")
                    .before("wait")
                    .with_system(random_moves_system)
                    .with_system(keyboard_moves_system),
            )
            .add_system_set_to_stage(
                TurnStage::PostRequest,
                SystemSet::new()
                    .label("fix input")
                    // .after("input")
                    .with_run_criteria(turn_ready)
                    .with_system(limit_snake_moves)
                    .with_system(default_snake_moves),
            )
            .add_system_set_to_stage(
                TurnStage::Simulate,
                SystemSet::new()
                    .label("simulate")
                    .with_run_criteria(turn_ready)
                    .with_system(reset_turn_system)
                    .with_system(slither_system.chain(movement_system)),
            )
            .add_system_set_to_stage(
                TurnStage::Simulate,
                SystemSet::new()
                    .label("collisions")
                    .after("simulate")
                    .with_system(collision_system)
                    .with_system(eat_food_system),
            )
            .add_system_set_to_stage(
                TurnStage::Simulate,
                SystemSet::new()
                    .label("deaths")
                    .after("collisions")
                    .with_system(death_system),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .label("grid transforms")
                    .with_system(draw_grid_objects),
            );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SnakeBundle::new(
            Player { id: 0 },
            GridPosition::new(16, 16),
        ))
        .insert(KeyboardMoves::wasd());
    commands
        .spawn_bundle(SnakeBundle::new(
            Player { id: 1 },
            GridPosition::new(15, 15),
        ))
        .insert(RandomMoves);
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
