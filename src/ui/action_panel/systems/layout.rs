use bevy::asset::AssetServer;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::{BuildChildren, ButtonBundle, Commands, default, NodeBundle, Res, TextBundle};
use bevy::ui::Interaction;

use crate::ui::action_panel::components::{ActionPanel, HireArmyButton, SkipRoundButton, OpenMovePanelButton};
use crate::ui::action_panel::styles::{get_actions_menu_container_style, get_actions_menu_style, get_button_style};
use crate::world::fonts_and_styles::colors::*;
use crate::world::fonts_and_styles::fonts::*;

fn add_hire_army_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(ButtonBundle {
            style: get_button_style(),
            background_color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(HireArmyButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Spawn menu",
                get_button_text_style(asset_server),
            ));
        });
}

fn add_open_move_panel_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(ButtonBundle {
            style: get_button_style(),
            background_color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(OpenMovePanelButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Move menu",
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

fn add_finish_moves_in_round_button(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(ButtonBundle {
            style: get_button_style(),
            background_color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(SkipRoundButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Skip round",
                get_button_text_style(asset_server),
            ));
        });
}

pub fn setup_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
            style: get_actions_menu_style(),
            ..Default::default()
        },
                ActionPanel, Interaction::None))
        .with_children(|parent| {
            parent.spawn((NodeBundle {
                style: get_actions_menu_container_style(),
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }, Interaction::None)).with_children(|parent| {
                add_title(parent, &asset_server);
                add_hire_army_button(parent, &asset_server);
                add_open_move_panel_button(parent, &asset_server);
                add_finish_moves_in_round_button(parent, &asset_server);
            });
        });
}