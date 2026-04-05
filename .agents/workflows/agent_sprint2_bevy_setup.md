---
description: 
---

# Agent: `bevy-setup` — Sprint 2: Rust / Bevy Setup & Physics Modeling

## Role
You are a **Rust systems engineer** specializing in Bevy ECS and game physics. Your job is to scaffold the Cargo workspace, wire up the Bevy app skeleton, and integrate the collision/physics layer so that towers register as static obstacles and creeps register as dynamic colliders.

## Context Documents (Read Before Acting)
- `01_legionRules.md` — Sections 12 (Creep Pathing) and 15 (Collision & Pathfinding Requirements) define the physics contract you must satisfy.
- `implementation_plan_md.resolved` — Section 1 (Architecture) defines the workspace layout and crate split.
- `03_DEVELOPMENT_ROADMAP.md` — Phase 0 and Phase 1 define the deliverable checklist.

## Workspace Layout to Create
```
/legion_td
├── Cargo.toml                  # workspace root
├── libs/
│   └── legion_core/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── components.rs   # all ECS components
│           ├── wave_data.rs    # wave constants from 01_legionRules.md §3
│           └── combat.rs       # damage matrix from 01_legionRules.md §6
└── apps/
    ├── headless_srvr/
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── web_client/
        ├── Cargo.toml
        └── src/main.rs
```

## Task List (execute in order)

### Task 1 — Workspace Cargo.toml
Create the root `Cargo.toml` declaring the workspace members: `libs/legion_core`, `apps/headless_srvr`, `apps/web_client`.

### Task 2 — Core Components (`legion_core/src/components.rs`)
Define the following Bevy `Component` structs. All numeric values must come from `01_legionRules.md` — do not invent defaults.

```rust
// Fighter (tower placed by player)
#[derive(Component)]
pub struct Fighter {
    pub owner: u8,          // player id 1-8
    pub legion: String,
    pub hp: f32,
    pub max_hp: f32,
    pub damage_min: f32,
    pub damage_max: f32,
    pub attack_speed: f32,  // seconds between attacks
    pub attack_range: f32,
    pub attack_type: AttackType,
    pub armor_type: ArmorType,
    pub point_value: u32,   // used for sell refund — see §4.5
    pub attack_timer: f32,
}

// Creep (wave enemy)
#[derive(Component)]
pub struct Creep {
    pub wave: u8,
    pub hp: f32,
    pub max_hp: f32,
    pub armor_type: ArmorType,
    pub attack_type: AttackType,
    pub speed: f32,
    pub bounty: u32,        // gold on kill — from §3 wave bounty table
    pub path_index: usize,
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
```

### Task 3 — Wave Data Constants (`legion_core/src/wave_data.rs`)
Hardcode the arrays from `01_legionRules.md §3` exactly. Use index 0 as a placeholder (waves are 1-indexed).

```rust
pub const WAVE_COUNT: [u32; 32]    = [0,36,45,40,36,36,36,30,36,45,3,54,45,45,26,36,45,35,45,36,3,36,48,36,35,45,36,36,18,30,3,15];
pub const WAVE_BOUNTY: [u32; 32]   = [0,3,3,4,5,5,5,6,6,5,51,5,6,7,12,9,8,10,8,10,86,10,9,11,11,9,12,12,23,14,123,0];
pub const WAVE_END_GOLD: [u32; 31] = [0,11,12,13,14,16,18,20,23,26,30,35,40,45,50,55,60,70,80,90,100,110,120,130,140,150,160,170,180,190,200];
pub const RECOMMEND_VALUE: [u32; 31] = [0,250,350,500,650,800,1000,1200,1450,1600,1850,2050,2400,2700,3100,3500,4000,4500,5000,5500,6000,6500,7100,7700,8500,9500,10600,11800,13000,14000,15000];
pub const SELL_PERCENT: f32        = 0.50;
pub const SELL_PERCENT_PREBATTLE: f32 = 0.90;
pub const STARTING_GOLD: u32       = 750;
pub const STARTING_LUMBER: u32     = 150;
pub const MAX_WAVE: u8             = 30;

pub fn build_timer_secs(wave: u8) -> u32 { 40 + (wave as u32 / 2) }

pub fn income_cap(wave: u8) -> u32 {
    let w = wave as f64;
    (0.025 * w.powi(3) + 0.05 * w.powi(2) + 4.0 * w + 20.0) as u32
}
```

### Task 4 — Damage Matrix (`legion_core/src/combat.rs`)
Implement the exact table from `01_legionRules.md §6`. No approximations.

```rust
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
        (Chaos, _)            => 1.00, // ignores all armor — boss waves 10,20,30,31
    }
}

pub fn calc_damage(base: f32, atk: AttackType, def: ArmorType) -> f32 {
    base * damage_multiplier(atk, def)
}
```

### Task 5 — Headless Server Bootstrap (`apps/headless_srvr/src/main.rs`)
Create a minimal Bevy app with `MinimalPlugins`. Add stubs for `GamePhase` state enum (`Preparation`, `Combat`, `Resolution`). No rendering. Verify it compiles with `cargo build -p headless_srvr`.

### Task 6 — Physics Integration
Add either `avian` or `bevy_rapier3d` as a dependency in `legion_core/Cargo.toml`. 
- Register `TowerObstacle` entities as `RigidBody::Static` with a `Collider::cuboid`.
- Register `CreepAgent` entities as `RigidBody::Dynamic` with a `Collider::ball` and locked Y-rotation.
- Write a system `register_tower_collider` that runs when a `Fighter` is spawned (use `Added<Fighter>`).
- Write a system `register_creep_collider` that runs when a `Creep` is spawned (use `Added<Creep>`).

## Acceptance Criteria
- [ ] `cargo build --workspace` passes with zero errors
- [ ] All wave constants in `wave_data.rs` match `01_legionRules.md §3` exactly
- [ ] Damage matrix covers all 25 attack×armor combinations (5×5)
- [ ] `headless_srvr` starts and prints "Legion TD server initialized" then exits cleanly
- [ ] Physics crate is wired; a tower entity spawned in a test has a `Collider` component attached

## Do NOT Do
- Do not implement rendering or UI (that belongs to `web_client` — Sprint 3 agent)
- Do not implement combat tick logic (Sprint 4 agent)
- Do not hardcode any game constants not sourced from `01_legionRules.md`
- Do not add networking (post-MVP per `implementation_plan_md.resolved`)
