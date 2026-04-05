---
description: 
---

# Agent: `legions-spells` — Sprints 6 & 7: Legions & Abilities

## Role
You are a **game content and ability systems engineer**. Your job is to replace all placeholder fighter data with real Legion fighter stats, implement the upgrade chain system, and build the ability component system that handles all 11 ability archetypes from `01_legionRules.md`. After this sprint, the game is feature-complete for a single-player sandbox against 30 waves.

## Prerequisites
- Sprint 5 (`economy-waves`) must be complete:
  - Full 30-wave simulation runs and verifies against `01_legionRules.md §3`
  - `FightersValue`, `PlayerEconomy`, `WaveDatabase` all working
  - `distribute_income` uses real formula from §4
  - `headless_srvr` verification log passes

## Context Documents (Read Before Acting)
- `01_legionRules.md` — Section 2 (Legions, 15 factions), Section 7 (King System), Section 8 (Mercenary System), Section 9 (Arena Mode)
- `02_TECH_STACK_ARCHITECTURE.md` — Section 5 (Database Schema for fighters table) — use this as the JSON schema shape
- `03_DEVELOPMENT_ROADMAP.md` — Phase 4 (Content Pass) — lists all ability archetypes and data extraction approach
- `05_ART_DIRECTION.md` — Section 5 (Legion color identities) — apply correct colors to each legion's units

## What You Must Build

### 1. Fighter Data JSON (`data/fighters.json`)
Create the canonical fighter data file. Schema per `02_TECH_STACK_ARCHITECTURE.md §5`:

```json
{
  "legions": {
    "nature": {
      "name": "Nature",
      "color_hex": "#3A7A1A",
      "builder_id": "u003",
      "fighters": [
        {
          "id": "h_nat_01",
          "name": "Treant",
          "tier": 1,
          "cost": 120,
          "point_value": 120,
          "hp": 350,
          "armor_type": "light",
          "damage_min": 18,
          "damage_max": 24,
          "attack_speed": 1.5,
          "attack_range": 100,
          "attack_type": "normal",
          "is_ranged": false,
          "abilities": ["heal_aura"],
          "upgrades_to": ["h_nat_02a", "h_nat_02b"]
        }
      ]
    }
  }
}
```

**Source note:** Real HP/damage values come from `war3map.w3u` (binary, not yet parsed). For this sprint, use the closest community-documented values for Legion TD Mega 3.41 fighters. Mark each entry with `"source": "community"` so it can be replaced when `w3u` parsing is complete. Do not invent stats — if unknown, use `"hp": null` and skip in-game rather than guessing.

**Required: at minimum 3 fully populated legions** to make the game playable for testing:
- **Nature** (Group A) — baseline, recommended first
- **Undead** (Group C) — different ability flavor
- **Goblin** (Group B) — Mech/ranged flavor, has custom MDX models in the ZIP

All 15 legions should have at least skeleton entries (name, color, builder_id) even if fighters are incomplete.

### 2. FighterDataAsset Resource
Load `data/fighters.json` at startup, same pattern as `WaveDatabase`.

```rust
#[derive(Resource, serde::Deserialize)]
pub struct FighterDatabase {
    pub legions: HashMap<String, LegionData>,
}

#[derive(serde::Deserialize)]
pub struct LegionData {
    pub name: String,
    pub color_hex: String,
    pub builder_id: String,
    pub fighters: Vec<FighterTemplate>,
}

#[derive(serde::Deserialize, Clone)]
pub struct FighterTemplate {
    pub id: String,
    pub name: String,
    pub tier: u8,
    pub cost: u32,
    pub point_value: u32,
    pub hp: f32,
    pub armor_type: String,
    pub damage_min: f32,
    pub damage_max: f32,
    pub attack_speed: f32,
    pub attack_range: f32,
    pub attack_type: String,
    pub is_ranged: bool,
    pub abilities: Vec<String>,
    pub upgrades_to: Vec<String>,
}
```

### 3. Ability Component System
From `03_DEVELOPMENT_ROADMAP.md §Phase 4.4` — implement all 11 ability archetypes. Each is a Bevy `Component` added to a `Fighter` entity at spawn time based on its `abilities` list in `fighters.json`.

