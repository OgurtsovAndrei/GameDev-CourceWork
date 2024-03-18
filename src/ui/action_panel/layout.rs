use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{AlignSelf, BuildChildren, ButtonBundle, Color, Commands, default, JustifySelf, NodeBundle, Res, Style, TextBundle, TextStyle, UiRect};
use bevy::prelude::Val::Px;
use bevy::utils::tracing::field::DebugValue;

use crate::ui::action_panel::components::{ActionPanel, DebugButton, MoveButton};
use crate::ui::action_panel::styles;
use crate::ui::action_panel::styles::{get_actions_menu_container_style, get_actions_menu_style, get_button_style};
use crate::world::fonts_and_styles::colors::*;
use crate::world::fonts_and_styles::fonts::*;

fn add_debug_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(ButtonBundle {
            style: get_button_style(),
            background_color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(MoveButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Capture army",
                get_button_text_style(asset_server),
            ));
        });
}

fn add_move_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(ButtonBundle {
            style: get_button_style(),
            background_color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(DebugButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Debug move",
                get_button_text_style(asset_server),
            ));
        });
}

fn add_title(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn(TextBundle::from_section(
        "Actions",
        get_title_text_style(asset_server),
    ));
}

pub fn setup_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: get_actions_menu_style(),
            ..Default::default()
        },
                ActionPanel))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: get_actions_menu_container_style(),
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }).with_children(|parent| {
                add_title(parent, &asset_server);
                add_move_button(parent, &asset_server);
                if cfg!(debug_assertions) {
                    add_debug_button(parent, &asset_server);
                }
            });
        });
}