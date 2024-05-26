use bevy::prelude::*;
use bevy::prelude::KeyCode;

use crate::space_ships::{get_count_spaceship_dict, SpaceShip, SpaceSipTextureAtlas};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::world::actions::{ActionsState, get_spaceship_index_by_type, reset_selected_for_move_ships};
use crate::world::actions::move_menu::animations::run_spaceship_moving_animation;
use crate::world::actions::move_menu::components::{CancelButton, EndMoveButton, MoveShipButton, SelectedSpaceshipsText, WinProbabilityText};
use crate::world::actions::move_menu::events::ShipMoved;
use crate::world::actions::move_menu::systems::battle::{move_army_to_planet};
use crate::world::fonts_and_styles::colors::*;
use crate::world::player::{Movable, Player};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{HEX_NOWHERE, HexGrid, Planet, SelectedHex};


const TOTAL_NUMBER_OF_SIMULATIONS: u32 = 100;

pub(in crate::world::actions::move_menu) fn update_end_move_button_disabled(
    mut button_query: Query<&mut BackgroundColor, With<EndMoveButton>>,
    grid: ResMut<HexGrid>,
) {
    if let Err(_) = button_query.get_single() {
        return;
    }
    let mut color = button_query.single_mut();
    if color.0 == HOVERED_BUTTON || color.clone().0 == PRESSED_BUTTON { return; }
    if get_selected_ships(&grid).len() == 0 {
        *color = DISABLED_BUTTON.into();
    } else {
        *color = NORMAL_BUTTON.into();
    }
}

pub(in crate::world::actions::move_menu) fn interact_with_end_move_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EndMoveButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<ActionsState>>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
    selected_hex: Res<SelectedHex>,
    mut grid: ResMut<HexGrid>,
    mut switched_turn: ResMut<NextState<TurnSwitchedState>>,
    handle: Res<SpaceSipTextureAtlas>,
    mut commands: Commands,
) {
    let player = current_player_query.single().clone();
    let hex_under_fight = selected_hex.hex.clone();
    let player2 = grid.planets.get(&hex_under_fight).unwrap().owner;


    for (interaction, mut color) in button_query.iter_mut() {
        if color.0 == DISABLED_BUTTON.into() { return; }
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
                let (winner, winner_army) = move_army_to_planet(player, player2, army, planet_under_fight.owner_army);
                planet_under_fight.owner = winner;
                planet_under_fight.owner_army = winner_army;
                for ship in planet_under_fight.owner_army.iter_mut() {
                    ship.ship_hex = hex_under_fight
                }
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
    keyboard_input: Res<Input<KeyCode>>,
    mut resources: ResMut<GameResources>,
    mut grid: ResMut<HexGrid>,
    selected_hex: Res<SelectedHex>,
    mut event_writer: EventWriter<ShipMoved>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    for (interaction, mut color, move_ship_button) in button_query.iter_mut() {
        let player = current_player_query.single();
        let current_hex = &selected_hex.hex_selected_for_move.clone();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                let all_ships_move = keyboard_input.pressed(KeyCode::Space);
                if current_hex.clone() == HEX_NOWHERE { return; }
                if !selected_hex.is_selected_for_move { return; }
                if grid.planets[current_hex].owner != player.clone() { return; }
                {   // Have to insert planet back
                    let mut planet = grid.planets.remove(current_hex).unwrap();
                    let mut is_new_ship_selected_to_move = false;
                    for ship in &mut planet.owner_army {
                        if ship.ship_type == move_ship_button.space_ship_type && !ship.is_selected_for_move {
                            ship.is_selected_for_move = true;
                            is_new_ship_selected_to_move = true;
                            if !all_ships_move { break; }
                        }
                    }
                    if is_new_ship_selected_to_move {
                        event_writer.send(ShipMoved);
                    }
                    grid.planets.insert(*current_hex, planet);
                }
                resources.set_changed();
                grid.set_changed();
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
        text.sections.get_mut(index).unwrap().value = format!("{count}");
    }
}

pub(in crate::world::actions::move_menu) fn recalculate_probability(
    mut event_reader: EventReader<ShipMoved>,
    current_player_query: Query<&Player, With<Movable>>,
    hex_grid: Res<HexGrid>,
    selected_hex: Res<SelectedHex>,
    mut text_query: Query<&mut Text, With<WinProbabilityText>>,
) {
    if let Err(_) = current_player_query.get_single() { return; }
    let current_player = current_player_query.single();
    if let Err(_) = text_query.get_single() { return; }
    let mut text = text_query.single_mut();
    for _ in event_reader.read() {
        let attack_player_army = get_all_selected_ships(&hex_grid);
        let hex_under_fight = selected_hex.hex.clone();
        let selected_planet = &hex_grid.planets[&hex_under_fight];
        let defense_player = selected_planet.owner.clone();
        let defense_player_army = selected_planet.owner_army.clone();
        let mut number_of_success: u32 = 0;
        for _ in 0..TOTAL_NUMBER_OF_SIMULATIONS {
            let (player, _) = move_army_to_planet(current_player.clone(), defense_player.clone(), attack_player_army.clone(), defense_player_army.clone());
            if player.id == current_player.id {
                number_of_success += 1;
            }
        }
        let probability = ((number_of_success as f64 / TOTAL_NUMBER_OF_SIMULATIONS as f64) * 100.0) as u32;
        text.sections[1].value = format!("{}", probability);
    }
}