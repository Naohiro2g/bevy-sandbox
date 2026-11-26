//! Hello, Bevy: 回る立方体と、光と、カメラ。

use bevy::prelude::*;

/// 回す立方体の印
#[derive(Component)]
struct Spin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy-sandbox".into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, spin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 立方体（1辺1、マイクラの1ブロックと同じ大きさ）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.35, 0.65, 0.3))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Spin,
    ));

    // 地面
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(6.0, 6.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.55, 0.45, 0.35))),
    ));

    // 光
    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // カメラ
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 3.0, 5.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
    ));

    // 文字
    commands.spawn((
        Text::new("Hello, Bevy!"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn spin(time: Res<Time>, mut cubes: Query<&mut Transform, With<Spin>>) {
    for mut transform in &mut cubes {
        transform.rotate_y(time.delta_secs() * 0.8);
    }
}
