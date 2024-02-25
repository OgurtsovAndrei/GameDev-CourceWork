use bevy::app::{App, PluginGroup, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_mod_picking::*;
use hexx::*;

use crate::spire_sheet::setup_grid;

mod hex_grid_withlight;
mod hex_grid;
mod field_of_move;
mod with_picture;
mod spire_sheet;
mod space_ships;


fn tut() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_editor_pls::prelude::EditorPlugin::default())
        .run()
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (1_100.0, 1_100.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(bevy_editor_pls::EditorPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, (spire_sheet::setup_camera, setup_grid, space_ships::spawn_ship))
        .run()
}

/*fn main() {
    // hex_grid_withlight::main();
    // hex_grid::main();
    // field_of_move::main();
    // with_picture::main();
    // spire_sheet::main();
    // tut();
}*/

