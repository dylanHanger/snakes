use bevy::{
    prelude::{
        default, Bundle, Component, ComputedVisibility, Entity, GlobalTransform, Handle, Image,
        Transform, Visibility,
    },
    render::texture::DEFAULT_IMAGE_HANDLE,
    sprite::Sprite,
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
    snake: Snake,
    segment: SnakeSegment,

    // Game things
    collidable: Collidable,
    actor: Actor,

    // Grid things
    position: GridPosition,
    scale: GridScale,

    // Rendering things
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}
impl SnakeBundle {
    pub fn new(position: GridPosition) -> Self {
        Self {
            snake: Snake::new(),
            segment: SnakeSegment,
            collidable: Collidable,
            actor: Actor,
            position,
            scale: GridScale::square(0.7),
            sprite: default(),
            transform: default(),
            global_transform: default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Bundle)]
pub struct SegmentBundle {
    // Snake things
    pub segment: SnakeSegment,

    // Game things
    pub collidable: Collidable,

    // Grid things
    pub position: GridPosition,
    pub scale: GridScale,

    // Rendering things
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub texture: Handle<Image>,
    /// User indication of whether an entity is visible
    pub visibility: Visibility,
    /// Algorithmically-computed indication of whether an entity is visible and should be extracted for rendering
    pub computed_visibility: ComputedVisibility,
}
impl SegmentBundle {
    pub fn new(position: GridPosition) -> Self {
        Self {
            segment: SnakeSegment,
            collidable: Collidable,
            position,
            scale: GridScale::square(0.6),
            sprite: default(),
            transform: default(),
            global_transform: default(),
            texture: DEFAULT_IMAGE_HANDLE.typed(),
            visibility: default(),
            computed_visibility: default(),
        }
    }
}
