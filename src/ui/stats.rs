use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{
    AlignSelf, Color, Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style,
    TextBundle, TextStyle,
};
use bevy::ui::node_bundles::ButtonBundle;
use bevy::ui::{UiRect, Val};

fn spawn_child(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
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
            parent.spawn(TextBundle::from_section(
                "Moves left",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

pub fn setup_stats(mut commands: Commands) {
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
            spawn_child(parent);
        });
}
