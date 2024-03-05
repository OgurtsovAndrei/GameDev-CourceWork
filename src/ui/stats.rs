use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::Query;
use bevy::hierarchy::{BuildChildren, ChildBuilder, Children};
use bevy::log::debug;
use bevy::prelude::{
    AlignSelf, Color, Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style,
    TextBundle, TextStyle,
};
use bevy::text::{Text, TextSection};
use bevy::ui::widget::Button;
use bevy::ui::{Interaction, UiRect, Val};

use crate::world::player::{Stats, Turn};

fn spawn_player_move(parent: &mut ChildBuilder, stats: &Stats) {
    parent
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect {
                    right: Val::Px(25.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            let num = stats.moves_left;
            parent.spawn(TextBundle::from_sections(vec![
                TextSection {
                    value: format!("Moves left {num}'s \n"),
                    style: TextStyle {
                        font: Default::default(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                TextSection {
                    value: "Player: 1".to_string(),
                    style: TextStyle {
                        font: Default::default(),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
            ]));
        });
}

pub fn setup_stats(
    mut query: Query<(
        &crate::world::player::Turn,
        &crate::world::player::Player,
        &crate::world::player::Stats,
    )>,
    mut commands: Commands,
) {
    for (turn, _, stats) in query.iter_mut() {
        debug!("Here {:?}", turn);
        if *turn == Turn::First {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        align_self: AlignSelf::Start,
                        justify_self: JustifySelf::Center,
                        flex_direction: FlexDirection::Row, // Horizontal layout
                        justify_content: JustifyContent::SpaceBetween, // This will ensure the spacing
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    spawn_player_move(parent, stats);
                });
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in interaction_query.iter_mut() {
        let text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}
