use bevy::asset::{Assets, AssetServer};
use bevy::input::Input;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy::text::{BreakLineOn, Text2dBounds};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use glam::{vec2, Vec2};
use hexx::{Hex, HexLayout, HexOrientation, shapes};
use rand::Rng;

use crate::space_ships::{get_spaceship_atlas, get_spaceship_sprite_bundle_by_type, SpaceShip, SpaceSipTextureAtlas};
use crate::space_ships::SpaceShipType::{Carrier, Destroyer, Frigate};
use crate::world::actions::ActionsState;
use crate::world::create_map_layout;
use crate::world::ownership::{OwnershipInfo, SpaceShipsInfo};
use crate::world::player::{Movable, Player};
use crate::world::resources::setup_resources;

const HEX_SIZE: Vec2 = Vec2::splat(75.0);
const FILE_GRID_HEIGHT_IN_FILE: usize = 1;
const GRID_WEIGHT_IN_FILE: usize = 6;

const DEFAULT_COLOR: bevy::prelude::Color = Color::WHITE;
const SELECTED_FOR_MOVE_COLOR: bevy::prelude::Color = Color::GREEN;
const SELECTED_COLOR: bevy::prelude::Color = Color::RED;

const RESOURCE_COLOR: bevy::prelude::Color = Color::ORANGE;
const INFLUENCE_COLOR: bevy::prelude::Color = Color::CYAN;

/// 3D Orthogrpahic camera setup
pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


#[derive(Debug, Clone)]
pub struct Planet {
    pub hex: Hex,
    pub resource: u32,
    pub influence: u32,
    pub owner: Player,
    pub owner_army: Vec<SpaceShip>,
}


impl Planet {
    pub(crate) fn new(
        hex: Hex,
        resource: u32,
        influence: u32,
        owner: Player,
        owner_army: Vec<SpaceShip>) -> Self {
        Self {
            hex,
            resource,
            influence,
            owner,
            owner_army,
        }
    }

    pub(crate) fn default(
        hex: Hex,
        resource: u32,
        influence: u32) -> Self {
        Self {
            hex,
            resource,
            influence,
            owner: Player { id: -1 },
            owner_army: vec![],
        }
    }
}

#[derive(Debug, Resource)]
pub struct HexGrid {
    // pub resources: HashMap<>
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
    pub planets: HashMap<Hex, Planet>,
}

pub(crate) fn setup_grid(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture = asset_server.load("images/Simple grid.png");
    let atlas = TextureAtlas::from_grid(
        texture,
        vec2(600.0, 700.0),
        GRID_WEIGHT_IN_FILE,
        FILE_GRID_HEIGHT_IN_FILE,
        Some(vec2(2.0, 2.0)),
        None,
    );

    let atlas = atlases.add(atlas);
    let layout = HexLayout {
        orientation: HexOrientation::Pointy,
        hex_size: HEX_SIZE,
        ..default()
    };
    let sprite_size = layout.rect_size();
    let mut planets: HashMap<Hex, Planet> = HashMap::new();
    let radius = 3;
    let map = create_map_layout::create_setup_field_map_for_radius(radius);

    let spaceship_grid_atlas = get_spaceship_atlas(&asset_server);
    let spaceship_grid_texture: Handle<TextureAtlas> = atlases.add(spaceship_grid_atlas);
    commands.insert_resource(SpaceSipTextureAtlas {
        spaceship_grid_atlas: spaceship_grid_texture.clone()
    });

    let entities = shapes::hexagon(Hex::ZERO, radius)
        .enumerate()
        .map(|(i, coord)| {
            let pos = layout.hex_to_world_pos(coord);
            let index = map[&i]; // i % (FILE_GRID_HEIGHT_IN_FILE * GRID_WEIGHT_IN_FILE);
            let planet = Planet::default(coord, (index + 1) as u32, i as u32);

            let entity = commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        color: DEFAULT_COLOR,
                        index,
                        custom_size: Some(sprite_size),
                        ..default()
                    },
                    texture_atlas: atlas.clone(),
                    transform: Transform {
                        translation: Vec3 { x: pos.x, y: pos.y, z: -10.0 },
                        scale: Vec3::splat(0.99),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                    parent.spawn(create_resource_text_bundle(font.clone(), planet.resource));
                    parent.spawn(create_influence_text_bundle(font.clone(), planet.influence));

                    // parent.spawn((create_ownership_text_bundle(font.clone())/*, OwnershipText { hex: coord }*/));

                    parent.spawn(create_resource_sprite_bundle(&asset_server));
                    parent.spawn(create_influence_sprite_bundle(&asset_server));
                    parent.spawn((get_ownership_frame(&asset_server, sprite_size), OwnershipInfo { hex: coord.clone() }));
                    spawn_space_ship_info_grid(parent, &spaceship_grid_texture, coord.clone(), font.clone())
                })
                .id();
            planets.insert(coord, planet);
            (coord, entity)
        })
        .collect();

    let mut grid = HexGrid { entities, layout, planets };
    setup_resources(&mut commands, &mut grid);
    commands.insert_resource(grid);
}

