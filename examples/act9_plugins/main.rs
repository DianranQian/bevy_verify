//! Act 9: Plugin 插件 — impl Plugin / 模块化。
//!
//! cargo run --example act9_plugins

mod components;
mod plugins;
mod systems;

use bevy::prelude::*;
use plugins::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 注册自定义插件——一行引入所有游戏逻辑
        .add_plugins(GamePlugin)
        .run();
}
