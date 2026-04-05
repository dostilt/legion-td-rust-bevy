use bevy::prelude::*;
use legion_core::components::Creep;
use legion_core::state::GamePhase;

pub struct PhasesPlugin;

impl Plugin for PhasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            check_round_end.run_if(in_state(GamePhase::Combat)),
            check_lane_cleared_system.run_if(in_state(GamePhase::Combat)),
            resolution_phase.run_if(in_state(GamePhase::Resolution)),
        ));
        app.add_systems(OnEnter(GamePhase::Preparation), resurrect_towers);
        app.add_systems(OnEnter(GamePhase::Combat), awaken_towers);
    }
}

fn check_round_end(
    creeps: Query<&Creep>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if creeps.is_empty() {
        next_phase.set(GamePhase::Resolution);
    }
}

fn resolution_phase(
    mut next_phase: ResMut<NextState<GamePhase>>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    *timer += time.delta_seconds();
    if *timer >= 3.0 {
        *timer = 0.0;
        // In Sprint 5, Wave logic will increment wave counter here
        next_phase.set(GamePhase::Preparation);
    }
}

fn resurrect_towers(
    mut fighters: Query<(
        Entity, 
        &mut legion_core::components::Fighter, 
        &mut Visibility, 
        &mut Transform,
        Option<&mut avian3d::prelude::Position>,
        Option<&mut avian3d::prelude::Rotation>
    )>,
    mut commands: Commands,
) {
    for (entity, mut fighter, mut vis, mut transform, mut opt_pos, mut opt_rot) in fighters.iter_mut() {
        if fighter.is_dead {
            fighter.is_dead = false;
            fighter.hp = fighter.max_hp;
            *vis = Visibility::Inherited;
            
            commands.entity(entity).insert(legion_core::components::TowerObstacle);
            commands.entity(entity).insert(avian3d::prelude::RigidBody::Static);
            commands.entity(entity).insert(avian3d::prelude::Collider::cylinder(0.4, 1.0));
        }
        
        // Return to normal grid
        fighter.is_teleported_mid = false;
        transform.translation = fighter.build_position;
        transform.rotation = Quat::IDENTITY;
        
        if let Some(pos) = opt_pos.as_deref_mut() {
            pos.0 = fighter.build_position;
        }
        if let Some(rot) = opt_rot.as_deref_mut() {
            *rot = avian3d::prelude::Rotation::default();
        }
        
        // Ensure static
        commands.entity(entity).insert(avian3d::prelude::RigidBody::Static);
        commands.entity(entity).insert(avian3d::prelude::LinearVelocity(Vec3::ZERO));
    }
}

fn awaken_towers(
    fighters: Query<Entity, With<legion_core::components::Fighter>>,
    mut commands: Commands,
    mut global_combat: ResMut<legion_core::state::GlobalCombatState>,
) {
    global_combat.engaged = false; // Reset team engagement!
    
    for entity in fighters.iter() {
        commands.entity(entity).insert(avian3d::prelude::RigidBody::Dynamic);
        commands.entity(entity).insert(avian3d::prelude::LockedAxes::ROTATION_LOCKED);
        commands.entity(entity).insert(avian3d::prelude::LinearVelocity(Vec3::ZERO));
    }
}

fn check_lane_cleared_system(
    creeps: Query<&Creep>,
    mut fighters: Query<(&mut legion_core::components::Fighter, &mut Transform)>,
) {
    let mut p1_has_creeps = false;
    for creep in creeps.iter() {
        if creep.owner == 1 {
            p1_has_creeps = true;
            break;
        }
    }
    
    if !p1_has_creeps {
        for (mut fighter, mut transform) in fighters.iter_mut() {
            if fighter.owner == 1 && !fighter.is_dead && !fighter.is_teleported_mid {
                fighter.is_teleported_mid = true;
                // Move towards king area (with some simple spread based on their original x so they don't perfectly stack)
                let new_x = fighter.build_position.x * 0.5;
                transform.translation = Vec3::new(new_x, 0.75, 14.0);
            }
        }
    }
}
