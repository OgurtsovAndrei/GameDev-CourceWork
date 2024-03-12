use bevy::app::{App, Plugin, Startup, Update};
use bevy::input::Input;
use bevy::prelude::{Camera, GlobalTransform, MouseButton, Query, Res, TextureAtlasSprite, Window, With};
use bevy::window::PrimaryWindow;
use hexx::Hex;

use crate::world::setup_world_grid::HexGrid;

pub mod navigations_systems;
pub mod player;
pub mod setup_world_grid;
pub mod resources;
pub mod turn;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let mut hex = Hex { x: -1000, y: -1000 };
        app
            .add_systems(Startup, (setup_world_grid::setup_camera, setup_world_grid::setup_grid))
            .add_systems(
                Update,
                move |buttons: Res<Input<MouseButton>>,
                      windows: Query<&Window, With<PrimaryWindow>>,
                      cameras: Query<(&Camera, &GlobalTransform)>,
                      grid: Res<HexGrid>,
                      tiles: Query<&mut TextureAtlasSprite>| {
                    setup_world_grid::handle_click_on_planet(buttons, windows, cameras, grid, tiles, &mut hex)
                },
            );
    }
}
