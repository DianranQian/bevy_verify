use bevy::prelude::*;

use crate::systems::{move_player, setup, show_score};

#[derive(Resource, Default)]
pub struct Score {
    pub total: u32,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, setup)
            .add_systems(Update, (move_player, show_score));
    }
}
