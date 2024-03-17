use bevy::asset::AssetServer;
use bevy::prelude::{Commands, Entity, Query, Res, With};

use crate::world::actions::spawn_menu::components::SpawnMenu;
use crate::world::actions::spawn_menu::systems::layout::{build_pause_menu, despawn_pause_menu};

pub(crate) mod components;
mod styles;
pub(crate) mod systems;

/*
use systems::interactions::*;
use systems::layout::*;

use crate::game::SimulationState;

use bevy::prelude::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter Systems
            .add_system(spawn_pause_menu.in_schedule(OnEnter(SimulationState::Paused)))
            // Systems
            .add_systems(
                (
                    interact_with_resume_button,
                    interact_with_main_menu_button,
                    interact_with_quit_button,
                )
                    .in_set(OnUpdate(SimulationState::Paused)),
            )
            // OnExit Systems
            .add_system(despawn_pause_menu.in_schedule(OnExit(SimulationState::Paused)));
    }
}
*/

pub fn spawn_spawning_space_ships_window(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Spawning Spawn Menu");
    build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_spawning_space_ships_window(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<SpawnMenu>>,
) {
    println!("DeSpawning Spawn Menu");
    despawn_pause_menu(commands, pause_menu_query)
}
