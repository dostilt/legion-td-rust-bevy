use bevy::prelude::*;
use legion_core::components::{Creep, CreepAgent, TargetLock, ArmorType, AttackType};
use legion_core::wave_data::{WAVE_COUNT, WAVE_BOUNTY};
use legion_core::state::GamePhase;

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GamePhase::Combat), spawn_wave_1);
    }
}

// Temporary for demo: just spawn wave 1 at startup
fn spawn_wave_1(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let count = WAVE_COUNT[1] as usize; // Wave 1 = 36 creeps
    let creep_mesh = meshes.add(Capsule3d::new(0.4, 1.0));
    let creep_material = materials.add(StandardMaterial {
        base_color: Srgba::hex("666666").unwrap().into(),
        ..default()
    });

    for i in 0..count {
        let spawn_point = Vec3::new(0.0, 0.5, -18.0);
        let offset = Vec3::new(
            (i as f32 % 6.0) * 1.2 - 3.0,
            0.0,
            (i as f32 / 6.0) * 1.2,
        );

        commands.spawn((
            Creep {
                owner: 1, // Represents creeps marching down Player 1's lane
                wave: 1,
                hp: 100.0,
                max_hp: 100.0,
                armor_type: ArmorType::Unarmored,
                attack_type: AttackType::Piercing,
                speed: 4.0,
                bounty: WAVE_BOUNTY[1],
                path_index: 0,
                damage: 5.0,
                attack_speed: 1.0,
                attack_timer: 0.0,
                attack_range: 2.0,
            },
            TargetLock(None),
            CreepAgent,
            PbrBundle {
                mesh: creep_mesh.clone(),
                material: creep_material.clone(),
                transform: Transform::from_translation(spawn_point + offset),
                ..default()
            },
        ));
    }
}
