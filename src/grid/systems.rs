use super::data::*;
use bevy::prelude::*;

// There are systems that handle drawing the grid world onto the window
pub fn draw_grid_objects(
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
