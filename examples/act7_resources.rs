//! Act 7: Resource 资源 — Res<T> / ResMut<T> / init_resource / insert_resource。
//!
//! cargo run --example act7_resources

use bevy::prelude::*;

// 1. 定义资源类型（Default trait 支持 init_resource）
#[derive(Resource, Default, Debug)]
struct Score {
    total: u32,
    multiplier: f32,
}

// 2. 只读访问：Res<T>
fn show_score(score: Res<Score>) {
    println!("Score: {} (x{:.1})", score.total, score.multiplier);
}

// 3. 可写访问：ResMut<T>
fn add_score(mut score: ResMut<Score>) {
    score.total += 100;
    score.multiplier += 0.1;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 4. 初始化资源（方式一：默认值）
        .init_resource::<Score>()
        // 方式二：自定义初始值
        // .insert_resource(Score { total: 0, multiplier: 1.0 })
        .add_systems(Update, (add_score, show_score))
        .run();
}
