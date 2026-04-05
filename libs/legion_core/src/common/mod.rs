use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum ArmorType {
    Unarmored,
    Light,
    Medium,
    Heavy,
    Fortified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum AttackType {
    Normal,
    Piercing,
    Magic,
    Siege,
    Chaos,
}

#[derive(Component, Debug, Clone)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Debug, Clone)]
pub struct Damage {
    pub min: i32,
    pub max: i32,
}

#[derive(Component, Debug, Clone)]
pub struct KingBadge;

#[derive(Component, Debug, Clone)]
pub struct FighterBadge;

#[derive(Component, Debug, Clone)]
pub struct CreepBadge {
    pub bounty: i32,
    pub waypoint_index: usize,
}
