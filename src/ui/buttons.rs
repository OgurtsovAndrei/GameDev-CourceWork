use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{
    default, AlignContent, AlignItems, AlignSelf, BackgroundColor, ButtonBundle, Color, Commands,
    FlexDirection, JustifyContent, JustifySelf, NodeBundle, PositionType, TextBundle, TextStyle,
    Transform, Val,
};
use bevy::ui::Val::{Percent, Px};
use bevy::ui::{Direction, Style, UiRect};

fn setup_next_move_button(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                // Margin to the right for 50px space between buttons
                margin: UiRect {
                    right: Val::Px(25.0), // Half of 50px as space is distributed between the buttons
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Next move",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn setup_pass_move_button(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                margin: UiRect {
                    left: Px(25.0), // Half of 50px for the space
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Pass",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

pub(crate) fn setup_buttons(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::End,
                justify_self: JustifySelf::Center,
                flex_direction: FlexDirection::Row, // Horizontal layout
                justify_content: JustifyContent::SpaceBetween, // This will ensure the spacing
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            setup_next_move_button(parent);
            setup_pass_move_button(parent);
        });
}
