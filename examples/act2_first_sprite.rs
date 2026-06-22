//! Act 2: 第一个精灵 — Camera2d + Sprite::from_color。
//!
//! cargo run --example act2_first_sprite

use bevy::prelude::*;

fn setup(mut commands: Commands) {
    // 必须有一个 Camera2d 才能看到画面
    commands.spawn(Camera2d);

    // 创建一个纯色方块精灵（100×100 像素）
    commands.spawn(Sprite::from_color(
        Color::srgb(0.2, 0.6, 1.0),
        Vec2::new(100.0, 100.0),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
