use bevy::prelude::{Button, Changed, Interaction, NextState, Query, ResMut, With};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::ui::bottom_panel::components::NextMoveButton;
use crate::world::player::{Movable, Player, Stats};

pub fn handle_finish_moves_in_round_button_click(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<NextMoveButton>),
    >,
    mut current_player_query: Query<&mut Stats, (With<Player>, With<Movable>)>,
    mut move_done_state: ResMut<NextState<TurnSwitchedState>>
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }
    let interaction = interaction_query.single();
    let mut current_stats = current_player_query.single_mut();
    match *interaction {
        Interaction::Pressed => {
            current_stats.moves_left = 0;
            move_done_state.set(TurnSwitchedState::OnTurnSwitched)
        }
        _ => {}
    }
}
