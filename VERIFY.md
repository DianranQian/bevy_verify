# Bevy 代码验证 — 运行命令

## 逐个检查

```bash
cargo check --example act1_hello
cargo check --example act2_first_sprite
cargo check --example act3_components
cargo check --example act4_systems
cargo check --example act5_queries
cargo check --example act6_input
cargo check --example act7_resources
cargo check --example act8_message
cargo check --example act9_plugins
cargo check --example act10_game
```

## 批量检查

```bash
for ex in act1_hello act2_first_sprite act3_components act4_systems act5_queries act6_input act7_resources act8_message act9_plugins act10_game; do
    echo "=== $ex ===" && cargo check --example $ex 2>&1 | tail -1
done
```

## 检查主文件

```bash
cargo check
```

## 逐个运行

```bash
cargo run --example act1_hello
cargo run --example act2_first_sprite
cargo run --example act3_components
cargo run --example act4_systems
cargo run --example act5_queries
cargo run --example act6_input
cargo run --example act7_resources
cargo run --example act8_message
cargo run --example act9_plugins
cargo run --example act10_game
```
