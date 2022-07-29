use std::ops::{Add, Mul};

use bevy::{
    asset::AssetPlugin,
    core_pipeline::CorePipelinePlugin,
    hierarchy::HierarchyPlugin,
    input::InputPlugin,
    math::{Rect, Vec2, Vec3, Vec3Swizzles},
    prelude::{
        default, App, Color, Commands, CoreStage, OrthographicCameraBundle, Plugin, Query, Res,
        SystemSet, Transform, UiCameraBundle, *,
    },
    render::RenderPlugin,
    sprite::{Sprite, SpriteBundle, SpritePlugin},
    text::TextPlugin,
    transform::TransformPlugin,
    ui::UiPlugin,
    window::{WindowPlugin, Windows},
    winit::WinitPlugin,
};

use crate::game::{
    food::prelude::Food,
    grid::prelude::{GameGrid, GridPosition, GridScale},
    input::prelude::keyboard_moves_system,
    players::prelude::{PlayerId, Players},
    snakes::prelude::Snake,
    turns::prelude::TurnStage,
};

pub struct HeadfulPlugin;
impl Plugin for HeadfulPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
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
            .add_plugin(UiPlugin::default());

        #[cfg(debug_assertions)]
        {
            use bevy_inspector_egui::widgets::InspectorQuery;

            type RootUINode = InspectorQuery<Entity, (With<Node>, Without<Parent>)>;

            app.add_plugin(bevy_inspector_egui::InspectorPlugin::<RootUINode>::new());
        }
        // Add everything related to displaying the game
        add_rendering(app);

        // Add everything related to the interface
        add_ui(app);

        app.add_startup_system(setup_cameras);
        app.add_system_to_stage(TurnStage::PostSimulate, scoreboard_system);
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
    app.add_startup_system(setup_ui);
}

#[derive(Component)]
struct ArenaArea;

#[derive(Component)]
struct ScoreboardUi;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let saira = asset_server.load("fonts/Saira.ttf");
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
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Sidebar title
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.), Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Snakes!",
                                    TextStyle {
                                        font: saira.clone(),
                                        font_size: 32.,
                                        color: Color::BLACK,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..default()
                            });
                        });

                    // Scoreboard
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                margin: Rect::all(Val::Px(10.)),
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // Header
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.), Val::Auto),
                                        flex_direction: FlexDirection::Row,
                                        flex_basis: Val::Px(0.),
                                        justify_content: JustifyContent::FlexEnd,
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // Header: Stats
                                    parent
                                        .spawn_bundle(NodeBundle {
                                            style: Style {
                                                justify_content: JustifyContent::SpaceEvenly,
                                                flex_basis: Val::Percent(50.0),
                                                ..default()
                                            },
                                            color: Color::NONE.into(),
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn_bundle(NodeBundle {
                                                    style: Style {
                                                        flex_basis: Val::Px(0.),
                                                        flex_grow: 1.,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    color: Color::NONE.into(),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn_bundle(TextBundle {
                                                        text: Text::with_section(
                                                            "Length",
                                                            TextStyle {
                                                                font: saira.clone(),
                                                                font_size: 20.,
                                                                color: Color::BLACK,
                                                            },
                                                            TextAlignment {
                                                                vertical: VerticalAlign::Center,
                                                                horizontal: HorizontalAlign::Center,
                                                            },
                                                        ),
                                                        style: Style {
                                                            flex_basis: Val::Px(0.),
                                                            flex_grow: 0.,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    });
                                                });
                                            parent
                                                .spawn_bundle(NodeBundle {
                                                    style: Style {
                                                        flex_basis: Val::Px(0.),
                                                        flex_grow: 1.,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    color: Color::NONE.into(),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn_bundle(TextBundle {
                                                        text: Text::with_section(
                                                            "Kills",
                                                            TextStyle {
                                                                font: saira.clone(),
                                                                font_size: 20.,
                                                                color: Color::BLACK,
                                                            },
                                                            TextAlignment {
                                                                vertical: VerticalAlign::Center,
                                                                horizontal: HorizontalAlign::Center,
                                                            },
                                                        ),
                                                        style: Style {
                                                            flex_basis: Val::Px(0.),
                                                            flex_grow: 0.,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    });
                                                });
                                            parent
                                                .spawn_bundle(NodeBundle {
                                                    style: Style {
                                                        flex_basis: Val::Px(0.),
                                                        flex_grow: 1.,
                                                        justify_content: JustifyContent::Center,
                                                        ..default()
                                                    },
                                                    color: Color::NONE.into(),
                                                    ..default()
                                                })
                                                .with_children(|parent| {
                                                    parent.spawn_bundle(TextBundle {
                                                        text: Text::with_section(
                                                            "Deaths",
                                                            TextStyle {
                                                                font: saira.clone(),
                                                                font_size: 20.,
                                                                color: Color::BLACK,
                                                            },
                                                            TextAlignment {
                                                                vertical: VerticalAlign::Center,
                                                                horizontal: HorizontalAlign::Center,
                                                            },
                                                        ),
                                                        style: Style {
                                                            flex_basis: Val::Px(0.),
                                                            flex_grow: 0.,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    });
                                                });
                                        });
                                });

                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.), Val::Auto),
                                        margin: Rect {
                                            top: Val::Px(15.),
                                            ..default()
                                        },
                                        flex_direction: FlexDirection::Column,
                                        flex_basis: Val::Px(0.),
                                        ..default()
                                    },
                                    color: Color::NONE.into(),
                                    ..default()
                                })
                                .insert(ScoreboardUi);
                        });
                });
        });
}

