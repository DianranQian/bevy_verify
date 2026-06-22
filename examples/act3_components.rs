//! Act 3: 组件 Component — #[derive(Component)] + spawn 附加。
//!
//! cargo run --example act3_components

use bevy::prelude::*;

#[derive(Component, Debug)]
struct Player {
    speed: f32,
}

#[derive(Component, Debug)]
struct Health {
    current: f32,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // spawn 时附加多个组件（Bundle 元组）
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(100.0, 0.0, 0.0),
        Player { speed: 300.0 },
        Health { current: 100.0 },
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
