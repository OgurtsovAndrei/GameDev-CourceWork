use bevy::hierarchy::BuildChildren;
use bevy::prelude::{AlignSelf, ButtonBundle, ChildBuilder, Color, Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style, TextBundle, TextStyle};
use crate::ui::bottom_panel::components::NextMoveButton;

fn setup_finish_moves_in_round_button(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NextMoveButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Finish moves in round",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ));
        });
}

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
            setup_finish_moves_in_round_button(parent);
        });
}