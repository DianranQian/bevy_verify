//! Act 8: Message 消息系统 — MessageWriter / MessageReader / EntityEvent。
//!
//! cargo run --example act8_message

use bevy::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// Part 1-4: 全局 Message —— MessageWriter / MessageReader
// ═══════════════════════════════════════════════════════════════════════

// 1. 定义消息类型
#[derive(Message, Debug)]
struct PlayerDied {
    score: u32,
}

// 消息携带的数据接收方
#[derive(Resource, Default)]
struct Score {
    total: u32,
}

// 2. 发送消息：MessageWriter<T> + write()
fn death_system(mut writer: MessageWriter<PlayerDied>) {
    writer.write(PlayerDied { score: 100 });
    println!("Sent: PlayerDied {{ score: 100 }}");
}

// 3. 接收消息：MessageReader<T> + read()
fn score_system(mut reader: MessageReader<PlayerDied>, mut score: ResMut<Score>) {
    for death in reader.read() {
        score.total += death.score;
        println!("Received: score += {} → total = {}", death.score, score.total);
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Part 5: EntityEvent —— 绑定到特定实体的 Observer
// ═══════════════════════════════════════════════════════════════════════

#[derive(Event, Debug)]
struct EntityDied {
    points: u32,
}

// 实体上注册 observer：当该实体收到 EntityDied 事件时触发
fn spawn_entity(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Enemy"),
            Transform::default(),
        ))
        .observe(|trigger: Trigger<EntityDied>| {
            println!(
                "Entity died! +{} points → entity: {:?}",
                trigger.event().points,
                trigger.entity(),
            );
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        // 4. 注册消息类型（必须先注册才能使用）
        .add_message::<PlayerDied>()
        // 系统顺序：Writer 在 Reader 之前
        .add_systems(Startup, spawn_entity)
        .add_systems(Update, (death_system, score_system))
        .run();
}
