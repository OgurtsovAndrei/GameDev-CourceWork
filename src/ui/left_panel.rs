use bevy::{app::{Plugin, Startup}, ecs::{component::Component, system::Commands}, hierarchy::{BuildChildren, ChildBuilder}, render::color::Color, text::TextStyle, ui::{node_bundles::{ButtonBundle, NodeBundle, TextBundle}, AlignSelf, JustifySelf, Style}};

pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_buttons);
    }
}

#[derive(Component)]
struct DebugButton;


fn add_debug_buttons(parent : &mut ChildBuilder) {
    parent.spawn(ButtonBundle {
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

fn setup_buttons(mut commands: Commands) {
    commands.spawn(NodeBundle {
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
