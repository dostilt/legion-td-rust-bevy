use bevy::prelude::*;

pub struct VfxPlugin;

impl Plugin for VfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_projectiles);
    }
}

// Very basic projectile placeholder
#[derive(Component)]
pub struct Projectile {
    pub target: Entity,
    pub speed: f32,
}

fn update_projectiles(
    mut commands: Commands,
    mut projectiles: Query<(Entity, &mut Transform, &Projectile)>,
    targets: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (entity, mut transform, proj) in projectiles.iter_mut() {
        if let Ok(target_transform) = targets.get(proj.target) {
            let dir = (target_transform.translation() - transform.translation).normalize_or_zero();
            transform.translation += dir * proj.speed * time.delta_seconds();
            if transform.translation.distance(target_transform.translation()) < 0.5 {
                commands.entity(entity).despawn();
            }
        } else {
            // Target died before projectile reached
            commands.entity(entity).despawn();
        }
    }
}
