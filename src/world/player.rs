use bevy::app::{Plugin, Update};
use bevy::ecs::{
    bundle::Bundle,
    component::Component,
    system::Commands,
};
use bevy::prelude::{Entity, IntoSystemConfigs, NextState, Query, Res, ResMut, State, With, Without};

use crate::game_state::UpdateUI;
use crate::ui::action_panel::plugin::TurnSwitchedState;

pub const INITIAL_MOVES: i32 = 10;

#[derive(Debug, Component, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Player {
    pub id: i32,
}

#[derive(Component)]
pub struct Stats {
    pub moves_left: i32,
}


#[derive(Component, Debug)]
pub struct Movable;


#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    stats: Stats,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player { id: 0 },
            stats: Stats {
                moves_left: INITIAL_MOVES,
            },
        }
    }
}

pub fn spawn_players(mut commands: Commands) {
    commands
        .spawn(PlayerBundle {
            player: Player { id: 1 },
            ..Default::default()
        })
        .insert(Movable);
    commands.spawn(PlayerBundle {
        player: Player { id: 2 },
        ..Default::default()
    });
}

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
    match turn_switch_state.get() {
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
        _ => {}
    }
}
