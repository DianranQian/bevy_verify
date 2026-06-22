use bevy::prelude::*;

#[derive(Message)]
pub struct BulletFired {
    pub position: Vec2,
}
