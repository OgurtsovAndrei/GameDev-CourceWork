use bevy::app::{App, PluginGroup, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

use game_state::{AppState, GameStatePlugin};
use ui::bottom_panel::BottomPanelPlugin;
use ui::left_panel::LeftPanelPlugin;
use ui::stats::StatsPlugin;
use world::player;
use world::WorldPlugin;
use world::turn::TurnPlugin;

use crate::game_state::{change_game_phase, GamePhaseState, toggle_game};

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
        .add_plugins(StatsPlugin)
        .add_plugins(BottomPanelPlugin)
        .add_plugins(LeftPanelPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(TurnPlugin)
        .add_systems(Startup, (
            player::spawn_players,
            space_ships::spawn_ship,
        ),
        )
        .add_systems(Update, toggle_game)
        .add_systems(Update, change_game_phase)
        .add_systems(Update, world::setup_world_grid::remove_grid)
        .add_systems(
            Update,
            (
                world::navigations_systems::zoom_system,
                world::navigations_systems::move_system,
            ),
        )
        .run()
}
