use bevy::app::{Plugin, Startup, Update};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::{With, Without};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::system::{Query, ResMut, Resource};
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{
    AlignSelf, Color, Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style,
    TextBundle, TextStyle,
};
use bevy::text::Text;

use crate::game_state::UpdateUI;
use crate::world::player::{Movable, Player, Stats, INITIAL_MOVES};

#[derive(Resource)]
pub struct Round {
    pub number: i32,
}

impl Default for Round {
    fn default() -> Self {
        Round { number: 1 }
    }
}

#[derive(Component)]
pub struct TurnText;

#[derive(Component)]
pub struct MovesLeftText;

#[derive(Component)]
pub struct RoundText;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Round::default())
            .add_systems(Startup, setup_stats).add_systems(
            Update,
            (
                update_round_number_text,
                update_turn_text,
                update_moves_left_text,
            )
                .in_set(UpdateUI::RenderText),
        );
    }
}

fn spawn_round_number(parent: &mut ChildBuilder) {
    let mut round_text = TextBundle::from_section(
        String::new(),
        TextStyle {
            font: Default::default(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );
    set_round_number_text(&mut round_text.text, 1);
    parent.spawn(round_text).insert(RoundText);
}

fn spawn_player_turn(parent: &mut ChildBuilder) {
    let mut player_turn_text = TextBundle::from_section(
        String::new(),
        TextStyle {
            font: Default::default(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );
    set_player_turn_text(&mut player_turn_text.text, 1);

    parent.spawn(player_turn_text).insert(TurnText);
}

fn spawn_player_moves(parent: &mut ChildBuilder) {
    let mut moves_left_text = TextBundle::from_section(
        String::new(),
        TextStyle {
            font: Default::default(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );
    set_moves_left_text(&mut moves_left_text.text, INITIAL_MOVES);
    parent.spawn(moves_left_text).insert(MovesLeftText);
}

fn setup_stats(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Start,
                justify_self: JustifySelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween, // This will ensure the spacing
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            spawn_round_number(parent);
            spawn_player_moves(parent);
            spawn_player_turn(parent);
        });
}

fn update_turn_text(
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
    mut turn_text_query: Query<&mut Text, With<TurnText>>,
) {
    let player = current_player_query.single();
    let mut turn_text = turn_text_query.single_mut();
    set_player_turn_text(&mut turn_text, player.id);
}

fn update_moves_left_text(
    current_player_query: Query<&Stats, (With<Player>, With<Movable>)>,
    mut moves_left_text_query: Query<&mut Text, With<MovesLeftText>>,
) {
    let stats = current_player_query.single();
    let mut moves_left_text = moves_left_text_query.single_mut();
    set_moves_left_text(&mut moves_left_text, stats.moves_left);
}

fn update_round_number_text(
    mut commands: Commands,
    mut round_text_query: Query<&mut Text, With<RoundText>>,
    mut current_player: Query<(Entity, &Player, &mut Stats), With<Movable>>,
    mut opposite_player: Query<(Entity, &mut Stats), Without<Movable>>,
    mut round_res: ResMut<Round>,
) {
    let mut round_text = round_text_query.single_mut();
    let (cur_id, cur_player, mut cur_stats) = current_player.single_mut();
    let (op_id, mut op_stats) = opposite_player.single_mut();
    if cur_stats.moves_left == 0 && op_stats.moves_left == 0 {
        let round = round_res.as_mut();
        round.number += 1;
        set_round_number_text(&mut round_text, round.number);
        cur_stats.moves_left = INITIAL_MOVES;
        op_stats.moves_left = INITIAL_MOVES;
        commands.entity(cur_id).remove::<Movable>();
        commands.entity(op_id).remove::<Movable>();
        if cur_player.id == 1 {
            commands.entity(cur_id).insert(Movable);
        } else {
            commands.entity(op_id).insert(Movable);
        }
    }
}

pub fn set_round_number_text(text: &mut Text, value: i32) {
    text.sections[0].value = format!("Round: {}", value.to_string());
}

pub fn set_moves_left_text(text: &mut Text, value: i32) {
    text.sections[0].value = format!("Moves left: {}", value.to_string());
}

pub fn set_player_turn_text(text: &mut Text, value: i32) {
    text.sections[0].value = format!("Player: {}", value.to_string());
}
