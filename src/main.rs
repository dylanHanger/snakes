mod collisions;
mod death;
mod food;
mod grid;
mod movement;
mod snakes;
mod turns;

use std::time::Duration;

use bevy::prelude::{
    default, App, ClearColor, Color, Commands, Component, CoreStage, DefaultPlugins, IntoChainSystem, OrthographicCameraBundle, Plugin,
    Sprite, SpriteBundle, SystemSet, WindowDescriptor,
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
            .insert_resource(Turn::new(Duration::from_millis(0), true))
            .add_event::<DeathEvent>()
            .add_startup_system(setup)
            .add_startup_system(setup_camera)
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::new()
                    .with_run_criteria(turn_ready)
                    .with_system(limit_snake_moves),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new()
                    .with_run_criteria(turn_ready)
                    .with_run_criteria(can_spawn_food)
                    .with_system(spawn_food_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(turn_ready)
                    .with_system(reset_turn_system)
                    .with_system(slither_system.chain(movement_system)), // Is this a flagrant abuse of system chaining to enforce order?
            )
            .add_system(turn_timer_system)
            .add_system(turn_ready_system)
            .add_system(random_moves_system)
            .add_system(keyboard_moves_system)
            .add_system(collision_system)
            .add_system(eat_food_system)
            .add_system(death_system)
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new().with_system(draw_grid_objects),
            );
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.8),
                ..default()
            },
            ..default()
        })
        .insert(GridPosition::new(4, 4))
        .insert(GridScale::square(0.7))
        .insert(Actor)
        .insert(Snake::new())
        .insert(SnakeSegment {
            player: PlayerId(0),
            direction: Direction::North,
        })
        .insert(KeyboardMoves::wasd());
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
