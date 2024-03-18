use bevy::asset::{Assets, AssetServer};
use bevy::input::Input;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::text::{BreakLineOn, Text2dBounds};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use glam::{vec2, Vec2};
use hexx::{Hex, HexLayout, HexOrientation, shapes};

use crate::space_ships::SpaceShip;
use crate::world::player::Player;
use crate::world::resources::setup_resources;

const HEX_SIZE: Vec2 = Vec2::splat(75.0);
const FILE_GRID_HEIGHT_IN_FILE: usize = 1;
const GRID_WEIGHT_IN_FILE: usize = 6;

const DEFAULT_COLOR: bevy::prelude::Color = Color::WHITE;
const SELECTED_COLOR: bevy::prelude::Color = Color::RED;

/// 3D Orthogrpahic camera setup
pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Component, Clone)]
pub struct Planet {
    pub index_in_grid: usize,
    pub resource: u32,
    pub influence: u32,
    pub owner: Player,
    pub owner_army: Vec<SpaceShip>,
}

impl Planet {
    pub(crate) fn new(
        index_in_grid: usize,
        resource: u32,
        influence: u32,
        owner: Player,
        owner_army: Vec<SpaceShip>) -> Self {
        Self {
            index_in_grid,
            resource,
            influence,
            owner,
            owner_army,
        }
    }

    pub(crate) fn default(
        index_in_grid: usize,
        resource: u32,
        influence: u32) -> Self {
        Self {
            index_in_grid,
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
    let entities = shapes::hexagon(Hex::ZERO, 2)
        .enumerate()
        .map(|(i, coord)| {
            let pos = layout.hex_to_world_pos(coord);
            let index = i % (FILE_GRID_HEIGHT_IN_FILE * GRID_WEIGHT_IN_FILE);
            let planet = Planet::default(i, (index + 1) as u32, (5 - index) as u32);

            let entity = commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        color: DEFAULT_COLOR,
                        index,
                        custom_size: Some(sprite_size),
                        ..default()
                    },
                    texture_atlas: atlas.clone(),
                    transform: Transform::from_xyz(pos.x, pos.y, -10.0),
                    ..default()
                })
                .with_children(|parent| {
                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                    parent.spawn(create_resource_text_bundle(font.clone(), planet.resource));
                    parent.spawn(create_influence_text_bundle(font.clone(), planet.index_in_grid as u32));
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

fn create_text_bundle(
    text: String,
    resource_text_style: TextStyle,
    resource_transform: Transform,
) -> Text2dBundle {
    let box_size = Vec2::new(50.0, 25.0);
    Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(text, resource_text_style.clone())],
            linebreak_behavior: BreakLineOn::NoWrap,
            alignment: Default::default(),
        },
        text_2d_bounds: Text2dBounds { size: box_size },
        transform: resource_transform,
        ..default()
    }
}

fn create_resource_text_bundle(font: Handle<Font>, value: u32) -> Text2dBundle {
    let resource_text_style: TextStyle = TextStyle {
        font,
        font_size: 42.0,
        color: Color::ORANGE,
    };

    let resource_transform = Transform {
        translation: Vec3::new(-45., 33., 0.5),
        scale: Vec3::splat(0.25), // Half the original size
        ..Default::default()
    };
    let text: String = format!("Resource {}", value);
    create_text_bundle(text, resource_text_style, resource_transform)
}

fn create_influence_text_bundle(font: Handle<Font>, value: u32) -> Text2dBundle {
    let resource_text_style: TextStyle = TextStyle {
        font,
        font_size: 42.0,
        color: Color::CYAN,
    };

    let resource_transform = Transform {
        translation: Vec3::new(-45., 26., 0.5),
        scale: Vec3::splat(0.25), // Half the original size
        ..Default::default()
    };

    create_text_bundle(
        format!("Influence {}", value),
        resource_text_style,
        resource_transform,
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
}

pub fn clear_selected(
    mut prev_pos: ResMut<SelectedHex>,
    grid: Res<HexGrid>,
    mut tiles: Query<&mut TextureAtlasSprite>) {
    prev_pos.is_selected = false;
    set_color_to_hex(&grid, &mut tiles, &prev_pos.hex, &DEFAULT_COLOR);
}

pub(crate) fn register_selected_hex(mut commands: Commands) {
    let hex = SelectedHex { hex: Hex::ZERO, is_selected: false };
    commands.insert_resource(hex);
}

/// Input interaction
pub(crate) fn handle_click_on_planet(
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGrid>,
    mut tiles: Query<&mut TextureAtlasSprite>,
    mut prev_pos: ResMut<SelectedHex>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        let cur_pos: Hex = grid.layout.world_pos_to_hex(pos);

        if !buttons.just_pressed(MouseButton::Left) { return; }

        if grid.entities.get(&cur_pos).is_none() {
            // prev_pos.is_selected = false;
            // set_color_to_hex(&grid, &mut tiles, &prev_pos.hex, &DEFAULT_COLOR);
            return;
        }

        if prev_pos.hex == cur_pos {
            if prev_pos.is_selected {
                set_color_to_hex(&grid, &mut tiles, &cur_pos, &DEFAULT_COLOR);
                prev_pos.is_selected = false;
            } else {
                set_color_to_hex(&grid, &mut tiles, &cur_pos, &SELECTED_COLOR);
                prev_pos.is_selected = true;
            }
        } else {
            let prv_pos_copy = prev_pos.hex.clone();
            prev_pos.hex = cur_pos;
            prev_pos.is_selected = true;
            set_color_to_hex(&grid, &mut tiles, &prv_pos_copy, &DEFAULT_COLOR);
            set_color_to_hex(&grid, &mut tiles, &cur_pos, &SELECTED_COLOR);
        }
    }
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
