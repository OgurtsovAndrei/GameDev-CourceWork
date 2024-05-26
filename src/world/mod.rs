use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::IntoSystemConfigs;
use crate::game_state::UpdateUI;

use crate::world::actions::ActionsPlugin;
use crate::world::button_click::UIClickPlugin;

pub mod navigations_systems;
pub mod player;
pub mod setup_world_grid;
pub mod resources;
pub(crate) mod actions;
pub(crate) mod fonts_and_styles;
pub mod ownership;
mod create_map_layout;
mod button_click;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(ActionsPlugin)
            .add_plugins(UIClickPlugin)
            .add_systems(Startup, (setup_world_grid::setup_camera, setup_world_grid::setup_grid))
            .add_systems(Startup, setup_world_grid::register_selected_hex)
            .add_systems(Update, (button_click::set_ui_clicked, setup_world_grid::handle_click_on_planet).chain().in_set(UpdateUI::FieldSelect));
    }
}
