use bevy::prelude::*;
use avian3d::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_lane, setup_king));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 35.0, 25.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_lane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Lane is 8 wide, 40 long.
    // Let's spawn a plane to represent the ground.
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(12.0, 40.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.7, 0.6, 0.4), // warm dirt/stone color
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(12.0, 0.1, 40.0),
    ));

    // Left Wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 2.0, 40.0)),
            transform: Transform::from_xyz(-6.5, 1.0, 0.0),
            material: materials.add(StandardMaterial { base_color: Color::srgb(0.4, 0.4, 0.4), ..default() }),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1.0, 2.0, 40.0),
    ));

    // Right Wall
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 2.0, 40.0)),
            transform: Transform::from_xyz(6.5, 1.0, 0.0),
            material: materials.add(StandardMaterial { base_color: Color::srgb(0.4, 0.4, 0.4), ..default() }),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(1.0, 2.0, 40.0),
    ));

    // Spawn Zone indicator
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(8.0, 4.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.3), // somewhat reddish
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.01, -18.0), // Far from the King
        ..default()
    });
}

fn setup_king(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(4.0, 0.5, 4.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.8),
            ..default()
        }),
        transform: Transform::from_xyz(0.0, 0.25, 18.0), // Near the right/king end
        ..default()
    });

    // The King unit
    commands.spawn((
        legion_core::components::King { team: 1, hp: 1000 },
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2.0, 3.0, 2.0)),
            material: materials.add(StandardMaterial {
                base_color: Srgba::hex("C8941A").unwrap().into(), // Paladin gold
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 2.0, 18.0),
            ..default()
        },
    ));

    // Spawn player economy
    commands.spawn(legion_core::components::PlayerEconomy {
        player_id: 1,
        gold: 2400, // Enough for multiple test units
        lumber: 0,
        fighters_value: 0,
    });
}
