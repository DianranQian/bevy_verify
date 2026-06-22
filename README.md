# Bevy Verify

Bevy 0.19 ECS 教学代码验证项目，配套《Bevy 编程入门》动画教程。

## 运行环境

- Rust 1.85+
- Bevy 0.19

## 示例清单

| Act | 示例 | 知识点 |
|-----|------|--------|
| 1 | `act1_hello` | `App::new()` + `add_plugins` + `run()` |
| 2 | `act2_first_sprite` | `Camera2d` + `Sprite` + `commands.spawn` |
| 3 | `act3_components` | `#[derive(Component)]` + `Transform` |
| 4 | `act4_systems` | `fn system()` + `add_systems` + 参数注入 |
| 5 | `act5_queries` | `Query<&T>` + `&mut` + `With<T>` |
| 6 | `act6_input` | `Res<ButtonInput<KeyCode>>` + WASD |
| 7 | `act7_resources` | `#[derive(Resource)]` + `ResMut` |
| 8 | `act8_message` | `MessageWriter` + `MessageReader` + `add_message` |
| 9 | `act9_plugins` | `impl Plugin` + 模块化目录 |
| 10 | `act10_game` | 射击游戏 — 移动、子弹、碰撞、计分、UI |

## 运行

```bash
# 编译检查
cargo check --example act10_game

# 运行
cargo run --example act1_hello
cargo run --example act10_game
```

## 批量检查

```bash
for ex in act1_hello act2_first_sprite act3_components act4_systems act5_queries act6_input act7_resources act8_message act9_plugins act10_game; do
    echo "=== $ex ===" && cargo check --example $ex 2>&1 | tail -1
done
```
