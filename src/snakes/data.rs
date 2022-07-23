use bevy::prelude::{Component, Entity};

use crate::movement::prelude::Direction;

#[derive(Component)]
pub struct Snake {
    pub direction: Direction,

    pub length: usize,
    pub body: Vec<Entity>,
}
impl Snake {
    pub fn new() -> Self {
        Self {
            direction: Direction::North,
            length: 6,
            body: vec![],
        }
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        self.body.is_empty() || direction != self.direction.opposite()
    }
}

#[derive(Component)]
pub struct SnakeSegment;
