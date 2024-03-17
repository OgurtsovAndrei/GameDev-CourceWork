use bevy::prelude::*;

use crate::world::actions::spawn_menu::systems::interactions::interact_with_end_spawn_button;

pub(crate) mod spawn_menu;

pub(crate) struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<ActionsState>()
            .add_systems(Update, (change_action_state))
            .add_systems(OnEnter(ActionsState::SpawningSpaceShips), spawn_menu::spawn_spawning_space_ships_window)
            .add_systems(Update, (
                interact_with_end_spawn_button,
            ).run_if(in_state(ActionsState::SpawningSpaceShips)))
            .add_systems(OnExit(ActionsState::SpawningSpaceShips), spawn_menu::despawn_spawning_space_ships_window)
        ;
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

pub fn change_action_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<ActionsState>>,
) {
    match current_state.get() {
        ActionsState::NoActionRunning => {
            if keyboard_input.just_pressed(KeyCode::S) { commands.insert_resource(NextState(Some(ActionsState::SpawningSpaceShips))) } else if keyboard_input.just_pressed(KeyCode::M) { commands.insert_resource(NextState(Some(ActionsState::MovingSpaceShips))) }
        }
        _ => { if keyboard_input.just_pressed(KeyCode::C) { commands.insert_resource(NextState(Some(ActionsState::NoActionRunning))) } }
    }
}
