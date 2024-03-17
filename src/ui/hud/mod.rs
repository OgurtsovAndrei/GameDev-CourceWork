use bevy::prelude::*;

use crate::game_state::UpdateUI;
use crate::ui::hud::systems::layout::spawn_hud;
use crate::ui::hud::systems::updates::{update_enemy_text, update_score_text};

mod components;
mod styles;
mod systems;

pub(crate) struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems TODO
            .add_systems(Startup, (spawn_hud))
            // Systems
            .add_systems(Update, (update_score_text, update_enemy_text).in_set(UpdateUI::RenderStats));
        // OnExit Systems
        // .add_system(despawn_hud.in_schedule(OnExit(AppState::Game)));
    }
}
