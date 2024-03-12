use bevy::{
    app::{Plugin, Startup, Update}, ecs::{
        component::Component, entity::Entity, query::{Changed, With, Without}, schedule::IntoSystemConfigs, system::{Commands, Query, ResMut}
    }, hierarchy::{BuildChildren, ChildBuilder}, log::info, render::color::Color, text::TextStyle, ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignSelf, Interaction, JustifySelf, Style,
    }
};

use crate::{game_state::UpdateUI, world::{player::{Movable, Player, Stats}, turn::TurnDone}};

pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_buttons)
            .add_systems(Update, (handle_dbg_button_click).in_set(UpdateUI::UserInput));
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
                    color: Color::BLACK,
                },
            ));
        });
}

fn handle_dbg_button_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<DebugButton>)>,
    mut current_player_query: Query<&mut Stats, (With<Player>, With<Movable>)>,
    mut turn_done_res : ResMut<TurnDone>
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }

    match interaction_query.single() {
        Interaction::Pressed => {
            let mut cur_stats = current_player_query.single_mut();
            cur_stats.moves_left -= 1;
            let f = turn_done_res.as_mut();
            f.value = true;
        }
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
