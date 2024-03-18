use bevy::prelude::{AlignItems, AlignSelf, Display, FlexDirection, JustifyContent, JustifySelf, PositionType, Style, Val};
use bevy::ui::UiRect;
use bevy::utils::default;

pub fn get_actions_menu_style() -> Style {
    Style {
        position_type: PositionType::Absolute,
        display: Display::Flex,
        justify_self: JustifySelf::Start,
        align_self: AlignSelf::Center,
        justify_content: JustifyContent::Center,
        ..Style::DEFAULT
    }
}

pub fn get_actions_menu_container_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect {
            left: Val::Px(20.0),
            right: Val::Px(20.0),
            top: Val::Px(20.0),
            bottom: Val::Px(20.0)
        },
        row_gap: Val::Px(8.0),
        ..default()
    }
}

pub fn get_button_style() -> Style {
    Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(160.0),
        height: Val::Px(60.0),
        ..Style::DEFAULT
    }
}