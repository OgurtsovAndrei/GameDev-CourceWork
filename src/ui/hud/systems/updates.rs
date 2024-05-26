use bevy::prelude::*;

use crate::ui::hud::components::{HudShortcutTipsText, HudTipsText, ScoreText};
use crate::ui::hud::resources::{MOVE_MENU_SELECTED_HEX_HUD_TEXT, MOVE_MENU_SELECTED_HEX_SHORTCUT_TEXT, MOVE_MENU_UNSELECTED_HEX_HUD_TEXT, MOVE_MENU_UNSELECTED_HEX_SHORTCUT_TEXT, NO_ACTION_RUNNING_SELECTED_HEX_HUD_TEXT, NO_ACTION_RUNNING_SELECTED_HEX_SHORTCUT_TEXT, NO_ACTION_RUNNING_UNSELECTED_HEX_HUD_TEXT, NO_ACTION_RUNNING_UNSELECTED_HEX_SHORTCUT_TEXT, SPAWN_MENU_HUD_TEXT, SPAWN_MENU_SHORTCUT_TEXT};
use crate::world::actions::ActionsState;
use crate::world::player::{Player, Stats};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::SelectedHex;

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>,
    game_resources: Res<GameResources>,
    player_query: Query<(&Player, &Stats)>,
) {
    if game_resources.is_changed() {
        let res = game_resources.clone();
        let resource_text = res.to_string();
        for mut text in text_query.iter_mut() {
            let winning_points_text = get_wnning_points_str(&player_query);
            text.sections[1].value = format!("{}", resource_text);
            text.sections[3].value = winning_points_text.clone()
        }
    }
}

fn get_wnning_points_str(player_query: &Query<(&Player, &Stats)>) -> String {
    let mut winning_points_text = "".to_string();
    for (player, stats) in player_query.iter() {
        winning_points_text += &*format!("Player {} -> {}\n", player.id, stats.win_points).to_string()
    }
    winning_points_text
}

pub fn update_hud_text(
    mut text_query: Query<&mut Text, With<HudTipsText>>,
    selected_hex: Res<SelectedHex>,
    state: Res<State<ActionsState>>,
) {
    let mut text = text_query.single_mut();
    match state.get() {
        ActionsState::NoActionRunning => {
            text.sections[0].value = if selected_hex.is_selected {
                NO_ACTION_RUNNING_SELECTED_HEX_HUD_TEXT.to_string()
            } else {
                NO_ACTION_RUNNING_UNSELECTED_HEX_HUD_TEXT.to_string()
            }
        }
        ActionsState::SpawningSpaceShips => {
            debug_assert!(selected_hex.is_selected);
            text.sections[0].value = SPAWN_MENU_HUD_TEXT.to_string()
        }
        ActionsState::MovingSpaceShips => {
            debug_assert!(selected_hex.is_selected);
            text.sections[0].value = if selected_hex.is_selected_for_move {
                MOVE_MENU_SELECTED_HEX_HUD_TEXT.to_string()
            } else { MOVE_MENU_UNSELECTED_HEX_HUD_TEXT.to_string() }
        }
    }
}

pub fn update_shortcuts_hud_text(
    mut text_query: Query<&mut Text, With<HudShortcutTipsText>>,
    selected_hex: Res<SelectedHex>,
    state: Res<State<ActionsState>>,
) {
    let mut text = text_query.single_mut();
    match state.get() {
        ActionsState::NoActionRunning => {
            text.sections[0].value = if selected_hex.is_selected {
                NO_ACTION_RUNNING_SELECTED_HEX_SHORTCUT_TEXT.to_string()
            } else {
                NO_ACTION_RUNNING_UNSELECTED_HEX_SHORTCUT_TEXT.to_string()
            }
        }
        ActionsState::SpawningSpaceShips => {
            debug_assert!(selected_hex.is_selected);
            text.sections[0].value = SPAWN_MENU_SHORTCUT_TEXT.to_string()
        }
        ActionsState::MovingSpaceShips => {
            debug_assert!(selected_hex.is_selected);
            text.sections[0].value = if selected_hex.is_selected_for_move {
                MOVE_MENU_SELECTED_HEX_SHORTCUT_TEXT.to_string()
            } else { MOVE_MENU_UNSELECTED_HEX_SHORTCUT_TEXT.to_string() }
        }
    }
}