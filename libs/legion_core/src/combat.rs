use crate::components::{ArmorType, AttackType};

pub fn damage_multiplier(atk: AttackType, def: ArmorType) -> f32 {
    use AttackType::*; use ArmorType::*;
    match (atk, def) {
        (Normal,   Unarmored) => 1.00, (Normal,   Light) => 1.00,
        (Normal,   Medium)    => 1.00, (Normal,   Heavy) => 0.70,
        (Normal,   Fortified) => 0.70,
        (Piercing, Unarmored) => 1.00, (Piercing, Light) => 1.50,
        (Piercing, Medium)    => 0.75, (Piercing, Heavy) => 0.50,
        (Piercing, Fortified) => 0.35,
        (Magic,    Unarmored) => 1.00, (Magic,    Light) => 1.25,
        (Magic,    Medium)    => 0.75, (Magic,    Heavy) => 0.75,
        (Magic,    Fortified) => 0.35,
        (Siege,    Unarmored) => 1.00, (Siege,    Light) => 0.50,
        (Siege,    Medium)    => 0.75, (Siege,    Heavy) => 1.00,
        (Siege,    Fortified) => 1.50,
        (Chaos, _)            => 1.00, // ignores all armor
    }
}

pub fn calc_damage(base: f32, atk: AttackType, def: ArmorType) -> f32 {
    base * damage_multiplier(atk, def)
}
