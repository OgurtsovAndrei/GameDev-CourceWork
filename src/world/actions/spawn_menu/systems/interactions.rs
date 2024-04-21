use bevy::prelude::*;

use crate::space_ships::{SpaceShip, SpaceShipType};
use crate::space_ships::SpaceShipType::{Battleship, Destroyer};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::world::actions::{ActionsState, reset_selected_for_buy_ships};
use crate::world::actions::spawn_menu::components::{CancelButton, EndSpawnButton, SpawnShipButton};
use crate::world::fonts_and_styles::colors::*;
use crate::world::player::{Movable, Player};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{HexGrid, SelectedHex};

pub(in crate::world::actions::spawn_menu) fn interact_with_end_spawn_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EndSpawnButton>),
    >,
    mut hex_grid: ResMut<HexGrid>,
    mut simulation_state_next_state: ResMut<NextState<ActionsState>>,
    mut switched_turn: ResMut<NextState<TurnSwitchedState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                for (_, mut planet) in hex_grid.planets.iter_mut() {
                    planet.owner_army.iter_mut().for_each(|unit| unit.is_selected_for_buy = false)
                }
                simulation_state_next_state.set(ActionsState::NoActionRunning);
                switched_turn.set(TurnSwitchedState::OnTurnSwitched)
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}


fn buy_ship(resources: &mut ResMut<GameResources>, grid: &mut ResMut<HexGrid>, selected_hex: &mut ResMut<SelectedHex>, player: &Player, interaction: &Interaction, color: &mut BackgroundColor, space_ship_type: SpaceShipType) {
    let mut player_resources = &resources.resources[player];
    match *interaction {
        Interaction::Pressed => {
            *color = PRESSED_BUTTON.into();
            if !selected_hex.is_selected { return; }

            let spaceship = SpaceShip {
                ship_type: space_ship_type,
                ship_owner: player.clone(),
                ship_hex: selected_hex.hex.clone(),
                is_selected_for_move: false,
                is_selected_for_buy: true,
            };
            let price = spaceship.get_price();

            if player_resources.influence < price.influence || player_resources.resources < price.resources { return; }
            let mut player_resources = resources.resources.remove(player).unwrap();
            player_resources -= price;

            resources.resources.insert(player.clone(), player_resources);
            let mut planet = grid.planets.remove(&selected_hex.hex).unwrap();
            planet.owner_army.push(spaceship);
            grid.planets.insert(selected_hex.hex.clone(), planet);
            resources.set_changed();
            grid.set_changed();
            selected_hex.set_changed();
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into();
        }
    }
}

pub(in crate::world::actions::spawn_menu) fn interact_with_spawn_ship_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &SpawnShipButton),
        (Changed<Interaction>),
    >,
    mut resources: ResMut<GameResources>,
    mut grid: ResMut<HexGrid>,
    mut selected_hex: ResMut<SelectedHex>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    for (interaction, mut color, spawn_ship_button) in button_query.iter_mut() {
        let player = current_player_query.single();
        buy_ship(&mut resources, &mut grid, &mut selected_hex, player, interaction, &mut color, spawn_ship_button.space_ship_type);
    }
}


pub(in crate::world::actions::spawn_menu) fn interact_cancel_button_click(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CancelButton>)>,
    mut action_state: ResMut<NextState<ActionsState>>,
    mut hex_grid: ResMut<HexGrid>,
    mut game_resources: ResMut<GameResources>,
) {
    if let Err(_) = button_query.get_single() {
        return;
    }
    let (interaction, mut color) = button_query.single_mut();

    match interaction {
        Interaction::Pressed => {
            *color = PRESSED_BUTTON.into();
            reset_selected_for_buy_ships(&mut hex_grid, &mut game_resources);
            action_state.set(ActionsState::NoActionRunning);
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into();
        }
    }
}

/*
pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}*/
