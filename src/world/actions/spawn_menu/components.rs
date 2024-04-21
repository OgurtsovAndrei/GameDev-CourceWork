use bevy::prelude::Component;
use crate::space_ships::SpaceShipType;

#[derive(Component)]
pub(super) struct SpawnMenu;

#[derive(Component)]
pub(super) struct EndSpawnButton;

#[derive(Component)]
pub(super) struct ShipsToBuyText;

#[derive(Component)]
pub(super) struct SpawnShipButton {
    pub(crate) space_ship_type: SpaceShipType
}

#[derive(Component)]
pub(super) struct CancelButton;