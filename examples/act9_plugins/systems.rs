use bevy::prelude::*;

use crate::components::Player;
use crate::plugins::game::Score;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Player,
    ));
}

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

pub fn show_score(score: Res<Score>) {
    println!("Score: {}", score.total);
}
