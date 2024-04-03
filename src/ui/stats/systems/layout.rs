use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{AlignSelf, Color, Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style, TextBundle, TextStyle};
use crate::ui::stats::components::{MovesLeftText, RoundText, TurnText};
use crate::ui::stats::systems::interaction::{set_moves_left_text, set_player_turn_text, set_round_number_text};
use crate::world::player::INITIAL_MOVES;

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

pub(in crate::ui::stats) fn setup_stats(mut commands: Commands) {
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

