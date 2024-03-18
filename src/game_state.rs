use bevy::app::{Plugin, Update};
use bevy::ecs::schedule::{apply_deferred, IntoSystemConfigs, IntoSystemSetConfigs, SystemSet};
use bevy::input::Input;
use bevy::prelude::{apply_state_transition, Commands, KeyCode, NextState, Res, State, States};
use crate::ui::actions_panel::plugin::TurnSwitchedState;

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

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.configure_sets(
            Update,
            (
                UpdateUI::NewRound,
                UpdateUI::RenderStats,
                UpdateUI::UserInput,
                UpdateUI::FlipTurn,
            )
                .chain(),
        )
            .add_systems(
                Update,
                apply_deferred
                    .after(UpdateUI::NewRound)
                    .before(UpdateUI::RenderStats),
            )
            .add_systems(
                Update,
                (apply_state_transition::<TurnSwitchedState>)
                    .after(UpdateUI::UserInput)
                    .before(UpdateUI::FlipTurn)
            );
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateUI {
    NewRound,
    RenderStats,
    UserInput,
    FlipTurn,
}

pub fn toggle_game(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match current_state.get() {
            AppState::MainMenuState => {
                commands.insert_resource(NextState(Some(AppState::GamePhaseState)))
            }
            AppState::GamePhaseState => {
                commands.insert_resource(NextState(Some(AppState::MainMenuState)))
            }
            AppState::GameOver => {
                commands.insert_resource(NextState(Some(AppState::MainMenuState)))
            }
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
            GamePhaseState::NewTurnPhase => {
                commands.insert_resource(NextState(Some(GamePhaseState::StrategiesPhase)))
            }
            GamePhaseState::StrategiesPhase => {
                commands.insert_resource(NextState(Some(GamePhaseState::TacticPhase)))
            }
            GamePhaseState::TacticPhase => {
                commands.insert_resource(NextState(Some(GamePhaseState::NewTurnPhase)))
            }
        }
    }
}
