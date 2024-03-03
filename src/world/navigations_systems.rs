use bevy::input::Input;
use bevy::prelude::{Camera, KeyCode, OrthographicProjection, Query, Res, Time, With};

pub(crate) fn zoom_system(
    mut query: Query<&mut OrthographicProjection,
        With<Camera>>, time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut projection in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Equals) {
            projection.scale -= 0.25 * time.delta_seconds();
        }
        if keyboard_input.pressed(KeyCode::Minus) {
            projection.scale += 0.25 * time.delta_seconds();
        }
    }
}