use bevy::prelude::*;
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_mod_picking::prelude::*;

#[derive(Component)]
pub struct SpaceShip;

pub(crate) fn spawn_ship(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let atlas = TextureAtlas::from_grid(
        asset_server.load("SpaceShips/Transport.png"),
        Vec2::new(400., 150.),
        1, 1, None, None);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas.add(atlas),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Option::from(Vec2::new(200., 75.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(-300., 300., 0.0),
            ..Default::default()
        }, On::<Pointer<Click>>::run(move || info!("Spaceship pressed")), PickableBundle::default(), SpaceShip {}
    )
    );
}