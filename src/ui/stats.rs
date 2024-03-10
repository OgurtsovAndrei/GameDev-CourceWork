
use bevy::app::{Plugin, Startup};
use bevy::ecs::component::Component;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{
    AlignSelf, Color, Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style,
    TextBundle, TextStyle,
};
use bevy::text::Text;

use crate::world::player::{Turn, INITIAL_MOVES};


#[derive(Component)]
pub struct TurnText;

#[derive(Component)]
pub struct MovesLeftText;

pub struct StatsPlugin;

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_stats);
    }
}

fn spawn_player_move(parent: &mut ChildBuilder) {
    let mut moves_left_text = TextBundle::from_section(
        String::new(),
        TextStyle {
            font: Default::default(),
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    );
    updates_moves_left_text(&mut moves_left_text.text, INITIAL_MOVES);
    parent.spawn(moves_left_text).insert(MovesLeftText);

    parent
        .spawn(TextBundle::from_section(
            Turn::First.to_string(),
            TextStyle {
                font: Default::default(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ))
        .insert(TurnText);
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
            spawn_player_move(parent);
        });
}

pub fn updates_moves_left_text(text: &mut Text, value: i32) {
    text.sections[0].value = format!("Moves left: {}", value.to_string());
}
