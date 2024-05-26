use std::fmt::Debug;

use bevy::prelude::{BackgroundColor, Button, Changed, Interaction, NextState, Query, Res, ResMut, State, With};
use hexx::{Hex};

use crate::ui::action_panel::components::{HireArmyButton, NextMoveButton, OpenMovePanelButton};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::world::actions::ActionsState;
use crate::world::actions::ActionsState::{MovingSpaceShips, NoActionRunning, SpawningSpaceShips};
use crate::world::fonts_and_styles::colors::{DISABLED_BUTTON, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::world::player::{Movable, Player, Stats};
use crate::world::setup_world_grid::{HexGrid, SelectedHex};

pub fn spawn_menu_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<HireArmyButton>)>,
    current_state: Res<State<ActionsState>>,
    mut mut_current_state: ResMut<NextState<ActionsState>>,
    selected_hex: Res<SelectedHex>,
    grid: Res<HexGrid>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }

    let (interaction, mut color) = interaction_query.single_mut();
    if *current_state.get() != NoActionRunning {
        *color = NORMAL_BUTTON.into();
        return;
    }

    let current_player = current_player_query.single();
    if !selected_hex.is_selected || !is_selected_hex_belongs_to_player(current_player, &grid, &selected_hex.hex) {
        return;
    }

    let new_color;
    match interaction {
        Interaction::Pressed => {
            new_color = PRESSED_BUTTON.into();
            mut_current_state.set(SpawningSpaceShips);
        }
        Interaction::Hovered => {
            new_color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            new_color = NORMAL_BUTTON.into()
        }
    }
    *color = new_color;
}

pub fn update_spawn_button_disabled(
    mut interaction_query: Query<&mut BackgroundColor, (With<Button>, With<HireArmyButton>, )>,
    selected_hex: Res<SelectedHex>,
    grid: Res<HexGrid>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    let mut binding = interaction_query.get_single_mut().unwrap();
    let current_player = current_player_query.single();
    if binding.clone().0 == HOVERED_BUTTON || binding.clone().0 == PRESSED_BUTTON { return; }
    if !selected_hex.is_selected || !is_selected_hex_belongs_to_player(current_player, &grid, &selected_hex.hex) {
        binding.0 = DISABLED_BUTTON.into();
        return;
    } else {
        binding.0 = NORMAL_BUTTON.into();
        return;
    }
}

pub fn update_move_button_disabled(
    mut interaction_query: Query<&mut BackgroundColor, (With<Button>, With<OpenMovePanelButton>, )>,
    selected_hex: Res<SelectedHex>,
    grid: Res<HexGrid>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    let player = current_player_query.single();
    let mut color = interaction_query.get_single_mut().unwrap();
    if color.0 == HOVERED_BUTTON || color.clone().0 == PRESSED_BUTTON { return; }
    if !selected_hex.is_selected || !(is_selected_hex_belongs_to_player(player, &grid, &selected_hex.hex) || is_selected_hex_has_neighbours(player, &grid, &selected_hex.hex)) {
        color.0 = DISABLED_BUTTON.into();
        return;
    } else {
        color.0 = NORMAL_BUTTON.into();
        return;
    }
}

pub fn handle_move_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<OpenMovePanelButton>)>,
    current_state: Res<State<ActionsState>>,
    selected_hex: Res<SelectedHex>,
    grid: Res<HexGrid>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
    mut mut_current_state: ResMut<NextState<ActionsState>>) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }

    let player = current_player_query.single();
    if !selected_hex.is_selected || !(is_selected_hex_belongs_to_player(player, &grid, &selected_hex.hex) || is_selected_hex_has_neighbours(player, &grid, &selected_hex.hex)) {
        return;
    }

    let (interaction, mut color) = interaction_query.single_mut();
    if *current_state.get() != NoActionRunning {
        *color = NORMAL_BUTTON.into();
        return;
    }

    match interaction {
        Interaction::Pressed => {
            *color = PRESSED_BUTTON.into();
            mut_current_state.set(MovingSpaceShips);
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into()
        }
    }
}

pub fn handle_finish_moves_in_round_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<NextMoveButton>),
    >,
    mut current_player_query: Query<&mut Stats, (With<Player>, With<Movable>)>,
    mut move_done_state: ResMut<NextState<TurnSwitchedState>>,
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }
    let (interaction, mut color) = interaction_query.single_mut();
    let mut current_stats = current_player_query.single_mut();
    match *interaction {
        Interaction::Pressed => {
            *color = PRESSED_BUTTON.into();
            current_stats.moves_left = 1;
            move_done_state.set(TurnSwitchedState::OnTurnSwitched)
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into();
        }
    }
}

pub(crate) fn is_selected_hex_belongs_to_player(player: &Player, grid: &HexGrid, selected_hex: &Hex) -> bool {
    grid.planets.get(selected_hex).unwrap().owner == *player
}

pub(crate) fn is_selected_hex_has_neighbours(player: &Player, grid: &HexGrid, selected_hex: &Hex) -> bool {
    Hex::NEIGHBORS_COORDS.iter().any(|delta| {
        let neighbour = *delta + *selected_hex;
        grid.planets.contains_key(&neighbour) && grid.planets.get(&neighbour).unwrap().owner == *player
    }
    )
}