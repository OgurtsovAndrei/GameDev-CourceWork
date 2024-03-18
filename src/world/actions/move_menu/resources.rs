use bevy::prelude::{Commands, Resource};

use crate::space_ships::SpaceShip;

#[derive(Debug, Resource)]
pub(crate) struct SelectedForMoveArmy {
    pub(crate) selected_ships: Vec<SpaceShip>,
}

impl Default for SelectedForMoveArmy {
    fn default() -> Self { Self { selected_ships: vec![] } }
}

pub(crate) fn setup_selected_for_move_army(mut commands: Commands) {
    commands.insert_resource(SelectedForMoveArmy::default())
}