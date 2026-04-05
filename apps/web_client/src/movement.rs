use bevy::prelude::*;
use avian3d::prelude::*;
use legion_core::components::{Creep, CreepAgent, TargetLock};
use legion_core::pathfinding::FlowField;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_creeps, move_fighters));
    }
}

fn move_creeps(
    flow_field: Res<FlowField>,
    mut query: Query<(Entity, &mut LinearVelocity, &mut Transform, &Creep, &mut TargetLock), With<CreepAgent>>,
    fighters: Query<(Entity, &legion_core::components::Fighter, &Transform), Without<CreepAgent>>,
    mut global_combat: ResMut<legion_core::state::GlobalCombatState>,
    mut gizmos: Gizmos,
) {
    let all_creep_positions: Vec<(Entity, Vec3)> = query.iter().map(|(e, _, t, _, _)| (e, t.translation)).collect();

    for (entity, mut velocity, mut transform, creep, mut target_lock) in query.iter_mut() {
        let pos = transform.translation;

        // Target lock persistence
        let mut target_pos = None;
        if let Some(target_entity) = target_lock.0 {
            if let Ok((_, f, f_transform)) = fighters.get(target_entity) {
                if f.is_dead {
                    target_lock.0 = None;
                } else {
                    target_pos = Some(f_transform.translation);
                }
            } else {
                target_lock.0 = None;
            }
        }

        // Scan if locked target is empty
        if target_lock.0.is_none() {
            let mut nearest_f_entity = None;
            let scan_range = if global_combat.engaged { f32::MAX } else { 6.0 };
            let mut min_dist = scan_range; // Creep sensory aggro range to break pathing

            for (f_entity, fighter, f_transform) in fighters.iter() {
                if !fighter.is_dead {
                    let dist = pos.distance(f_transform.translation);
                    if dist <= min_dist {
                        min_dist = dist;
                        nearest_f_entity = Some(f_entity);
                        target_pos = Some(f_transform.translation);
                    }
                }
            }
            if nearest_f_entity.is_some() {
                target_lock.0 = nearest_f_entity;
                global_combat.engaged = true; // Signal the rest of the wave!
            }
        }

        let mut in_attack_range = false;
        let mut final_dir = flow_field.get_direction(pos); // Default pathfinding King vector

        if let Some(t_pos) = target_pos {
            let dist = pos.distance(t_pos);
            if dist <= creep.attack_range {
                in_attack_range = true;
            } else {
                final_dir = Vec2::new(t_pos.x - pos.x, t_pos.z - pos.z).normalize_or_zero();
                gizmos.line(pos, t_pos, Color::srgb(1.0, 0.0, 0.0)); // Visual Aggro Line
            }
        }

        // Stop completely if we are actively in range to fight a tower
        if in_attack_range {
            velocity.0 = Vec3::ZERO;
        } else {
            // Boid separation
            let separation_radius = 0.8;
            let mut separation = Vec2::ZERO;
            let mut count = 0;

            for (other_entity, other_pos) in &all_creep_positions {
                if entity == *other_entity { continue; }
                let dist = pos.distance(*other_pos);
                if dist < separation_radius && dist > 0.001 {
                    let push = (pos - *other_pos).normalize() / dist;
                    separation += Vec2::new(push.x, push.z);
                    count += 1;
                }
            }

            if count > 0 {
                separation /= count as f32;
                final_dir = (final_dir + separation * 0.5).normalize_or_zero();
            }

            let speed = creep.speed;
            velocity.0 = Vec3::new(final_dir.x * speed, velocity.y, final_dir.y * speed);
        }
        
        // Face the direction of movement or target
        if velocity.0.length_squared() > 0.001 {
            if let Ok(dir) = Dir3::new(Vec3::new(velocity.0.x, 0.0, velocity.0.z)) {
                transform.look_to(dir, Dir3::Y);
            }
        } else if let Some(t_pos) = target_pos {
            let look_dir = t_pos - pos;
            if let Ok(dir) = Dir3::new(Vec3::new(look_dir.x, 0.0, look_dir.z)) {
                transform.look_to(dir, Dir3::Y);
            }
        }
        
        // Despawn if near the king
        if pos.z > 16.0 {
            // Reached target
        }
    }
}

