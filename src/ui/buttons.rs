use bevy::ecs::component::Component;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{AlignSelf, ButtonBundle, Color, Commands,
    FlexDirection, JustifyContent, JustifySelf, NodeBundle, TextBundle, TextStyle, Val,
};
use bevy::ui::Val::Px;
use bevy::ui::{Style, UiRect};

#[derive(Component)]
pub struct NextMoveButton;

#[derive(Component)]
pub struct PassButton;

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
        .insert(NextMoveButton)
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
        .insert(PassButton)
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
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            setup_next_move_button(parent);
            setup_pass_move_button(parent);
        });
}
