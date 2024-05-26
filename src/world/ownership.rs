use bevy::app::{App, Update};
use bevy::prelude::{Color, Component, IntoSystemConfigs, OnEnter,  Plugin, Query, Res, ResMut, Text, TextureAtlasSprite, With};
use bevy::sprite::Sprite;
use hexx::Hex;

use crate::game_state::UpdateUI;
use crate::space_ships::SpaceShipType;
use crate::ui::action_panel::plugin::TurnSwitchedState;
use crate::world::player::{Movable, Player};
use crate::world::resources::{GameResources};
use crate::world::setup_world_grid::HexGrid;

#[derive(Component, Debug, Clone)]
pub struct OwnershipInfo {
    pub(crate) hex: Hex,
}

#[derive(Component, Debug, Clone)]
pub struct SpaceShipsInfo {
    pub(crate) hex: Hex,
    pub(crate) space_ship_type: SpaceShipType,
}


pub struct OwnershipPlugin;

impl Plugin for OwnershipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_ownership_texts.in_set(UpdateUI::RenderStats));
        app.add_systems(Update, update_spaceships_texture.in_set(UpdateUI::RenderStats));
        app.add_systems(OnEnter(TurnSwitchedState::OnDefaultState), update_player_resources.in_set(UpdateUI::NewRound));
        app.add_systems(Update, update_spaceships_text_value.in_set(UpdateUI::RenderStats));
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


fn update_spaceships_texture(mut image_query: Query<(&mut TextureAtlasSprite, &SpaceShipsInfo)>,
                             grid_res: Res<HexGrid>) {
    for (mut sprite, owner) in image_query.iter_mut() {
        let planet = &grid_res.planets[&owner.hex];
        match planet.owner.id {
            -1 => {
                // text.sections[0].value = "Owner: None".to_string();
                sprite.color = Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 };
            }
            1 => {
                sprite.color = Color::WHITE;
            }
            2 => {
                sprite.color = Color::WHITE;
            }
            _ => { panic!() }
        }
    }
}

pub(self) fn update_player_resources(player_query: Query<&Player, With<Movable>>,
                           hex_grid: Res<HexGrid>, 
                           mut game_resources: ResMut<GameResources>,) {
    if let Err(_) = player_query.get_single() { return; }
    let player = player_query.single();
    game_resources.update(&hex_grid, player);
}

fn update_spaceships_text_value(mut text_query: Query<(&mut Text, &SpaceShipsInfo)>,
                                grid_res: Res<HexGrid>) {
    for (mut sprite, owner) in text_query.iter_mut() {
        let planet = &grid_res.planets[&owner.hex];
        let count = planet.owner_army.iter().filter(|spaceship| { spaceship.ship_type == owner.space_ship_type && !spaceship.is_selected_for_move && !spaceship.is_selected_for_buy }).count();
        sprite.sections[0].value = format!("{count} - ").to_string();
        sprite.sections[0].style.color = match planet.owner.id {
            -1 => Color::Rgba { red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0 },
            1 => Color::WHITE,
            2 => Color::WHITE,
            _ => panic!(),
        }
    }
}