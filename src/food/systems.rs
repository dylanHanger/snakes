use bevy::{
    prelude::{default, Color, Commands, Entity, Query, Res, Sprite, SpriteBundle, With},
};

use crate::{
    grid::prelude::{GameGrid, GridPosition, GridScale},
    snakes::prelude::Snake,
};

use super::data::Food;

pub fn can_spawn_food(food: Query<&GridPosition, With<Food>>) -> bool {
    food.iter().count() == 0
}

pub fn spawn_food_system(
    mut commands: Commands,
    occupied: Query<&GridPosition>,
    grid: Res<GameGrid>,
) {
    let position = grid.get_unoccupied_position(&occupied);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.8, 0.6),
                ..default()
            },
            ..default()
        })
        .insert(position)
        .insert(GridScale::square(0.5))
        .insert(Food { value: 5 });
}

pub fn eat_food_system(
    mut commands: Commands,
    mut snakes: Query<(&mut Snake, &GridPosition)>,
    food: Query<(Entity, &Food, &GridPosition)>,
) {
    for (mut snake, snake_pos) in snakes.iter_mut() {
        for (food_ent, food, food_pos) in food.iter() {
            if snake_pos == food_pos {
                snake.length += food.value as usize;

                commands.entity(food_ent).despawn();
            }
        }
    }
}
