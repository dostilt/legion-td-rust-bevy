---
description: 
---

# Agent: `combat-sandbox` — Sprint 4: Combat Sandbox

## Role
You are a **Bevy ECS combat systems engineer**. Your job is to make the game interactive: players can dynamically place and sell towers during the build phase, towers auto-attack creeps during combat, the NavMesh/flow field updates on tower change, and rounds end when all creeps are dead or have leaked through.

## Prerequisites
- Sprint 3 (`visual-prototype`) must be complete:
  - Lane renders, creeps spawn and path around a static tower wall
  - `legion_core::pathfinding` module exists with a working flow field or NavMesh
  - Floating gold text works on creep removal
  - `GamePhase` state enum exists in `headless_srvr`

## Context Documents (Read Before Acting)
- `01_legionRules.md` — Section 4 (Economy/Sell), Section 5 (Fighter Value System), Section 6 (Damage Matrix), Section 15 (Collision), Section 16 (Visual Pacing)
- `04_STARTER_CODE.md` — Steps 2 and 3: the Go combat loop is the reference logic — translate it to Bevy ECS systems
- `implementation_plan_md.resolved` — Sprint 4 goal: "Players can build towers dynamically (updating the NavMesh). Towers attack creeps in range. End round when creeps die."

## What You Must Build

### 1. Build Phase Input System
During `GamePhase::Preparation`, clicking in the lane places a fighter.

```rust
fn handle_build_input(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut pathfinding: ResMut<PathfindingGrid>, // or NavMesh
    economy: Query<&mut PlayerEconomy>,
    selected_fighter_type: Res<SelectedFighterType>,
    phase: Res<State<GamePhase>>,
) {
    if *phase.get() != GamePhase::Preparation { return; }
    if !mouse.just_pressed(MouseButton::Left) { return; }
    // Raycast mouse position to lane plane → grid cell
    // Check: player has enough gold (fighter cost from selected_fighter_type)
    // Check: cell is not already occupied
    // Check: placing here does not fully block the creep path (see §15 anti-maze rule)
    // If valid: spawn Fighter entity, deduct gold, mark cell occupied, recompute pathfinding
}
```

**Anti-maze rule** (from `01_legionRules.md §15`): After computing the new flow field/NavMesh with the proposed tower added, verify at least one valid path still exists from spawn to King. If not, reject the placement and show a red flash on the invalid cell.

### 2. Sell System
From `01_legionRules.md §4.5`:
- During Preparation: refund = `point_value × 0.90`
- During Combat: refund = `point_value × 0.50`

```rust
fn handle_sell_input(
    keyboard: Res<ButtonInput<KeyCode>>, // or right-click
    selected_unit: Res<SelectedUnit>,
    mut commands: Commands,
    fighters: Query<(Entity, &Fighter, &Transform)>,
    mut economy: Query<&mut PlayerEconomy>,
    phase: Res<State<GamePhase>>,
    mut pathfinding: ResMut<PathfindingGrid>,
) {
    // On sell key press:
    // 1. Calculate refund from §4.5 (check current phase)
    // 2. Add gold back to player economy
    // 3. Despawn fighter entity
    // 4. Unmark cell in pathfinding grid, recompute flow field
    // 5. Spawn gold pop text at fighter position
}
```

### 3. Attack System (Combat Phase Only)
Runs every tick during `GamePhase::Combat`. This is the core loop from `04_STARTER_CODE.md §Step 3`, translated to Bevy.

```rust
fn fighter_attack_system(
    time: Res<Time>,
    mut fighters: Query<(&mut Fighter, &GlobalTransform)>,
    mut creeps: Query<(Entity, &mut Creep, &GlobalTransform)>,
    mut commands: Commands,
    mut economy: Query<&mut PlayerEconomy>,
    phase: Res<State<GamePhase>>,
) {
    if *phase.get() != GamePhase::Combat { return; }
    let dt = time.delta_secs();

    for (mut fighter, f_transform) in fighters.iter_mut() {
        fighter.attack_timer -= dt;
        if fighter.attack_timer > 0.0 { continue; }

        // Find nearest creep within attack_range
        let target = find_nearest_creep_in_range(
            f_transform.translation(), fighter.attack_range, &creeps
        );
        let Some((creep_entity, mut creep, _)) = target else { continue };

        fighter.attack_timer = fighter.attack_speed;

        // Apply damage matrix from §6
        let base_dmg = rand_range(fighter.damage_min, fighter.damage_max);
        let dmg = calc_damage(base_dmg, fighter.attack_type, creep.armor_type);
        creep.hp -= dmg;

        if creep.hp <= 0.0 {
            on_creep_death(creep_entity, &creep, &mut economy, &mut commands);
        }
    }
}

fn on_creep_death(
    entity: Entity,
    creep: &Creep,
    economy: &mut Query<&mut PlayerEconomy>,
    commands: &mut Commands,
) {
    // Award bounty to the owning player (owner of the lane that killed it)
    // Spawn floating gold text `+N` at creep position
    // Despawn creep entity
    commands.entity(entity).despawn_recursive();
}
```

