use bevy::prelude::*;

use crate::space_ships::{get_count_spaceship_dict, SpaceShip, SpaceShipType};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::world::actions::{ActionsState, get_spaceship_index_by_type, reset_selected_for_buy_ships};
use crate::world::actions::spawn_menu::components::{CancelButton, EndSpawnButton, ShipsToBuyText, SpawnShipButton};
use crate::world::fonts_and_styles::colors::*;
use crate::world::player::{Movable, Player};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{HexGrid, SelectedHex};


pub(in crate::world::actions::spawn_menu) fn update_end_spawn_button_disabled(
    mut button_query: Query<&mut BackgroundColor, With<EndSpawnButton>>,
    hex_grid: ResMut<HexGrid>,
) {
    if let Err(_) = button_query.get_single() {
        return;
    }
    let mut color = button_query.single_mut();
    if color.0 == HOVERED_BUTTON.into() || color.0 == PRESSED_BUTTON.into() { return; }
    if hex_grid.planets.iter().any(|(hex, planet)| planet.owner_army.iter().any(|unit| unit.is_selected_for_buy)) {
        *color = NORMAL_BUTTON.into();
    } else {
        *color = DISABLED_BUTTON.into();
    }
}

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
        if color.0 == DISABLED_BUTTON.into() { return; }
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


pub(in crate::world::actions::spawn_menu) fn update_ships_to_buy_text(
    grid: Res<HexGrid>,
    selected_hex: Res<SelectedHex>,
    mut text_query: Query<(&mut Text), (With<ShipsToBuyText>)>,
    game_state: Res<State<ActionsState>>,
) {
    if !selected_hex.is_selected || *game_state != ActionsState::SpawningSpaceShips { return; }

    let ships_to_buy = get_all_ships_to_buy(&grid, &selected_hex);
    let dict = get_count_spaceship_dict(ships_to_buy);

    let mut text = text_query.single_mut();

    for (t, count) in dict {
        let index = get_spaceship_index_by_type(t);
        text.sections.get_mut(index).unwrap().value = format!("{count}\n");
    }
}

fn get_all_ships_to_buy(grid: &Res<HexGrid>, selected_hex: &Res<SelectedHex>) -> Vec<SpaceShip> {
    let planet = &grid.planets[&selected_hex.hex];

    let army = planet.owner_army.clone();
    let mut vec: Vec<SpaceShip> = vec![];
    for mut spaceship in army.iter() {
        if spaceship.is_selected_for_buy {
            vec.push(spaceship.clone())
        }
    }
    vec
}


fn buy_ship(resources: &mut ResMut<GameResources>, grid: &mut ResMut<HexGrid>, selected_hex: &mut ResMut<SelectedHex>, player: &Player, interaction: &Interaction, color: &mut BackgroundColor, space_ship_type: SpaceShipType) -> bool {
    let mut player_resources = &resources.resources[player];
    match *interaction {
        Interaction::Pressed => {
            *color = PRESSED_BUTTON.into();
            if !selected_hex.is_selected { return false; }

            let spaceship = SpaceShip {
                ship_type: space_ship_type,
                ship_owner: player.clone(),
                ship_hex: selected_hex.hex.clone(),
                is_selected_for_move: false,
                is_selected_for_buy: true,
            };
            let price = spaceship.get_price();

            if player_resources.influence < price.influence || player_resources.resources < price.resources { return false; }
            let mut player_resources = resources.resources.remove(player).unwrap();
            player_resources -= price;

            resources.resources.insert(player.clone(), player_resources);
            let mut planet = grid.planets.remove(&selected_hex.hex).unwrap();
            planet.owner_army.push(spaceship);
            grid.planets.insert(selected_hex.hex.clone(), planet);
            resources.set_changed();
            grid.set_changed();
            selected_hex.set_changed();
            true
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
            false
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into();
            false
        }
    }
}

pub(in crate::world::actions::spawn_menu) fn interact_with_spawn_ship_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &SpawnShipButton),
        (Changed<Interaction>),
    >,
    keyboard_input: Res<Input<KeyCode>>, // Include keyboard input for shortcut detection
    mut resources: ResMut<GameResources>,
    mut grid: ResMut<HexGrid>,
    mut selected_hex: ResMut<SelectedHex>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    for (interaction, mut color, spawn_ship_button) in button_query.iter_mut() {
        let player = current_player_query.single();
        if keyboard_input.pressed(KeyCode::Space) {
            while buy_ship(&mut resources, &mut grid, &mut selected_hex, player, interaction, &mut color, spawn_ship_button.space_ship_type) {}
        } else { buy_ship(&mut resources, &mut grid, &mut selected_hex, player, interaction, &mut color, spawn_ship_button.space_ship_type); }
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
