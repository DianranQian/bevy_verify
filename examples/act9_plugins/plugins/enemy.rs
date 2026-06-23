use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        // Enemy 相关系统在此注册
        // 组件已在 components.rs 中定义
    }
}
