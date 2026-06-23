//! Act 9: Plugin 插件 — impl Plugin / PluginGroup / add_plugins / 模块化。
//!
//! cargo run --example act9_plugins

mod components;
mod plugins;
mod resources;
mod systems;

use bevy::prelude::*;
use plugins::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 一行引入全部游戏插件（PluginGroup）
        .add_plugins(GamePlugins)
        .run();
}
