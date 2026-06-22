//! Bevy 编程入门 — 代码合集（各章节独立示例见 examples/）

use bevy::prelude::*;

// ── 组件 ────────────────────────────────────────────────────────────────

#[derive(Component, Debug)]
struct Player;

#[derive(Component, Debug)]
struct Health {
    current: f32,
}

// ── 资源 ────────────────────────────────────────────────────────────────

#[derive(Resource, Default, Debug)]
struct Score {
    total: u32,
}

// ── 消息 ────────────────────────────────────────────────────────────────

#[derive(Message, Debug)]
struct PlayerDied {
    score: u32,
}

// ── 系统 ────────────────────────────────────────────────────────────────

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Player,
        Health { current: 100.0 },
    ));
}

fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 300.0 * time.delta_secs();
    for mut tf in &mut query {
        if input.pressed(KeyCode::KeyW) { tf.translation.y += speed; }
        if input.pressed(KeyCode::KeyS) { tf.translation.y -= speed; }
        if input.pressed(KeyCode::KeyA) { tf.translation.x -= speed; }
        if input.pressed(KeyCode::KeyD) { tf.translation.x += speed; }
    }
}

fn add_score(mut score: ResMut<Score>) {
    score.total += 10;
}

fn show_score(score: Res<Score>) {
    println!("Score: {}", score.total);
}

fn death_system(
    mut writer: MessageWriter<PlayerDied>,
    query: Query<&Health, With<Player>>,
) {
    for health in &query {
        if health.current <= 0.0 {
            writer.write(PlayerDied { score: 100 });
        }
    }
}

fn score_system(mut reader: MessageReader<PlayerDied>, mut score: ResMut<Score>) {
    for death in reader.read() {
        score.total += death.score;
    }
}

// ── 入口 ────────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_message::<PlayerDied>()
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, add_score, show_score, death_system, score_system))
        .run();
}
