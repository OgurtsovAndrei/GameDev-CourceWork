use std::collections::HashMap;
use std::sync::Mutex;

use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_mod_picking::prelude::*;
use glam::vec2;
use once_cell::unsync::Lazy;
use rand::Rng;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Component, Debug)]
pub(crate) enum SpaceShipType {
    Carrier,
    Destroyer,
    Frigate,
    Battleship,
    Fighter,
}

pub fn get_random_spaceship() -> SpaceShipType {
    let n = rand::thread_rng().gen_range(0..5);
    let spaceship_vec: Vec<SpaceShipType> = vec![SpaceShipType::Carrier, SpaceShipType::Destroyer, SpaceShipType::Frigate, SpaceShipType::Battleship, SpaceShipType::Fighter];
    return spaceship_vec[n];
}

pub struct SpaceShipCharacteristics {
    id: usize,
    power: u32,
}

impl SpaceShipCharacteristics {
    pub fn get_by_spaceship_type(ship_type: SpaceShipType) -> Self {
        match ship_type {
            SpaceShipType::Carrier => SpaceShipCharacteristics { id: 6, power: 8 },
            SpaceShipType::Fighter => SpaceShipCharacteristics { id: 1, power: 9 },
            SpaceShipType::Destroyer => SpaceShipCharacteristics { id: 18, power: 5 },
            SpaceShipType::Frigate => SpaceShipCharacteristics { id: 11, power: 8 },
            SpaceShipType::Battleship => SpaceShipCharacteristics { id: 22, power: 3 },
        }
    }
}


pub(crate) fn spawn_ship(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = get_spaceship_atlas(&asset_server);
    let (ship_type, sprite) = get_random_sprite();
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            sprite,
            transform: Transform::from_xyz(-300., 300., 0.0),
            ..Default::default()
        },
        On::<Pointer<Click>>::run(move || info!("Spaceship pressed")),
        PickableBundle::default(),
        ship_type,
    ));
}

const SHIP_SIZE: Vec2 = Vec2::splat(35.0);
const COLUMNS_IN_TEXTURE_FILE: usize = 8;
const ROWS_IN_TEXTURE_FILE: usize = 6;

pub fn get_random_sprite() -> (SpaceShipType, TextureAtlasSprite) {
    let ship_type = get_random_spaceship();
    let id = SpaceShipCharacteristics::get_by_spaceship_type(ship_type).id;
    let sprite = TextureAtlasSprite {
        index: id,
        custom_size: Option::from(SHIP_SIZE),
        ..Default::default()
    };
    return (ship_type, sprite);
}

pub fn get_spaceship_atlas(asset_server: &Res<AssetServer>) -> TextureAtlas {
    let texture = asset_server.load("kenney_simple-space/Tilesheet/simpleSpace_tilesheet@2.png");
    let atlas = TextureAtlas::from_grid(
        texture,
        vec2(128., 128.),
        COLUMNS_IN_TEXTURE_FILE,
        ROWS_IN_TEXTURE_FILE,
        Some(vec2(0.0, 0.0)),
        None,
    );
    return atlas;
}