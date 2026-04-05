use bevy::prelude::*;

// Fighter (tower placed by player)
#[derive(Component)]
pub struct Fighter {
    pub owner: u8,          // player id 1-8
    pub legion: String,
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32, // Dynamic movement speed when active
    pub damage_min: f32,
    pub damage_max: f32,
    pub attack_speed: f32,  // seconds between attacks
    pub attack_range: f32,
    pub aggro_range: f32,   // range before breaking formation
    pub attack_type: AttackType,
    pub armor_type: ArmorType,
    pub point_value: u32,   // used for sell refund
    pub attack_timer: f32,
    pub round_built: u32,
    pub is_dead: bool,
    pub build_position: Vec3,
    pub is_teleported_mid: bool,
}

// Creep (wave enemy)
#[derive(Component)]
pub struct Creep {
    pub owner: u8,
    pub wave: u8,
    pub hp: f32,
    pub max_hp: f32,
    pub armor_type: ArmorType,
    pub attack_type: AttackType,
    pub speed: f32,
    pub bounty: u32,        // gold on kill
    pub path_index: usize,
    pub damage: f32,
    pub attack_speed: f32,
    pub attack_timer: f32,
    pub attack_range: f32,
}

// Player economy state
#[derive(Component)]
pub struct PlayerEconomy {
    pub player_id: u8,
    pub gold: u32,
    pub lumber: u32,
    pub fighters_value: u32,
}

// King
#[derive(Component)]
pub struct King {
    pub team: u8,
    pub hp: u32,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum AttackType { Normal, Piercing, Magic, Siege, Chaos }

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum ArmorType { Unarmored, Light, Medium, Heavy, Fortified }

// Collision marker — added to fighters to register as static obstacles
#[derive(Component)]
pub struct TowerObstacle;

// Added to creeps for dynamic avoidance
#[derive(Component)]
pub struct CreepAgent;

// Tracks persistent combat targets
#[derive(Component)]
pub struct TargetLock(pub Option<Entity>);
