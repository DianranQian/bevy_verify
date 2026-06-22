use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct Score {
    pub total: u32,
}
