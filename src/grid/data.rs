use bevy::prelude::{Component, Deref, DerefMut, IVec2, Query, Vec2};
use rand::Rng;

// Every object has a grid position and occupies one grid cell
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition(IVec2);
impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }
}

// And a size that represents its size on the grid (it still only occupies one cell, this is for rendering only)
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct GridScale(Vec2);
impl GridScale {
    pub fn square(side: f32) -> Self {
        Self(Vec2::new(side, side))
    }
}

// The grid has a finite size
pub struct GameGrid {
    pub width: u32,
    pub height: u32,
}
impl GameGrid {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn contains_position(&self, position: &GridPosition) -> bool {
        (position.x >= 0 && position.x < self.width as i32)
            && (position.y >= 0 && position.y < self.height as i32)
    }

    pub fn get_unoccupied_position(&self, occupied: &Query<&GridPosition>) -> GridPosition {
        // TODO: This will hang if there are no unoccupied positions
        let mut rng = rand::thread_rng();
        'outer: loop {
            let p = GridPosition::new(
                rng.gen_range(0..self.width as i32),
                rng.gen_range(0..self.height as i32),
            );
            for &obstacle in occupied.iter() {
                if obstacle == p {
                    continue 'outer;
                }
            }
            break 'outer p;
        }
    }
}
