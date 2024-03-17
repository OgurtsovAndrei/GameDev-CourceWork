use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Commands, Query},
    },
};
use bevy::prelude::{NextState, Res, ResMut, State};

use crate::game_state::UpdateUI;
use crate::ui::left_panel::TurnSwitchedState;

use super::player::{Movable, Player, Stats};

pub struct TurnPlugin;

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, determine_turn.in_set(UpdateUI::FlipTurn));
    }
}

fn determine_turn(
    mut commands: Commands,
    current_player_query: Query<(Entity, &Stats), (With<Player>, With<Movable>)>,
    opposite_player_query: Query<(Entity, &Stats), (With<Player>, Without<Movable>)>,
    turn_switch_state: Res<State<TurnSwitchedState>>,
    mut turn_switch_state_mutable: ResMut<NextState<TurnSwitchedState>>,
) {
    match turn_switch_state.get(){
        TurnSwitchedState::OnTurnSwitched => {
            let (cur_id, cur_stats) = current_player_query.single();
            let (op_id, op_stats) = opposite_player_query.single();
            if op_stats.moves_left > 0 {
                commands.entity(cur_id).remove::<Movable>();
                commands.entity(op_id).insert(Movable);
            } else if op_stats.moves_left == 0 && cur_stats.moves_left > 0 {} else if cur_stats.moves_left == 0 && cur_stats.moves_left == 0 {
                commands.entity(cur_id).remove::<Movable>();
            } else {
                panic!("Should never happen");
            }
            turn_switch_state_mutable.set(TurnSwitchedState::OnDefaultState);
        }
        _ => {
        }
    }
}
