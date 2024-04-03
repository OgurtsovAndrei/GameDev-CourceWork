use bevy::app::{Plugin, Startup, Update};
use bevy::prelude::{IntoSystemConfigs, OnEnter, States};
use bevy::utils::default;

use crate::game_state::UpdateUI;
use crate::game_state::UpdateUI::FlipTurn;
use crate::ui::action_panel::systems::{interaction, layout};
use crate::ui::action_panel::systems::interaction::handle_finish_moves_in_round_button_click;
use crate::world::setup_world_grid::clear_selected;

pub struct ActionPanelPlugin;

impl Plugin for ActionPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, layout::setup_buttons)
            .add_state::<TurnSwitchedState>()
            .add_systems(Update, (
                interaction::handle_dbg_button_click,
                interaction::handle_move_button_click,
                interaction::hire_army_button_click,
                handle_finish_moves_in_round_button_click,
            ).in_set(UpdateUI::UserInput))
            .add_systems(Update, (
                interaction::update_spawn_button_disabled,
                interaction::update_move_button_disabled,
            ).in_set(UpdateUI::RenderStats))
            .add_systems(OnEnter(TurnSwitchedState::OnTurnSwitched), (clear_selected, crate::world::actions::clear_action_state).in_set(FlipTurn),
            )
        ;
    }
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub(crate) enum TurnSwitchedState {
    #[default]
    OnDefaultState,
    OnTurnSwitched,
}


