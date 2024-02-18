// use bevy::app::{App, Plugin, Startup, Update};
// use bevy::DefaultPlugins;
// use bevy::prelude::{Commands, Component, Query, Res, ResMut, Resource, Time, Timer, TimerMode, With};
//
// #[derive(Component)]
// struct Person;
//
// #[derive(Component)]
// struct Name(String);
//
// #[derive(Component)]
// struct SecondName(String);
//
// #[derive(Resource)]
// struct GreetTimer(Timer);
//
// fn update_people(mut query: Query<&mut Name, With<Person>>) {
//     for mut name in &mut query {
//         if name.0 == "Elaina" {
//             name.0 = "Elaina Hume".to_string();
//             break; // We don’t need to change any other names
//         }
//     }
// }
//
// fn add_people(mut commands: Commands) {
//     println!("Startup");
//     commands.spawn((Person, Name("Elaina".to_string()), SecondName("Proctor".to_string())));
//     commands.spawn((Person, Name("Renzo".to_string()), SecondName("Hume".to_string())));
//     commands.spawn((Person, Name("Zayna".to_string()), SecondName("Nieves".to_string())));
// }
//
// fn hello_world() {
//     println!("hello world!");
// }
//
// fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in &query {
//             println!("hello {}!", name.0);
//         }
//     }
// }
//
// pub struct HelloPlugin;
//
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         app
//             .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
//             .add_systems(Startup, add_people)
//             .add_systems(Update, greet_people);
//     }
// }
//
// fn main() {
//     App::new()
//         .add_plugins((DefaultPlugins, HelloPlugin))
//         .run();
// }
/*use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use hexx::*;

struct Hex {
    position: hex::Hex,
}

struct HexBundle {
    hex: Hex,
}

// Default Main System that sets up your bevy application
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_cursor)
        .run();
}

// This system setups up your bevy application and creates two hexagons
fn setup(mut commands: Commands) {
    let point_a = hex(10, -5);
    let point_b = hex(-8, 15);
    // commands.spawn_bundle(HexBundle { hex: Hex { position: point_a } /* initialization for other components */ });
    // commands.spawn_bundle(HexBundle { hex: Hex { position: point_b } /* initialization for other components */ });
    add_hexagon_mesh(&mut commands, Vec3::new(-3.0, 0.0, 0.0));
    add_hexagon_mesh(&mut commands, Vec3::new(3.0, 0.0, 0.0));

    commands.spawn(Camera2dBundle::default());
}

fn generate_hexagon_mesh() -> Mesh {
    let radius = 1.0;
    let depth = 0.1;

    // Define vertices for hexagon
    let angle_step = std::f32::consts::PI / 3.0;
    let vertices = (0..6).map(|i| {
        let angle = angle_step * (i as f32);
        let (x, y) = (radius * angle.cos(), radius * angle.sin());
        [x, y, depth]
    });

    // Define triangle indices
    let indices = &[0, 1, 2, 0, 2, 3, 0, 3, 4, 0, 4, 5];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U16(indices.to_vec())));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices.collect::<Vec<_>>());
    mesh
}

fn add_hexagon_mesh(commands: &mut Commands, position: Vec3) {
    let hex_mesh = generate_hexagon_mesh();
    commands.spawn_bundle(PbrBundle {
        mesh: commands.insert_resource(hex_mesh),
        material: commands.insert_resource(ColorMaterial::from(Color::rgb(0.0, 1.0, 0.0))),
        transform: Transform::from_translation(position),
        ..Default::default()
    });
}

fn draw_cursor(
    mut cmds: Commands,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let (camera, camera_transform) = camera_query.single();

    let cursor_position = match windows.single().cursor_position() {
        Some(pos) => pos,
        None => { return; }
    };

    // Calculate a world position based on the cursor's position.
    let point = match camera.viewport_to_world_2d(camera_transform, cursor_position) {
        Some(pos) => pos,
        None => { return; }
    };

    let hex_center = HexLayout {
        hex_size: Vec2::new(1.0, 1.0), // experiment with the sizes
        orientation: HexOrientation::Flat,
        ..Default::default()
    }
        .hex_to_world_pos(point.into());

    let color_material = materials.add(Color::rgb(0.0, 1.0, 0.0).into());

    // Create sprite bundle with a Transform component to position the sprite
    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::ALICE_BLUE,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(hex_center.x, hex_center.y, 0.0)),
        ..Default::default()
    };

    cmds.spawn(sprite_bundle);
}*/

use bevy::prelude::*;
use hexx::*;

mod hex_grid_withlight;
mod hex_grid;
mod field_of_move;
mod with_picture;
mod spire_sheet;

fn main() {
    // hex_grid_withlight::main();
    hex_grid::main();
    // field_of_move::main();
    // with_picture::main();
    // spire_sheet::main();
}

