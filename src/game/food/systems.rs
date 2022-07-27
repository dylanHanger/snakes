use std::ops::{Add, Mul};

use bevy::prelude::{
    default, Color, Commands, Entity, EventWriter, Query, Res, ResMut, Sprite, SpriteBundle, With,
};

use crate::game::{
    death::prelude::DeathEvent,
    grid::prelude::{GameGrid, GridPosition, GridScale},
    snakes::prelude::Snake,
    DespawnedFoods,
};

use super::{
    config::FoodConfig,
    data::{Food, Rottable},
};

pub fn can_spawn_food(food: Query<&GridPosition, With<Food>>) -> bool {
    food.iter().count() == 0
}

pub fn spawn_food_system(
    mut commands: Commands,
    occupied: Query<&GridPosition>,
    grid: Res<GameGrid>,
    food_config: Res<FoodConfig>,
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
        .insert(Food::new(food_config.last_for_turns));
}

pub fn eat_food_system(
    mut commands: Commands,
    mut snakes: Query<(Entity, &mut Snake, &GridPosition)>,
    food: Query<(Entity, &Food, &GridPosition)>,
    mut processed_entities: ResMut<DespawnedFoods>,
    mut deaths: EventWriter<DeathEvent>,
    food_config: Res<FoodConfig>,
) {
    for (snake_ent, mut snake, snake_pos) in snakes.iter_mut() {
        for (food_ent, food, food_pos) in food.iter() {
            if snake_pos == food_pos {
                let growth = (food_config.growth_amount as f32 * food.get_factor()).round() as i32;
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

                if !processed_entities.contains(&food_ent) {
                    commands.entity(food_ent).despawn();
                    processed_entities.insert(food_ent);
                }
            }
        }
    }
}

pub fn rotting_system(
    mut commands: Commands,
    mut rottable_foods: Query<(Entity, &mut Food), With<Rottable>>,
    mut processed_entities: ResMut<DespawnedFoods>,
) {
    for (e, mut food) in rottable_foods.iter_mut() {
        food.value -= 0.1;
        if food.value < 0. && !processed_entities.contains(&e) {
            commands.entity(e).despawn();
            processed_entities.insert(e);
        }
    }
}

pub fn color_food(mut foods: Query<(&mut Sprite, &mut GridScale, &Food)>) {
    fn lerp<T>(a: T, b: T, t: f32) -> T
    where
        T: Add<Output = T> + Mul<f32, Output = T>,
    {
        a * t + b * (1. - t)
    }

    for (mut sprite, mut scale, food) in foods.iter_mut() {
        let alpha = food.get_factor() * 0.5 + 0.5;

        let color = lerp(Color::GREEN, Color::RED, alpha);
        sprite.color = color;

        let size = lerp(0.6, 0.3, alpha);
        scale.x = size;
        scale.y = size;
    }
}
