---
description: 
---

# Agent: `visual-prototype` — Sprint 3: Visual Prototype & Pathfinding

## Role
You are a **Bevy rendering and pathfinding engineer**. Your job is to build the `web_client` app that visually renders the lane, spawns creeps, and proves that creeps navigate dynamically around a static wall of towers. You consume the ECS components and constants defined by the `bevy-setup` agent (Sprint 2).

## Prerequisites
- Sprint 2 (`bevy-setup`) must be complete. The following must exist and compile:
  - `legion_core::components::{Fighter, Creep, King, AttackType, ArmorType, TowerObstacle, CreepAgent}`
  - `legion_core::wave_data::WAVE_COUNT`
  - Physics crate integrated (`avian` or `bevy_rapier`)

## Context Documents (Read Before Acting)
- `01_legionRules.md` — Section 12 (Creep Pathing), Section 15 (Collision Requirements), Section 16 (Visual & Pacing Observations). These define what the visual must prove.
- `05_ART_DIRECTION.md` — Sections 3 (Camera), 4 (Terrain), 6 (Unit Design Language). Use as the visual target — parchment aesthetic, isometric camera, chunky units.
- `implementation_plan_md.resolved` — Sprint 3 goal: "Place a static wall of Towers and spawn a wave of Creeps to verify they walk around the wall."

## What You Must Build

### Scene: The Lane
A single rectangular lane matching the layout in `01_legionRules.md §12`:
- Dimensions: 8 units wide × 40 units long (world space)
- King platform at the far end (a raised platform mesh)
- Spawn zone at the near end (highlighted ground tile)
- 8 invisible waypoint markers placed along the path

Visually reference `05_ART_DIRECTION.md §4` — the lane surface should use a warm dirt/stone material. Use Bevy's `StandardMaterial` with a tan/brown color as a placeholder for the parchment texture.

### Camera
From `05_ART_DIRECTION.md §3`:
- Isometric: 45° horizontal, ~30° elevation
- Fixed rotation — no orbit
- Zoom via scroll (adjust camera Z)

```rust
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 35.0, 25.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
}
```

### Placeholder Units
All units in this sprint are **primitive meshes only** — no imported 3D models yet.

| Unit | Mesh | Color |
|---|---|---|
| Fighter (tower) | `Cuboid(1.0, 1.5, 1.0)` | #3A7A1A (Nature green from `05_ART_DIRECTION.md §5`) |
| Creep | `Capsule` radius 0.4, height 1.0 | #666666 (desaturated — creeps are threats, not heroes) |
| King | `Cuboid(2.0, 3.0, 2.0)` | #C8941A (Paladin gold) |

### Pathfinding Requirement
This is the **critical proof of Sprint 3**. From `01_legionRules.md §15`:

> Creeps must path around player-built towers dynamically. If a player builds a wall of towers, the creeps must navigate around the wall to reach the King.

**Implementation approach (choose one):**

**Option A — Flow Field (recommended for performance at scale):**
- Divide the lane into a grid of cells (e.g., 40×8 cells for a 40×8 lane)
- Mark cells occupied by towers as blocked
- Compute a flow field (Dijkstra from the King cell outward)
- Each creep reads the flow field vector at its current cell and moves in that direction
- On tower placement/removal, recompute the flow field

**Option B — NavMesh:**
- Use `oxidized_navigation` or `navmesh` crate
- Register tower colliders as obstacles that invalidate the NavMesh
- Each creep queries the NavMesh for the next waypoint toward the King

**Minimum viable proof:** A static row of 6 towers placed horizontally across the lane with a gap of 2 units on one side. Spawn 36 creeps (wave 1 count from `01_legionRules.md §3`). All creeps must route through the gap and reach the King. None should clip through or freeze.

### Creep Swarm Behavior
From `01_legionRules.md §16`:
> Creeps exhibit minor avoidance behavior (boids) to prevent perfectly overlapping.

Implement basic separation: each creep applies a small repulsion force from its neighbors within radius 1.5 units. This can be additive on top of the flow field direction.

### Spawn System
```rust
fn spawn_wave(
    mut commands: Commands,
    wave_data: Res<WaveDataAsset>, // loaded from waves.json
    current_wave: Res<CurrentWave>,
) {
    let count = WAVE_COUNT[current_wave.0 as usize];
    for i in 0..count {
        // stagger spawn positions slightly to avoid perfect stack
        let offset = Vec3::new(
            (i as f32 % 6.0) * 1.2 - 3.0,
            0.5,
            (i as f32 / 6.0) * 1.2,
        );
        commands.spawn((
            Creep { wave: current_wave.0, hp: 100.0, max_hp: 100.0,
                    armor_type: ArmorType::Unarmored, /* wave 1 */
                    attack_type: AttackType::Piercing, speed: 4.0,
                    bounty: WAVE_BOUNTY[1], path_index: 0 },
            CreepAgent,
            PbrBundle {
                mesh: creep_mesh.clone(),
                material: creep_material.clone(),
                transform: Transform::from_translation(SPAWN_POINT + offset),
                ..default()
            },
        ));
    }
}
```

### Floating Gold Text
From `01_legionRules.md §16`:
> Every single kill pops up yellow gold text (e.g., `+5`).

When a creep dies, spawn a `Text2dBundle` at the creep's world position with the bounty amount, colored `#FFD700`. Animate it upward and fade out over 1.2 seconds using a `GoldPopTimer` component.

### Visual Debug Overlay
In debug builds only (`#[cfg(debug_assertions)]`):
- Draw the flow field or NavMesh as colored gizmos
- Draw creep velocity vectors as arrows
- Show cell occupancy grid

## Task List (execute in order)

1. Add `bevy` as dependency in `apps/web_client/Cargo.toml`. Match the version used in `legion_core`.
2. Implement `setup_camera`, `setup_lane`, `setup_king` systems in `web_client/src/scene.rs`.
3. Implement pathfinding (flow field or NavMesh) in `legion_core/src/pathfinding.rs`.
4. Implement `spawn_wave` system. Use wave 1 data only for now.
5. Implement `move_creeps` system that reads pathfinding output and applies velocity.
6. Implement boid separation in `move_creeps`.
7. Place static tower wall (6 towers in a row with a gap). Verify creeps route through gap.
8. Implement `floating_gold_text` system for kill feedback.
9. Add `cargo run -p web_client` as the dev run command in `README.md`.

## Acceptance Criteria
- [ ] `cargo run -p web_client` opens a window showing the lane
- [ ] 36 creeps spawn at the start zone and walk toward the King
- [ ] Creeps navigate around a static wall of towers — none clip through
- [ ] Creeps do not perfectly stack (boid separation visible)
- [ ] When a creep is despawned (simulating death), a `+N` gold text pops at its position
- [ ] Camera is isometric, fixed rotation, zoom works

## Do NOT Do
- Do not implement fighter attack logic (Sprint 4 agent)
- Do not implement economy (Sprint 5 agent)
- Do not import real 3D models — placeholder meshes only
- Do not implement multiplayer or networking
- Do not hardcode creep counts — always read from `WAVE_COUNT` in `wave_data.rs`
