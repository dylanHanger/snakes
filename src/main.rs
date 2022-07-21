mod grid;

use bevy::prelude::*;
use grid::prelude::*;

struct SnakesPlugin;
impl Plugin for SnakesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            .insert_resource(GameGrid::new(8, 8))
            .add_startup_system(setup)
            .add_startup_system(setup_camera)
            .add_system(draw_grid_objects);
    }
}

fn setup(mut commands: Commands, grid: Res<GameGrid>) {
    for x in 0..grid.width {
        for y in 0..grid.height {
            let parity = (x + y) % 2;
            commands
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: if parity == 0 {
                            Color::WHITE * 0.1
                        } else {
                            Color::WHITE * 0.9
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(GridPosition::new(x as i32, y as i32))
                .insert(GridScale::square(0.95));
        }
    }
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
