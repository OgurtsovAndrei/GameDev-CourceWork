use bevy::asset::AssetServer;
use bevy::prelude::{Color, JustifyContent, Res, TextStyle};
use bevy::ui::{AlignSelf, FlexDirection, JustifySelf, Style};
use bevy::utils::default;

pub(in crate::ui::game_result) fn get_panel_style() -> Style {
    return Style {
        display: Default::default(),
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::SpaceBetween,
        ..default()
    };
}


pub fn get_result_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 40.0,
        color: Color::rgb(1.0, 1.0, 0.0),
    }
}