```rust
// Auras (passive, affect nearby allies or enemies)
#[derive(Component)] pub struct FrostAura    { pub radius: f32, pub slow_factor: f32 }
#[derive(Component)] pub struct HealAura     { pub radius: f32, pub hp_per_sec: f32 }

// On-hit effects
#[derive(Component)] pub struct SplashDamage { pub radius: f32, pub dmg_factor: f32 }
#[derive(Component)] pub struct Biotoxin     { pub dps: f32, pub duration: f32 }
#[derive(Component)] pub struct TripleAttack { pub extra_hits: u8 }

// Triggered abilities
#[derive(Component)] pub struct ManaShield   { pub mana: f32, pub max_mana: f32 }
#[derive(Component)] pub struct Berserk      { pub hp_threshold: f32, pub speed_bonus: f32 }
#[derive(Component)] pub struct RaiseDead    { pub skeleton_count: u8 }
#[derive(Component)] pub struct MarkTarget   { pub dmg_amp: f32, pub duration: f32 }
#[derive(Component)] pub struct GuardianSpirit { pub revive_hp_fraction: f32 }
#[derive(Component)] pub struct Catastrophe  { pub radius: f32, pub damage: f32 }
```

**Systems to implement per ability type:**

| Ability | System | Trigger |
|---|---|---|
| `FrostAura` | `frost_aura_system` | Every tick during Combat — apply slow to creeps in radius |
| `HealAura` | `heal_aura_system` | Every tick — restore HP to fighters in radius |
| `SplashDamage` | Modify `fighter_attack_system` | On attack — deal secondary AoE |
| `Biotoxin` | `biotoxin_system` | On hit — attach `PoisonDot` component to creep |
| `TripleAttack` | Modify `fighter_attack_system` | After primary hit — fire N additional hits |
| `ManaShield` | `mana_shield_system` | On damage received — absorb with mana first |
| `Berserk` | `berserk_system` | When HP < threshold — boost attack speed |
| `RaiseDead` | `raise_dead_system` | On creep kill — spawn skeleton Creep ally |
| `MarkTarget` | `mark_target_system` | On attack — attach `Marked` debuff to creep |
| `GuardianSpirit` | `guardian_spirit_system` | On fighter death — revive at fraction HP once |
| `Catastrophe` | `catastrophe_system` | Periodic cast — AoE nuke centered on fighter |

### 4. Upgrade System
When a player right-clicks a placed fighter during Preparation phase and presses U (or similar), show upgrade options from `upgrades_to` list.

```rust
fn handle_upgrade_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    selected: Res<SelectedUnit>,
    fighter_db: Res<FighterDatabase>,
    fighters: Query<(&Fighter, &OwnerPlayer)>,
    mut economy: Query<&mut PlayerEconomy>,
    mut commands: Commands,
    phase: Res<State<GamePhase>>,
) {
    if *phase.get() != GamePhase::Preparation { return; }
    // 1. Get selected fighter's template ID
    // 2. Look up upgrades_to list in fighter_db
    // 3. Show upgrade choice UI (if 1 option: auto-select; if 2: show buttons)
    // 4. Cost = new_template.cost - current_template.cost
    // 5. Check player has enough gold
    // 6. Despawn current fighter entity
    // 7. Spawn new fighter entity at same position with new template stats
    // 8. Deduct upgrade cost from player gold
    // 9. Update FightersValue
}
```

### 5. Legion Selection UI
Replace the 3-placeholder HUD from Sprint 4 with a real Legion picker shown at game start (before wave 1 prep phase begins).

- Show all 15 legion names and their color (from `05_ART_DIRECTION.md §5`)
- On selection: set `CurrentLegion` resource, filter fighter palette to show only that legion's tier-1 fighters
- For legions with incomplete data: show as greyed out / "coming soon"

### 6. Mercenary Stubs (Partial)
From `01_legionRules.md §8`: mercs are bought with lumber during Preparation and spawn in the opponent's lane at wave start.

In this sprint, implement the **lumber spending UI only** — no actual cross-lane spawning yet (that requires networking, post-MVP). When a player "sends" a merc, deduct lumber and log the intent. The actual spawn will be wired in the multiplayer sprint.

