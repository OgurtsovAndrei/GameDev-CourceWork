use bevy::app::Startup;
use bevy::prelude::{Plugin};
use crate::ui::bottom_panel::systems::layout::setup_bottom_panel_buttons;

pub struct BottomPanelPlugin;

impl Plugin for BottomPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_bottom_panel_buttons);
    }
}