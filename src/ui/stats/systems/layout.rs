use bevy::asset::AssetServer;
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{Commands, NodeBundle, Res, TextBundle};
use bevy::utils::default;
use crate::ui::stats::components::{MovesLeftText, RoundText, TurnText};
use crate::ui::stats::styles::{get_stats_container_style, get_stats_style, get_stats_text_style};
use crate::ui::stats::systems::interaction::{set_player_turn_text, set_round_number_text};
use crate::world::fonts_and_styles::colors::BACKGROUND_COLOR;
use crate::world::player::INITIAL_MOVES;

fn spawn_round_number(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let mut round_text = TextBundle::from_section(
        String::new(), get_stats_text_style(asset_server),
    );
    set_round_number_text(&mut round_text.text, 1);
    parent.spawn(round_text).insert(RoundText);
}

fn spawn_player_turn(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    let mut player_turn_text = TextBundle::from_section(
        String::new(), get_stats_text_style(asset_server),
    );
    set_player_turn_text(&mut player_turn_text.text, 1);

    parent.spawn(player_turn_text).insert(TurnText);
}


pub(in crate::ui::stats) fn setup_stats(mut commands: Commands,
                                        asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: get_stats_style(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: get_stats_container_style(),
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            }).with_children(|parent| {
                spawn_round_number(parent, &asset_server);
                spawn_player_turn(parent, &asset_server);
            });
        });
}

