use bevy::app::{App, PluginGroup, Startup};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;

use game_state::{AppState, GameStatePlugin};
use world::player;
use world::WorldPlugin;

use crate::game_state::{change_game_phase, GamePhaseState, toggle_game};
use crate::ui::action_panel::plugin::ActionPanelPlugin;
use crate::ui::bottom_panel::plugin::BottomPanelPlugin;
use crate::ui::hud::HudPlugin;
use crate::ui::stats::plugin::StatsPlugin;
use crate::world::ownership::OwnershipPlugin;
use crate::world::player::TurnPlugin;

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
        .add_plugins(OwnershipPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(StatsPlugin)
        .add_plugins(BottomPanelPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(ActionPanelPlugin)
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
