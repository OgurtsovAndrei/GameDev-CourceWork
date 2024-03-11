use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::world::player::Player;
use crate::world::setup_world_grid::{HexGrid, Planet};

#[derive(Debug)]
struct PlayerResources {
    pub player: Player,
    pub planets: HashMap<usize, Planet>,
    pub influence: u32,
    pub resources: u32,
}

#[derive(Debug, Resource)]
struct GameResources {
    pub resources: HashMap<Player, PlayerResources>,
}

pub fn update_resources(mut game_resources: ResMut<GameResources>) {
    let res = &mut game_resources.resources;
    for (id, x) in res.iter_mut() {
        let planets = &x.planets;
        for (ind, planet) in planets {
            x.resources += planet.resource;
            x.influence += planet.influence;
        }
    }
}

pub fn setup_resources(mut commands: &mut Commands, grid: &HexGrid) {
    let planets = &grid.planets;
    let player1 = Player { id: 1 };
    let player1_planets: HashMap<usize, Planet> = HashMap::from([(0usize, planets[&0usize].clone())]);
    let player2 = Player { id: 2 };
    let player2_planets: HashMap<usize, Planet> = HashMap::from([(20usize, planets[&18usize].clone())]);
    let player1_res = PlayerResources { player: player1.clone(), planets: player1_planets, influence: 0, resources: 0 };
    let player2_res = PlayerResources { player: player2.clone(), planets: player2_planets, influence: 0, resources: 0 };
    let mut resources = HashMap::new();
    resources.insert(player1, player1_res);
    resources.insert(player2, player2_res);
    commands.insert_resource(GameResources { resources });
}
