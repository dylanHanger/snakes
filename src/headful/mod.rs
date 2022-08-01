use std::{
    ops::{Add, Mul},
    time::Duration,
};

use bevy::{
    asset::AssetPlugin,
    core_pipeline::CorePipelinePlugin,
    hierarchy::HierarchyPlugin,
    input::InputPlugin,
    math::{Vec2, Vec3, Vec3Swizzles},
    prelude::{
        default, App, Color, Commands, CoreStage, Plugin, Query, Res, SystemSet, Transform, *,
    },
    render::{texture::ImageSettings, RenderPlugin},
    sprite::{Sprite, SpritePlugin},
    text::TextPlugin,
    transform::TransformPlugin,
    ui::{UiPlugin, UiRect},
    window::{WindowPlugin, Windows},
    winit::WinitPlugin,
};
use bevy_easings::{Ease, EaseFunction, EasingType, EasingsPlugin};
use iyes_loopless::{
    prelude::{ConditionSet, IntoConditionalSystem},
    state::{CurrentState, NextState},
};

use crate::game::{
    food::prelude::Food,
    grid::prelude::{GameGrid, GridPosition, GridScale},
    input::prelude::keyboard_moves_system,
    players::prelude::{PlayerId, Players},
    turns::prelude::TurnStage,
    GameState,
};

pub struct HeadfulPlugin;
impl Plugin for HeadfulPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
            .insert_resource(WindowDescriptor {
                width: 900.,
                height: 600.,
                title: "Snakes!".to_string(),
                ..default()
            })
            .insert_resource(ImageSettings::default_nearest());

        app.add_plugin(InputPlugin::default())
            .add_plugin(HierarchyPlugin::default())
            .add_plugin(TransformPlugin::default())
            .add_plugin(WindowPlugin::default())
            .add_plugin(AssetPlugin::default())
            .add_plugin(WinitPlugin::default())
            .add_plugin(RenderPlugin::default())
            .add_plugin(CorePipelinePlugin::default())
            .add_plugin(SpritePlugin::default())
            .add_plugin(TextPlugin::default())
            .add_plugin(UiPlugin::default())
            .add_plugin(EasingsPlugin);

        // Add everything related to displaying the game
        add_rendering(app);

        // Add everything related to the interface
        add_ui(app);

        app.add_startup_system(setup_cameras);
        app.add_system_set_to_stage(
            TurnStage::Request,
            SystemSet::new()
                .label("input")
                // Read input from the keyboard
                .with_system(keyboard_moves_system),
        );
    }
}

fn add_ui(app: &mut App) {
    app.add_startup_system(setup_ui)
        .add_startup_system_to_stage(StartupStage::PostStartup, init_scoreboard)
        .add_system(update_scoreboard)
        .add_system_set(
            ConditionSet::new()
                .with_system(button_interactions)
                .with_system(pause_button_text)
                .with_system(toggle_pause.run_if(button_clicked::<PauseButton>))
                .into(),
        );
}
#[derive(Component)]
struct ArenaArea;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        // Root node
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // Main window
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_grow: 2.,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .insert(ArenaArea);

            // Sidebar
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::FlexStart,
                        flex_grow: 1.,
                        flex_basis: Val::Px(0.),
                        padding: UiRect {
                            top: Val::Undefined,
                            ..UiRect::all(Val::Px(10.))
                        },
                        ..default()
                    },
                    color: Color::rgb(0.5, 0.5, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_sidebar_header(parent, asset_server.load("fonts/Saira.ttf"));
                    spawn_scoreboard(parent, asset_server.load("fonts/Saira.ttf"));
                    spawn_playback_controls(parent, asset_server.load("fonts/Saira.ttf"));
                });
        });
}

fn spawn_sidebar_header(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_basis: Val::Px(0.),
                flex_shrink: 0.,
                margin: UiRect::all(Val::Px(10.)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Snakes!",
                TextStyle {
                    font,
                    font_size: 32.,
                    color: Color::BLACK,
                },
            ));
        });
}

#[derive(Component)]
struct ScoreboardUi;
#[derive(Component)]
struct Scoreline;

fn spawn_scoreboard(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect {
                    bottom: Val::Px(10.),
                    ..default()
                },
                flex_shrink: 1.,
                flex_grow: 1.,
                ..default()
            },
            color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..default()
        })
        .with_children(|parent| {
            // A heading row with the score labels
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexEnd,
                        size: Size::new(Val::Percent(100.), Val::Auto),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(NodeBundle {
                        style: Style {
                            flex_grow: 1. / 3.,
                            flex_shrink: 0.,
                            ..default()
                        },
                        color: Color::NONE.into(),
                        ..default()
                    });
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::SpaceAround,
                                flex_basis: Val::Percent(2. / 3.),
                                flex_grow: 2. / 3.,
                                flex_shrink: 0.,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            for stat in ["Length", "Kills", "Deaths"] {
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            flex_grow: 1.,
                                            flex_basis: Val::Px(0.),
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        },
                                        color: Color::NONE.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn_bundle(
                                            TextBundle::from_section(
                                                stat,
                                                TextStyle {
                                                    font: font.clone(),
                                                    font_size: 20.,
                                                    color: Color::BLACK,
                                                },
                                            )
                                            .with_style(Style {
                                                flex_basis: Val::Px(0.),
                                                flex_grow: 0.,
                                                ..default()
                                            }),
                                        );
                                    });
                            }
                        });
                });
            // The actual player scores
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        overflow: Overflow::Hidden,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .insert(ScoreboardUi);
        });
}

