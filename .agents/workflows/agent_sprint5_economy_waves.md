---
description: 
---

# Agent: `economy-waves` — Sprint 5: Economy & Full Wave Simulation

## Role
You are a **game economy and data systems engineer**. Your job is to connect all economy formulas from the game rules, load the full 30-wave dataset, and produce a verified full-game simulation. After this sprint, running the `headless_srvr` should produce a complete play-through log matching the expected gold values from `01_legionRules.md`.

## Prerequisites
- Sprint 4 (`combat-sandbox`) must be complete:
  - `GamePhase` state machine works (Preparation → Combat → Resolution loop)
  - Fighters attack, creeps die with bounty, creeps leak and decrement King HP
  - Sell refund logic exists (phase-aware)
  - `PlayerEconomy` component tracks `gold` and `lumber`

## Context Documents (Read Before Acting)
- `01_legionRules.md` — Section 3 (Wave System, all tables), Section 4 (Economy, all formulas), Section 5 (Fighter Value System), Section 8 (Mercenary System), Section 11 (Anti-Cheat note)
- `04_STARTER_CODE.md` — Step 1 (wave extraction script) and Step 5 (Docker Compose) for reference on data loading pattern
- `implementation_plan_md.resolved` — Sprint 5 goal: "Connect the `waves.json` and income math. Track gold/lumber correctly based on kills and leaks."

## What You Must Build

### 1. waves.json (Data File)
Generate `data/waves.json` from the constants in `01_legionRules.md §3`. This file is loaded at runtime by both `headless_srvr` and `web_client`.

```json
{
  "waves": [
    {
      "wave": 1,
      "count": 36,
      "bounty_per_kill": 3,
      "end_round_gold": 11,
      "armor_type": "unarmored",
      "attack_type": "piercing",
      "is_air": false,
      "is_boss": false,
      "is_ranged": false,
      "build_timer_seconds": 40,
      "income_cap": 24
    }
    // ... waves 2–31
  ]
}
```

All 31 values for each field come from `01_legionRules.md §3`. Use the `income_cap` formula: `0.025w³ + 0.05w² + 4w + 20`. Wave 31 (Arena) has `end_round_gold: 0` and `bounty_per_kill: 0`.

### 2. Bevy Asset: WaveDataAsset
Load `waves.json` as a Bevy asset using `serde_json`. Make it available as a `Resource`.

```rust
// legion_core/src/wave_data.rs (extend existing file)

#[derive(Resource, serde::Deserialize, Debug)]
pub struct WaveDatabase {
    pub waves: Vec<WaveEntry>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct WaveEntry {
    pub wave: u8,
    pub count: u32,
    pub bounty_per_kill: u32,
    pub end_round_gold: u32,
    pub armor_type: String,      // parse to ArmorType enum
    pub attack_type: String,     // parse to AttackType enum
    pub is_air: bool,
    pub is_boss: bool,
    pub is_ranged: bool,
    pub build_timer_seconds: u32,
    pub income_cap: u32,
}

impl WaveEntry {
    pub fn armor(&self) -> ArmorType { /* match string */ }
    pub fn attack(&self) -> AttackType { /* match string */ }
}
```

### 3. Resolution Phase: Income Distribution
This replaces the placeholder `resolution_phase` stub from Sprint 4. Runs at the start of every Resolution phase.

**Step-by-step (from `01_legionRules.md §4`):**

```rust
fn distribute_income(
    wave: Res<CurrentWave>,
    wave_db: Res<WaveDatabase>,
    mut economy: Query<(&mut PlayerEconomy, &FightersValue)>,
    recommend_values: Res<RecommendValues>, // loaded from RECOMMEND_VALUE array
) {
    let w = wave.0 as usize;
    let entry = &wave_db.waves[w - 1];

    for (mut eco, fv) in economy.iter_mut() {
        // 1. Wave completion bonus (flat gold)
        let completion_gold = entry.end_round_gold;

        // 2. Player income (proportional to fighter value vs recommended value)
        //    income_cap = entry.income_cap
        //    ratio = fighters_value / recommend_value[wave]
        //    player_income = (ratio * income_cap).min(income_cap) as u32
        let recommend = RECOMMEND_VALUE[w] as f32;
        let ratio = (fv.value as f32 / recommend).min(1.0);
        let player_income = (ratio * entry.income_cap as f32) as u32;

        // 3. Add both to player gold
        eco.gold += completion_gold + player_income;

        // 4. Log for headless verification
        #[cfg(feature = "headless")]
        println!(
            "[Wave {}] Player {} | +{}g completion | +{}g income | total: {}g",
            w, eco.player_id, completion_gold, player_income, eco.gold
        );
    }
}
```

### 4. FightersValue Component
Tracks the total point value of a player's placed fighters. Must update whenever a fighter is built or sold.

```rust
#[derive(Component, Default)]
pub struct FightersValue {
    pub value: u32,   // sum of Fighter::point_value for all fighters owned by this player
}

fn update_fighters_value(
    fighters: Query<(&Fighter, &OwnerPlayer)>,
    mut fv_query: Query<(&PlayerEconomy, &mut FightersValue)>,
) {
    // Reset all
    for (_, mut fv) in fv_query.iter_mut() { fv.value = 0; }
    // Accumulate
    for (fighter, owner) in fighters.iter() {
        if let Ok((_, mut fv)) = fv_query.get_mut(owner.entity) {
            fv.value += fighter.point_value;
        }
    }
}
```

### 5. Lumber System
From `01_legionRules.md §8` — lumber is the currency for mercenaries (Sprint 6). In Sprint 5, just implement the tracking and starting value.

