use bevy::prelude::*;
use bevy::utils::HashMap;
use hexx::Hex;

use crate::game_state::UpdateUI;
use crate::space_ships::{SpaceShipCharacteristics, SpaceShipType};
use crate::space_ships::SpaceShipType::{Carrier, Destroyer, Frigate};
use crate::ui::action_panel::systems::interaction::{has_neighbour_in_planets, is_selected_hex_belongs_to_player, is_selected_hex_has_neighbours};
use crate::world::actions::move_menu::animations::animation_tick;
use crate::world::actions::move_menu::plugin::MoveMenuPlugin;
use crate::world::actions::spawn_menu::plugin::SpawnMenuPlugin;
use crate::world::fonts_and_styles::fonts::{get_info_text_style, get_win_probability_text_style};
use crate::world::player::{Movable, Player};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{create_sprite_bundle_with_image, HexGrid, Planet, SelectedHex};

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
            .add_systems(Update, update_lock.in_set(UpdateUI::RenderStats))
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

pub fn update_lock(
    mut grid_mut: ResMut<HexGrid>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let player = current_player_query.single();
    let mut vec_to_add_lock = vec![];
    let mut vec_to_remove_lock = vec![];
    for (hex, planet) in grid_mut.planets.iter() {
        match &planet.planet_locked_entity {
            None => {
                if !has_neighbour_in_planets(player, hex, &grid_mut.planets) && !is_selected_hex_belongs_to_player(&player, &grid_mut, hex) { vec_to_add_lock.push(hex.clone()) }
            }
            Some(_) => {
                if has_neighbour_in_planets(player, hex, &grid_mut.planets) || is_selected_hex_belongs_to_player(&player, &grid_mut, hex) { vec_to_remove_lock.push(hex.clone()) }
            }
        }
    }
    let (entities, hexes, planets) = &mut grid_mut.get_tuple();
    for hex in vec_to_add_lock.iter() {
        let planet = planets.get_mut(hex).unwrap();
        planet.planet_locked_entity = spawn_bundle_on_hex_with_entities(entities, hex, &mut commands, create_hex_locked_bundle(&assets));
    }
    for hex in vec_to_remove_lock.iter() {
        let planet = grid_mut.planets.get_mut(hex).unwrap();
        despawn_lock_entity(planet, &mut commands);
    }
}

fn spawn_bundle_on_hex_with_entities(
    entity_map: &HashMap<Hex, Entity>,
    pos: &Hex,
    commands: &mut Commands,
    bundle: SpriteBundle,
) -> Option<Entity> {
    let Some(cur_entity) = entity_map.get(pos).copied() else {
        return None;
    };
    let parent_entity = commands.get_entity(cur_entity);
    let mut entity = None;
    parent_entity.unwrap().with_children(|parent| {
        entity = Some(parent.spawn(bundle).id());
    });
    return entity;
}

fn create_hex_locked_bundle(asset_server: &Res<AssetServer>) -> SpriteBundle {
    let image_path = "kenney - Simpe Icons/lock_closed.png".to_string();
    let transform = Transform {
        translation: Vec3::new(-53., -13., 1.0),
        scale: Vec3::splat(0.15),
        ..Default::default()
    };
    create_sprite_bundle_with_image(asset_server, image_path, transform, Color::WHITE)
}

fn despawn_lock_entity(planet: &mut Planet, commands: &mut Commands) {
    if let Some(entity) = planet.planet_locked_entity {
        if let Some(mut entity_details) = commands.get_entity(entity) {
            entity_details.despawn();
        }
        planet.planet_locked_entity = None;
    }
}

pub(self) fn reset_selected_for_buy_ships(hex_grid: &mut ResMut<HexGrid>, mut player_resources: &mut ResMut<GameResources>) {
    for (_, mut planet) in hex_grid.planets.iter_mut() {
        let player = planet.owner;
        if (player.id != -1) {
            let mut resources = player_resources.resources.remove(&player).unwrap();
            planet.owner_army.iter_mut().filter(|spaceship| spaceship.is_selected_for_buy).for_each(
                |spaceship| resources += SpaceShipCharacteristics::get_by_spaceship_type(spaceship.ship_type).price
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

pub(self) fn get_win_probability_text(asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle::from_sections(
        [
            TextSection::new(
                "Win probability: ",
                get_win_probability_text_style(asset_server),
            ),
            TextSection::new(
                "0",
                get_win_probability_text_style(asset_server),
            ),
            TextSection::new(
                "%",
                get_win_probability_text_style(asset_server),
            )
        ]
    )
}


pub(self) fn get_buy_ship_stats_text(asset_server: &Res<AssetServer>) -> TextBundle {
    let carrier = SpaceShipCharacteristics::get_by_spaceship_type(Carrier);
    let destroyer = SpaceShipCharacteristics::get_by_spaceship_type(Destroyer);
    let frigate = SpaceShipCharacteristics::get_by_spaceship_type(Frigate);
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    "0",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    " - Carrier ",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    format!("(cost - {}; power - {})\n", carrier.price.resources, carrier.power),
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    "0",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    " - Destroyer ",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    format!("(cost - {}; power - {})\n", destroyer.price.resources, destroyer.power),
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    "0",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    " - Frigate ",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    format!("(cost - {}; power - {})\n", frigate.price.resources, frigate.power),
                    get_info_text_style(asset_server),
                ),
            ],
            alignment: TextAlignment::Center,
            ..default()
        },
        ..default()
    }
}

pub(self) fn get_move_ship_stats_text(asset_server: &Res<AssetServer>) -> TextBundle {
    let carrier = SpaceShipCharacteristics::get_by_spaceship_type(Carrier);
    let destroyer = SpaceShipCharacteristics::get_by_spaceship_type(Destroyer);
    let frigate = SpaceShipCharacteristics::get_by_spaceship_type(Frigate);
    TextBundle {
        text: Text {
            sections: vec![
                TextSection::new(
                    "0",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    " - Carrier ",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    format!("(power - {})\n", carrier.power),
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    "0",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    " - Destroyer ",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    format!("(power - {})\n", destroyer.power),
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    "0",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    " - Frigate ",
                    get_info_text_style(asset_server),
                ),
                TextSection::new(
                    format!("(power - {})\n", frigate.power),
                    get_info_text_style(asset_server),
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
        SpaceShipType::Carrier => { 0 }
        SpaceShipType::Destroyer => { 3 }
        SpaceShipType::Frigate => { 6 }
        SpaceShipType::Battleship => { 9 }
        SpaceShipType::Fighter => { 12 }
    }
}