fn scoreboard_system(
    mut commands: Commands,
    ui: Query<Entity, With<ScoreboardUi>>,
    players: Res<Players>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(scoreboard_entity) = ui.get_single() {
        let saira = asset_server.load("fonts/Saira.ttf");

        // Clear the scoreboard
        commands.entity(scoreboard_entity).despawn_descendants();

        // Sort the players
        let mut players = players.values().collect::<Vec<_>>();
        players.sort_by_key(|p| p.score);

        // Place the players
        for &details in players.iter() {
            let scoreline_bundle = NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Auto),
                    flex_direction: FlexDirection::Row,
                    flex_basis: Val::Px(0.),
                    margin: Rect {
                        bottom: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            };

            let section_bundle = || NodeBundle {
                style: Style {
                    flex_grow: 1.,
                    align_items: AlignItems::Center,
                    flex_basis: Val::Px(0.),
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            };

            let scoreline = commands
                .spawn()
                .insert_bundle(scoreline_bundle)
                .with_children(|parent| {
                    parent
                        .spawn_bundle(section_bundle())
                        .with_children(|parent| {
                            // Player: Color
                            parent.spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(15.), Val::Px(15.)),
                                    padding: Rect::all(Val::Px(10.)),
                                    margin: Rect {
                                        left: Val::Px(5.),
                                        right: Val::Px(5.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                color: details.color.into(),
                                ..default()
                            });

                            // Player: Name
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    details.name.to_string(),
                                    TextStyle {
                                        font: saira.clone(),
                                        font_size: 24.,
                                        color: Color::BLACK,
                                    },
                                    TextAlignment {
                                        vertical: VerticalAlign::Center,
                                        horizontal: HorizontalAlign::Center,
                                    },
                                ),
                                ..default()
                            });
                        });

                    let stat_bundle = || NodeBundle {
                        style: Style {
                            flex_basis: Val::Px(0.),
                            flex_grow: 1.,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        color: Color::NONE.into(),
                        ..default()
                    };

                    let stat_text = |value| TextBundle {
                        text: Text::with_section(
                            value,
                            TextStyle {
                                font: saira.clone(),
                                font_size: 20.,
                                color: Color::BLACK,
                            },
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        style: Style {
                            flex_basis: Val::Px(0.),
                            flex_grow: 0.,
                            ..default()
                        },
                        ..default()
                    };

                    // Player: Stats
                    parent
                        .spawn_bundle(section_bundle())
                        .with_children(|parent| {
                            parent.spawn_bundle(stat_bundle()).with_children(|parent| {
                                parent.spawn_bundle(stat_text(format!(
                                    "{:3}/{:03}",
                                    details.score.current_length, details.score.max_length
                                )));
                            });
                            parent.spawn_bundle(stat_bundle()).with_children(|parent| {
                                parent.spawn_bundle(stat_text(format!("{}", details.score.kills)));
                            });
                            parent.spawn_bundle(stat_bundle()).with_children(|parent| {
                                parent.spawn_bundle(stat_text(format!("{}", details.score.deaths)));
                            });
                        });
                })
                .id();

            commands.entity(scoreboard_entity).add_child(scoreline);
        }
    }
}

fn add_rendering(app: &mut App) {
    app.add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .label("rendering")
            .with_system(color_players)
            .with_system(color_food)
            .with_system(draw_grid_objects),
    );

    #[cfg(debug_assertions)]
    app.add_startup_system(add_debug_grid);
}

fn add_debug_grid(mut commands: Commands, grid: Option<Res<GameGrid>>) {
    if let Some(grid) = grid {
        for x in 0..grid.width {
            for y in 0..grid.height {
                let color = if (x + y) % 2 == 0 {
                    Color::WHITE * 0.1
                } else {
                    Color::WHITE * 0.3
                };
                // let color = Color::WHITE * 0.1;
                commands
                    .spawn_bundle(SpriteBundle {
                        sprite: Sprite { color, ..default() },
                        transform: Transform::from_xyz(0., 0., 0.),
                        ..default()
                    })
                    .insert(GridPosition::new(x as i32, y as i32))
                    .insert(GridScale::square(0.95));
            }
        }
    }
}

fn color_players(
    mut player_objects: Query<(&PlayerId, &mut Sprite, Option<&Snake>)>,
    players: Res<Players>,
) {
    for (player, mut sprite, head) in player_objects.iter_mut() {
        if let Some(details) = players.get(player) {
            sprite.color = details.color;
            if head.is_none() {
                sprite.color *= 0.6;
            }
        }
    }
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
        let window = windows.get_primary().unwrap();

        let window_size = Vec2::new(window.width(), window.height());
        let node_size = Vec2::new(node.size.x, node.size.y);

        let cell_size = f32::min(
            node_size.x / grid.width as f32,
            node_size.y / grid.height as f32,
        );
        let cell_offset = 0.5 * cell_size;

        let grid_size = Vec2::new(grid.width as f32, grid.height as f32) * cell_size;
        let grid_offset = transform.translation.xy() - 0.5 * (window_size + grid_size);

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

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
