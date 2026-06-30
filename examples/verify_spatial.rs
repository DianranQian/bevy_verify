//! Bevy 空间 API 验证 — Transform / GlobalTransform / ChildOf / Children。
//!
//! cargo run --example verify_spatial

use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (verify_transform, verify_global, verify_hierarchy))
        .run();
}

// ═══ Transform ═══

fn verify_transform() {
    println!("\n========== Transform ==========");

    // from_xyz — 从坐标创建
    let t = Transform::from_xyz(1.0, 2.0, 0.0);
    println!("from_xyz(1,2,0)      → translation={:?}", t.translation);

    // from_rotation — 从四元数创建
    let q = Quat::from_rotation_z(PI / 4.0);
    let t2 = Transform::from_rotation(q);
    println!("from_rotation(45°)   → rotation={:?}", t2.rotation);

    // from_scale
    let t_s = Transform::from_scale(Vec3::splat(2.0));
    println!("from_scale(2)        → scale={:?}", t_s.scale);

    // with_translation — 返回新实例，不修改原值
    let t3 = t.with_translation(Vec3::new(5.0, 0.0, 0.0));
    println!("with_translation(5,0)→ {:?}  (原值: {:?})", t3.translation, t.translation);

    // with_scale
    let t4 = t.with_scale(Vec3::splat(3.0));
    println!("with_scale(3)        → scale={:?}", t4.scale);

    // with_rotation
    let t5 = t.with_rotation(q);
    println!("with_rotation(45°)   → rotation={:?}", t5.rotation);

    // rotate_axis — 增量旋转，就地修改
    let mut t6 = Transform::default();
    t6.rotate_axis(Dir3::Z, PI / 6.0);
    println!("rotate_axis(Z, 30°)  → rotation={:?}", t6.rotation);
    t6.rotate_axis(Dir3::Z, PI / 6.0);
    println!("  再 rotate_axis 30°  → rotation={:?} (叠加)", t6.rotation);

    // rotate — 通过四元数增量旋转
    let mut t7 = Transform::default();
    t7.rotate(Quat::from_rotation_z(PI / 2.0));
    println!("rotate(90° Z)        → rotation={:?}", t7.rotation);

    // look_at
    let mut t8 = Transform::IDENTITY;
    t8.look_at(Vec3::new(0.0, 1.0, 0.0), Dir3::Y);
    println!("look_at(0,1,0)       → rotation={:?}", t8.rotation);

    // transform_point — 局部坐标 → 父空间坐标
    let t9 = Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_z(PI / 4.0));
    let local_pt = Vec3::new(1.0, 0.0, 0.0);
    let parent_pt = t9.transform_point(local_pt);
    println!(
        "transform_point: 局部 {:?} + rot45° → 父空间 {:?}",
        local_pt, parent_pt,
    );

    // translate_around — 绕某点旋转
    let mut t10 = Transform::from_xyz(2.0, 0.0, 0.0);
    t10.translate_around(Vec3::ZERO, Quat::from_rotation_z(PI / 2.0));
    println!("translate_around: (2,0) 绕原点90° → {:?}", t10.translation);

    // mul_transform — 组合：父 Transform × 子 Transform = 全局 Transform
    let parent_t = Transform::from_xyz(1.0, 0.0, 0.0);
    let child_t = Transform::from_xyz(2.0, 0.0, 0.0);
    let combined = parent_t.mul_transform(child_t);
    println!("parent(1,0) * child(2,0) → {:?}", combined.translation);

    // 四元数旋转向量验证
    let pt = Vec3::new(1.0, 0.0, 0.0);
    let rotation = Quat::from_rotation_z(PI / 2.0);
    let rotated = rotation * pt;
    println!(
        "Quat 旋转: {:?} 绕 Z 90° → {:?} (预期 ≈ (0,1,0))",
        pt, rotated,
    );
}

// ═══ GlobalTransform ═══

fn verify_global() {
    println!("\n========== GlobalTransform ==========");

    // from_translation
    let gt = GlobalTransform::from_translation(Vec3::new(3.0, 4.0, 0.0));
    let (_scale, _rotation, translation) = gt.to_scale_rotation_translation();
    println!("from_translation(3,4) → {translation:?}");

    // translation / rotation / scale — 只读访问
    println!("translation()  → {:?}", gt.translation());
    println!("rotation()     → {:?}", gt.rotation());
    println!("scale()        → {:?}", gt.scale());

    // 手动构造含 scale + rotation + translation 的 GlobalTransform
    let gt2 = GlobalTransform::from_translation(Vec3::new(1.0, 0.0, 0.0))
        * GlobalTransform::from_rotation(Quat::from_rotation_z(PI / 4.0))
        * GlobalTransform::from_scale(Vec3::splat(2.0));
    println!(
        "组合 GT: pos={:?} rot={:?} scale={:?}",
        gt2.translation(),
        gt2.rotation(),
        gt2.scale(),
    );

    // to_matrix → Mat4
    let mat = gt2.to_matrix();
    let (_s, _r, _t) = mat.to_scale_rotation_translation();
    println!("to_matrix → Mat4 → rot={_r:?} pos={_t:?}");

    // compute_transform → 从世界矩阵反推局部 Transform
    let local = gt2.compute_transform();
    println!(
        "compute_transform → translation={:?} rotation={:?} scale={:?}",
        local.translation, local.rotation, local.scale,
    );

    // transform_point — 局部点 → 世界空间
    let local_p = Vec3::new(1.0, 0.0, 0.0);
    let world_p = gt2.transform_point(local_p);
    println!("transform_point({local_p:?}) → world {world_p:?}");

    // reparented_to — 换父实体，保持世界位置不变
    let new_parent = GlobalTransform::from_translation(Vec3::new(5.0, 0.0, 0.0));
    let new_local = gt2.reparented_to(&new_parent);
    println!(
        "reparented_to: 原世界位置 {:?} → 新局部 {:?}",
        gt2.translation(),
        new_local.translation,
    );
    let recomputed = new_parent.mul_transform(new_local);
    println!("  验证: 新父 × 新局部 → {:?} (应 ≈ 原世界位置)", recomputed.translation());
}

