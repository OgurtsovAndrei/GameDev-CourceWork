use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{
        component::Component,
        query::{Changed, With},
        system::{Commands, Query, Res},
    },
    hierarchy::{BuildChildren, ChildBuilder},
    render::color::Color,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignSelf, Interaction, JustifySelf, Style,
    },
};

use crate::world::player::{Player, Stats, Turn};

use super::stats::{self, MovesLeftText};

pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_buttons)
                .add_systems(Update, handle_dbg_button_click);
    }
}

#[derive(Component)]
struct DebugButton;

fn add_debug_buttons(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            ..Default::default()
        })
        .insert(DebugButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Debug move",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn handle_dbg_button_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<DebugButton>)>,
    mut player_query: Query<(&Player, &mut Stats, &Turn)>,
    mut moves_left_text_query: Query<&mut Text, With<MovesLeftText>>,
    turn_query: Res<Turn>
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }
    if let Err(_) = moves_left_text_query.get_single_mut() {
        return;
    }

    match interaction_query.single() {
        Interaction::Pressed => {
            let (_, mut stats, _) = player_query.iter_mut().find(|(_, _, turn)| **turn == *turn_query).unwrap();
            stats.moves_left-=1;
            let mut text = moves_left_text_query.single_mut();
            stats::updates_moves_left_text(&mut text, stats.moves_left);
        },
        _ => {}
    }
}

fn setup_buttons(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: bevy::ui::FlexDirection::Column,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Start,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            if cfg!(debug_assertions) {
                add_debug_buttons(parent);
            }
        });
}
