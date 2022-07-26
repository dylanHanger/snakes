use bevy::prelude::{
    default, Color, Commands, Entity, EventWriter, Query, Res, Sprite, SpriteBundle, With,
};

use crate::{
    death::prelude::DeathEvent,
    grid::prelude::{GameGrid, GridPosition, GridScale},
    snakes::prelude::Snake,
};

use super::data::{Food, Rottable};

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
        .insert(Rottable)
        .insert(Food::new(5));
}

pub fn eat_food_system(
    mut commands: Commands,
    mut snakes: Query<(Entity, &mut Snake, &GridPosition)>,
    food: Query<(Entity, &Food, &GridPosition)>,
    mut deaths: EventWriter<DeathEvent>,
) {
    for (snake_ent, mut snake, snake_pos) in snakes.iter_mut() {
        for (food_ent, food, food_pos) in food.iter() {
            if snake_pos == food_pos {
                let growth = (5. * food.get_factor()).round() as i32;
                if growth < 0 {
                    let shrinkage = growth.unsigned_abs();
                    if let Some(new_length) = snake.length.checked_sub(shrinkage) {
                        snake.length = new_length;
                    } else {
                        deaths.send(DeathEvent {
                            target: snake_ent,
                            culprit: None,
                        })
                    }
                } else {
                    snake.length += growth as u32;
                }
                commands.entity(food_ent).despawn();
            }
        }
    }
}

pub fn rotting_system(
    mut commands: Commands,
    mut rottable_foods: Query<(Entity, &mut Food), With<Rottable>>,
) {
    for (e, mut food) in rottable_foods.iter_mut() {
        food.value -= 0.1;
        if food.value < 0. {
            commands.entity(e).despawn()
        }
    }
}