// ═══ ChildOf / Children ═══

fn verify_hierarchy(world: &mut World) {
    println!("\n========== ChildOf / Children ==========");

    // Helper: get entity name
    fn name_of(world: &World, entity: Entity) -> String {
        world.get::<Name>(entity).map(|n| n.to_string()).unwrap_or_default()
    }

    // 1. spawn + ChildOf — 一步指定父实体
    let parent = world
        .spawn((Name::new("Parent"), Transform::from_xyz(10.0, 0.0, 0.0)))
        .id();
    world.spawn((
        Name::new("Child1"),
        ChildOf(parent),
        Transform::from_xyz(5.0, 0.0, 0.0),
    ));
    println!("1. spawn(ChildOf): parent={parent:?}");

    // 2. with_children — 批量生成子实体
    let parent2 = world
        .spawn((Name::new("Parent2"), Transform::default()))
        .id();
    world.entity_mut(parent2).with_children(|builder| {
        builder.spawn((Name::new("C2_a"), Transform::from_xyz(1.0, 0.0, 0.0)));
        builder.spawn((Name::new("C2_b"), Transform::from_xyz(2.0, 0.0, 0.0)));
    });
    println!("2. with_children: parent2={parent2:?} 下批量创建 2 个子实体");

    // 验证 Children 列表
    if let Some(c) = world.get::<Children>(parent2) {
        println!("   Children 列表 ({} 个):", c.len());
        for (i, child) in c.iter().enumerate() {
            println!("     [{i}] = {child:?} ({})", name_of(world, child));
        }
    }

    // 3. add_child — 动态附加已有实体
    let orphan = world
        .spawn((Name::new("Orphan"), Transform::from_xyz(0.0, 5.0, 0.0)))
        .id();
    world.entity_mut(parent2).add_child(orphan);
    println!("3. add_child: {orphan:?} → parent2");

    // 4. insert_child — 在指定索引插入
    let new_first = world
        .spawn((Name::new("FirstBorn"), Transform::default()))
        .id();
    world.entity_mut(parent2).insert_child(0, new_first);
    println!("4. insert_child(0): {new_first:?} 插入到索引 0");

    // 查看当前顺序
    if let Some(c) = world.get::<Children>(parent2) {
        print!("   当前顺序: ");
        for child in c.iter() {
            print!("{} ", name_of(world, child));
        }
        println!();
    }

    // 5. swap — 交换两个位置的子实体
    {
        let mut c = world.get_mut::<Children>(parent2).unwrap();
        if c.len() >= 3 {
            c.swap(1, 2);
            println!("5. swap(1, 2): 交换索引 1 和 2");
        }
    }
    if let Some(c) = world.get::<Children>(parent2) {
        print!("   交换后: ");
        for child in c.iter() {
            print!("{} ", name_of(world, child));
        }
        println!();
    }

    // 6. sort_by — 稳定排序
    {
        let mut c = world.get_mut::<Children>(parent2).unwrap();
        c.sort_by(|a, b| a.index().cmp(&b.index()));
        println!("6. sort_by(index): 按 Entity::index 排序");
    }
    if let Some(c) = world.get::<Children>(parent2) {
        print!("   排序后: ");
        for child in c.iter() {
            print!("{} ", name_of(world, child));
        }
        println!();
    }

    // 7. sort_by_key — 等价于 sort_by，自动提取 key
    {
        let mut c = world.get_mut::<Children>(parent2).unwrap();
        c.sort_by_key(|e| std::cmp::Reverse(e.index()));
        println!("7. sort_by_key(Reverse(index)): 按 index 逆序");
    }
    if let Some(c) = world.get::<Children>(parent2) {
        print!("   逆序后: ");
        for child in c.iter() {
            print!("{} ", name_of(world, child));
        }
        println!();
    }

    // 8. iter — 遍历
    if let Some(c) = world.get::<Children>(parent2) {
        print!("8. iter 遍历: ");
        for child in c.iter() {
            print!("{} ", name_of(world, child));
        }
        println!();
    }

    // 9. detach_all_children — 解除所有父子关系
    world.entity_mut(parent2).detach_all_children();
    println!("9. detach_all_children: parent2 所有子实体关系已解除");
    if let Some(c) = world.get::<Children>(parent2) {
        println!("   剩余 {} 个子实体", c.len());
    }

    println!("\n========== 完成 ==========");
}
