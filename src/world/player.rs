
use bevy::ecs::{bundle::Bundle, component::Component, system::Commands};

pub const INITIAL_MOVES: i32 = 10;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Stats {
    pub moves_left: i32,
}

#[derive(Component, Debug, Clone, Eq, PartialEq, Hash)]
pub enum Turn {
    First,
    Second
}

impl Turn {
    pub fn flip(self) -> Turn {
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
