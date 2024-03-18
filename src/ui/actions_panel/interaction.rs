use bevy::prelude::{Button, Changed, Interaction, NextState, Query, ResMut, With};
use crate::ui::actions_panel::components::DebugButton;
use crate::ui::actions_panel::plugin::TurnSwitchedState;
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
