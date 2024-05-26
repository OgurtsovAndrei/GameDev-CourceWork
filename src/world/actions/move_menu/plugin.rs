use bevy::app::{App, Startup, Update};
use bevy::prelude::{in_state, IntoSystemConfigs, OnEnter, OnExit, Plugin};
use crate::game_state::UpdateUI;
use crate::world::actions::{ActionsState, move_menu};
use crate::world::actions::move_menu::despawn_move_space_ships_window;
use crate::world::actions::move_menu::events::ShipMoved;
use crate::world::actions::move_menu::resources::setup_selected_for_move_army;
use crate::world::actions::move_menu::systems::interactions::{clear_spaceships_selection, interact_with_cancel_button, interact_with_end_move_button, interact_with_move_ship_button, recalculate_probability, update_end_move_button_disabled, update_selected_spaceships_text};
use crate::world::setup_world_grid::clear_move_selected;

pub(in crate::world::actions) struct MoveMenuPlugin;

impl Plugin for MoveMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(ActionsState::MovingSpaceShips), move_menu::spawn_move_space_ships_window)
            .add_systems(Update,
                         update_selected_spaceships_text
                             .in_set(UpdateUI::RenderStats).run_if(in_state(ActionsState::MovingSpaceShips)))
            .add_systems(Update, (
                interact_with_end_move_button,
                interact_with_move_ship_button,
                interact_with_cancel_button,
                update_end_move_button_disabled,
                recalculate_probability
            ).run_if(in_state(ActionsState::MovingSpaceShips)))
            .add_systems(OnExit(ActionsState::MovingSpaceShips), (
                despawn_move_space_ships_window,
                clear_move_selected,
                clear_spaceships_selection
            ))
            .add_systems(Startup, setup_selected_for_move_army)
            .add_event::<ShipMoved>();
    }
}