use bevy::prelude::{Component, KeyCode};

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
}
