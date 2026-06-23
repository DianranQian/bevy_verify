use bevy::prelude::*;

// 组件定义 —— 纯数据结构
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health(pub u32);
