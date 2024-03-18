use std::fmt::Write;

use bevy::prelude::*;
use bevy::utils::HashMap;
use hexx::Hex;

use crate::space_ships::SpaceShipType;
use crate::world::player::Player;
use crate::world::setup_world_grid::{HexGrid, Planet};

#[derive(Debug, Clone)]
pub(crate) struct PlayerResources {
    pub player: Player,
    pub planets: HashMap<Hex, Planet>,
    pub influence: u32,
    pub resources: u32,
}

#[derive(Debug, Resource, Clone)]
pub(crate) struct GameResources {
    pub resources: HashMap<Player, PlayerResources>,
}

impl GameResources {
    pub(crate) fn to_string(&self) -> String {
        let mut resource_text = String::new();
        for (player, player_resources) in &self.resources {
            write!(&mut resource_text, "Player {:?} : {:?}\n", player.id, (player_resources.resources, player_resources.influence)).unwrap();
        }
        return resource_text;
    }
}

pub fn update_planet_owners(grid: Res<HexGrid>, mut game_resources: ResMut<GameResources>) {
    let res = &mut game_resources.resources;
    for (_player, player_res) in res.iter_mut() { player_res.planets.clear() }
    let planets = &grid.planets;
    for (id_in_grid, planet) in planets {
        let current_owner = planet.owner.clone();
        let player_resource = res.get_mut(&current_owner).unwrap();
        player_resource.planets.insert(id_in_grid.clone(), planet.clone());
    }
}

pub fn update_resources(mut game_resources: ResMut<GameResources>) {
    let res = &mut game_resources.resources;
    for (_id, x) in res.iter_mut() {
        let planets = &x.planets;
        for (_ind, planet) in planets {
            x.resources += planet.resource;
            x.influence += planet.influence;
        }
    }
}

const INITIAL_RESOURCES: u32 = 10;
const INITIAL_INFLUENCE: u32 = 5;

pub fn setup_resources(mut commands: &mut Commands, grid: &mut HexGrid) {
    let planets = &mut grid.planets;
    let player1 = Player { id: 1 };
    let player1_home_hex = Hex { x: -2, y: 2 };
    let player2_home_hex = Hex { x: 2, y: -2 };
    let mut planet = planets.remove(&player1_home_hex).unwrap();
    planet.owner_army.push(SpaceShipType::Battleship);
    planets.insert(player1_home_hex.clone(), planet);

    let player1_planets: HashMap<Hex, Planet> = HashMap::from([(player1_home_hex.clone(), planets[&player1_home_hex].clone())]);
    let player2 = Player { id: 2 };
    let player2_planets: HashMap<Hex, Planet> = HashMap::from([(player2_home_hex.clone(), planets[&player2_home_hex].clone())]);
    let player1_res = PlayerResources { player: player1.clone(), planets: player1_planets, influence: INITIAL_INFLUENCE, resources: INITIAL_RESOURCES };
    let player2_res = PlayerResources { player: player2.clone(), planets: player2_planets, influence: INITIAL_INFLUENCE, resources: INITIAL_RESOURCES };
    let mut resources = HashMap::new();
    resources.insert(player1, player1_res);
    resources.insert(player2, player2_res);
    commands.insert_resource(GameResources { resources });
}
