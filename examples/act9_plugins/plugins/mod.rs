pub mod enemy;
pub mod player;
pub mod score;

use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

// PluginGroup —— 打包一组插件（DefaultPlugins 也是 PluginGroup）
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::new::<Self>()
            .add(PlayerPlugin)
            .add(EnemyPlugin)
            .add(ScorePlugin)
    }
}
