use bevy::prelude::*;

use crate::ui::hud::components::{EnemyText, ScoreText};
use crate::world::resources::GameResources;

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, game_resources: Res<GameResources>) {
    if game_resources.is_changed() {
        let res = game_resources.clone();
        let resource_text = res.to_string();
        for mut text in text_query.iter_mut() {
            text.sections[1].value = format!("{}", resource_text);
        }
    }
}

pub fn update_enemy_text(
    mut text_query: Query<&mut Text, With<EnemyText>>,
    // enemy_query: Query<Entity, With<Enemy>>,
) {
    // let count = enemy_query.iter().count();
    // for mut text in text_query.iter_mut() {
    //     text.sections[0].value = format!("{}", count.to_string());
    // }
    return;
}
