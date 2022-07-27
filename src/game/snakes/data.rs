use bevy::{
    prelude::{default, Bundle, Color, Component, Entity},
    sprite::SpriteBundle,
};

use crate::game::{
    collisions::prelude::Collidable,
    grid::prelude::{GridPosition, GridScale},
    movement::prelude::Direction,
    Actor,
};
#[derive(Component)]
pub struct Snake {
    pub length: u32,
    pub body: Vec<Entity>,

    pub direction: Direction,
}
impl Snake {
    pub fn new() -> Self {
        Self {
            length: 6,
            body: vec![],

            direction: Direction::North,
        }
    }

    pub fn can_move(&self, direction: Direction) -> bool {
        self.body.is_empty() || direction != self.direction.opposite()
    }
}

#[derive(Bundle)]
pub struct SnakeBundle {
    // Snake things
    _snake: Snake,
    _segment: SnakeSegment,

    // Game things
    _collidable: Collidable,
    _actor: Actor,

    // Grid things
    _position: GridPosition,
    _scale: GridScale,

    // Rendering things
    #[bundle]
    _sprite: SpriteBundle,
}
impl SnakeBundle {
    pub fn new(position: GridPosition) -> Self {
        Self {
            _snake: Snake::new(),
            _segment: SnakeSegment,
            _collidable: Collidable,
            _actor: Actor,
            _position: position,
            _scale: GridScale::square(0.7),
            _sprite: SpriteBundle::default(),
        }
    }
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Bundle)]
pub struct SegmentBundle {
    // Snake things
    _segment: SnakeSegment,

    // Game things
    _collidable: Collidable,

    // Grid things
    _position: GridPosition,
    _scale: GridScale,

    // Rendering things
    #[bundle]
    _sprite: SpriteBundle,
}
impl SegmentBundle {
    pub fn new(position: GridPosition) -> Self {
        Self {
            _segment: SnakeSegment,
            _collidable: Collidable,
            _position: position,
            _scale: GridScale::square(0.6),
            _sprite: SpriteBundle {
                sprite: bevy::sprite::Sprite {
                    color: Color::rgb(0.6, 0.6, 0.6),
                    ..default()
                },
                ..default()
            },
        }
    }
}