```rust
fn handle_send_mercenary(
    mut eco: Query<&mut PlayerEconomy>,
    merc_selection: Res<SelectedMercenary>,
    phase: Res<State<GamePhase>>,
) {
    if *phase.get() != GamePhase::Preparation { return; }
    // Deduct lumber cost
    // Log: "Player N sent MercType to opponent lane"
    // Stub: do not actually spawn cross-lane
}
```

### 7. King Abilities (from §7)
From `01_legionRules.md §7` — King has auto-cast abilities that trigger during Combat:

Implement `king_autocast_system` with 2 placeholder abilities:
- **Cleave:** Every 8 seconds, King deals AoE damage to creeps within radius 4.0
- **Regeneration:** King heals 2 HP per second when not at max HP

These activate during Combat phase when King HP > 0.

### 8. Visual: Legion Colors on Units
From `05_ART_DIRECTION.md §5` — each legion has a `color_hex`. Apply it to the fighter mesh material:

```rust
fn colorize_fighter_by_legion(
    mut materials: ResMut<Assets<StandardMaterial>>,
    fighters: Query<(Entity, &Fighter, &Handle<StandardMaterial>), Added<Fighter>>,
    fighter_db: Res<FighterDatabase>,
    current_legion: Res<CurrentLegion>,
) {
    if let Some(legion) = fighter_db.legions.get(&current_legion.id) {
        let color = Color::hex(&legion.color_hex).unwrap_or(Color::WHITE);
        for (_, _, mat_handle) in fighters.iter() {
            if let Some(mat) = materials.get_mut(mat_handle) {
                mat.base_color = color;
            }
        }
    }
}
```

## Task List (execute in order)

1. Create `data/fighters.json` with Nature, Undead, Goblin fully populated + skeleton entries for remaining 12.
2. Implement `FighterDatabase` asset and load at startup.
3. Implement all 11 `Ability` components (struct definitions only — no system logic yet).
4. Implement `FrostAura` and `HealAura` systems (passive auras — simplest to verify).
5. Implement `SplashDamage`, `Biotoxin`, `TripleAttack` (modify attack system).
6. Implement `ManaShield`, `Berserk`, `RaiseDead`, `MarkTarget`, `GuardianSpirit`, `Catastrophe`.
7. Implement `handle_upgrade_input` with cost delta and entity swap.
8. Implement Legion selection UI screen (before wave 1).
9. Apply legion colors to fighter materials via `colorize_fighter_by_legion`.
10. Implement `handle_send_mercenary` (lumber deduct + log stub).
11. Implement `king_autocast_system` (cleave + regen).
12. Run headless sim with Nature legion, all 30 waves — verify no panics.

## Acceptance Criteria
- [ ] `data/fighters.json` exists; Nature, Undead, Goblin legions have complete fighter stats
- [ ] All 15 legions appear in the legion selection screen (incomplete ones greyed out)
- [ ] Fighter palette updates to show selected legion's fighters after legion pick
- [ ] Upgrade system works: selecting a Tier 1 fighter and pressing U offers Tier 2 choices
- [ ] Upgrade cost = new cost − old cost; player gold deducted correctly
- [ ] `FrostAura` slows creeps within radius; creep speed visually decreases
- [ ] `HealAura` restores HP to fighters; HP bar on affected fighters rises
- [ ] `SplashDamage` deals secondary damage to creeps adjacent to primary target
- [ ] `GuardianSpirit` revives the fighter once when it would die
- [ ] King autocasts Cleave every 8 seconds during combat — visible AoE effect
- [ ] Fighter mesh color matches legion color from `05_ART_DIRECTION.md §5`
- [ ] `cargo run -p headless_srvr -- --waves=30 --legion=nature` completes without panic

## Do NOT Do
- Do not implement actual cross-lane mercenary spawning (requires multiplayer)
- Do not parse `war3map.w3u` binary — use community-sourced values, marked as such
- Do not implement netcode or matchmaking (post-MVP per `implementation_plan_md.resolved`)
- Do not invent fighter stats if unknown — use `null` and skip gracefully
- Do not skip the `05_ART_DIRECTION.md` legion colors — visual identity matters for testing