fn spawn_space_ship_info_grid(parent: &mut ChildBuilder, spaceship_grid_texture: &Handle<TextureAtlas>, hex: Hex, font: Handle<Font>) {
    let all_space_ships = vec![Carrier, Destroyer, Frigate /*Battleship, Fighter*/];

    let spaceships_info_text_style: TextStyle = TextStyle {
        font,
        font_size: 42.0,
        color: Color::WHITE,
    };

    for (id, space_ship_type) in all_space_ships.into_iter().enumerate() {
        let transform = Transform {
            translation: Vec3 { x: 60.0, y: (30.0 - 15.0 * id as f32), z: 1.0 },
            scale: Vec3::splat(0.4),
            ..default()
        };
        let image_sprite = (
            get_spaceship_sprite_bundle_by_type(&spaceship_grid_texture, space_ship_type, transform),
            SpaceShipsInfo { hex: hex.clone(), space_ship_type: space_ship_type.clone() }
        );

        let text_transform = Transform {
            translation: Vec3::new(48., (30.0 - 15.0 * id as f32), 0.5),
            scale: Vec3::splat(0.3),
            ..Default::default()
        };

        let text_sprite = (
            create_text_bundle_with_anchor("".to_string(), spaceships_info_text_style.clone(), text_transform, Anchor::CenterRight),
            SpaceShipsInfo { hex: hex.clone(), space_ship_type: space_ship_type.clone() }
        );


        parent.spawn(image_sprite);
        parent.spawn(text_sprite);
    }
}

fn get_ownership_frame(asset_server: &Res<AssetServer>, sprite_size: Vec2) -> SpriteBundle {
    let image_path = "kenney - Simpe Icons/my_hex.png".to_string();
    let transform = Transform {
        translation: Vec3::new(0., 0., 0.01),
        scale: Vec3::splat(1.003),
        ..Default::default()
    };
    let c = if rand::thread_rng().gen_bool(0.5) { Color::DARK_GREEN } else { Color::MAROON };
    let color = c;
    return SpriteBundle {
        texture: asset_server.load(image_path).clone().into(),
        sprite: Sprite {
            color,
            anchor: Anchor::Center,
            custom_size: Some(sprite_size),
            ..default()
        },
        transform,
        ..default()
    };
}

fn create_resource_sprite_bundle(asset_server: &Res<AssetServer>) -> SpriteBundle {
    let image_path = "kenney - Simpe Icons/resource_planks.png".to_string();
    let transform = Transform {
        translation: Vec3::new(-59., 40., 1.0),
        scale: Vec3::splat(0.1),
        ..Default::default()
    };
    create_sprite_bundle_with_image(asset_server, image_path, transform, RESOURCE_COLOR)
}

fn create_influence_sprite_bundle(asset_server: &Res<AssetServer>) -> SpriteBundle {
    let image_path = "kenney - Simpe Icons/crown_b.png".to_string();
    let transform = Transform {
        translation: Vec3::new(-59., 25., 1.0),
        scale: Vec3::splat(0.1),
        ..Default::default()
    };
    create_sprite_bundle_with_image(asset_server, image_path, transform, INFLUENCE_COLOR)
}

fn create_sprite_bundle_with_image(asset_server: &Res<AssetServer>, image_path: String, transform: Transform, color: Color) -> SpriteBundle {
    (SpriteBundle {
        texture: asset_server.load(image_path).clone().into(),
        sprite: Sprite {
            color,
            anchor: Anchor::TopCenter,
            ..default()
        },
        transform,
        ..default()
    })
}

fn create_text_bundle(
    text: String,
    resource_text_style: TextStyle,
    resource_transform: Transform,
) -> Text2dBundle {
    create_text_bundle_with_anchor(text, resource_text_style, resource_transform, Anchor::TopCenter)
}

