use bevy::prelude::{BackgroundColor, Button, Changed, Interaction, NextState, Query, Res, ResMut, State, With};
use crate::ui::action_panel::components::{DebugButton, OpenMovePanelButton};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::world::actions::ActionsState;
use crate::world::actions::ActionsState::{MovingSpaceShips, NoActionRunning};
use crate::world::fonts_and_styles::colors::{HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};
use crate::world::player::{Movable, Player, Stats};

pub fn handle_dbg_button_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<DebugButton>)>,
    mut current_player_query: Query<&mut Stats, (With<Player>, With<Movable>)>,
    mut turn_switched_state: ResMut<NextState<TurnSwitchedState>>,
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }

    match interaction_query.single() {
        Interaction::Pressed => {
            let mut cur_stats = current_player_query.single_mut();
            cur_stats.moves_left -= 1;
            turn_switched_state.set(TurnSwitchedState::OnTurnSwitched);
        }
        _ => {}
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
    match interaction {
        Interaction::Pressed => {
            if *current_state.get() == NoActionRunning {
                *color = PRESSED_BUTTON.into();
                mut_current_state.set(MovingSpaceShips);
            }
        }
        Interaction::Hovered => {
            *color = HOVERED_BUTTON.into();
        }
        Interaction::None => {
            *color = NORMAL_BUTTON.into()
        }
    }
}