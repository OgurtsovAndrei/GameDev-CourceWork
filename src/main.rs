use bevy::app::{App, PluginGroup, Startup};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_mod_picking::DefaultPickingPlugins;
use hexx::*;
use serde::Deserialize;

use game_state::AppState;

use crate::game_state::{change_game_phase, toggle_game, GamePhaseState};

mod game_state;
mod space_ships;
mod ui;
mod world;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_100.0, 1_100.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_state::<GamePhaseState>()
        // .add_plugins(bevy_editor_pls::EditorPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_systems(
            Startup,
            (
                world::setup_world_grid::setup_camera,
                world::setup_world_grid::setup_grid,
            ),
        )
        .add_systems(Startup, (space_ships::spawn_ship))
        .add_systems(Startup, ui::buttons::setup_buttons)
        .add_systems(Startup, ui::stats::setup_stats)
        .add_systems(Startup, ui::stats::setup_stats)
        .add_systems(Update, world::setup_world_grid::handle_input)
        .add_systems(Update, toggle_game)
        .add_systems(Update, change_game_phase)
        .add_systems(Update, world::setup_world_grid::remove_grid)
        .add_systems(Update, world::navigations_systems::zoom_system)
        .run()
}
