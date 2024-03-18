use bevy::prelude::*;

use crate::world::actions::move_menu::components::*;
use crate::world::actions::move_menu::components::MoveMenu;
use crate::world::actions::move_menu::styles::*;
use crate::world::fonts_and_styles::colors::*;
use crate::world::fonts_and_styles::fonts::*;

pub fn spawn_move_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Pause Menu");
    build_move_menu(&mut commands, &asset_server);
}

pub(crate) fn despawn_move_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<MoveMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

// System Piping Example
pub(crate) fn build_move_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                style: get_move_menu_style(),
                z_index: ZIndex::Local(1), // See Ref. 1
                ..default()
            },
            MoveMenu {},
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: get_move_menu_container_style(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Move Menu",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Info \nabout \nspaceships \non selected \nplanet",
                                get_info_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // End spawn actions Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            EndMoveButton {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Perform Move",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Main Menu Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MoveShip1Button {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Move Ship1",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                    // Quit Button
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MoveShip2Button {},
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Move Ship2",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });
                });
        })
        .id();

    pause_menu_entity
}

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs
