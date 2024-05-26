use std::collections::HashMap;
use std::fmt;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use glam::vec2;
use hexx::Hex;
use rand::Rng;

use crate::space_ships::SpaceShipType::{Battleship, Carrier, Destroyer, Fighter, Frigate};
use crate::world::player::Player;
use crate::world::resources::PlayerResources;
use crate::world::setup_world_grid::HEX_NOWHERE;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub(crate) enum SpaceShipType {
    Carrier,
    Destroyer,
    Frigate,
    Battleship,
    Fighter,
}


impl fmt::Display for SpaceShipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Carrier => { "Carrier" }
            Destroyer => { "Destroyer" }
            Frigate => { "Frigate" }
            Battleship => { "Battleship" }
            Fighter => { "Fighter" }
        }
        )
    }
}

pub(crate) fn get_random_spaceship() -> SpaceShipType {
    let n = rand::thread_rng().gen_range(0..5);
    let all_space_ships = vec![Carrier, Destroyer, Frigate, Battleship, Fighter];
    let spaceship_vec: Vec<SpaceShipType> = all_space_ships;
    return spaceship_vec[n];
}

pub(crate) fn get_count_spaceship_dict(spaceship_list: Vec<SpaceShip>) -> HashMap<SpaceShipType, usize> {
    let mut dict = HashMap::from(
        [
            (Carrier, 0),
            (Destroyer, 0),
            (Frigate, 0),
            (Battleship, 0),
            (Fighter, 0)
        ]);

    spaceship_list.iter().for_each(|spaceship| {
        let t = spaceship.ship_type;
        let value = dict[&t];
        dict.insert(t, value + 1);
    });

    dict
}

pub struct SpaceShipCharacteristics {
    pub id: usize,
    pub power: u32,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Component)]
pub(crate) struct SpaceShip {
    pub ship_type: SpaceShipType,
    pub ship_owner: Player,
    pub ship_hex: Hex,
    pub is_selected_for_move: bool,
    pub is_selected_for_buy: bool,
}

impl SpaceShip {
    fn new(ship_type: SpaceShipType) -> Self {
        Self {
            ship_type,
            ship_owner: Player { id: -1 },
            ship_hex: HEX_NOWHERE,
            is_selected_for_move: false,
            is_selected_for_buy: false,
        }
    }

    pub(crate) fn get_price(&self) -> PlayerResources {
        match self.ship_type {
            Carrier => PlayerResources { influence: 0, resources: 5 },
            Destroyer => PlayerResources { influence: 0, resources: 5 },
            Frigate => PlayerResources { influence: 0, resources: 5 },
            Battleship => PlayerResources { influence: 0, resources: 5 },
            Fighter => PlayerResources { influence: 0, resources: 5 },
        }
    }
}

impl SpaceShip {
    fn format(&self) -> String {
        format!("{:?}", self.ship_type)
    }
}

impl fmt::Debug for SpaceShip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl SpaceShipCharacteristics {
    pub fn get_by_spaceship_type(ship_type: SpaceShipType) -> Self {
        match ship_type {
            Carrier => SpaceShipCharacteristics { id: 6, power: 2 },
            Fighter => SpaceShipCharacteristics { id: 1, power: 8 },
            Destroyer => SpaceShipCharacteristics { id: 18, power: 6 },
            Frigate => SpaceShipCharacteristics { id: 11, power: 9 },
            Battleship => SpaceShipCharacteristics { id: 22, power: 5 },
        }
    }
}

/*pub(crate) fn spawn_ship(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = get_spaceship_atlas(&asset_server);
    let (ship_type, sprite) = get_random_sprite();
    let texture = texture_atlas.add(atlas);
    commands.spawn((
        get_spaceship_sprite_bundle_by_type(&texture, ship_type, Transform::from_xyz(300., 300., 0.)),
        On::<Pointer<Click>>::run(move || info!("Spaceship pressed")),
        PickableBundle::default(),
        SpaceShip::new(ship_type),
    ));
}*/

#[derive(Resource)]
pub(crate) struct SpaceSipTextureAtlas {
    pub(crate) spaceship_grid_atlas: Handle<TextureAtlas>,
}

pub(crate) fn get_spaceship_sprite_bundle_by_type(spaceship_grid_texture: &Handle<TextureAtlas>, ship_type: SpaceShipType, transform: Transform) -> SpriteSheetBundle {
    let sprite = get_sprite_by_type(ship_type);
    SpriteSheetBundle {
        texture_atlas: spaceship_grid_texture.clone(),
        sprite,
        transform,
        ..Default::default()
    }
}

const SHIP_SIZE: Vec2 = Vec2::splat(35.0);
const COLUMNS_IN_TEXTURE_FILE: usize = 8;
const ROWS_IN_TEXTURE_FILE: usize = 6;

pub fn get_random_sprite() -> (SpaceShipType, TextureAtlasSprite) {
    let ship_type = get_random_spaceship();
    let sprite = get_sprite_by_type(ship_type);
    return (ship_type, sprite);
}

pub fn get_sprite_by_type(ship_type: SpaceShipType) -> TextureAtlasSprite {
    let id = SpaceShipCharacteristics::get_by_spaceship_type(ship_type).id;
    let sprite = TextureAtlasSprite {
        index: id,
        custom_size: Option::from(SHIP_SIZE),
        anchor: Anchor::CenterRight,
        ..Default::default()
    };
    sprite
}

pub(crate) fn get_spaceship_atlas(asset_server: &Res<AssetServer>) -> TextureAtlas {
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