```rust
// Starting lumber: 150 (from §Economy System, Starting Resources)
// Lumber is spent when sending mercenaries (not yet implemented)
// Lumber trickle per wave: determined by Lumberjack upgrades (Sprint 6)
// For now: add a flat lumber_trickle of 5 per wave completion as a placeholder
fn award_lumber(mut economy: Query<&mut PlayerEconomy>) {
    for mut eco in economy.iter_mut() {
        eco.lumber += 5; // placeholder — real value comes from Lumberjack tech in Sprint 6
    }
}
```

### 6. Build Phase Timer from Wave Data
Replace the hardcoded 40-second timer with the formula from `01_legionRules.md §3`:

```rust
fn start_preparation_phase(
    wave: Res<CurrentWave>,
    wave_db: Res<WaveDatabase>,
    mut timer: ResMut<PreparationTimer>,
) {
    let entry = &wave_db.waves[wave.0 as usize - 1];
    timer.duration = Duration::from_secs(entry.build_timer_seconds as u64);
    timer.reset();
}
```

Wave 1 = 40s, Wave 15 = 47s, Wave 30 = 55s — all from `build_timer_seconds` in `waves.json`.

### 7. Wave Spawner: Full 30 Waves
The spawner now reads from `WaveDatabase` for every wave, not just wave 1.

```rust
fn spawn_wave(
    mut commands: Commands,
    wave: Res<CurrentWave>,
    wave_db: Res<WaveDatabase>,
    meshes: ..., materials: ...,
) {
    let entry = &wave_db.waves[wave.0 as usize - 1];
    let armor = entry.armor();
    let attack = entry.attack();
    let bounty = entry.bounty_per_kill;
    let count = entry.count;

    for i in 0..count {
        let offset = Vec3::new((i as f32 % 6.0) * 1.2 - 3.0, 0.5, (i as f32 / 6.0) * 1.2);
        commands.spawn((
            Creep { wave: wave.0, hp: base_hp_for_wave(wave.0),
                    max_hp: base_hp_for_wave(wave.0),
                    armor_type: armor, attack_type: attack,
                    speed: speed_for_wave(wave.0, entry.is_air),
                    bounty, path_index: 0 },
            // ... mesh bundle
        ));
    }
}
```

Note: `base_hp_for_wave` and `speed_for_wave` are not in `01_legionRules.md` — the original game reads these from `war3map.w3u`. Use placeholder scaling for now: `hp = 80.0 + wave * 15.0`, `speed = 4.0` (non-air), `speed = 5.5` (air waves: 5, 13, 21, 29). Sprint 6 will replace with real values.

### 8. Headless Simulation Verification
The `headless_srvr` must now be able to simulate all 30 waves and output a log that can be checked against `01_legionRules.md` tables.

Run command: `cargo run -p headless_srvr -- --waves=30 --fighters=auto`

Expected log format:
```
[Wave 01] spawned 36 creeps (Unarmored/Piercing) | timer: 40s
[Wave 01] 36 killed, 0 leaked | bounty earned: 108g | completion: 11g | income: Ng
[Wave 10] spawned 3 creeps (Light/Chaos BOSS) | timer: 45s
[Wave 10] 3 killed, 0 leaked | bounty earned: 153g | completion: 30g | income: Ng
...
[Wave 30] FINAL BOSS
```

Verify against `01_legionRules.md §3`:
- Wave 1: 36 creeps, bounty=3, end_gold=11 ✓
- Wave 10: 3 creeps, bounty=51, end_gold=30 ✓  
- Wave 20: 3 creeps, bounty=86, end_gold=100 ✓
- Wave 30: 3 creeps, bounty=123, end_gold=200 ✓

### 9. Anti-Cheat Note
From `01_legionRules.md §11`:
> Any reimplementation must have server-authoritative resource tracking. Never trust client-reported gold/lumber values.

In `headless_srvr`, all gold changes must go through a single `fn award_gold(player_id, amount, reason)` function that logs every transaction. No direct mutation of `eco.gold` outside this function.

## Task List (execute in order)

1. Generate `data/waves.json` with all 31 wave entries matching `01_legionRules.md §3`.
2. Add `serde_json` dependency and implement `WaveDatabase` asset loading.
3. Implement `FightersValue` component and `update_fighters_value` system.
4. Replace stub `resolution_phase` with full `distribute_income` + `award_lumber`.
5. Replace hardcoded build timer with `start_preparation_phase` reading from `wave_db`.
6. Upgrade `spawn_wave` to support all 30 waves using `WaveDatabase`.
7. Implement `headless_srvr` simulation loop with `--waves=N` CLI flag.
8. Run headless simulation for all 30 waves and verify gold output matches §3 tables.
9. Add `award_gold` audit function — all gold mutations go through it.

## Acceptance Criteria
- [ ] `data/waves.json` exists with 31 entries; all values match `01_legionRules.md §3` exactly
- [ ] Headless sim prints correct bounty/end_gold for waves 1, 10, 20, 30
- [ ] Build timer at wave 1 = 40s, wave 30 = 55s
- [ ] Income formula `0.025w³ + 0.05w² + 4w + 20` is used, not approximated
- [ ] Wave 5 spawns air creeps; wave 10 spawns boss (3 creeps, Chaos attack)
- [ ] All gold changes go through `award_gold` and appear in headless log
- [ ] `cargo run -p headless_srvr -- --waves=30` completes without panic

## Do NOT Do
- Do not implement real mercenary purchasing (Sprint 6)
- Do not implement real legion/fighter ability data (Sprint 6)
- Do not implement real `war3map.w3u` creep HP values — use placeholder scaling
- Do not implement multiplayer or matchmaking (post-MVP)
- Do not skip the headless verification — it is the proof of correctness for this sprint
