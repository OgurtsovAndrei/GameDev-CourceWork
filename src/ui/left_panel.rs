use bevy::{
    app::{Plugin, Startup, Update}, ecs::{
        component::Component, entity::Entity, query::{Changed, With, Without}, schedule::IntoSystemConfigs, system::{Commands, Query, ResMut},
    }, hierarchy::{BuildChildren, ChildBuilder}, log::info, render::color::Color, text::TextStyle, ui::{
        AlignSelf,
        Interaction,
        JustifySelf, node_bundles::{ButtonBundle, NodeBundle, TextBundle}, Style, widget::Button,
    },
};
use bevy::prelude::{NextState, OnEnter, Res, State, States};
use bevy::ui::{AlignContent, UiRect};
use bevy::ui::Val::Px;
use bevy::utils::{default, info};

use crate::{game_state::UpdateUI, world::{player::{Movable, Player, Stats}}};
use crate::game_state::UpdateUI::FlipTurn;
use crate::ui::left_panel::TurnSwitchedState::{OnDefaultState, OnTurnSwitched};
use crate::world::setup_world_grid::clear_selected;

pub struct LeftPanelPlugin;

impl Plugin for LeftPanelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_buttons)
            .add_state::<TurnSwitchedState>()
            .add_systems(Update, handle_dbg_button_click.in_set(UpdateUI::UserInput))
            .add_systems(OnEnter(OnTurnSwitched),
                         (clear_selected,
                          crate::world::actions::clear_action_state).in_set(FlipTurn),
            )
        ;
    }
}

#[derive(Component)]
struct DebugButton;

#[derive(Component)]
struct MoveButton;

fn add_debug_button(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                margin: UiRect {
                    bottom: Px(30.0),
                    ..default()
                },
                ..default()
            },
            ..Default::default()
        })
        .insert(MoveButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Move army to planet",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ));
        });
}

fn add_move_button(parent: &mut ChildBuilder) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                ..default()
            },
            ..Default::default()
        })
        .insert(DebugButton)
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Debug move",
                TextStyle {
                    font: Default::default(),
                    font_size: 40.0,
                    color: Color::BLACK,
                },
            ));
        });
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub(crate) enum TurnSwitchedState {
    #[default]
    OnDefaultState,
    OnTurnSwitched,
}

pub fn reset_turn_switched(current_state: Res<State<TurnSwitchedState>>, mut commands: Commands) {
    match current_state.get() {
        OnTurnSwitched => {
            commands.insert_resource(NextState(Some(OnDefaultState)))
        }
        _OnDefaultState => {}
    }
}

fn handle_dbg_button_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<DebugButton>)>,
    mut current_player_query: Query<&mut Stats, (With<Player>, With<Movable>)>,
    mut turn_switched_state: ResMut<NextState<TurnSwitchedState>>,
) {
    if let Err(_) = interaction_query.get_single() {
        return;
    }

    match interaction_query.single() {
        Interaction::Pressed => {
            let mut cur_stats = current_player_query.single_mut();
            cur_stats.moves_left -= 1;
            turn_switched_state.set(OnTurnSwitched);
        }
        _ => {}
    }
}

fn setup_buttons(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: bevy::ui::FlexDirection::Column,
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Start,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            if cfg!(debug_assertions) {
                add_debug_button(parent);
                add_move_button(parent)
            }
        });
}
