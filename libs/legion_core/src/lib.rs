pub mod components;
pub mod wave_data;
pub mod combat;
pub mod pathfinding;
pub mod state;

use bevy::prelude::*;
use avian3d::prelude::*;
use components::*;

pub struct LegionCorePlugin;

impl Plugin for LegionCorePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<state::GamePhase>();
        app.insert_resource(pathfinding::FlowField::new(8, 40, IVec2::new(4, 38))); // King target at Z=18 (+Z)
        
        app.add_systems(Update, pathfinding::update_flow_field_system);
        app.add_systems(Update, (
            register_tower_collider,
            register_creep_collider,
        ));
    }
}

// Sprint 2 Task 6 - Physics Integration
fn register_tower_collider(
    mut commands: Commands,
    query: Query<Entity, Added<Fighter>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            TowerObstacle,
            RigidBody::Static,
            Collider::cuboid(1.0, 1.5, 1.0),
        ));
    }
}

fn register_creep_collider(
    mut commands: Commands,
    query: Query<Entity, Added<CreepAgent>>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            RigidBody::Dynamic,
            Collider::cylinder(0.4, 1.0),
            LockedAxes::ROTATION_LOCKED,
            Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
        ));
    }
}
