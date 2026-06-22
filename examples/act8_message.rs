//! Act 8: Message 消息系统 — MessageWriter / MessageReader / add_message。
//!
//! cargo run --example act8_message

use bevy::prelude::*;

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
    // 模拟：条件满足时发送消息
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        // 4. 注册消息类型（必须先注册才能使用）
        .add_message::<PlayerDied>()
        // 系统顺序：Writer 在 Reader 之前
        .add_systems(Update, (death_system, score_system))
        .run();
}
