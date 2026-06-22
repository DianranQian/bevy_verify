//! Act 1: Hello Bevy — 三行代码启动 Bevy 应用。
//!
//! cargo run --example act1_hello

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
