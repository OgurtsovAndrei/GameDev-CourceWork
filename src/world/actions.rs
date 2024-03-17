use bevy::prelude::*;

pub(crate) struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<ActionsState>()
            .add_systems(Update, (change_action_state))
            .add_systems(OnEnter(ActionsState::SpawningSpaceShips), spawn_spawning_space_ships_window)
            .add_systems(OnExit(ActionsState::SpawningSpaceShips), despawn_spawning_space_ships_window)
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

pub fn spawn_spawning_space_ships_window() {
    println!("SPAWN")
}

pub fn despawn_spawning_space_ships_window() {
    println!("DESPAWN")
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
