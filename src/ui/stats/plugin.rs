use bevy::app::{Plugin, Startup, Update};
use bevy::prelude::IntoSystemConfigs;
use crate::game_state::UpdateUI;
use crate::ui::stats::resources::Round;
use crate::ui::stats::systems::interaction::{update_round_number_text, update_turn_text, update_win_points_number};
use crate::ui::stats::systems::layout::setup_stats;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Round::default())
            .add_systems(Startup, setup_stats)
            .add_systems(Update,
            (
                update_round_number_text,
                update_turn_text,
            )
                .in_set(UpdateUI::RenderStats),
        )
            .add_systems(Update,
                         (update_win_points_number, update_round_number_text).chain().in_set(UpdateUI::NewRound));
    }
}