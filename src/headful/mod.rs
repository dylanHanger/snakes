use std::{
    ops::{Add, Mul},
    time::Duration,
};

use bevy::{
    diagnostic::DiagnosticsPlugin,
    log::LogPlugin,
    math::{Vec2, Vec3, Vec3Swizzles},
    prelude::{default, App, Color, Commands, Plugin, Query, Res, Transform, *},
    render::texture::ImagePlugin,
    sprite::Sprite,
    time::TimePlugin,
    ui::UiRect,
    window::{PrimaryWindow, WindowPlugin, WindowResolution},
};
use bevy_easings::{Ease, EaseFunction, EasingType, EasingsPlugin};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use copypasta::{ClipboardContext, ClipboardProvider};

use crate::game::{
    food::prelude::Food,
    grid::prelude::{GameGrid, GridPosition, GridScale},
    input::prelude::keyboard_moves_system,
    players::prelude::{PlayerId, Players},
    turns::{config::TurnConfig, prelude::Turn},
    GameState, RngSeed,
};

pub struct HeadfulPlugin;
impl Plugin for HeadfulPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)));

        app.add_plugins(
            DefaultPlugins
                .build()
                .disable::<TaskPoolPlugin>()
                .disable::<TypeRegistrationPlugin>()
                .disable::<FrameCountPlugin>()
                .disable::<TimePlugin>()
                .disable::<LogPlugin>()
                .disable::<DiagnosticsPlugin>()
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Snakes".to_string(),
                        resolution: WindowResolution::new(900., 580.),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
        )
        .add_plugin(EasingsPlugin);

        #[cfg(debug_assertions)]
        {
            use bevy_inspector_egui::quick::*;

            type RootUINode = (With<Node>, Without<Parent>);

            app.add_plugin(FilterQueryInspectorPlugin::<RootUINode>::default());
        }

        // Add everything related to displaying the game
        add_rendering(app);

        // Add everything related to the interface
        add_ui(app);

        app.add_startup_system(
            (|mut next_state: ResMut<NextState<GameState>>| {
                next_state.set(GameState::Paused);
            })
            .run_if(|turn_config: Res<TurnConfig>| turn_config.start_paused),
        );

        app.add_startup_system(setup_cameras);
        // Read input from the keyboard
        app.add_system(keyboard_moves_system);
    }
}

fn add_ui(app: &mut App) {
    app.add_startup_system(setup_ui)
        .add_startup_system(init_scoreboard.in_base_set(StartupSet::PostStartup))
        .add_system(update_scoreboard)
        .add_system(update_progress_bar)
        .add_systems((
            button_interactions,
            pause_button_text,
            // Button interactions
            toggle_pause.run_if(button_clicked::<PauseButton>),
            step_once.run_if(button_clicked::<StepButton>),
            copy_seed.run_if(button_clicked::<SeedButton>),
        ));
}

#[derive(Component)]
struct ArenaArea;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, seed: Res<RngSeed>) {
    commands
        // Root node
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Seed info
            parent
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect::bottom(Val::Px(0.)),
                        ..default()
                    },
                    z_index: ZIndex::Global(1),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                position: UiRect::all(Val::Px(0.)),
                                padding: UiRect::all(Val::Px(5.)),
                                size: Size::new(Val::Undefined, Val::Undefined),
                                ..default()
                            },
                            background_color: Color::NONE.into(),
                            ..default()
                        })
                        .insert(SeedButton)
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                seed.0.to_string(),
                                TextStyle {
                                    font: asset_server.load("fonts/Saira.ttf"),
                                    font_size: 20.,
                                    color: Color::GRAY,
                                },
                            ));
                        });
                });
            // Main window
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_grow: 2.,
                        ..default()
                    },
                    ..default()
                })
                .insert(ArenaArea);

            // Sidebar
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        flex_grow: 1.,
                        flex_basis: Val::Px(0.),
                        padding: UiRect {
                            top: Val::Undefined,
                            ..UiRect::all(Val::Px(10.))
                        },
                        ..default()
                    },
                    background_color: Color::rgb(0.5, 0.5, 0.5).into(),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_scoreboard(parent, asset_server.load("fonts/Saira.ttf"));
                    spawn_progress_bar(parent);
                    spawn_playback_controls(parent, asset_server.load("fonts/Saira.ttf"));
                });
        });
}

#[derive(Component)]
struct TurnProgressBar;

fn spawn_progress_bar(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(30.)),
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(2.)),
                margin: UiRect {
                    bottom: Val::Px(10.),
                    ..default()
                },
                ..default()
            },
            background_color: Color::rgb(0.6, 0.6, 0.6).into(),
            ..default()
        })
        .with_children(|parent| {
            // Bar (fills up)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(43.), Val::Percent(100.)),
                        ..default()
                    },
                    background_color: Color::rgb(0.3, 0.3, 0.3).into(),
                    ..default()
                })
                .insert(TurnProgressBar);
        });
}

fn update_progress_bar(
    mut progress_bar: Query<&mut Style, With<TurnProgressBar>>,
    turn: Res<Turn>,
) {
    let progress = 100. * (turn.current as f32 / turn.max as f32);
    if let Ok(mut progress_bar) = progress_bar.get_single_mut() {
        progress_bar.size.width = Val::Percent(progress);
    }
}

#[derive(Component)]
struct ScoreboardUi;
#[derive(Component)]
struct Scoreline;

