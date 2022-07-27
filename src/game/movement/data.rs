use std::str::FromStr;

use bevy::prelude::{Component, Deref, DerefMut};

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

    pub fn cardinals() -> [Direction; 4] {
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }
}
impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();

        match s.as_str() {
            "0" | "north" | "n" => Ok(Direction::North),
            "1" | "east" | "e" => Ok(Direction::East),
            "2" | "south" | "s" => Ok(Direction::South),
            "3" | "west" | "w" => Ok(Direction::West),
            _ => Err(()),
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
