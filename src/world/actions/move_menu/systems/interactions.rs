use bevy::prelude::*;
use bevy_editor_pls::egui::Grid;
use hexx::Hex;

use crate::space_ships::{get_count_spaceship_dict, SpaceShip, SpaceSipTextureAtlas};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::world::actions::{ActionsState, get_spaceship_index_by_type, reset_selected_for_move_ships};
use crate::world::actions::move_menu::animations::run_spaceship_moving_animation;
use crate::world::actions::move_menu::components::{CancelButton, EndMoveButton, MoveShipButton, SelectedSpaceshipsText};
use crate::world::fonts_and_styles::colors::*;
use crate::world::player::{Movable, Player};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{HEX_NOWHERE, HexGrid, Planet, SelectedHex};


pub (in crate::world::actions::move_menu) fn update_end_move_button_disabled(
    mut button_query: Query<&mut BackgroundColor, With<EndMoveButton>>,
    grid: ResMut<HexGrid>,
) {
    if let Err(_) = button_query.get_single() {
        return;
    }
    let mut color = button_query.single_mut();
    if color.0 == HOVERED_BUTTON || color.clone().0 == PRESSED_BUTTON { return; }
    if get_selected_ships(&grid).len() == 0 {
        color.0 = DISABLED_BUTTON.clone()
    } else {
        color.0 = NORMAL_BUTTON.clone();
    }
}

pub(in crate::world::actions::move_menu) fn interact_with_end_move_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EndMoveButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<ActionsState>>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
    mut selected_hex: ResMut<SelectedHex>,
    mut grid: ResMut<HexGrid>,
    mut switched_turn: ResMut<NextState<TurnSwitchedState>>,
    handle: Res<SpaceSipTextureAtlas>,
    mut commands: Commands,
) {
    let player = current_player_query.single().clone();
    let hex_under_fight = selected_hex.hex.clone();
    let player2 = grid.planets.get(&hex_under_fight).unwrap().owner;


    for (interaction, mut color) in button_query.iter_mut() {
        if color.0 == DISABLED_BUTTON.into() { return }
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                let army = get_selected_ships(&grid);
                reset_selected_ships(&mut grid);
                for ship in army.iter() {
                    let ship = ship.clone();
                    run_spaceship_moving_animation(ship.ship_type, ship.ship_hex, hex_under_fight.clone(), &grid, &handle, &mut commands)
                }
                for ship in army.iter() {
                    assert_eq!(ship.ship_owner, player.clone())
                }
                let mut planet_under_fight: Planet = grid.planets.remove(&hex_under_fight).unwrap();
                let (winner, winner_army) = perform_fight(player, player2, army, planet_under_fight.owner_army);
                planet_under_fight.owner = winner;
                planet_under_fight.owner_army = winner_army;
                grid.planets.insert(selected_hex.hex, planet_under_fight);
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

fn perform_fight(player1: Player, player2: Player, army1: Vec<SpaceShip>, army2: Vec<SpaceShip>) -> (Player, Vec<SpaceShip>) {
    if army1.len() > army2.len() { return (player1, army1); } else { return (player2, army2); }
}

pub(crate) fn clear_spaceships_selection(mut grid: ResMut<HexGrid>) {
    for (_, planet) in grid.planets.iter_mut() {
        for spaceship in planet.owner_army.iter_mut() {
            if spaceship.is_selected_for_move { println!("Deselected: {:?}", &spaceship.ship_type); }
            spaceship.is_selected_for_move = false
        }
    }
}

pub(self) fn get_selected_ships(grid: &ResMut<HexGrid>) -> Vec<SpaceShip> {
    let mut ships = vec![];
    for (_, planet) in grid.planets.iter() {
        let army = planet.owner_army.clone();
        for mut spaceship in army.into_iter() {
            if spaceship.is_selected_for_move {
                ships.push(spaceship.clone())
            }
        }
    }
    return ships;
}

pub(self) fn reset_selected_ships(grid: &mut ResMut<HexGrid>) {
    for (_, planet) in grid.planets.iter_mut() {
        let mut ships_left = vec![];
        let army = planet.owner_army.clone();
        for mut spaceship in army.into_iter() {
            if !spaceship.is_selected_for_move {
                ships_left.push(spaceship);
            }
        }
        planet.owner_army = ships_left;
    }
}

pub(self) fn get_all_selected_ships(grid: &Res<HexGrid>) -> Vec<SpaceShip> {
    let mut ships = vec![];
    for (_, planet) in grid.planets.iter() {
        let army = planet.owner_army.clone();
        for mut spaceship in army.iter() {
            if spaceship.is_selected_for_move {
                ships.push(spaceship.clone())
            }
        }
    }
    return ships;
}

pub(in crate::world::actions::move_menu) fn interact_with_move_ship_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &MoveShipButton),
        Changed<Interaction>,
    >,
    mut resources: ResMut<GameResources>,
    mut grid: ResMut<HexGrid>,
    mut selected_hex: ResMut<SelectedHex>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    for (interaction, mut color, move_ship_button) in button_query.iter_mut() {
        let player = current_player_query.single();
        let current_hex = &selected_hex.hex_selected_for_move.clone();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                if current_hex.clone() == HEX_NOWHERE { return; }
                if !selected_hex.is_selected_for_move { return; }
                if grid.planets[current_hex].owner != player.clone() { return; }
                {   // Have to insert planet back
                    let mut planet = grid.planets.remove(current_hex).unwrap();
                    let selected_ship_idx = planet.owner_army.iter().position(|space_ship|
                        { !space_ship.is_selected_for_move && space_ship.ship_type == move_ship_button.space_ship_type });
                    if let Some(idx) = selected_ship_idx { planet.owner_army[idx].is_selected_for_move = true; }
                    grid.planets.insert(*current_hex, planet);
                }
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
}

pub(in crate::world::actions::move_menu) fn interact_with_cancel_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CancelButton>)>,
    mut simulation_state_next_state: ResMut<NextState<ActionsState>>,
    mut hex_grid: ResMut<HexGrid>,
) {
    if let Err(_) = button_query.get_single() {
        return;
    }
    let (interaction, mut color) = button_query.single_mut();
    match interaction {
        Interaction::Pressed => {
            *color = PRESSED_BUTTON.into();
            reset_selected_for_move_ships(&mut hex_grid);
            simulation_state_next_state.set(ActionsState::NoActionRunning);
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into();
        }
    }
}

pub(in crate::world::actions::move_menu) fn update_selected_spaceships_text(
    grid: Res<HexGrid>,
    mut text_query: Query<(&mut Text), (With<SelectedSpaceshipsText>)>,
) {
    if let Err(_) = text_query.get_single_mut() {
        return;
    }
    let all_ships = get_all_selected_ships(&grid);
    let dict = get_count_spaceship_dict(all_ships);

    let mut text = text_query.single_mut();

    for (t, count) in dict {
        let index = get_spaceship_index_by_type(t);
        text.sections.get_mut(index).unwrap().value = format!("{count}\n");
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
