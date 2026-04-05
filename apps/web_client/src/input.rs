use bevy::prelude::*;
use legion_core::components::{Fighter, PlayerEconomy, AttackType, ArmorType, TowerObstacle, TargetLock};
use avian3d::prelude::*;
use legion_core::state::GamePhase;
use legion_core::pathfinding::FlowField;

// We use an empty plugin here
pub struct InputPlugin;

#[derive(Resource, Default, PartialEq, Clone, Copy)]
pub enum FighterSelection {
    #[default]
    Footman,
    Archer,
    Mage,
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FighterSelection>();
        app.add_systems(Update, (
            handle_build_input.run_if(in_state(GamePhase::Preparation)),
            handle_sell_input,
            start_combat_hotkey.run_if(in_state(GamePhase::Preparation)),
            handle_selection_input,
        ));
    }
}

fn handle_selection_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut selection: ResMut<FighterSelection>,
) {
    if keyboard.just_pressed(KeyCode::Digit1) {
        *selection = FighterSelection::Footman;
        println!("Selected Footman (120g)");
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        *selection = FighterSelection::Archer;
        println!("Selected Archer (135g)");
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        *selection = FighterSelection::Mage;
        println!("Selected Mage (160g)");
    }
}

fn start_combat_hotkey(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_phase: ResMut<NextState<GamePhase>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_phase.set(GamePhase::Combat);
    }
}

fn handle_build_input(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pathfinding: ResMut<FlowField>,
    mut economy: Query<&mut PlayerEconomy>,
    selection: Res<FighterSelection>,
) {
    if !mouse.just_pressed(MouseButton::Left) { return; }
    
    let Ok(window) = windows.get_single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return; };

    // Simple raycast to ground plane (Y = 0)
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else { return; };
    let distance = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y));
    
    if let Some(d) = distance {
        let intersection = ray.get_point(d);
        
        // Snap to grid (1x1 cells)
        let grid_x = intersection.x.round();
        let grid_z = intersection.z.round();
        
        // Bounds check our 8x40 lane (-4..4, -20..20)
        if grid_x < -3.0 || grid_x > 3.0 || grid_z < -18.0 || grid_z > 18.0 {
            // Out of bounds
            return;
        }

        let world_pos = Vec3::new(grid_x, 0.75, grid_z);
        let _grid_pos = pathfinding.pos_to_grid(world_pos);

        // No more anti-maze check: player can build anywhere!
        
        // Assuming single player economy exists
        if let Ok(mut econ) = economy.get_single_mut() {
            let (cost, _name, hp, speed, d_min, d_max, a_spd, a_rng, a_aggro, a_type, arm_type, mesh_handle, mat_color) = match *selection {
                FighterSelection::Footman => (
                    120, "Footman", 100.0, 3.5, 10.0, 12.0, 1.0, 2.0, 6.0, AttackType::Normal, ArmorType::Heavy,
                    meshes.add(Cuboid::new(0.8, 1.5, 0.8)),
                    Color::srgb(0.2, 0.4, 0.8) // Blue
                ),
                FighterSelection::Archer => (
                    135, "Archer", 70.0, 3.2, 15.0, 18.0, 0.8, 6.0, 10.0, AttackType::Piercing, ArmorType::Light,
                    meshes.add(Cylinder::new(0.3, 1.8)),
                    Color::srgb(0.2, 0.8, 0.3) // Green
                ),
                FighterSelection::Mage => (
                    160, "Mage", 60.0, 3.0, 25.0, 30.0, 1.5, 4.0, 8.0, AttackType::Magic, ArmorType::Unarmored,
                    meshes.add(Capsule3d::new(0.5, 1.2)),
                    Color::srgb(0.6, 0.2, 0.8) // Purple
                ),
            };

            if econ.gold >= cost {
                econ.gold -= cost;
                
                commands.spawn((
                    Fighter {
                        owner: 1,
                        legion: "Nature".to_string(), // Placeholder name
                        hp,
                        max_hp: hp,
                        speed,
                        damage_min: d_min,
                        damage_max: d_max,
                        attack_speed: a_spd,
                        attack_range: a_rng,
                        aggro_range: a_aggro,
                        attack_type: a_type,
                        armor_type: arm_type,
                        point_value: cost,
                        attack_timer: 0.0,
                        round_built: 1, // Placeholder for wave 1
                        is_dead: false,
                        build_position: world_pos,
                        is_teleported_mid: false,
                    },
                    TargetLock(None),
                    PbrBundle {
                        mesh: mesh_handle,
                        material: materials.add(StandardMaterial {
                            base_color: mat_color,
                            ..default()
                        }),
                        transform: Transform::from_translation(world_pos),
                        ..default()
                    },
                    TowerObstacle,
                    RigidBody::Static,
                    Collider::cylinder(0.4, 1.0),
                    Restitution::new(0.0).with_combine_rule(CoefficientCombine::Min),
                    Friction::new(0.0).with_combine_rule(CoefficientCombine::Min),
                ));
            }
        }
    }
}

fn handle_sell_input(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    fighters: Query<(Entity, &Fighter, &Transform)>,
    mut economy: Query<&mut PlayerEconomy>,
    phase: Res<State<GamePhase>>,
) {
    if !mouse.just_pressed(MouseButton::Right) { return; }

    let Ok(window) = windows.get_single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return; };

    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_pos) else { return; };
    let distance = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y));
    
    if let Some(d) = distance {
        let intersection = ray.get_point(d);

        // Find the closest fighter to the click within 1.0 unit
        let mut closest_entity = None;
        let mut min_dist = 1.0;
        let mut refund_value = 0;

        for (entity, fighter, transform) in fighters.iter() {
            let dist = transform.translation.distance(intersection);
            if dist < min_dist {
                min_dist = dist;
                closest_entity = Some(entity);
                
                // Same round refund 100%, else 50%. (Hardcoding 1 as wave for sprint 4 mockup)
                let pct = if fighter.round_built == 1 && *phase.get() == GamePhase::Preparation { 1.00 } else { 0.50 };
                refund_value = (fighter.point_value as f32 * pct) as u32;
            }
        }

        if let Some(e) = closest_entity {
            commands.entity(e).despawn_recursive();
            if let Ok(mut econ) = economy.get_single_mut() {
                econ.gold += refund_value;
            }
            // A VFX popup would be triggered here in a complete event system
        }
    }
}