fn move_fighters(
    mut query: Query<(Entity, &mut LinearVelocity, &mut Transform, &legion_core::components::Fighter, &mut TargetLock)>,
    creeps: Query<(Entity, &Creep, &Transform), Without<legion_core::components::Fighter>>,
    mut global_combat: ResMut<legion_core::state::GlobalCombatState>,
) {
    let all_fighter_positions: Vec<(Entity, Vec3)> = query.iter().map(|(e, _, t, _, _)| (e, t.translation)).collect();

    for (entity, mut velocity, mut transform, fighter, mut target_lock) in query.iter_mut() {
        if fighter.is_dead || fighter.is_teleported_mid {
            continue;
        }

        let pos = transform.translation;
        
        // Persistent Target Locking
        let mut target_pos = None;
        if let Some(target_entity) = target_lock.0 {
            if let Ok((_, _, c_transform)) = creeps.get(target_entity) {
                target_pos = Some(c_transform.translation);
            } else {
                target_lock.0 = None;
            }
        }

        if target_lock.0.is_none() {
            let mut nearest_c_entity = None;
            let scan_range = if global_combat.engaged { f32::MAX } else { fighter.aggro_range };
            let mut min_dist = scan_range; // Strictly hold formation if nothing in range!

            for (c_entity, _creep, c_transform) in creeps.iter() {
                let dist = pos.distance(c_transform.translation);
                if dist <= min_dist {
                    min_dist = dist;
                    nearest_c_entity = Some(c_entity);
                    target_pos = Some(c_transform.translation);
                }
            }
            if nearest_c_entity.is_some() {
                target_lock.0 = nearest_c_entity;
                global_combat.engaged = true; // Signal the rest of the towers!
            }
        }

        if let Some(t_pos) = target_pos {
            let dist = pos.distance(t_pos);
            if dist <= fighter.attack_range {
                // In range! Stop completely.
                velocity.0 = Vec3::ZERO;
            } else {
                // March towards locked target
                let mut final_dir = Vec2::new(t_pos.x - pos.x, t_pos.z - pos.z).normalize_or_zero();

                // Simple Boid separation vs other fighters
                let separation_radius = 0.8;
                let mut separation = Vec2::ZERO;
                let mut count = 0;

                for (other_entity, other_pos) in &all_fighter_positions {
                    if entity == *other_entity { continue; }
                    let dist = pos.distance(*other_pos);
                    if dist < separation_radius && dist > 0.001 {
                        let push = (pos - *other_pos).normalize() / dist;
                        separation += Vec2::new(push.x, push.z);
                        count += 1;
                    }
                }

                if count > 0 {
                    separation /= count as f32;
                    final_dir = (final_dir + separation * 0.5).normalize_or_zero();
                }

                let speed = fighter.speed;
                velocity.0 = Vec3::new(final_dir.x * speed, velocity.y, final_dir.y * speed);
            }
        } else {
            // No target mapped! HOLD FORMATION stoically!
            velocity.0 = Vec3::ZERO;
        }
        
        // Face the direction of movement or target
        if velocity.0.length_squared() > 0.001 {
            if let Ok(dir) = Dir3::new(Vec3::new(velocity.0.x, 0.0, velocity.0.z)) {
                transform.look_to(dir, Dir3::Y);
            }
        } else if let Some(t_pos) = target_pos {
            let look_dir = t_pos - pos;
            if let Ok(dir) = Dir3::new(Vec3::new(look_dir.x, 0.0, look_dir.z)) {
                transform.look_to(dir, Dir3::Y);
            }
        }
    }
}
