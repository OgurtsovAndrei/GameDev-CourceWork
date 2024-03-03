use bevy::app::{App, PluginGroup, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use hexx::*;
use serde::Deserialize;

use game_state::AppState;

use crate::game_state::{change_game_phase, GamePhaseState, toggle_game};
use crate::spire_sheet::remove_grid;

mod hex_grid_withlight;
mod hex_grid;
mod field_of_move;
mod with_picture;
mod spire_sheet;
mod space_ships;
mod game_state;


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
        .add_systems(Startup, (spire_sheet::setup_camera, spire_sheet::setup_grid))
        .add_systems(Startup, (space_ships::spawn_ship))
        .add_systems(Update, spire_sheet::handle_input)
        .add_systems(Update, toggle_game)
        .add_systems(Update, change_game_phase)
        .add_systems(Update, remove_grid)
        .add_systems(Update, zoom_in)
        .run()
}

pub fn zoom_in(
    mut query: Query<&mut OrthographicProjection,
        With<Camera>>, time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut projection in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Equals) {
            projection.scale -= 0.25 * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Minus) {
            projection.scale += 0.25 * time.delta_seconds();
        }
        println!("Current zoom scale: {}", projection.scale);
    }
}