use bevy::prelude::{Mut, Res, ResMut};
use hexx::Hex;
use crate::world::setup_world_grid::{HexGrid, SelectedHex};

pub(crate) fn determine_winner(selected_hexes: Res<SelectedHex>,
hex_grid: ResMut<HexGrid>) {
    let enemy_hex = selected_hexes.hex;
    // hex_grid.planets
}