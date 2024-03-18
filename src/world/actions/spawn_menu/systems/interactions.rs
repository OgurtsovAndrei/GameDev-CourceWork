use bevy::prelude::*;

use crate::space_ships::{SpaceShip, SpaceShipType};
use crate::world::actions::ActionsState;
use crate::world::actions::spawn_menu::components::{EndSpawnButton, SpawnShip1Button};
use crate::world::fonts_and_styles::colors::*;
use crate::world::player::{Movable, Player};
use crate::world::resources::GameResources;
use crate::world::setup_world_grid::{HexGrid, SelectedHex};

pub fn interact_with_end_spawn_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EndSpawnButton>),
    >,
    mut simulation_state_next_state: ResMut<NextState<ActionsState>>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                simulation_state_next_state.set(ActionsState::NoActionRunning);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn interact_with_spawn_ship1_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SpawnShip1Button>),
    >,
    mut resources: ResMut<GameResources>,
    mut grid: ResMut<HexGrid>,
    mut selected_hex: ResMut<SelectedHex>,
    current_player_query: Query<&Player, (With<Player>, With<Movable>)>,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        let player = current_player_query.single();
        let mut player_resources = &resources.resources[player];
        let space_ship_cost: u32 = 5;
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                if !selected_hex.is_selected { return; }
                if player_resources.resources < space_ship_cost { return; }
                let mut player_resources = resources.resources.remove(player).unwrap();
                player_resources.resources -= space_ship_cost;
                resources.resources.insert(player.clone(), player_resources);
                let mut planet = grid.planets.remove(&selected_hex.hex).unwrap();
                planet.owner_army.push(SpaceShip {
                    ship_type: SpaceShipType::Destroyer,
                    ship_owner: player.clone(),
                    ship_hex: selected_hex.hex.clone(),
                    is_selected_for_move: false,
                });
                grid.planets.insert(selected_hex.hex.clone(), planet);
                resources.set_changed();
                grid.set_changed();
                selected_hex.set_changed();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
/*
pub fn interact_with_quit_button(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    for (interaction, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}*/
