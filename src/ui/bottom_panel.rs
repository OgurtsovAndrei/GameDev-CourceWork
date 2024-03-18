use bevy::app::{Plugin, Startup, Update};
use bevy::ecs::component::Component;
use bevy::ecs::query::{Changed, With};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::system::{Query, ResMut};
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::prelude::{AlignSelf, ButtonBundle, Color, Commands, FlexDirection, JustifyContent, JustifySelf, NextState, NodeBundle, TextBundle, TextStyle};
use bevy::ui::widget::Button;
use bevy::ui::{Interaction, Style};

use crate::game_state::UpdateUI;
use crate::ui::actions_panel::plugin::TurnSwitchedState;
use crate::world::player::{Movable, Player, Stats};


#[derive(Component)]
pub struct NextMoveButton;

#[derive(Component)]
pub struct PassButton;

pub struct BottomPanelPlugin;

impl Plugin for BottomPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_bottom_panel_buttons)
            .add_systems(
                Update,
                (handle_finish_moves_in_round_button_click).in_set(UpdateUI::UserInput),
            );
    }
}

fn setup_finish_moves_in_round_button(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NextMoveButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Finish moves in round",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ));
        });
}

fn setup_bottom_panel_buttons(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::End,
                justify_self: JustifySelf::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            setup_finish_moves_in_round_button(parent);
        });
}

fn handle_finish_moves_in_round_button_click(
    interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<NextMoveButton>),
    >,
    mut current_player_query: Query<&mut Stats, (With<Player>, With<Movable>)>,
    mut move_done_state: ResMut<NextState<TurnSwitchedState>>
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }
    let interaction = interaction_query.single();
    let mut current_stats = current_player_query.single_mut();
    match *interaction {
        Interaction::Pressed => {
            current_stats.moves_left = 0;
            move_done_state.set(TurnSwitchedState::OnTurnSwitched)
        }
        _ => {}
    }
}
