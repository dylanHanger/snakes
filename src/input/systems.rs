use bevy::prelude::{Commands, Entity, Input, KeyCode, Query, Res, ResMut, With, Without};
use rand::Rng;

use crate::{
    food::prelude::Food,
    grid::prelude::{CellType, GameGrid, GridPosition, Map},
    movement::prelude::{Direction, MoveIntent},
    players::prelude::Player,
    snakes::prelude::{Snake, SnakeSegment},
    turns::prelude::Turn,
    Actor,
};

use super::data::{AiMoves, ExternalMoves, KeyboardMoves, RandomMoves};

pub fn random_moves_system(
    mut commands: Commands,
    q: Query<Entity, (With<Actor>, With<RandomMoves>, Without<MoveIntent>)>,
) {
    let mut rng = rand::thread_rng();

    for e in q.iter() {
        let random_move = match rng.gen_range(0..4) {
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
    q: Query<(Entity, &KeyboardMoves), With<Actor>>,
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
    agents: Query<(Entity, &ExternalMoves), With<Actor>>,
) {
    for (e, agent) in agents.iter() {
        if let Some(answer) = agent.recv() {
            commands.entity(e).insert(MoveIntent::from(answer));
        }
    }
}

pub fn init_external_agents(agents: Query<(&ExternalMoves, &Player)>, grid: Res<GameGrid>) {
    for (agent, player) in agents.iter() {
        println!("Starting game");
        // The game size
        agent.send(format!("{} {}\n", grid.width, grid.height));

        // Your snake ID
        agent.send(format!("{}\n", player.id));
    }
}

pub fn external_update_system(
    mut turn: ResMut<Turn>,
    agents: Query<&ExternalMoves, With<Actor>>,
    snakes: Query<(&GridPosition, &Snake, Option<&Player>)>,
    segments: Query<&GridPosition, With<SnakeSegment>>,
    food: Query<&GridPosition, With<Food>>,
) {
    turn.requested = true;

    let mut sorted_snakes = snakes
        .iter()
        .collect::<Vec<(&GridPosition, &Snake, Option<&Player>)>>();
    sorted_snakes.sort_by_key(|(_, _, p)| {
        if let Some(player) = p {
            player.id
        } else {
            u32::MAX
        }
    });
    for agent in agents.iter() {
        // Send food
        agent.send(format!("{}\n", food.iter().count()));
        for position in food.iter() {
            agent.send(format!("{} {}\n", position.x, position.y))
        }

        // Send snakes
        agent.send(format!("{}\n", sorted_snakes.len()));
        for (position, snake, player) in &sorted_snakes {
            if let Some(player) = player {
                agent.send(format!("{} ", player.id));
            } else {
                agent.send("-1".to_string());
            }
            agent.send(format!("{} {} {}", snake.length, position.x, position.y));
            for &body_part in &snake.body {
                if let Ok(position) = segments.get(body_part) {
                    agent.send(format!(" {} {}", position.x, position.y))
                }
            }
            agent.send("\n".to_string());
        }
    }
}

pub fn ai_moves_system(
    mut commands: Commands,
    agents: Query<(Entity, &AiMoves, &GridPosition, Option<&Snake>), With<Actor>>,
    positions: Query<(
        &GridPosition,
        Option<&Player>,
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
            map[position] = CellType::Food { value: food.value };
            food_positions.push(position);
        }
    }

    let compute_utility = |direction: Direction, position: &GridPosition, agent: &AiMoves| -> i32 {
        let mut utility = 0;

        let next_position = &position.step(direction);

        // Find best food to go to from that position
        let mut best_food = None;
        let mut best_distance = i32::MAX;

        let distance = |a: &GridPosition, b: &GridPosition| (a.x - b.x).abs() + (a.y - b.y).abs();

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

            if agent > &AiMoves::Medium {
                let tile = map[*food_pos];
                if let CellType::Food { value } = tile {
                    utility += value.floor() as i32 - d;
                }
            }
        }

        if agent > &AiMoves::Easy {
            if !grid.contains_position(next_position) {
                utility -= 500
            } else if let CellType::Snake { .. } = map[*next_position] {
                utility -= 1000;
            }
        }

        if agent > &AiMoves::Medium {
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
