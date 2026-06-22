//! Act 6: 输入 Input — Res<ButtonInput<KeyCode>> + WASD 移动。
//!
//! cargo run --example act6_input

use bevy::prelude::*;

#[derive(Component)]
struct Player;

fn move_player(
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

// just_pressed 示例：只在按下瞬间触发一次
fn on_space(input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        println!("Space pressed!");
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Player,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, on_space))
        .run();
}
