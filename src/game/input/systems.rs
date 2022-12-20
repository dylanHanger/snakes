use bevy::prelude::{Commands, Entity, Input, KeyCode, Or, Query, Res, ResMut, With, Without};
use bevy_turborand::{DelegatedRng, RngComponent, TurboRand};

use crate::game::{
    death::prelude::Respawning,
    food::{config::FoodConfig, prelude::Food},
    grid::prelude::{CellType, GameGrid, GridPosition, Map},
    movement::prelude::{Direction, MoveIntent},
    players::prelude::{PlayerId, Players},
    snakes::prelude::{Snake, SnakeSegment},
    turns::{config::TurnConfig, prelude::Turn},
    Actor,
};

use super::data::{BuiltinAi, CustomAi, KeyboardInput, RandomAi};

pub fn random_moves_system(
    mut commands: Commands,
    mut q: Query<(Entity, &mut RngComponent), (With<Actor>, With<RandomAi>, Without<MoveIntent>)>,
) {
    for (e, mut rand) in q.iter_mut() {
        let rng = rand.get_mut();
        let random_move = match rng.u32(0..4) {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        };

        commands.entity(e).insert(MoveIntent::from(random_move));
    }
}

pub fn keyboard_moves_system(
    mut commands: Commands,
    q: Query<(Entity, &KeyboardInput), With<Actor>>,
    input: Res<Input<KeyCode>>,
) {
    for (e, controls) in q.iter() {
        let direction = if input.just_pressed(controls.north) {
            Some(Direction::North)
        } else if input.just_pressed(controls.east) {
            Some(Direction::East)
        } else if input.just_pressed(controls.south) {
            Some(Direction::South)
        } else if input.just_pressed(controls.west) {
            Some(Direction::West)
        } else {
            None
        };

        if let Some(dir) = direction {
            commands.entity(e).insert(MoveIntent::from(dir));
        }
    }
}

pub fn external_moves_system(
    mut commands: Commands,
    agents: Query<(Entity, &CustomAi), With<Actor>>,
) {
    for (e, agent) in agents.iter() {
        if let Some(answer) = agent.recv() {
            commands.entity(e).insert(MoveIntent::from(answer));
        }
    }
}

pub fn kill_external_agents(mut commands: Commands, agents: Query<Entity, With<CustomAi>>) {
    for e in agents.iter() {
        commands.entity(e).remove::<CustomAi>();
    }
}

pub fn init_external_agents(
    agents: Query<(&CustomAi, &PlayerId)>,
    grid: Res<GameGrid>,
    players: Res<Players>,
    food_config: Res<FoodConfig>,
    turn_config: Res<TurnConfig>,
) {
    for (agent, player) in agents.iter() {
        // The game size
        agent.send(format!("{} {}\n", grid.width, grid.height));

        // The food details
        agent.send(format!(
            "{} {}\n",
            food_config.initial_lifetime, food_config.initial_value
        ));

        // The number of snakes and our snake ID
        agent.send(format!("{} {}\n", players.len(), player.id));

        // The turn details
        agent.send(format!(
            "{} {}\n",
            turn_config.max_turns,
            if turn_config.wait_for_all {
                -1
            } else {
                turn_config.turn_time as i64
            }
        ));
    }
}

pub fn external_error_system(agents: Query<&CustomAi>) {
    for agent in agents.iter() {
        if let Some(msg) = agent.recv_err() {
            if !agent.silent {
                println!("{}", msg);
            }
        }
    }
}

pub fn request_turn_system(mut turn: ResMut<Turn>) {
    turn.requested = true;
}

