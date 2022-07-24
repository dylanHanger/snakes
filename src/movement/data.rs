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
            Direction::East => 1,
            Direction::West => -1,
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
impl KeyboardMoves {
    pub fn wasd() -> Self {
        Self {
            north: KeyCode::W,
            east: KeyCode::D,
            south: KeyCode::S,
            west: KeyCode::A,
        }
    }
    pub fn arrows() -> Self {
        Self {
            north: KeyCode::Up,
            east: KeyCode::Right,
            south: KeyCode::Down,
            west: KeyCode::Left,
        }
    }
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

#[derive(Component, Deref, DerefMut)]
pub struct LastMove {
    pub direction: Direction,
}
impl From<Direction> for LastMove {
    fn from(direction: Direction) -> Self {
        Self { direction }
    }
}
