use bevy::input::Input;
use bevy::prelude::{Camera, KeyCode, OrthographicProjection, Query, Res, Time, Transform, With};
use glam::Vec3;

pub(crate) fn zoom_system(
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut projection in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Equals) {
            projection.scale -= 0.4 * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Minus) {
            projection.scale += 0.4 * time.delta_seconds();
        }
    }
}

pub(crate) fn move_system(
    mut camera_query: Query<(&Camera, &mut Transform)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let translation_speed = 400.0; // Adjust this value to your liking
    for (_camera, mut transform) in camera_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        transform.translation += time.delta_seconds() * translation_speed * direction;
    }
}