fn init_scoreboard(
    mut commands: Commands,
    ui: Query<Entity, With<ScoreboardUi>>,
    players: Res<Players>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(scoreboard_entity) = ui.get_single() {
        let font = asset_server.load("fonts/Saira.ttf");
        commands.entity(scoreboard_entity).despawn_descendants();
        for (player, details) in players.iter() {
            // A row for each player
            let scoreline = commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        size: Size::new(Val::Percent(100.), Val::Auto),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            top: Val::Px(0.),
                            ..default()
                        },
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .insert(Scoreline)
                .insert(*player)
                .with_children(|parent| {
                    // Player details
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                flex_grow: 1.,
                                flex_basis: Val::Px(0.),
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Player color
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(20.), Val::Px(20.)),
                                        margin: UiRect::all(Val::Px(5.)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    color: details.color.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(ImageBundle {
                                        image: asset_server.load("textures/dead.png").into(),
                                        style: Style {
                                            size: Size::new(Val::Percent(90.), Val::Percent(90.)),
                                            ..default()
                                        },
                                        color: Color::NONE.into(), // Hidden by default (via 100% opacity)
                                        ..default()
                                    });
                                });
                            // Player name
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style { ..default() },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle::from_section(
                                        details.name.to_string(),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 20.,
                                            color: Color::BLACK,
                                        },
                                    ));
                                });
                        });

                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                flex_grow: 2.,
                                flex_basis: Val::Px(0.),
                                flex_shrink: 0.,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // This is an ugly hack to make the code smaller
                            let stats = [
                                format!(
                                    "{}/{}",
                                    details.score.current_length, details.score.max_length
                                ),
                                details.score.kills.to_string(),
                                details.score.deaths.to_string(),
                            ];
                            for stat in stats {
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            flex_grow: 1.,
                                            flex_basis: Val::Px(0.),
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        },
                                        color: Color::NONE.into(),
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn_bundle(
                                            TextBundle::from_section(
                                                stat,
                                                TextStyle {
                                                    font: font.clone(),
                                                    font_size: 20.,
                                                    color: Color::BLACK,
                                                },
                                            )
                                            .with_style(Style {
                                                flex_basis: Val::Px(0.),
                                                flex_grow: 0.,
                                                ..default()
                                            }),
                                        );
                                    });
                            }
                        });
                })
                .id();

            commands.entity(scoreboard_entity).add_child(scoreline);
        }
    }
}

fn update_scoreboard(
    mut commands: Commands,
    scorelines: Query<(Entity, &Style, &Node, &Children, &PlayerId), With<Scoreline>>,
    mut text: Query<&mut Text, With<Parent>>,
    icons: Query<&UiColor, (With<UiImage>, With<Parent>)>,
    all_children: Query<&Children>,
    players: Res<Players>,
) {
    let mut scorelines = scorelines.iter().collect::<Vec<_>>();
    scorelines.sort_by(|(_, _, _, _, a), (_, _, _, _, b)| {
        let player_a = players.get(*a).unwrap();
        let player_b = players.get(*b).unwrap();
        player_a
            .score
            .cmp(&player_b.score)
            .then_with(|| (*b).cmp(*b))
            .reverse()
    });

    for (rank, (entity, style, node, children, player_id)) in scorelines.into_iter().enumerate() {
        let new_y = Val::Px(node.size.y * rank as f32);

        let new_style = Style {
            position: UiRect {
                top: new_y,
                ..style.position
            },
            ..*style
        };
        commands.entity(entity).insert(style.clone().ease_to(
            new_style,
            EaseFunction::QuadraticOut,
            EasingType::Once {
                duration: Duration::from_millis(100),
            },
        ));

        // The children of the scoreline are [player details, stats]
        if let Some(details) = children.get(0) {
            // children of the details are [player color, player name]
            if let Ok(detail_children) = all_children.get(*details) {
                if let Some(color) = detail_children.get(0) {
                    let color_children = all_children.get(*color).unwrap();
                    if let Some(dead_icon) = color_children.get(0) {
                        let color = icons.get(*dead_icon).unwrap();
                        let new_color = if players.get(player_id).unwrap().is_dead {
                            Color::WHITE
                        } else {
                            Color::NONE
                        };

                        commands.entity(*dead_icon).insert(color.ease_to(
                            UiColor(new_color),
                            EaseFunction::QuadraticInOut,
                            EasingType::Once {
                                duration: Duration::from_millis(75),
                            },
                        ));
                    }
                }
            }
        }

        if let Some(stats) = children.get(1) {
            // the children of the stats are [length, kills, deaths]
            if let Ok(stat_children) = all_children.get(*stats) {
                let score = players
                    .get(player_id)
                    .expect("Player details not found")
                    .score;

                let &length_wrapper = stat_children.get(0).unwrap();
                let length = all_children.get(length_wrapper).unwrap();
                if let Ok(mut length_text) = text.get_mut(*length.get(0).unwrap()) {
                    length_text.sections[0].value =
                        format!("{:3}/{:3}", score.current_length, score.max_length);
                }

                let &kills_wrapper = stat_children.get(1).unwrap();
                let kills = all_children.get(kills_wrapper).unwrap();
                if let Ok(mut kills_text) = text.get_mut(*kills.get(0).unwrap()) {
                    kills_text.sections[0].value = score.kills.to_string();
                }

                let &deaths_wrapper = stat_children.get(2).unwrap();
                let deaths = all_children.get(deaths_wrapper).unwrap();
                if let Ok(mut deaths_text) = text.get_mut(*deaths.get(0).unwrap()) {
                    deaths_text.sections[0].value = score.deaths.to_string();
                }
            }
        }
    }
}

