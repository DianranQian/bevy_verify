//! Act 10: 实战 Mini 游戏 — 玩家移动 + 射击 + 碰撞 + 计分。
//!
//! cargo run --example act10_game

mod components;
mod messages;
mod resources;
mod score_ui;
mod systems;

use bevy::prelude::*;
use messages::BulletFired;
use resources::Score;
use score_ui::{setup_score_display, update_score_display};
use systems::{
    check_collision, cleanup_bullets, move_bullet, move_player, setup, shoot, spawn_bullet,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_message::<BulletFired>()
        .add_systems(Startup, (setup, setup_score_display))
        .add_systems(Update, (
            move_player,
            shoot,
            spawn_bullet,
            move_bullet,
            check_collision,
            cleanup_bullets,
            update_score_display,
        ))
        .run();
}