fn spawn_scoreboard(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.)),
                margin: UiRect {
                    top: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..default()
                },
                flex_shrink: 1.,
                flex_grow: 1.,
                ..default()
            },
            background_color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..default()
        })
        .with_children(|parent| {
            // A heading row with the score labels
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexEnd,
                        size: Size::new(Val::Percent(100.), Val::Auto),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1. / 3.,
                            flex_shrink: 0.,
                            ..default()
                        },
                        ..default()
                    });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::SpaceAround,
                                flex_basis: Val::Percent(2. / 3.),
                                flex_grow: 2. / 3.,
                                flex_shrink: 0.,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for stat in ["Length", "Kills", "Deaths"] {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            flex_grow: 1.,
                                            flex_basis: Val::Px(0.),
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(
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
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        overflow: Overflow::Hidden,
                        ..default()
                    },
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
                .spawn(NodeBundle {
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
                    ..default()
                })
                .insert(Scoreline)
                .insert(*player)
                .with_children(|parent| {
                    // Player details
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                flex_grow: 1.,
                                flex_basis: Val::Px(0.),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            // Player color
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(20.), Val::Px(20.)),
                                        margin: UiRect::all(Val::Px(5.)),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: details.color.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(ImageBundle {
                                        image: asset_server.load("textures/dead.png").into(),
                                        style: Style {
                                            size: Size::new(Val::Percent(90.), Val::Percent(90.)),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                            // Player name
                            parent
                                .spawn(NodeBundle {
                                    style: Style { ..default() },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
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
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::SpaceAround,
                                align_items: AlignItems::Center,
                                flex_grow: 2.,
                                flex_basis: Val::Px(0.),
                                flex_shrink: 0.,
                                ..default()
                            },
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
                                    .spawn(NodeBundle {
                                        style: Style {
                                            flex_grow: 1.,
                                            flex_basis: Val::Px(0.),
                                            justify_content: JustifyContent::Center,
                                            margin: UiRect::all(Val::Px(5.)),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent.spawn(
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
    icons: Query<&BackgroundColor, (With<UiImage>, With<Parent>)>,
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
            .then_with(|| (*b).cmp(*a))
            .reverse()
    });

    for (rank, (entity, style, node, children, player_id)) in scorelines.into_iter().enumerate() {
        let new_y = Val::Px(node.size().y * rank as f32);

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
                            BackgroundColor(new_color),
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
struct UiButton;
#[derive(Component)]
struct SeedButton;
#[derive(Component)]
struct PauseButton;
#[derive(Component)]
struct StepButton;

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
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<UiButton>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match interaction {
            Interaction::Clicked => *color = BackgroundColor(BUTTON_CLICK),
            Interaction::Hovered => *color = BackgroundColor(BUTTON_HOVER),
            Interaction::None => *color = BackgroundColor(BUTTON_NORMAL),
        }
    }
}

fn copy_seed(seed: Res<RngSeed>) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(seed.0.to_owned()).unwrap();
    // TODO: Some kind of feedback that the seed was copied
}

fn step_once(current_state: Res<State<GameState>>, mut next_state: ResMut<NextState<GameState>>) {
    if current_state.0 != GameState::Step {
        next_state.set(GameState::Step);
    }
}

fn toggle_pause(
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if current_state.0 != GameState::Paused {
        next_state.set(GameState::Paused);
    } else {
        next_state.set(GameState::Running)
    }
}

fn pause_button_text(
    pause_buttons: Query<&Children, (With<Button>, With<PauseButton>)>,
    mut text: Query<&mut Text, With<Parent>>,
    current_state: Res<State<GameState>>,
) {
    for children in pause_buttons.iter() {
        let mut text = text.get_mut(children[0]).unwrap();
        if current_state.0 != GameState::Running {
            text.sections[0].value = "Resume".to_string();
        } else {
            text.sections[0].value = "Pause".to_string();
        }
    }
}

fn spawn_playback_controls(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(NodeBundle {
            style: Style { ..default() },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
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
                    background_color: BUTTON_NORMAL.into(),
                    ..default()
                })
                .insert(UiButton)
                .insert(PauseButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
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
            parent
                .spawn(ButtonBundle {
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
                    background_color: BUTTON_NORMAL.into(),
                    ..default()
                })
                .insert(UiButton)
                .insert(StepButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Step",
                            TextStyle {
                                font: font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn add_rendering(app: &mut App) {
    app.add_startup_system(spawn_grid_background)
        .add_systems((color_food, draw_grid_objects).in_base_set(CoreSet::PostUpdate));
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

fn spawn_grid_background(mut commands: Commands, grid: Res<GameGrid>) {
    // I don't like having an entity for every grid square but oh well
    for x in 0..grid.width {
        for y in 0..grid.height {
            let color = Color::rgb(0.05, 0.05, 0.05); // Very dark grey
            commands
                .spawn(SpriteBundle {
                    sprite: Sprite { color, ..default() },
                    ..default()
                })
                .insert(GridScale::square(0.90)) // Almost a full square
                .insert(GridPosition::new(x as i32, y as i32));
        }
    }
}

fn draw_grid_objects(
    arena: Query<(&Node, &GlobalTransform), With<ArenaArea>>,
    mut objects: Query<(&GridPosition, &GridScale, &mut Transform), Without<Node>>,
    grid: Res<GameGrid>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((node, transform)) = arena.get_single() {
        if let Ok(window) = primary_window.get_single() {
            let window_size = Vec2::new(window.width(), window.height());
            let node_size = node.size();

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
    commands.spawn(Camera2dBundle::default());
}
