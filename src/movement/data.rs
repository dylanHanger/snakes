use bevy::prelude::{Component, Deref, DerefMut, KeyCode};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    pub fn delta_x(&self) -> i32 {
        match self {
            Direction::East => -1,
            Direction::West => 1,
            _ => 0,
        }
    }
    pub fn delta_y(&self) -> i32 {
        match self {
            Direction::South => -1,
            Direction::North => 1,
            _ => 0,
        }
    }
    pub fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Component)]
pub struct RandomMoves;

#[derive(Component)]
pub struct KeyboardMoves {
    pub north: KeyCode,
    pub east: KeyCode,
    pub south: KeyCode,
    pub west: KeyCode,
}

#[derive(Component, Deref, DerefMut)]
pub struct MoveIntent {
    pub direction: Direction,
}
impl From<Direction> for MoveIntent {
    fn from(direction: Direction) -> Self {
        Self { direction }
    }
}
