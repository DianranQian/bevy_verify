use bevy::prelude::*;

use crate::components::{Bullet, Enemy, Player};
use crate::messages::BulletFired;
use crate::resources::Score;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    // 玩家
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(0.0, -300.0, 0.0),
        Player,
    ));
    // 敌人
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.3, 0.3), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(100.0, 200.0, 0.0),
        Enemy,
    ));
}

// 玩家移动（Act 6: Input + Act 5: Query）
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

// 射击（Act 8: Message）
pub fn shoot(
    input: Res<ButtonInput<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    mut writer: MessageWriter<BulletFired>,
) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok(tf) = player.single() {
            writer.write(BulletFired {
                position: tf.translation.xy(),
            });
        }
    }
}

pub fn spawn_bullet(
    mut reader: MessageReader<BulletFired>,
    mut commands: Commands,
) {
    for bullet in reader.read() {
        commands.spawn((
            Sprite::from_color(Color::srgb(1.0, 0.9, 0.0), Vec2::new(10.0, 20.0)),
            Transform::from_xyz(bullet.position.x, bullet.position.y, 0.0),
            Bullet { speed: 500.0 },
        ));
    }
}

// 子弹移动
pub fn move_bullet(
    mut query: Query<(&mut Transform, &Bullet)>,
    time: Res<Time>,
) {
    for (mut tf, bullet) in &mut query {
        tf.translation.y += bullet.speed * time.delta_secs();
    }
}

// 清理飞出屏幕的子弹
pub fn cleanup_bullets(
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    mut commands: Commands,
) {
    for (entity, tf) in &bullets {
        if tf.translation.y > 500.0 {
            commands.entity(entity).despawn();
        }
    }
}

// 碰撞检测（Act 7: Resource + Act 5: Query）
pub fn check_collision(
    bullets: Query<(Entity, &Transform), With<Bullet>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    for (b_entity, b_tf) in &bullets {
        for (e_entity, e_tf) in &enemies {
            let dist = b_tf.translation.distance(e_tf.translation);
            if dist < 30.0 {
                commands.entity(b_entity).despawn();
                commands.entity(e_entity).despawn();
                score.total += 100;
            }
        }
    }
}
