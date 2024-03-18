use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{AlignSelf, BuildChildren, ButtonBundle, Color, Commands, default, JustifySelf, NodeBundle, Style, TextBundle, TextStyle, UiRect};
use bevy::prelude::Val::Px;
use crate::ui::action_panel::components::{DebugButton, MoveButton};


fn add_debug_button(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                margin: UiRect {
                    bottom: Px(30.0),
                    ..default()
                },
                ..default()
            },
            ..Default::default()
        })
        .insert(MoveButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Move army to planet",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ));
        });
}

fn add_move_button(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                ..default()
            },
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

pub fn setup_buttons(mut commands: Commands) {
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
                add_debug_button(parent);
                add_move_button(parent)
            }
        });
}