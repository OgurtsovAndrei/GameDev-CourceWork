use bevy::prelude::{BackgroundColor, Button, Changed, Interaction, NextState, Query, Res, ResMut, State, With};
use crate::ui::action_panel::components::{DebugButton, HireArmyButton, OpenMovePanelButton};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::world::actions::ActionsState;
use crate::world::actions::ActionsState::{MovingSpaceShips, NoActionRunning, SpawningSpaceShips};
use crate::world::fonts_and_styles::colors::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::world::player::{Movable, Player, Stats};


pub fn hire_army_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<HireArmyButton>)>,
    current_state: Res<State<ActionsState>>,
    mut mut_current_state: ResMut<NextState<ActionsState>>,
) {
    if let Err(_) = interaction_query.get_single() {
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
            mut_current_state.set(SpawningSpaceShips);
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into()
        }
    }
}

pub fn handle_move_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<OpenMovePanelButton>)>,
    current_state: Res<State<ActionsState>>,
    mut mut_current_state: ResMut<NextState<ActionsState>>) {
    if let Err(_) = interaction_query.get_single() {
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

pub fn handle_dbg_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>, With<DebugButton>)>,
    mut turn_switched_state: ResMut<NextState<TurnSwitchedState>>,
    current_state: Res<State<ActionsState>>
) {
    if let Err(_) = interaction_query.get_single() {
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
            turn_switched_state.set(TurnSwitchedState::OnTurnSwitched);
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into()
        }
    }
}