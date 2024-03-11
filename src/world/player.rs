use bevy::ecs::{
    bundle::Bundle,
    component::Component,
    system::{Commands, Resource},
};

pub const INITIAL_MOVES: i32 = 10;

#[derive(Component)]
pub struct Player {
    pub id: i32,
}

#[derive(Component)]
pub struct Stats {
    pub moves_left: i32,
}


#[derive(Component, Debug)]
pub struct Movable;


#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    stats: Stats,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            player: Player { id: 0 },
            stats: Stats {
                moves_left: INITIAL_MOVES,
            },
        }
    }
}

pub fn spawn_players(mut commands: Commands) {
    commands
        .spawn(PlayerBundle {
            player: Player { id: 1 },
            ..Default::default()
        })
        .insert(Movable);
    commands.spawn(PlayerBundle {
        player: Player { id: 2 },
        ..Default::default()
    });
}
