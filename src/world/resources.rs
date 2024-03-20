use std::fmt::Write;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use bevy::prelude::*;
use bevy::utils::HashMap;
use hexx::Hex;

use crate::space_ships::{SpaceShip, SpaceShipType};
use crate::world::player::Player;
use crate::world::setup_world_grid::{HexGrid, Planet};

#[derive(Debug, Clone, Copy)]
pub(crate) struct PlayerResources {
    pub influence: u32,
    pub resources: u32,
}

impl Add for PlayerResources {
    type Output = PlayerResources;

    fn add(self, rhs: Self) -> Self::Output {
        PlayerResources {
            influence: self.influence + rhs.influence,
            resources: self.resources + rhs.resources,
        }
    }
}

impl AddAssign for PlayerResources {
    fn add_assign(&mut self, rhs: Self) {
        self.influence += rhs.influence;
        self.resources += rhs.resources;
    }
}

impl Sub for PlayerResources {
    type Output = PlayerResources;
    fn sub(self, rhs: Self) -> Self::Output {
        PlayerResources {
            influence: self.influence - rhs.influence,
            resources: self.resources - rhs.resources,
        }
    }
}

impl SubAssign for PlayerResources {
    fn sub_assign(&mut self, rhs: Self) {
        self.influence -= rhs.influence;
        self.resources -= rhs.resources;

    }
}


impl Default for PlayerResources {
    fn default() -> Self {
        PlayerResources {
            influence: 0,
            resources: 0,
        }
    }
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

impl GameResources {
    pub fn update(&mut self, value: &HexGrid) {
        let mut players_stats_map: HashMap<Player, PlayerResources> = HashMap::from(
            [(Player { id: -1 }, PlayerResources::default()), (Player { id: 1 }, PlayerResources::default()), (Player { id: 2 }, PlayerResources::default())]
        );

        for (_, planet) in &value.planets {
            let mut player_map = players_stats_map.remove(&planet.owner).unwrap();
            player_map.resources += planet.resource;
            player_map.influence += planet.influence;
            players_stats_map.insert(planet.owner, player_map);
        }


        for (player, stats) in players_stats_map {
            if let None = self.resources.get(&player) {
                continue;
            }
            let mut current_resources = self.resources.remove(&player).unwrap();

            info!("{:?}, {:?}, {:?}", player, stats.influence, stats.resources);
            current_resources.influence += stats.influence;
            current_resources.resources += stats.resources;

            self.resources.insert(player, current_resources);
        }
    }
}


const INITIAL_RESOURCES: u32 = 10;
const INITIAL_INFLUENCE: u32 = 5;

pub fn setup_resources(mut commands: &mut Commands, grid: &mut HexGrid) {
    let planets = &mut grid.planets;
    let player1 = Player { id: 1 };
    let player2 = Player { id: 2 };
    let player1_home_hex = Hex { x: -2, y: 2 };
    let player2_home_hex = Hex { x: 2, y: -2 };

    let mut planet1 = planets.remove(&player1_home_hex).unwrap();
    planet1.owner = player1;
    planet1.owner_army.push(SpaceShip {
        ship_type: SpaceShipType::Battleship,
        ship_owner: player1,
        ship_hex: player1_home_hex,
        is_selected_for_move: false,
        is_selected_for_buy: false,
    });
    planets.insert(player1_home_hex, planet1);


    let mut planet2 = planets.remove(&player2_home_hex).unwrap();
    planet2.owner = player2;
    planets.insert(player2_home_hex, planet2);

    let player1_res = PlayerResources { influence: INITIAL_INFLUENCE, resources: INITIAL_RESOURCES };
    let player2_res = PlayerResources { influence: INITIAL_INFLUENCE, resources: INITIAL_RESOURCES };
    let mut resources = HashMap::new();
    resources.insert(player1, player1_res);
    resources.insert(player2, player2_res);
    commands.insert_resource(GameResources { resources });
}
