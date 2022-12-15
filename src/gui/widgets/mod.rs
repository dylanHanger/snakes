use bevy::{
    prelude::{
        default, AssetServer, BuildChildren, ChildBuilder, Color, Commands, Component,
        DespawnRecursiveExt, Entity, Handle, ImageBundle, NodeBundle, Query, Res, TextBundle, With,
    },
    text::{Font, TextStyle},
    ui::{
        AlignItems, FlexDirection, JustifyContent, Overflow, PositionType, Size, Style, UiRect, Val,
    },
};

use crate::game::players::prelude::Players;

use super::Scoreline;

#[derive(Component)]
pub struct Leaderboard;

pub fn spawn_leaderboard(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        // A container for each row in the leaderboard
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                margin: UiRect {
                    top: Val::Px(10.),
                    bottom: Val::Px(10.),
                    ..default()
                },
                padding: UiRect::all(Val::Px(5.)),
                flex_grow: 1.,
                flex_shrink: 1.,
                ..default()
            },
            background_color: Color::rgb(0.3, 0.3, 0.3).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                // A heading row
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        ..default()
                    },
                    #[cfg(debug_assertions)]
                    background_color: Color::ALICE_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    let mut create_field = |text: &str, width: Val| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_basis: width,
                                    flex_grow: 0.,
                                    flex_shrink: 1.,
                                    size: Size::new(width, Val::Auto),
                                    margin: UiRect::all(Val::Px(5.)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    overflow: Overflow::Hidden,
                                    ..default()
                                },
                                #[cfg(debug_assertions)]
                                background_color: Color::ANTIQUE_WHITE.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(
                                    TextBundle::from_section(
                                        text,
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
                    };

                    // Player
                    create_field("Player", Val::Percent(50.));
                    // Length
                    create_field("Length", Val::Percent(50. / 3.));
                    // Kills
                    create_field("Kills", Val::Percent(50. / 3.));
                    // Deaths
                    create_field("Deaths", Val::Percent(50. / 3.));
                });

            parent
                // A wrapper around the player rows
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        flex_grow: 1.,
                        ..default()
                    },
                    #[cfg(debug_assertions)]
                    background_color: Color::AQUAMARINE.into(),
                    ..default()
                })
                // Add the leaderboard component to the wrapper so we can rearrange its children
                .insert(Leaderboard);
        });
}

pub fn init_leaderboard(
    mut commands: Commands,
    ui: Query<Entity, With<Leaderboard>>,
    players: Res<Players>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(leaderboard) = ui.get_single() {
        let font: Handle<Font> = asset_server.load("fonts/Saira.ttf");

        // Wipe the board
        commands.entity(leaderboard).despawn_descendants();
        // Populate it with players
        for (player_id, player) in players.iter() {
            // A row for each player
            let row = commands
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceAround,
                        // position_type: PositionType::Absolute,
                        // position: UiRect {
                        //     top: Val::Px(0.),
                        //     ..default()
                        // },
                        ..default()
                    },
                    #[cfg(debug_assertions)]
                    background_color: Color::ALICE_BLUE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Player
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::all(Val::Px(5.)),
                                flex_grow: 0.,
                                flex_shrink: 1.,
                                flex_basis: Val::Percent(50.),
                                size: Size::new(Val::Percent(50.), Val::Auto),
                                overflow: Overflow::Hidden,
                                ..default()
                            },
                            #[cfg(debug_assertions)]
                            background_color: Color::DARK_GRAY.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(
                                TextBundle::from_section(
                                    &player.name,
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
                    let mut create_field = |text: &str, width: Val| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::all(Val::Px(5.)),
                                    flex_grow: 0.,
                                    flex_shrink: 1.,
                                    flex_basis: width,
                                    size: Size::new(width, Val::Auto),
                                    overflow: Overflow::Hidden,
                                    ..default()
                                },
                                #[cfg(debug_assertions)]
                                background_color: Color::DARK_GRAY.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(
                                    TextBundle::from_section(
                                        text,
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
                    };
                    // Length
                    create_field("Length", Val::Percent(50. / 3.));
                    // Kills
                    create_field("Kills", Val::Percent(50. / 3.));
                    // Deaths
                    create_field("Deaths", Val::Percent(50. / 3.));
                })
                .insert(Scoreline)
                .insert(*player_id)
                .id();

            commands.entity(leaderboard).add_child(row);
        }
    }
}
