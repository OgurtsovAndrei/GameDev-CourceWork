use bevy::prelude::*;

use crate::game_state::UpdateUI;
use crate::space_ships::SpaceShipType;
use crate::ui::action_panel::systems::interaction::{is_selected_hex_belongs_to_player, is_selected_hex_has_neighbours};
use crate::world::actions::move_menu::animations::animation_tick;
use crate::world::actions::move_menu::plugin::MoveMenuPlugin;
use crate::world::actions::spawn_menu::plugin::SpawnMenuPlugin;
use crate::world::fonts_and_styles::fonts::get_info_text_style;
use crate::world::player::{Movable, Player};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{HexGrid, SelectedHex};

pub(crate) mod spawn_menu;


pub(crate) mod move_menu;

pub(crate) struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<ActionsState>()
            .add_plugins(MoveMenuPlugin)
            .add_plugins(SpawnMenuPlugin)
            .add_systems(Update, change_action_state.in_set(UpdateUI::UserInput))
            .add_systems(Update, animation_tick);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub(crate) enum ActionsState {
    #[default]
    NoActionRunning,
    // ChoosingAction,
    SpawningSpaceShips,
    MovingSpaceShips,
}

pub fn clear_action_state(mut commands: Commands) {
    commands.insert_resource(NextState(Some(ActionsState::NoActionRunning)))
}

fn change_action_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    current_state: Res<State<ActionsState>>,
    mut grid_mut: ResMut<HexGrid>,
    mut player_resources: ResMut<GameResources>,
    selected_hex: Res<SelectedHex>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    let player = current_player_query.single();
    match current_state.get() {
        ActionsState::NoActionRunning => {
            let grid = grid_mut.as_ref();
            if keyboard_input.just_pressed(KeyCode::S) && is_selected_hex_belongs_to_player(player, grid, &selected_hex.hex) {
                commands.insert_resource(NextState(Some(ActionsState::SpawningSpaceShips)))
            } else if keyboard_input.just_pressed(KeyCode::M) && (is_selected_hex_belongs_to_player(player, grid, &selected_hex.hex) || is_selected_hex_has_neighbours(player, grid, &selected_hex.hex)) {
                commands.insert_resource(NextState(Some(ActionsState::MovingSpaceShips)))
            }
        }
        _ => {
            if keyboard_input.just_pressed(KeyCode::C) {
                reset_selected_for_move_ships(&mut grid_mut);
                reset_selected_for_buy_ships(&mut grid_mut, &mut player_resources);
                commands.insert_resource(NextState(Some(ActionsState::NoActionRunning)))
            }
        }
    }
}

pub(self) fn reset_selected_for_buy_ships(hex_grid: &mut ResMut<HexGrid>, mut player_resources: &mut ResMut<GameResources>) {
    for (_, mut planet) in hex_grid.planets.iter_mut() {
        let player = planet.owner;
        if (player.id != -1) {
            let mut resources = player_resources.resources.remove(&player).unwrap();
            planet.owner_army.iter_mut().filter(|spaceship| spaceship.is_selected_for_buy).for_each(
                |spaceship| resources += spaceship.get_price()
            );
            planet.owner_army.retain(|spaceship| !spaceship.is_selected_for_buy);
            player_resources.resources.insert(player, resources);
        }
    }
}

pub(self) fn reset_selected_for_move_ships(hex_grid: &mut ResMut<HexGrid>) {
    hex_grid.planets.iter_mut().for_each(|(_, mut planet)| {
        planet.owner_army.iter_mut().for_each(|ship| ship.is_selected_for_move = false);
    });
}

pub(self) fn get_ship_stats_text(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    "Carrier: ",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "0\n",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "Destroyer: ",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "0\n",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "Frigate: ",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "0\n",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "Battleship: ",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "0\n",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "Fighter: ",
                    get_info_text_style(&asset_server),
                ),
                TextSection::new(
                    "0\n",
                    get_info_text_style(&asset_server),
                ),
            ],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

pub(self) fn get_spaceship_index_by_type(space_ship_type: SpaceShipType) -> usize {
    match space_ship_type {
        SpaceShipType::Carrier => { 1 }
        SpaceShipType::Destroyer => { 3 }
        SpaceShipType::Frigate => { 5 }
        SpaceShipType::Battleship => { 7 }
        SpaceShipType::Fighter => { 9 }
    }
}