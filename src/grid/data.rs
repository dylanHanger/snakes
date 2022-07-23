use bevy::prelude::{Component, Deref, DerefMut, IVec2, Vec2};

// Every object has a grid position and occupies one grid cell
#[derive(Component, Deref, DerefMut, Clone, Copy)]
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
}
