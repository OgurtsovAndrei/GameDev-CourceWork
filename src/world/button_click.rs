use bevy::app::{App, Plugin, Update};
use bevy::prelude::{Changed, Entity, IntoSystemConfigs, Query, ResMut, Resource, With};
use bevy::ui::{Interaction};
use crate::game_state::UpdateUI;

pub struct UIClickPlugin;

#[derive(Resource)]
pub struct IsButtonClicked {
    pub value: bool,
}

impl Plugin for UIClickPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IsButtonClicked {
            value: false
        }).add_systems(Update, reset_ui_clicked.in_set(UpdateUI::NewRound));
    }
}

pub fn set_ui_clicked(query: Query<(&Interaction, Entity), Changed<Interaction>>,
                  mut is_button_clicked: ResMut<IsButtonClicked>) {
    if !query.is_empty() {
        is_button_clicked.value = true
    }
}

fn reset_ui_clicked(mut is_button_clicked: ResMut<IsButtonClicked>) {
    is_button_clicked.value = false
}