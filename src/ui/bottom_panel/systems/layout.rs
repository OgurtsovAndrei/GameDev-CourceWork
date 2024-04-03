use bevy::hierarchy::BuildChildren;
use bevy::prelude::{AlignSelf, Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style, TextBundle, TextStyle};

pub fn setup_bottom_panel_buttons(mut commands: Commands) {
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
        });
}