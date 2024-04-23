use bevy::prelude::*;
use hexx::Hex;
use rand::Rng;

use crate::space_ships::{get_spaceship_sprite_bundle_by_type, SpaceShipType, SpaceSipTextureAtlas};
use crate::world::setup_world_grid::HexGrid;

#[derive(Resource)]
pub struct AnimationUnitsStorage {
    vec: Vec<AnimationUit>,
}

#[derive(Copy, Clone, Component)]
pub struct AnimationUit {
    from: Vec2,
    to: Vec2,
    time_left: f32,
}

pub fn animation_tick(
    mut animation_query: Query<(Entity, &mut Transform, &mut AnimationUit), With<AnimationUit>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut animation_unit) in animation_query.iter_mut() {
        if animation_unit.time_left > 1.0 {
            commands.entity(entity).despawn();
            continue;
        }
        let speed = (animation_unit.to.clone() - animation_unit.from.clone());
        let delta = speed * time.delta_seconds();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        animation_unit.time_left += time.delta_seconds();
    }
}

pub fn run_spaceship_moving_animation(
    ship_type: SpaceShipType, from: Hex, to: Hex, hex_grid: &HexGrid,
    // asset_server: Res<AssetServer>,
    // mut atlases: ResMut<Assets<TextureAtlas>>,
    handle: &Res<SpaceSipTextureAtlas>,
    commands: &mut Commands,
) {
    let mut rng = rand::thread_rng();
    let starting_position = Vec2 { x: rng.gen_range(-40.0..40.0), y: rng.gen_range(-40.0..40.0) } + hex_grid.layout.hex_to_world_pos(from);
    let end_position = Vec2 { x: rng.gen_range(-40.0..40.0), y: rng.gen_range(-40.0..40.0) } + hex_grid.layout.hex_to_world_pos(to);

    let image_sprite: (SpriteSheetBundle, ) = (
        get_spaceship_sprite_bundle_by_type(&handle.spaceship_grid_atlas, ship_type, Transform {
            translation: Vec3 { x: starting_position.x, y: starting_position.y, z: 0. },
            rotation: Default::default(),
            scale: Vec3::splat(0.7),
        }),
    );
    commands.spawn((
        image_sprite,
        AnimationUit {
            from: starting_position,
            to: end_position,
            time_left: 0.0,
        }
    ));
}