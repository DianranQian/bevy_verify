use bevy::prelude::*;

use crate::resources::Score;
use crate::systems::{setup_common, show_score};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, setup_common)
            .add_systems(Update, show_score);
    }
}
