use bevy::app::{App, Update};
use bevy::prelude::{Component, IntoSystemConfigs, Plugin, Query, Res, Text};
use hexx::Hex;
use crate::game_state::UpdateUI;
use crate::world::setup_world_grid::HexGrid;

#[derive(Component, Debug, Clone)]
pub struct OwnershipText {
    pub(crate) hex : Hex
}

pub struct OwnershipPlugin;

impl Plugin for OwnershipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ownership_texts.in_set(UpdateUI::RenderStats));
    }
}


fn update_ownership_texts(mut text_query: Query<(&mut Text, &OwnershipText)>,
grid_res: Res<HexGrid>) {
    for (mut text, owner) in text_query.iter_mut() {
        let planet = &grid_res.planets[&owner.hex];
        match planet.owner.id {
            -1 => {
                text.sections[0].value = "Owner: None".to_string();
            }
            id => {
                text.sections[0].value = format!("Owner: Player {}", id)
            }
        }

    }
}