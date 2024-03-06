use bevy::asset::{AssetServer, Assets};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::Input;
use bevy::prelude::*;
use bevy::text::{BreakLineOn, Text2dBounds};
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use glam::{vec2, Vec2};
use hexx::{shapes, Hex, HexLayout, HexOrientation};

use crate::space_ships::SpaceShip;

const HEX_SIZE: Vec2 = Vec2::splat(75.0);
const FILE_GRID_HEIGHT_IN_FILE: usize = 1;
const GRID_WEIGHT_IN_FILE: usize = 6;

/// 3D Orthogrpahic camera setup
pub(crate) fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Debug, Resource)]
pub struct Planet {
    pub index_in_grid: usize,
    pub resource: u32,
    pub influence: u32,
}

#[derive(Debug, Resource)]
pub struct HexGrid {
    // pub resources: HashMap<>
    pub entities: HashMap<Hex, Entity>,
    pub layout: HexLayout,
    pub planets: HashMap<usize, Planet>,
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
    let mut planets: HashMap<usize, Planet> = HashMap::new();
    let entities = shapes::hexagon(Hex::ZERO, 2)
        .enumerate()
        .map(|(i, coord)| {
            let pos = layout.hex_to_world_pos(coord);
            let index = i % (FILE_GRID_HEIGHT_IN_FILE * GRID_WEIGHT_IN_FILE);
            let planet = Planet {
                index_in_grid: i,
                resource: (index + 1) as u32,
                influence: (5 - index) as u32,
            };
            let entity = commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
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
                    parent.spawn(create_influence_text_bundle(font.clone(), planet.influence));
                })
                .id();
            planets.insert(i, planet);
            (coord, entity)
        })
        .collect();
    commands.insert_resource(HexGrid {
        entities,
        layout,
        planets,
    });
}

fn create_text_bundle(
    text: String,
    resource_text_style: TextStyle,
    resource_transform: Transform,
) -> Text2dBundle {
    let box_size = Vec2::new(50.0, 25.0);
    return Text2dBundle {
        text: Text {
            sections: vec![TextSection::new(text, resource_text_style.clone())],
            linebreak_behavior: BreakLineOn::NoWrap,
            alignment: Default::default(),
        },
        text_2d_bounds: Text2dBounds { size: box_size },
        transform: resource_transform,
        ..default()
    };
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
    return create_text_bundle(text, resource_text_style, resource_transform);
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

/// Input interaction
pub(crate) fn handle_input(
    buttons: Res<Input<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    grid: Res<HexGrid>,
    mut tiles: Query<&mut TextureAtlasSprite>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    if let Some(pos) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world_2d(cam_transform, p))
    {
        let hex_pos = grid.layout.world_pos_to_hex(pos);
        let Some(entity) = grid.entities.get(&hex_pos).copied() else {
            return;
        };
        if !buttons.just_pressed(MouseButton::Left) {
            return;
        }
        let Ok(mut sprite) = tiles.get_mut(entity) else {
            return;
        };
        sprite.index = (sprite.index + 1) % (FILE_GRID_HEIGHT_IN_FILE * GRID_WEIGHT_IN_FILE);
    }
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
