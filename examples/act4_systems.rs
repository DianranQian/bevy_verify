//! Act 4: 系统 System — fn 函数 + add_systems 注册。
//!
//! cargo run --example act4_systems

use bevy::prelude::*;

fn hello_system() {
    println!("Hello from Bevy system!");
}

fn count_system(time: Res<Time>) {
    // 每帧打印一次（运行 -qh 模式会更明显）
    println!("Delta time: {:.4}s", time.delta_secs());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 多个系统注册到同一个 Schedule
        .add_systems(Update, (hello_system, count_system))
        .run();
}
