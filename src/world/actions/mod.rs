use bevy::prelude::*;

use crate::world::actions::move_menu::plugin::MoveMenuPlugin;
use crate::world::actions::spawn_menu::plugin::SpawnMenuPlugin;
use crate::world::setup_world_grid::{HexGrid};

pub(crate) mod spawn_menu;

pub(crate) mod move_menu;

pub(crate) struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<ActionsState>()
            .add_plugins(MoveMenuPlugin)
            .add_plugins(SpawnMenuPlugin)
            .add_systems(Update, change_action_state);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub(crate) enum ActionsState {
    #[default]
    NoActionRunning,
    // ChoosingAction,
    SpawningSpaceShips,
    MovingSpaceShips,
}

pub fn clear_action_state(mut commands: Commands) {
    commands.insert_resource(NextState(Some(ActionsState::NoActionRunning)))
}

fn change_action_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<ActionsState>>,
    hex_grid: ResMut<HexGrid>,
) {
    match current_state.get() {
        ActionsState::NoActionRunning => {
            if keyboard_input.just_pressed(KeyCode::S) { commands.insert_resource(NextState(Some(ActionsState::SpawningSpaceShips))) } else if keyboard_input.just_pressed(KeyCode::M) { commands.insert_resource(NextState(Some(ActionsState::MovingSpaceShips))) }
        }
        _ => {
            if keyboard_input.just_pressed(KeyCode::C) {
                reset_selected_ships(hex_grid);
                commands.insert_resource(NextState(Some(ActionsState::NoActionRunning)))
            }
        }
    }
}


pub(self) fn reset_selected_ships(mut hex_grid: ResMut<HexGrid>) {
    hex_grid.planets.iter_mut().for_each(|(_, mut planet)| {
        planet.owner_army.iter_mut().for_each(|ship| ship.is_selected_for_move = false);
    });
}