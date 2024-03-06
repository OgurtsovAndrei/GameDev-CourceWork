use std::default;

use bevy::ecs::{
    bundle::Bundle,
    component::Component,
    schedule::States,
    system::{Commands, Resource},
};

pub const INITIAL_MOVES: i32 = 10;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stats {
    pub moves_left: i32,
}

#[derive(Resource, Component, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum Turn {
    #[default]
    First,
    Second,
}

impl ToString for Turn {
    fn to_string(&self) -> String {
        match self {
            Turn::First => "Player 1".to_string(),
            Turn::Second => "Player 2".to_string(),
        }
    }
}

impl Turn {
    pub fn flip(&self) -> Turn {
        match self {
            Turn::First => Turn::Second,
            Turn::Second => Turn::First,
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    turn: Turn,
    player: Player,
    stats: Stats,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            turn: Turn::First,
            player: Player,
            stats: Stats {
                moves_left: INITIAL_MOVES,
            },
        }
    }
}

pub fn spawn_players(mut commands: Commands) {
    commands.spawn(PlayerBundle {
        turn: Turn::First,
        ..Default::default()
    });
    commands.spawn(PlayerBundle {
        turn: Turn::Second,
        ..Default::default()
    });
}