pub fn external_update_system(
    agents: Query<&CustomAi, With<Actor>>,
    snakes: Query<
        (Option<&GridPosition>, Option<&Snake>, &PlayerId),
        Or<((With<Snake>, With<GridPosition>), With<Respawning>)>, // Either it is has a snake and a position, or it is respawning
    >,
    segments: Query<&GridPosition, With<SnakeSegment>>,
    food: Query<(Entity, &GridPosition, &Food)>,
    players: Res<Players>,
) {
    let mut player_info = snakes
        .iter()
        .map(|(position, snake, player)| {
            let score = players.get(player).unwrap().score;
            let body = position.and_then(|pos| {
                snake.map(|snake| {
                    let body_parts = snake
                        .body
                        .iter()
                        .map(|&body_part| *segments.get(body_part).unwrap())
                        .collect::<Vec<_>>();
                    let mut body = vec![*pos];
                    body.extend(body_parts);
                    body
                })
            });

            (*player, score, body)
        })
        .collect::<Vec<_>>();
    player_info.sort_by_key(|(p, _, _)| p.id);

    let mut sorted_food = food.iter().collect::<Vec<_>>();
    sorted_food.sort_by_key(|(e, _, _)| e.index());

    for agent in agents.iter() {
        // Send food
        agent.send(format!("{}\n", food.iter().count()));
        for (_, position, food) in food.iter() {
            agent.send(format!("{} {} {}\n", food.lifetime, position.x, position.y));
        }

        // Send snakes
        for (player, score, body) in player_info.iter() {
            agent.send(format!(
                "{} {} {} {}",
                player.id,
                score.kills,
                score.deaths,
                body.as_ref().map_or(0, |body| body.len())
            ));

            if let Some(body) = body {
                for body_part in body.iter() {
                    agent.send(format!(" {} {}", body_part.x, body_part.y));
                }
            }
            agent.send("\n".into());
        }
    }
}

pub fn ai_moves_system(
    mut commands: Commands,
    agents: Query<(Entity, &BuiltinAi, &GridPosition, Option<&Snake>), With<Actor>>,
    positions: Query<(
        &GridPosition,
        Option<&PlayerId>,
        Option<&SnakeSegment>,
        Option<&Food>,
    )>,
    grid: Res<GameGrid>,
) {
    // Construct map
    let mut map = Map::new(grid.as_ref());
    let mut food_positions = vec![];
    for (&position, player, segment, food) in positions.iter() {
        if segment.is_some() {
            map[position] = CellType::Snake {
                id: player.map(|p| p.id),
            };
        } else if let Some(food) = food {
            map[position] = CellType::Food {
                lifetime: food.lifetime,
            };
            food_positions.push(position);
        }
    }

    let compute_utility =
        |direction: Direction, position: &GridPosition, agent: &BuiltinAi| -> i32 {
            let mut utility = 0;

            let next_position = &position.step(direction);

            // Find best food to go to from that position
            let mut best_food = None;
            let mut best_distance = i32::MAX;

            let distance =
                |a: &GridPosition, b: &GridPosition| (a.x - b.x).abs() + (a.y - b.y).abs();

            for food_position in &food_positions {
                let d = distance(food_position, next_position);

                if d < best_distance {
                    best_food = Some(food_position);
                    best_distance = d;
                }
            }

            // Add to the utility if this direction moves us closer to the best apple
            if let Some(food_pos) = best_food {
                let d = distance(next_position, food_pos);
                utility -= d;

                if agent > &BuiltinAi::Medium {
                    let tile = map[*food_pos];
                    if let CellType::Food { lifetime } = tile {
                        let value = lifetime as f32 / 10.;
                        utility += value.floor() as i32 - d;
                    }
                }
            }

            if agent > &BuiltinAi::Easy {
                if !grid.contains_position(next_position) {
                    utility -= 500
                } else if let CellType::Snake { .. } = map[*next_position] {
                    utility -= 1000;
                }
            }

            if agent > &BuiltinAi::Medium {
                let num_neighbours = Direction::cardinals()
                    .iter()
                    .map(|&d| {
                        let next_next_position = next_position.step(d);
                        if !grid.contains_position(&next_next_position) {
                            return 0.25;
                        } else if let CellType::Snake { .. } = map[next_next_position] {
                            return 1.0;
                        }
                        -0.25
                    })
                    .sum::<f32>();
                utility -= 5 * num_neighbours.round() as i32;
            }

            utility
        };

    // Compute move for each player
    let cardinals = Direction::cardinals();
    for (e, agent, position, snake) in agents.iter() {
        let mut options = cardinals
            .iter()
            .filter(|&&direction| {
                if let Some(snake) = snake {
                    if !snake.can_move(direction) {
                        return false;
                    }
                }
                true
            })
            .collect::<Vec<&Direction>>();
        options.sort_by_key(|&&direction| compute_utility(direction, position, agent));
        let direction = *options.pop().unwrap_or(&Direction::East);

        commands.entity(e).insert(MoveIntent::from(direction));
    }
}
