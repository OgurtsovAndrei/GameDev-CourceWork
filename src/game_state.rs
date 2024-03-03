use bevy::input::Input;
use bevy::prelude::{Commands, KeyCode, NextState, Res, State, States};

#[derive(States, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum GamePhaseState {
    #[default]
    NewTurnPhase,
    StrategiesPhase,
    TacticPhase,
}

#[derive(States, Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenuState,
    GamePhaseState,
    GameOver,
}

pub fn toggle_game(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match current_state.get() {
            AppState::MainMenuState => { commands.insert_resource(NextState(Some(AppState::GamePhaseState))) }
            AppState::GamePhaseState => { commands.insert_resource(NextState(Some(AppState::MainMenuState))) }
            AppState::GameOver => { commands.insert_resource(NextState(Some(AppState::MainMenuState))) }
        }
    }
}


pub fn change_game_phase(
    mut commands: Commands,
    current_state: Res<State<GamePhaseState>>,
    current_app_state: Res<State<AppState>>,
) {
    if current_app_state.get() == &AppState::GamePhaseState {
        match current_state.get() {
            GamePhaseState::NewTurnPhase => { commands.insert_resource(NextState(Some(GamePhaseState::StrategiesPhase))) }
            GamePhaseState::StrategiesPhase => { commands.insert_resource(NextState(Some(GamePhaseState::TacticPhase))) }
            GamePhaseState::TacticPhase => { commands.insert_resource(NextState(Some(GamePhaseState::NewTurnPhase))) }
        }
    }
}