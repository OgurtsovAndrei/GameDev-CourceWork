use bevy::prelude::{Query, Text, Visibility, With};
use crate::ui::game_result::components::{ResultPanel, WinnerText};
use crate::world::player::{Player, Stats};
use crate::ui::stats::systems::interaction::MAX_WIN_POINTS;

pub(in crate::ui::game_result) fn update_game_result(
    mut panel_query: Query<&mut Visibility, With<ResultPanel>>,
    mut winner_text_query: Query<&mut Text, With<WinnerText>>,
    player_query: Query<(&Player, &Stats)>,
) {
    for (player, stats) in player_query.iter() {
        if (stats.win_points == MAX_WIN_POINTS) {
            let mut text = winner_text_query.single_mut();
            text.sections[1].value = player.to_string();
            *panel_query.single_mut() = Visibility::Visible;
        }
    }
}