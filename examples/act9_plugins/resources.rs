use bevy::prelude::*;

// 资源定义 —— 全局唯一
#[derive(Resource, Default)]
pub struct Score {
    pub total: u32,
}
