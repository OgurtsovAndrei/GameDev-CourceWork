use bevy::app::{App, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin};
use crate::world::actions::{ActionsState, spawn_menu};
use crate::world::actions::spawn_menu::systems::interactions::{interact_cancel_button_click, interact_with_end_spawn_button, interact_with_spawn_ship1_button};

pub(in crate::world::actions) struct SpawnMenuPlugin;


impl Plugin for SpawnMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(ActionsState::SpawningSpaceShips), spawn_menu::spawn_spawning_space_ships_window)
            .add_systems(Update, (
                interact_with_end_spawn_button,
                interact_with_spawn_ship1_button,
                interact_cancel_button_click
            ).run_if(in_state(ActionsState::SpawningSpaceShips)))
            .add_systems(OnExit(ActionsState::SpawningSpaceShips), spawn_menu::despawn_spawning_space_ships_window);
    }
}