fn create_text_bundle_with_anchor(
    text: String,
    resource_text_style: TextStyle,
    resource_transform: Transform,
    anchor: Anchor,
) -> Text2dBundle {
    let box_size = Vec2::new(50.0, 25.0);
    Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(text, resource_text_style.clone())],
            linebreak_behavior: BreakLineOn::NoWrap,
            ..default()
        },
        text_2d_bounds: Text2dBounds { size: box_size },
        transform: resource_transform,
        text_anchor: anchor,
        ..default()
    }
}

fn create_resource_text_bundle(font: Handle<Font>, value: u32) -> Text2dBundle {
    let resource_text_style: TextStyle = TextStyle {
        font,
        font_size: 42.0,
        color: RESOURCE_COLOR,
    };

    let resource_transform = Transform {
        translation: Vec3::new(-45., 40., 0.5),
        scale: Vec3::splat(0.35),
        ..Default::default()
    };
    let text: String = format!("{}", value);
    create_text_bundle(text, resource_text_style, resource_transform)
}

fn create_influence_text_bundle(font: Handle<Font>, value: u32) -> Text2dBundle {
    let resource_text_style: TextStyle = TextStyle {
        font,
        font_size: 42.0,
        color: INFLUENCE_COLOR,
    };

    let resource_transform = Transform {
        translation: Vec3::new(-45., 26., 0.5),
        scale: Vec3::splat(0.35), // Half the original size
        ..Default::default()
    };

    create_text_bundle(
        format!("{}", value),
        resource_text_style,
        resource_transform,
    )
}

fn create_ownership_text_bundle(font: Handle<Font>) -> Text2dBundle {
    let resource_text_style: TextStyle = TextStyle {
        font,
        font_size: 42.0,
        color: Color::GREEN,
    };
    let transform = Transform {
        translation: Vec3::new(0., 0., 0.5),
        scale: Vec3::splat(0.35),
        ..Default::default()
    };
    create_text_bundle(
        format!("Owner: {}", "None"),
        resource_text_style,
        transform,
    )
}

pub(crate) fn remove_grid(
    mut commands: Commands,
    mut entities: Query<Entity, With<SpaceShip>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        println!("{}", 42);
        for entity in &mut entities {
            println!("{}", entity.index());
            commands.entity(entity).despawn()
        }
    }
}

#[derive(Debug, Resource, Clone)]
pub(crate) struct SelectedHex {
    pub hex: Hex,
    pub is_selected: bool,
    pub hex_selected_for_move: Hex,
    pub is_selected_for_move: bool,
}

pub fn clear_selected(
    mut selecred_hex: ResMut<SelectedHex>,
    grid: Res<HexGrid>,
    mut tiles: Query<&mut TextureAtlasSprite>) {
    selecred_hex.is_selected = false;
    selecred_hex.is_selected_for_move = false;
    set_color_to_hex(&grid, &mut tiles, &selecred_hex.hex, &DEFAULT_COLOR);
    set_color_to_hex(&grid, &mut tiles, &selecred_hex.hex_selected_for_move, &DEFAULT_COLOR);
    selecred_hex.hex = HEX_NOWHERE;
    selecred_hex.hex_selected_for_move = HEX_NOWHERE;
}

pub(crate) fn register_selected_hex(mut commands: Commands) {
    let hex = SelectedHex { hex: HEX_NOWHERE, is_selected: false, hex_selected_for_move: HEX_NOWHERE, is_selected_for_move: false };
    commands.insert_resource(hex);
}

pub(crate) const HEX_NOWHERE: Hex = Hex::new(i32::MAX, i32::MAX);

pub(crate) fn clear_move_selected(
    mut selecred_hex: ResMut<SelectedHex>,
    grid: Res<HexGrid>,
    mut tiles: Query<&mut TextureAtlasSprite>)
{
    selecred_hex.is_selected_for_move = false;
    set_color_to_hex(&grid, &mut tiles, &selecred_hex.hex_selected_for_move, &DEFAULT_COLOR);
    selecred_hex.hex_selected_for_move = HEX_NOWHERE;
}

