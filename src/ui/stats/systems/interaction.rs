use bevy::prelude::{Commands, Entity, Query, Res, ResMut, Text, With};
use crate::ui::stats::components::{MovesLeftText, RoundText, TurnText};
use crate::ui::stats::resources::Round;
use crate::world::player::{INITIAL_MOVES, Movable, Player, Stats};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::HexGrid;

pub(in crate::ui::stats) fn reset_player(commands: &mut Commands, id: Entity, stats: &mut Stats) {
    stats.moves_left = INITIAL_MOVES;
    commands.entity(id).remove::<Movable>();
}

pub(in crate::ui::stats) fn set_round_number_text(text: &mut Text, value: i32) {
    text.sections[0].value = format!("Round: {}", value.to_string());
}

pub(in crate::ui::stats) fn set_moves_left_text(text: &mut Text, value: i32) {
    text.sections[0].value = format!("Moves left: {}", value.to_string());
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

pub(in crate::ui::stats) fn update_moves_left_text(
    current_player_query: Query<&Stats, (With<Player>, With<Movable>)>,
    mut moves_left_text_query: Query<&mut Text, With<MovesLeftText>>,
) {
    let stats = current_player_query.single();
    let mut moves_left_text = moves_left_text_query.single_mut();
    set_moves_left_text(&mut moves_left_text, stats.moves_left);
}

pub fn update_round_number_text(
    mut commands: Commands,
    mut round_text_query: Query<&mut Text, With<RoundText>>,
    mut players: Query<(Entity, &Player, &mut Stats)>,
    mut round_res: ResMut<Round>,
    grid: Res<HexGrid>,
    mut game_resources: ResMut<GameResources>,
) {
    let mut round_text = round_text_query.single_mut();

    if players.iter().all(|(_, _, stats)| stats.moves_left == 0) {
        let round = round_res.as_mut();
        round.number += 1;
        set_round_number_text(&mut round_text, round.number);
        players.iter_mut().for_each(|(entity, player, mut stats)| {
            reset_player(&mut commands, entity, &mut stats);
            if player.id == 1 {
                commands.entity(entity).insert(Movable);
            }
        });
        game_resources.update(&grid);
    }
}
