use std::ops::{Index, IndexMut};

use bevy::prelude::{Component, Deref, DerefMut, IVec2, Query, Vec2, With};
use bevy_turborand::rng::{CellState, Rng};
use serde::Deserialize;

use crate::game::{collisions::prelude::Collidable, movement::prelude::Direction};

// Every object has a grid position and occupies one grid cell
#[derive(Component, Deref, DerefMut, Clone, Copy, PartialEq, Eq)]
pub struct GridPosition(IVec2);
impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(IVec2::new(x, y))
    }

    pub fn step(&self, direction: Direction) -> GridPosition {
        Self::new(self.x + direction.delta_x(), self.y + direction.delta_y())
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

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct GameGrid {
    pub width: usize,
    pub height: usize,
}
impl GameGrid {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn contains_position(&self, position: &GridPosition) -> bool {
        (position.x >= 0 && position.x < self.width as i32)
            && (position.y >= 0 && position.y < self.height as i32)
    }

    pub fn get_unoccupied_position(
        &self,
        occupied: &Query<&GridPosition, With<Collidable>>,
        rng: &mut Rng<CellState>,
    ) -> GridPosition {
        // TODO: This will hang if there are no unoccupied positions
        'outer: loop {
            let p = GridPosition::new(
                rng.i32(0..self.width as i32),
                rng.i32(0..self.height as i32),
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
impl Default for GameGrid {
    fn default() -> Self {
        Self::new(32, 32)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Food { value: f32 },
    Snake { id: Option<u32> },
}
#[derive(Deref)]
pub struct Map(Vec<Vec<CellType>>);
impl Map {
    pub fn new(grid: &GameGrid) -> Self {
        Self(vec![vec![CellType::Empty; grid.width]; grid.height])
    }
}
impl Index<GridPosition> for Map {
    type Output = CellType;

    fn index(&self, index: GridPosition) -> &Self::Output {
        if index.x < 0
            || index.x > self.0[1].len() as i32
            || index.y < 0
            || index.y > self.len() as i32
        {
            panic!("Grid position out of bounds");
        } else {
            &self.0[index.y as usize][index.x as usize]
        }
    }
}
impl IndexMut<GridPosition> for Map {
    fn index_mut(&mut self, index: GridPosition) -> &mut Self::Output {
        if index.x < 0
            || index.x > self.0[1].len() as i32
            || index.y < 0
            || index.y > self.len() as i32
        {
            panic!("Grid position out of bounds");
        } else {
            &mut self.0[index.y as usize][index.x as usize]
        }
    }
}