/// Input interaction
pub(crate) fn handle_click_on_planet(
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGrid>,
    mut tiles: Query<&mut TextureAtlasSprite>,
    mut selected_hex: ResMut<SelectedHex>,
    current_state: Res<State<ActionsState>>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        let cur_pos: Hex = grid.layout.world_pos_to_hex(pos);
        if !buttons.just_pressed(MouseButton::Left) { return; }

        if let ActionsState::MovingSpaceShips = current_state.get() {
            if !selected_hex.is_selected || selected_hex.hex == cur_pos { return; }

            if grid.entities.get(&cur_pos).is_none() {
                // prev_pos.is_selected = false;
                // set_color_to_hex(&grid, &mut tiles, &prev_pos.hex, &DEFAULT_COLOR);
                return;
            }

            if current_player_query.get_single().unwrap().clone() != grid.planets.get(&cur_pos).unwrap().owner {
                return;
            };


            if !are_hex_neighbours(&selected_hex.hex, &cur_pos) {
                return;
            }

            if selected_hex.hex_selected_for_move == cur_pos {
                if selected_hex.is_selected_for_move {
                    set_color_to_hex(&grid, &mut tiles, &cur_pos, &DEFAULT_COLOR);
                    selected_hex.is_selected_for_move = false;
                    selected_hex.hex_selected_for_move = HEX_NOWHERE;
                } else {
                    set_color_to_hex(&grid, &mut tiles, &cur_pos, &SELECTED_FOR_MOVE_COLOR);
                    selected_hex.is_selected_for_move = true;
                }
            } else {
                let prv_pos_copy = selected_hex.hex_selected_for_move.clone();
                selected_hex.hex_selected_for_move = cur_pos;
                selected_hex.is_selected_for_move = true;
                set_color_to_hex(&grid, &mut tiles, &prv_pos_copy, &DEFAULT_COLOR);
                set_color_to_hex(&grid, &mut tiles, &cur_pos, &SELECTED_FOR_MOVE_COLOR);
            }

            return;
        }

        if grid.entities.get(&cur_pos).is_none() {
            // prev_pos.is_selected = false;
            // set_color_to_hex(&grid, &mut tiles, &prev_pos.hex, &DEFAULT_COLOR);
            return;
        }

        if selected_hex.hex == cur_pos {
            if selected_hex.is_selected {
                set_color_to_hex(&grid, &mut tiles, &cur_pos, &DEFAULT_COLOR);
                selected_hex.is_selected = false;
                selected_hex.hex = HEX_NOWHERE;
            } else {
                set_color_to_hex(&grid, &mut tiles, &cur_pos, &SELECTED_COLOR);
                selected_hex.is_selected = true;
            }
        } else {
            let prv_pos_copy = selected_hex.hex.clone();
            selected_hex.hex = cur_pos;
            selected_hex.is_selected = true;
            set_color_to_hex(&grid, &mut tiles, &prv_pos_copy, &DEFAULT_COLOR);
            set_color_to_hex(&grid, &mut tiles, &cur_pos, &SELECTED_COLOR);
        }
    }
}

fn are_hex_neighbours(first: &Hex, second: &Hex) -> bool {
    let delta = *first - *second;
    return Hex::NEIGHBORS_COORDS.contains(&delta);
}


fn set_color_to_hex(
    grid: &Res<HexGrid>,
    tiles: &mut Query<&mut TextureAtlasSprite>,
    pos: &Hex,
    color: &Color,
) {
    let Some(cur_entity) = grid.entities.get(pos).copied() else {
        return;
    };

    let Ok(mut cur_sprite) = tiles.get_mut(cur_entity) else {
        return;
    };

    cur_sprite.color = *color;
}

fn handle_clicks(
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        if !buttons.just_pressed(MouseButton::Left) {
            return;
        }
        println!("{}", pos)
    }
}

fn cursor_enter_window(mut events: EventReader<CursorEntered>) {
    for event in events.read() {
        println!("Enter window {:?}", event)
    }
}

fn cursor_ddd(mut events: EventReader<CursorMoved>) {
    for event in events.read() {
        println!("Enter window {:?}", event)
    }
}

fn cursor_click(mut events: EventReader<MouseButtonInput>) {
    for event in events.read() {
        println!("Click happens: {:?}", event)
    }
}

fn cursor_position(q_windows: Query<&Window, With<PrimaryWindow>>) {
    // Games typically only have one window (the primary window)
    if let Some(position) = q_windows.single().cursor_position() {
        println!("Cursor is inside the primary window, at {:?}", position);
    } else {
        println!("Cursor is not in the game window.");
    }
}
