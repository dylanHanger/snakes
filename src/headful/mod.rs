use std::ops::{Add, Mul};

use bevy::{
    asset::AssetPlugin,
    core_pipeline::CorePipelinePlugin,
    hierarchy::HierarchyPlugin,
    input::InputPlugin,
    math::{Vec2, Vec3},
    prelude::{
        App, Color, Commands, CoreStage, OrthographicCameraBundle, Plugin, Query, Res, SystemSet,
        Transform,
    },
    render::RenderPlugin,
    sprite::{Sprite, SpritePlugin},
    transform::TransformPlugin,
    window::{WindowPlugin, Windows},
    winit::WinitPlugin,
};

use crate::game::{
    food::prelude::Food,
    grid::prelude::{GameGrid, GridPosition, GridScale},
    input::prelude::keyboard_moves_system,
    players::prelude::{Player, PlayerColors},
    snakes::prelude::Snake,
    turns::prelude::TurnStage,
};

pub struct HeadfulPlugin;
impl Plugin for HeadfulPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(InputPlugin::default())
            .add_plugin(HierarchyPlugin::default())
            .add_plugin(TransformPlugin::default())
            .add_plugin(WindowPlugin::default())
            .add_plugin(AssetPlugin::default())
            .add_plugin(WinitPlugin::default())
            .add_plugin(RenderPlugin::default())
            .add_plugin(CorePipelinePlugin::default())
            .add_plugin(SpritePlugin::default());

        // Add everything related to displaying the game
        add_rendering(app);

        app.add_system_set_to_stage(
            TurnStage::Request,
            SystemSet::new()
                .label("input")
                // Read input from the keyboard
                .with_system(keyboard_moves_system),
        );
    }
}

fn add_rendering(app: &mut App) {
    app.insert_resource(PlayerColors::default())
        .add_startup_system(setup_camera)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .label("rendering")
                .with_system(color_players)
                .with_system(color_food)
                .with_system(draw_grid_objects),
        );
}

fn color_players(
    mut players: Query<(&Player, &mut Sprite, Option<&Snake>)>,
    colors: Res<PlayerColors>,
) {
    for (player, mut sprite, head) in players.iter_mut() {
        if let Some(mut color) = colors.get(player).cloned() {
            if head.is_none() {
                color *= 0.6;
            }
            sprite.color = color;
        }
    }
}

fn color_food(mut foods: Query<(&mut Sprite, &mut GridScale, &Food)>) {
    fn lerp<T>(a: T, b: T, t: f32) -> T
    where
        T: Add<Output = T> + Mul<f32, Output = T>,
    {
        a * t + b * (1. - t)
    }

    for (mut sprite, mut scale, food) in foods.iter_mut() {
        let alpha = food.get_factor() * 0.5 + 0.5;

        let color = lerp(Color::GREEN, Color::RED, alpha);
        sprite.color = color;

        let size = lerp(0.6, 0.3, alpha);
        scale.x = size;
        scale.y = size;
    }
}
fn draw_grid_objects(
    mut q: Query<(&GridPosition, &GridScale, &mut Transform)>,
    windows: Res<Windows>,
    grid: Res<GameGrid>,
) {
    let window = windows.get_primary().unwrap();

    let cell_size = f32::min(
        window.width() / grid.width as f32,
        window.height() / grid.height as f32,
    );

    let window_size = Vec2::new(window.width(), window.height());
    let grid_size = Vec2::new(grid.width as f32, grid.height as f32);

    let offset = 0.5 * (cell_size - window_size);
    let centering = 0.5 * (window_size - (grid_size * cell_size));

    for (pos, scale, mut transform) in q.iter_mut() {
        // Scale the sprite based on the grid size and window size
        transform.scale = Vec3::new(scale.x * cell_size, scale.y * cell_size, 1.0);

        // Translate the sprite based on the grid size and window size
        let x = pos.x as f32 * cell_size + offset.x + centering.x;
        let y = pos.y as f32 * cell_size + offset.y + centering.y;

        transform.translation.x = x;
        transform.translation.y = y;
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
