use bevy::asset::{Assets, AssetServer};
use bevy::input::Input;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use glam::{vec2, Vec2};
use hexx::{Hex, HexLayout, HexOrientation, shapes};

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
                }).with_children(|parent| {
                let mut resource_transform = Transform::from_xyz(-5., -5., -1.);
                resource_transform.scale = Vec3::splat(0.5);

                let mut influence_transform = Transform::from_xyz(5., -5., -1.);
                influence_transform.scale = Vec3::splat(0.5);

                let resource_transform = Transform {
                    translation: Vec3::new(-5., -5., -0.5),
                    scale: Vec3::splat(0.5), // Half the original size
                    ..Default::default()
                };

                let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                // Resource text
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "123",
                        TextStyle {
                            font: font.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    ),
                    transform: resource_transform,
                    ..Default::default()
                });
            })
                .id();
            planets.insert(i, planet);
            (coord, entity)
        })
        .collect();
    commands.insert_resource(HexGrid { entities, layout, planets });
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
