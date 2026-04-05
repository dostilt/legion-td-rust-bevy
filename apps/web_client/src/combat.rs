use bevy::prelude::*;
use legion_core::components::{Fighter, Creep, TargetLock, King, PlayerEconomy};
use legion_core::combat::calc_damage;
use legion_core::state::GamePhase;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            fighter_attack_system.run_if(in_state(GamePhase::Combat)),
            creep_attack_system.run_if(in_state(GamePhase::Combat)),
            creep_leak_system.run_if(in_state(GamePhase::Combat)),
            king_death_system,
        ));
        app.add_systems(Update, draw_health_bars);
    }
}

// Basic RNG without adding full rand crate constraint for simplicity
fn rand_range(min: f32, max: f32) -> f32 {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f32();
    let fraction = secs.fract();
    min + (max - min) * fraction
}

fn fighter_attack_system(
    time: Res<Time>,
    mut fighters: Query<(&mut Fighter, &GlobalTransform, &TargetLock)>,
    mut creeps: Query<(Entity, &mut Creep, &GlobalTransform, &mut Transform)>,
    mut commands: Commands,
    mut economy: Query<&mut PlayerEconomy>,
    mut gizmos: Gizmos,
) {
    let dt = time.delta_seconds();

    for (mut fighter, f_transform, target_lock) in fighters.iter_mut() {
        if fighter.is_dead { continue; }
        
        fighter.attack_timer -= dt;
        if fighter.attack_timer > 0.0 { continue; }

        let f_pos = f_transform.translation();
        
        // Exclusively attack TargetLock
        if let Some(target_entity) = target_lock.0 {
            // Get mutable access back to the actual target creep to damage it
            if let Ok((_, mut creep, c_global, mut c_transform)) = creeps.get_mut(target_entity) {
                let dist = f_pos.distance(c_global.translation());
                
                // Only strike if it hasn't somehow violently out-ranged us
                if dist <= fighter.attack_range {
                    fighter.attack_timer = fighter.attack_speed;
                    
                    let base_dmg = rand_range(fighter.damage_min, fighter.damage_max);
                    let dmg = calc_damage(base_dmg, fighter.attack_type, creep.armor_type);
                    
                    creep.hp -= dmg;
                    
                    // Attack Visual Laser!
                    gizmos.line(f_pos, c_transform.translation, Color::srgb(1.0, 1.0, 0.0));
                    gizmos.circle(c_transform.translation, Dir3::Y, 0.5, Color::srgb(1.0, 1.0, 0.0));
                    
                    // Bouncing effect for taking hits
                    c_transform.translation.y = 0.8;
                    
                    if creep.hp <= 0.0 {
                        commands.entity(target_entity).despawn_recursive();
                        if let Ok(mut econ) = economy.get_single_mut() {
                            econ.gold += creep.bounty;
                        }
                    }
                }
            }
        }
    }
}

fn creep_attack_system(
    time: Res<Time>,
    mut creeps: Query<(&mut Creep, &GlobalTransform, &TargetLock)>,
    mut fighters: Query<(Entity, &mut Fighter, &mut Visibility, &GlobalTransform, &mut Transform)>,
    mut commands: Commands,
    mut gizmos: Gizmos,
) {
    let dt = time.delta_seconds();

    for (mut creep, c_transform, target_lock) in creeps.iter_mut() {
        creep.attack_timer -= dt;
        let c_pos = c_transform.translation();
        
        if let Some(target_entity) = target_lock.0 {
            if creep.attack_timer <= 0.0 {
                // Verify distance to explicitly locked target
                if let Ok((entity, mut fighter, mut vis, f_global, mut f_transform)) = fighters.get_mut(target_entity) {
                    let dist = c_pos.distance(f_global.translation());
                    
                    if dist <= creep.attack_range {
                        creep.attack_timer = creep.attack_speed;
                        
                        let dmg = calc_damage(creep.damage, creep.attack_type, fighter.armor_type);
                        fighter.hp -= dmg;
                        
                        gizmos.circle(f_transform.translation, Dir3::Y, 0.7, Color::srgb(1.0, 0.0, 0.0));
                        f_transform.translation.y = 1.0;
                        
                        // Creep scoring a kill
                        if fighter.hp <= 0.0 {
                            fighter.is_dead = true;
                            *vis = Visibility::Hidden;
                            // Turn off obstacle/collision so creeps can walk past
                            commands.entity(entity).remove::<legion_core::components::TowerObstacle>();
                            commands.entity(entity).remove::<avian3d::prelude::Collider>();
                            commands.entity(entity).remove::<avian3d::prelude::RigidBody>();
                        }
                    }
                }
            }
        }
    }
}

fn creep_leak_system(
    creeps: Query<(Entity, &GlobalTransform, &Creep)>,
    mut kings: Query<&mut King>,
    mut commands: Commands,
) {
    for (entity, transform, creep) in creeps.iter() {
        // King platform is at z = 18.0. If creep goes past 16.0 it's in the red zone.
        if transform.translation().z > 16.0 {
            if let Ok(mut king) = kings.get_single_mut() {
                // Determine remaining HP of the creep and deal that damage to the King
                let damage_to_king = creep.hp.max(1.0) as u32;
                king.hp = king.hp.saturating_sub(damage_to_king);
            }
            // Leak despawn
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn king_death_system(
    kings: Query<&King>,
    mut next_state: ResMut<NextState<GamePhase>>,
) {
    if let Ok(king) = kings.get_single() {
        if king.hp == 0 {
            next_state.set(GamePhase::GameOver);
        }
    }
}

fn draw_health_bars(
    fighters: Query<(&Fighter, &GlobalTransform)>,
    creeps: Query<(&Creep, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    let y_offset = Vec3::new(0.0, 1.3, 0.0);
    
    // Draw for fighters
    for (fighter, transform) in fighters.iter() {
        if fighter.is_dead { continue; }
        let pos = transform.translation() + y_offset;
        let hp_pct = fighter.hp / fighter.max_hp.max(1.0);
        
        let start = pos - Vec3::X * 0.5;
        let end = start + Vec3::X * hp_pct;
        
        // Draw foreground (green)
        gizmos.line(start, end, Color::srgb(0.1, 0.8, 0.1));
        // Draw background (red) perfectly adjacent to avoid Z-fighting
        gizmos.line(end, pos + Vec3::X * 0.5, Color::srgb(0.7, 0.1, 0.1));
    }

    // Draw for creeps
    for (creep, transform) in creeps.iter() {
        let pos = transform.translation() + y_offset;
        let hp_pct = creep.hp / creep.max_hp.max(1.0);
        
        let start = pos - Vec3::X * 0.5;
        let end = start + Vec3::X * hp_pct;
        
        gizmos.line(start, end, Color::srgb(0.1, 0.8, 0.1));
        gizmos.line(end, pos + Vec3::X * 0.5, Color::srgb(0.7, 0.1, 0.1));
    }
}
