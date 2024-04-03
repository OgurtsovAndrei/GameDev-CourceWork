use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Round {
    pub number: i32,
}

impl Default for Round {
    fn default() -> Self {
        Round { number: 1 }
    }
}