use bevy::prelude::*;
use crate::space_ships::SpaceShipType;
use crate::space_ships::SpaceShipType::{Carrier, Destroyer, Frigate};
use crate::world::actions::get_buy_ship_stats_text;

use crate::world::actions::spawn_menu::components::*;
use crate::world::actions::spawn_menu::components::SpawnMenu;
use crate::world::actions::spawn_menu::styles::*;
use crate::world::fonts_and_styles::colors::*;
use crate::world::fonts_and_styles::fonts::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Pause Menu");
    build_spawn_menu(&mut commands, &asset_server);
}

pub(crate) fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<SpawnMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

// System Piping Example
pub(crate) fn build_spawn_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let pause_menu_entity = commands
        .spawn((
            NodeBundle {
                style: get_spawn_menu_style(),
                z_index: ZIndex::Local(1), // See Ref. 1
                ..default()
            },
            SpawnMenu,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: get_spawn_menu_container_style(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                }, Interaction::None))
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Spawn Menu",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn((get_buy_ship_stats_text(&asset_server), ShipsToBuyText));
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            EndSpawnButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Perform buy",
                                        get_button_text_style(&asset_server),
                                    )],
                                    alignment: TextAlignment::Center,
                                    ..default()
                                },
                                ..default()
                            });
                        });

                    spawn_ship_button(&asset_server, parent, Carrier);
                    spawn_ship_button(&asset_server, parent, Destroyer);
                    spawn_ship_button(&asset_server, parent, Frigate);
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            CancelButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style { ..default() },
                                text: Text {
                                    sections: vec![TextSection::new(
                                        "Cancel",
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

fn spawn_ship_button(asset_server: &&Res<AssetServer>, parent: &mut ChildBuilder, space_ship_type: SpaceShipType) {
    parent
        .spawn((
            ButtonBundle {
                style: get_button_style(),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            SpawnShipButton {
                space_ship_type
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        format!("Spawn {}", space_ship_type),
                        get_button_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        });
}

// References
// 1. UI Z-Index
// https://github.com/bevyengine/bevy/blob/latest/examples/ui/z_index.rs
