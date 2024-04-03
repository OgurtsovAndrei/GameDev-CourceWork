use bevy::asset::AssetServer;
use bevy::hierarchy::BuildChildren;
use bevy::prelude::{ChildBuilder, Commands, NodeBundle, Res, TextBundle};
use bevy::prelude::Visibility::Hidden;
use bevy::text::TextSection;
use bevy::utils::default;
use crate::ui::game_result::components::{ResultPanel, WinnerText};
use crate::ui::game_result::styles::{get_panel_style, get_result_text_style};


fn setup_game_over_text(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn(TextBundle::from_section(
        "Game over",
        get_result_text_style(asset_server),
    )
    );
}

fn setup_winner_text(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent.spawn(TextBundle::from_sections([
        TextSection {
            value: "Winner is: ".to_string(),
            style: get_result_text_style(asset_server),
        },
        TextSection {
            value: "None".to_string(),
            style: get_result_text_style(asset_server),
        }
    ]
    ))
        .insert(WinnerText);
}

pub(in crate::ui::game_result) fn setup_result_panel(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(NodeBundle {
        style: get_panel_style(),
        visibility: Hidden,
        ..default()
    }).insert(ResultPanel)
        .with_children(|parent| {
            setup_game_over_text(parent, &asset_server);
            setup_winner_text(parent, &asset_server);
        });
}