use bevy::app::{Startup, Update};
use bevy::prelude::{IntoSystemConfigs, Plugin};
use crate::game_state::UpdateUI;
use crate::ui::bottom_panel::systems::interaction::handle_finish_moves_in_round_button_click;
use crate::ui::bottom_panel::systems::layout::setup_bottom_panel_buttons;

pub struct BottomPanelPlugin;

impl Plugin for BottomPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_bottom_panel_buttons)
            .add_systems(
                Update,
                (handle_finish_moves_in_round_button_click).in_set(UpdateUI::UserInput),
            );
    }
}