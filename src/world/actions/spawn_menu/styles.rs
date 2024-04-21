use bevy::prelude::*;

pub fn get_spawn_menu_style() -> Style {
    Style {
        position_type: PositionType::Absolute, // Needed to display separately from HUD.
        display: Display::Flex,                // Hidden by Default
        justify_self: JustifySelf::End,
        align_self: AlignSelf::FlexStart,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        // width: Val::Percent(180.),
        height: Val::Percent(70.),
        ..Style::DEFAULT
    }
}

pub fn get_spawn_menu_container_style() -> Style {
    Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(250.0),
        height: Val::Px(400.0),
        row_gap: Val::Px(8.0),
        column_gap: Val::Px(8.0),
        ..Style::DEFAULT
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
