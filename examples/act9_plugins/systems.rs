use bevy::prelude::*;

use crate::components::{Enemy, Health, Player};
use crate::resources::Score;

// ── 通用 Setup（跨插件共享） ──────────────────────────

pub fn setup_common(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Player,
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.3, 0.3), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(100.0, 200.0, 0.0),
        Enemy,
        Health(3),
    ));
}

// ── Player 系统 ──────────────────────────────────────

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 300.0 * time.delta_secs();
    for mut tf in &mut query {
        if input.pressed(KeyCode::KeyW) {
            tf.translation.y += speed;
        }
        if input.pressed(KeyCode::KeyS) {
            tf.translation.y -= speed;
        }
        if input.pressed(KeyCode::KeyA) {
            tf.translation.x -= speed;
        }
        if input.pressed(KeyCode::KeyD) {
            tf.translation.x += speed;
        }
    }
}

// ── Score 系统 ───────────────────────────────────────

pub fn show_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.total);
    }
}
