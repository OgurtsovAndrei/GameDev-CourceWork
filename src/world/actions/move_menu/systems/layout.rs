use bevy::prelude::*;
use crate::space_ships::SpaceShipType;
use crate::space_ships::SpaceShipType::{Carrier, Destroyer, Frigate};
use crate::world::actions::{get_ship_stats_text, get_win_probability_text};

use crate::world::actions::move_menu::components::*;
use crate::world::actions::move_menu::components::MoveMenu;
use crate::world::actions::move_menu::styles::*;
use crate::world::fonts_and_styles::colors::*;
use crate::world::fonts_and_styles::fonts::*;
use crate::world::player::{Movable, Player};
use crate::world::setup_world_grid::{HexGrid, SelectedHex};

pub fn spawn_move_menu(selected_hex: Res<SelectedHex>,
                       hex_grid: Res<HexGrid>,
                       current_player_query: Query<&Player, With<Movable>>,
                       commands: Commands,
                       asset_server: Res<AssetServer>) {
    println!("Spawning Pause Menu");
    build_move_menu(selected_hex, hex_grid, current_player_query, commands, &asset_server);
}

pub(crate) fn despawn_move_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<MoveMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

pub(crate) fn build_move_menu(selected_hex: Res<SelectedHex>,
                              hex_grid: Res<HexGrid>,
                              current_player_query: Query<&Player, With<Movable>>, 
                              mut commands: Commands, asset_server: &Res<AssetServer>) -> Entity {
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
                    
                    if let Ok(player) = current_player_query.get_single() {
                        let id = hex_grid.planets[&selected_hex.hex].owner.id;
                        if player.id != id {
                            parent.spawn((get_win_probability_text(asset_server), WinProbabilityText));
                        }
                    }
                    
                    parent.spawn((get_ship_stats_text(asset_server), SelectedSpaceshipsText));
                    parent
                        .spawn((
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            EndMoveButton,
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
                    build_move_space_ship_button(asset_server, parent, Carrier);
                    build_move_space_ship_button(asset_server, parent, Destroyer);
                    build_move_space_ship_button(asset_server, parent, Frigate);
                    // Spawn second ship type button
                    parent
                        .spawn((
                            CancelButton,
                            ButtonBundle {
                                style: get_button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
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

fn build_move_space_ship_button(asset_server: &Res<AssetServer>, parent: &mut ChildBuilder, space_ship_type: SpaceShipType) {
    parent
        .spawn((
            ButtonBundle {
                style: get_button_style(),
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            MoveShipButton {
                space_ship_type
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style { ..default() },
                text: Text {
                    sections: vec![TextSection::new(
                        format!("Move {}", space_ship_type),
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
