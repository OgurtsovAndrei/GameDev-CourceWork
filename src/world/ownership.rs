use bevy::app::{App, Update};
use bevy::prelude::{Color, Component, IntoSystemConfigs, Plugin, Query, Res};
use bevy::sprite::Sprite;
use hexx::Hex;

use crate::game_state::UpdateUI;
use crate::world::setup_world_grid::HexGrid;

#[derive(Component, Debug, Clone)]
pub struct OwnershipInfo {
    pub(crate) hex: Hex,
}

pub struct OwnershipPlugin;

impl Plugin for OwnershipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ownership_texts.in_set(UpdateUI::RenderStats));
    }
}


fn update_ownership_texts(mut text_query: Query<(&mut Sprite, &OwnershipInfo)>,
                          grid_res: Res<HexGrid>) {
    for (mut sprite, owner) in text_query.iter_mut() {
        let planet = &grid_res.planets[&owner.hex];
        match planet.owner.id {
            -1 => {
                // text.sections[0].value = "Owner: None".to_string();
                sprite.color = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }
            }
            1 => {
                sprite.color = Color::DARK_GREEN
            }
            2 => {
                sprite.color = Color::MAROON
            }
            _ => { panic!() }
        }
    }
}