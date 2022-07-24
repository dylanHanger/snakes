use bevy::prelude::{Component, Deref, DerefMut, Entity};

use crate::movement::prelude::Direction;

#[derive(Component)]
pub struct Snake {
    pub length: usize,
    pub body: Vec<Entity>,
}
impl Snake {
    pub fn new() -> Self {
        Self {
            length: 6,
            body: vec![],
        }
    }
}

#[derive(Deref, DerefMut, Clone, Copy, PartialEq, Eq)]
pub struct PlayerId(pub u32);
#[derive(Component)]
pub struct SnakeSegment {
    pub player: PlayerId,

    pub direction: Direction,
}
