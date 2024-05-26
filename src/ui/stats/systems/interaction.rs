use bevy::log::info;
use bevy::prelude::{Commands, Entity, NextState, Query, Res, ResMut, State, Text, With};
use hexx::Hex;

use crate::game_state::{AppState, GamePhaseState};
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::ui::stats::components::{MovesLeftText, RoundText, TurnText};
use crate::ui::stats::resources::Round;
use crate::world::player::{INITIAL_MOVES, Movable, Player, Stats};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{HexGrid, Planet};

static CENTRAL_HEX: Hex = Hex { x: 0, y: 0 };
pub(crate) const MAX_WIN_POINTS: i32 = 5;

pub(in crate::ui::stats) fn reset_player(commands: &mut Commands, id: Entity, stats: &mut Stats) {
    stats.moves_left = INITIAL_MOVES;
    commands.entity(id).remove::<Movable>();
}

pub(in crate::ui::stats) fn set_round_number_text(text: &mut Text, value: i32) {
    text.sections[0].value = format!("Round: {}", value.to_string());
}


pub(in crate::ui::stats) fn set_player_turn_text(text: &mut Text, value: i32) {
    text.sections[0].value = format!("Player: {}", value.to_string());
}


pub(in crate::ui::stats) fn update_turn_text(
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
    mut turn_text_query: Query<&mut Text, With<TurnText>>,
) {
    let player = current_player_query.single();
    let mut turn_text = turn_text_query.single_mut();
    set_player_turn_text(&mut turn_text, player.id);
}

pub fn update_win_points_number(
    grid: Res<HexGrid>,
    mut players: Query<(&Player, &mut Stats)>,
    mut game_phase: ResMut<NextState<AppState>>,
) {
    if players.iter().all(|(_, stats)| stats.moves_left == 0) {
        let planet: &Planet = grid.planets.get(&CENTRAL_HEX).unwrap();
        players.iter_mut().for_each(|(player, mut stats)| {
            info!("Before Player: {:?} win_points: {:?}", player.id, stats.win_points);
            if planet.owner.id == player.id {
                stats.win_points += 1;
                if (stats.win_points == MAX_WIN_POINTS) {
                    game_phase.set(AppState::GameOver)
                }
            }
            info!("After Player: {:?} win_points: {:?}", player.id, stats.win_points);
        })
    }
}

pub fn update_round_number_text(
    mut commands: Commands,
    mut round_text_query: Query<&mut Text, With<RoundText>>,
    mut players: Query<(Entity, &Player, &mut Stats)>,
    mut round_res: ResMut<Round>,
    turn_switch_state: Res<State<TurnSwitchedState>>,
    mut turn_switch_state_mutable: ResMut<NextState<TurnSwitchedState>>,
) {
    let mut round_text = round_text_query.single_mut();
    match turn_switch_state.get() {
        TurnSwitchedState::OnTurnSwitched => {
            let round = round_res.as_mut();
            round.number += 1;
            set_round_number_text(&mut round_text, round.number);
            players.iter_mut().for_each(|(entity, player, mut stats)| {
                reset_player(&mut commands, entity, &mut stats);
                if player.id == 1 {
                    commands.entity(entity).insert(Movable);
                }
            });
            turn_switch_state_mutable.set(TurnSwitchedState::OnDefaultState);
        }
        _ => {
        }
    }
}
