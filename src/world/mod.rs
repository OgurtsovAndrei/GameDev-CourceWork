use bevy::app::{App, Plugin, Startup, Update};

pub mod navigations_systems;
pub mod player;
pub mod setup_world_grid;
pub mod resources;
pub mod turn;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_world_grid::setup_camera, setup_world_grid::setup_grid))
            .add_systems(Startup, setup_world_grid::register_selected_hex)
            .add_systems(Update, (setup_world_grid::handle_click_on_planet));
    }
}