// Button types
#[derive(Component)]
struct PauseButton;

const BUTTON_NORMAL: Color = Color::rgb(0.35, 0.35, 0.35);
const BUTTON_HOVER: Color = Color::rgb(0.45, 0.45, 0.45);
const BUTTON_CLICK: Color = Color::rgb(0.25, 0.25, 0.25);

// A generic run condition that activates when the button type is clicked
fn button_clicked<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}

fn button_interactions(
    mut query: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => *color = UiColor(BUTTON_CLICK),
            Interaction::Hovered => *color = UiColor(BUTTON_HOVER),
            Interaction::None => *color = UiColor(BUTTON_NORMAL),
        }
    }
}

fn toggle_pause(mut commands: Commands, current_state: Res<CurrentState<GameState>>) {
    if current_state.0 == GameState::Running {
        commands.insert_resource(NextState(GameState::Paused));
    } else {
        commands.insert_resource(NextState(GameState::Running));
    }
}

fn pause_button_text(
    pause_buttons: Query<&Children, (With<Button>, With<PauseButton>)>,
    mut text: Query<&mut Text, With<Parent>>,
    current_state: Res<CurrentState<GameState>>,
) {
    for children in pause_buttons.iter() {
        let mut text = text.get_mut(children[0]).unwrap();
        if current_state.0 == GameState::Paused {
            text.sections[0].value = "Resume".to_string();
        } else {
            text.sections[0].value = "Pause".to_string();
        }
    }
}

fn spawn_playback_controls(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: BUTTON_NORMAL.into(),
            ..default()
        })
        .insert(PauseButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    "Pause",
                    TextStyle {
                        font: font.clone(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                ..default()
            });
        });
}

fn add_rendering(app: &mut App) {
    app.add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .label("rendering")
            .with_system(color_food)
            .with_system(draw_grid_objects),
    );
}

fn color_food(mut foods: Query<(&mut Sprite, &mut GridScale, &Food)>) {
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

fn draw_grid_objects(
    arena: Query<(&Node, &GlobalTransform), With<ArenaArea>>,
    mut objects: Query<(&GridPosition, &GridScale, &mut Transform), Without<Node>>,
    grid: Res<GameGrid>,
    windows: Res<Windows>,
) {
    if let Ok((node, transform)) = arena.get_single() {
        if let Some(window) = windows.get_primary() {
            let window_size = Vec2::new(window.width(), window.height());
            let node_size = Vec2::new(node.size.x, node.size.y);

            let cell_size = f32::min(
                node_size.x / grid.width as f32,
                node_size.y / grid.height as f32,
            );
            let cell_offset = 0.5 * cell_size;

            let grid_size = Vec2::new(grid.width as f32, grid.height as f32) * cell_size;
            let grid_offset = transform.translation().xy() - 0.5 * (window_size + grid_size);

            for (pos, scale, mut transform) in objects.iter_mut() {
                // Scale the sprite based on the grid size and window size
                transform.scale = Vec3::new(scale.x * cell_size, scale.y * cell_size, 1.0);

                // Translate the sprite based on the grid size and window size
                let x = pos.x as f32 * cell_size + cell_offset + grid_offset.x;
                let y = pos.y as f32 * cell_size + cell_offset + grid_offset.y;

                transform.translation.x = x;
                transform.translation.y = y;
            }
        }
    }
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
