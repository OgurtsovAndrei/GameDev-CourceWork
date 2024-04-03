use bevy::asset::AssetServer;
use bevy::prelude::{AlignItems, Color, Display, FlexDirection, JustifyContent, Res, Style, TextStyle};
use bevy::ui::JustifySelf;

pub(in crate::ui::stats) fn get_stats_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_self: JustifySelf::Center,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}

pub(in crate::ui::stats) fn get_stats_container_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_self: JustifySelf::Center,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..Style::DEFAULT
    }
}


pub(in crate::ui::stats) fn get_stats_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 35.0,
        color: Color::rgb(1.0, 1.0, 1.0),
    }
}