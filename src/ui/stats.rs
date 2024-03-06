use bevy::ecs::component::Component;
use bevy::ecs::query::{Changed, With, Without};
use bevy::ecs::system::{Query, ResMut};
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{
    AlignSelf, Color, Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style,
    TextBundle, TextStyle,
};
use bevy::text::Text;
use bevy::ui::widget::Button;
use bevy::ui::Interaction;

use crate::world::player::{Player, Stats, Turn, INITIAL_MOVES};

use super::buttons::NextMoveButton;

#[derive(Component)]
pub struct TurnText;

#[derive(Component)]
pub struct MovesLeftText;

fn spawn_player_move(parent: &mut ChildBuilder) {
    parent
        .spawn(TextBundle::from_section(
            format!("Moves left {INITIAL_MOVES}"),
            TextStyle {
                font: Default::default(),
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        ))
        .insert(MovesLeftText);
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

pub fn setup_stats(mut commands: Commands) {
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

pub fn update_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<NextMoveButton>)>,
    mut player_query: Query<(&Player, &mut Stats, &Turn)>,
    mut player_number_text_query: Query<&mut Text, (With<TurnText>, Without<MovesLeftText>)>,
    mut moves_left_text_query: Query<&mut Text, (With<MovesLeftText>, Without<TurnText>)>,
    mut turn_query: ResMut<Turn>,
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }
    let interaction = interaction_query.single();
    let mut player_number = player_number_text_query.single_mut();
    let mut moves_left = moves_left_text_query.single_mut();
    let current_turn = turn_query.as_mut();
    match *interaction {
        Interaction::Pressed => {
            *current_turn = current_turn.flip();
            for (_, mut stats, turn) in player_query.iter_mut() {
                if turn == current_turn {
                    moves_left.sections[0].value = format!("Moves left: {}", stats.moves_left.to_string());
                    player_number.sections[0].value = current_turn.to_string();
                } else {
                    stats.moves_left -= 1;
                }
            }
        }
        _ => {}
    }
}
