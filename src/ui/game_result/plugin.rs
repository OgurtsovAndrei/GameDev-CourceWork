use bevy::app::{App, Startup};
use bevy::prelude::{OnEnter, Plugin};
use crate::game_state::AppState;
use crate::ui::game_result::systems::interaction::update_game_result;
use crate::ui::game_result::systems::layout::setup_result_panel;

pub struct GameResultPlugin;

impl Plugin for GameResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_result_panel)
            .add_systems(OnEnter(AppState::GameOver), update_game_result);
    }
}