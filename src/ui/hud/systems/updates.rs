use bevy::prelude::*;

use crate::ui::hud::components::{ScoreText, SpaceShipsText};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{HexGrid, SelectedHex};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, game_resources: Res<GameResources>) {
    if game_resources.is_changed() {
        let res = game_resources.clone();
        let resource_text = res.to_string();
        for mut text in text_query.iter_mut() {
            text.sections[1].value = format!("{}", resource_text);
        }
    }
}

pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<SpaceShipsText>>,
    selected_hex: Res<SelectedHex>,
    game_resources: Res<HexGrid>,
) {
    if selected_hex.is_changed() {
        let hex = &selected_hex.hex;
        let is_selected = &selected_hex.is_selected;

        for mut text in text_query.iter_mut() {
            if !is_selected {
                text.sections[1].value = "Nothing to show, hex not selected".to_string();
            } else {
                let planet = game_resources.planets.get(hex).unwrap();
                let mut vec_for_show = vec![];
                for ship in planet.owner_army.iter() { if !ship.is_selected_for_move { vec_for_show.push(ship.clone()) } }
                text.sections[1].value = format!("{:?}", vec_for_show);
            }
        }
    }
}
