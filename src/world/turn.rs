use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        entity::Entity,
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Commands, Query, ResMut, Resource},
    },
};

use crate::game_state::UpdateUI;

use super::player::{Movable, Player, Stats};

pub struct TurnPlugin;

#[derive(Resource, Debug)]
pub struct TurnDone {
    pub value: bool,
}

impl Default for TurnDone {
    fn default() -> Self {
        TurnDone { value: false }
    }
}

impl Plugin for TurnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(TurnDone::default())
            .add_systems(Update, (determine_turn).in_set(UpdateUI::FlipTurn));
    }
}

fn determine_turn(
    mut commands: Commands,
    current_player_query: Query<(Entity, &Stats), (With<Player>, With<Movable>)>,
    opposite_player_query: Query<(Entity, &Stats), (With<Player>, Without<Movable>)>,
    mut turn_done_res: ResMut<TurnDone>,
) {
    let turn_done = turn_done_res.as_mut();

    if turn_done.value {
        turn_done.value = false;

        let (cur_id, cur_stats) = current_player_query.single();
        let (op_id, op_stats) = opposite_player_query.single();

        if op_stats.moves_left > 0 {
            commands.entity(cur_id).remove::<Movable>();
            commands.entity(op_id).insert(Movable);
        } else if op_stats.moves_left == 0 && cur_stats.moves_left > 0 {
        } else if cur_stats.moves_left == 0 && cur_stats.moves_left == 0 {
            commands.entity(cur_id).remove::<Movable>();
        } else {
            panic!("Should never happen");
        }
     }
}
