use bevy::prelude::Component;
use crate::space_ships::SpaceShipType;

#[derive(Component)]
pub(super) struct MoveMenu;

#[derive(Component)]
pub(super) struct EndMoveButton;

#[derive(Component)]
pub(super) struct MoveShipButton {
    pub(crate) space_ship_type: SpaceShipType,
}


#[derive(Component)]
pub(super) struct CancelButton;


#[derive(Component)]
pub(super) struct SelectedSpaceshipsText;