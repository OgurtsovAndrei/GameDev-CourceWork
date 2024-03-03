// use bevy::prelude::*;
//
// pub fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(WindowPlugin {
//             primary_window: Some(Window {
//                 resolution: (1_000.0, 1_000.0).into(),
//                 ..default()
//             }),
//             ..default()
//         }))
//         .add_systems(Startup, (crate::hex_grid::setup_camera, setup))
//         .run();
// }
//
//
// fn setup(
//     commands: &mut Commands,
//     asset_server: Res<AssetServer>,
//     mut materials: ResMut<Assets<StandardMaterial>>) {
//     // Load the hexagon mesh
//     let hexagon_handle = asset_server.load("hexagon.glb#Mesh0/Primitive0");
//
//     // Load the texture
//     let texture_handle = asset_server.load("./images/img.png");
//
//     // Create a new material with the texture
//     let material = materials.add(StandardMaterial::from(texture_handle).into());
//
//
//     // Spawn a hexagon mesh bundle
//     commands.spawn(PbrBundle {
//         mesh: hexagon_handle,
//         material: material,
//         transform: Transform::default(),
//         ..Default::default()
//     });
// }