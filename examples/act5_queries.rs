//! Act 5: Query 查询 — Query<&T> / Query<&mut T> / With<T> 过滤器。
//!
//! cargo run --example act5_queries

use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

fn move_player(mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    let speed = 200.0 * time.delta_secs();
    for mut tf in &mut query {
        tf.translation.x += speed;
    }
}

fn print_enemies(query: Query<&Transform, With<Enemy>>) {
    for tf in &query {
        println!("Enemy at: ({:.1}, {:.1})", tf.translation.x, tf.translation.y);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Player 实体
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(-200.0, 0.0, 0.0),
        Player,
    ));

    // Enemy 实体
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.3, 0.3), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(200.0, 0.0, 0.0),
        Enemy,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, print_enemies))
        .run();
}