### 4. Creep Leak System
When a creep reaches `path_index >= waypoints.len()` (i.e., it has passed all waypoints and entered the King zone):

```rust
fn creep_leak_system(
    mut creeps: Query<(Entity, &Creep, &GlobalTransform)>,
    mut kings: Query<&mut King>,
    mut commands: Commands,
    waypoints: Res<LaneWaypoints>,
) {
    for (entity, creep, transform) in creeps.iter() {
        if is_in_king_zone(transform.translation()) {
            // Decrement King HP by 1 (standard leak)
            // For boss creeps (wave 10, 20, 30): decrement by more (TBD per §7)
            if let Ok(mut king) = kings.get_single_mut() {
                king.hp = king.hp.saturating_sub(1);
            }
            commands.entity(entity).despawn_recursive();
            // Check king death → trigger GameOver
        }
    }
}
```

### 5. Round End System
```rust
fn check_round_end(
    creeps: Query<&Creep>,
    mut next_phase: ResMut<NextState<GamePhase>>,
    mut wave: ResMut<CurrentWave>,
    phase: Res<State<GamePhase>>,
) {
    if *phase.get() != GamePhase::Combat { return; }
    if creeps.is_empty() {
        next_phase.set(GamePhase::Resolution);
    }
}
```

### 6. Phase Transition System
```rust
fn resolution_phase(
    mut next_phase: ResMut<NextState<GamePhase>>,
    mut wave: ResMut<CurrentWave>,
    time: Res<Time>,
    mut timer: Local<f32>,
) {
    // During Resolution:
    // - Show "Level Complete" text (placeholder)
    // - Wait 3 seconds
    // - Increment wave counter
    // - Transition to Preparation for next wave
    // Do not distribute gold — that is Sprint 5's job
    *timer += time.delta_secs();
    if *timer >= 3.0 {
        *timer = 0.0;
        wave.0 += 1;
        next_phase.set(GamePhase::Preparation);
    }
}
```

### 7. Visual Feedback Requirements
From `01_legionRules.md §16`:

- **Projectile:** When a fighter attacks, spawn a small sphere that travels from fighter to creep in 0.15 seconds. Remove on arrival. Use fighter's legion color (placeholder: white for now).
- **Floating damage number:** On hit, show damage dealt as white floating text (same system as gold pop, different color/string).
- **King HP indicator:** A row of heart icons (or HP bar) above the King mesh. Update every time `king.hp` changes.
- **Combat speed:** From §16 — "A wave of 36 creeps engages 10-15 fighters and resolves within 15–20 seconds." At 20 ticks/second, the attack speed and damage values from `01_legionRules.md` should produce this pacing naturally. If a test with wave 1 creeps and basic fighters runs longer than 30 seconds, debug the damage values.

### 8. Fighter Selection UI (Minimal)
A simple HUD showing 3 placeholder fighter types the player can select (keyboard 1/2/3). Display name and gold cost. No real legion data yet — those come in Sprint 6.

| Key | Name | Cost | Attack | Armor |
|---|---|---|---|---|
| 1 | Footman | 120g | Normal | Heavy |
| 2 | Archer | 135g | Piercing | Light |
| 3 | Mage | 160g | Magic | Unarmored |

All placeholder — Sprint 6 replaces with real legion data.

## Task List (execute in order)

1. Add `GamePhase` state to both `headless_srvr` and `web_client` apps.
2. Implement `handle_build_input` with anti-maze validation.
3. Implement `handle_sell_input` with correct phase-based refund from §4.5.
4. Implement `fighter_attack_system` using `calc_damage` from `legion_core::combat`.
5. Implement `creep_leak_system` connected to `King` HP.
6. Implement `check_round_end` and `resolution_phase` phase transitions.
7. Implement projectile visual (traveling sphere).
8. Implement floating damage text (white) alongside existing gold text (yellow).
9. Implement King HP bar above King mesh.
10. Implement minimal 3-fighter HUD (keys 1/2/3).
11. Wire all systems into the `web_client` Bevy app with correct `run_if` phase conditions.

## Acceptance Criteria
- [ ] Player can click to place a fighter during Preparation phase
- [ ] Placing a tower that would fully block the path is rejected (red flash on cell)
- [ ] Selling a fighter during Preparation returns 90% gold; during Combat returns 50%
- [ ] Fighters auto-attack the nearest creep in range during Combat phase
- [ ] Damage matrix produces correct values — verify: Piercing vs Light = 1.5×, Siege vs Fortified = 1.5×, Chaos vs anything = 1.0×
- [ ] Creeps that reach the King zone decrement King HP
- [ ] When all creeps are dead, game transitions to Resolution then Preparation
- [ ] A wave of 36 (wave 1) creeps + 10 placed fighters resolves within 30 seconds
- [ ] Projectile travels from attacker to target visually
- [ ] King HP bar updates in real time

## Do NOT Do
- Do not implement income distribution or lumber — Sprint 5's job
- Do not implement real legion data (specific fighters, abilities) — Sprint 6's job
- Do not implement multiplayer or networking
- Do not skip the anti-maze path validation — it is required by `01_legionRules.md §15`
