use bevy::prelude::{Mut, Res, ResMut};
use hexx::Hex;
use crate::world::setup_world_grid::{HexGrid, SelectedHex};

pub(crate) fn determine_winner(selected_hexes: Res<SelectedHex>,
mut hex_grid: ResMut<HexGrid>) {
    let enemy_hex = selected_hexes.hex;
    let mut planet = hex_grid.planets.get_mut(&enemy_hex);
    if let None = planet {
        return;
    }
}