---
description: agent_00_orchestrator
---

# Agent Orchestrator — Legion TD Bevy Implementation

## Overview
This file is the **entry point for the Antigravity workflow**. It defines the agent execution order, dependencies between agents, shared context, and the handoff protocol between sprints. Load this file first before dispatching any individual agent.

---

## Agent Roster

| File | Agent ID | Sprint | Status |
|---|---|---|---|
| `agent_sprint2_bevy_setup.md` | `bevy-setup` | Sprint 2 | Ready |
| `agent_sprint3_visual_prototype.md` | `visual-prototype` | Sprint 3 | Blocked by Sprint 2 |
| `agent_sprint4_combat_sandbox.md` | `combat-sandbox` | Sprint 4 | Blocked by Sprint 3 |
| `agent_sprint5_economy_waves.md` | `economy-waves` | Sprint 5 | Blocked by Sprint 4 |
| `agent_sprint6_7_legions_spells.md` | `legions-spells` | Sprint 6–7 | Blocked by Sprint 5 |

---
# Agent Orchestrator — Legion TD Bevy Implementation (Restructured)

## Overview
This is the authoritative entry point for the Antigravity workflow. 
**Architectural Mandate:** Follow a Modular ECS Pattern. Logic must be encapsulated in feature-specific directories within `libs/legion_core/src/` to ensure SOLID compliance and easy iteration.

---

## Agent Roster
| File | Agent ID | Sprint | Status |
|---|---|---|---|
| `agent_sprint2_bevy_setup.md` | `bevy-setup` | Sprint 2 | Ready |
| `agent_sprint3_visual_prototype.md` | `visual-prototype` | Sprint 3 | Blocked |
| `agent_sprint4_combat_sandbox.md` | `combat-sandbox` | Sprint 4 | Blocked |
| `agent_sprint5_economy_waves.md` | `economy-waves` | Sprint 5 | Blocked |
| `agent_sprint6_7_legions_spells.md` | `legions-spells` | Sprint 6–7 | Blocked |

---

## Core Modular Architecture (The "Tight & Nice" Rule)
To prevent "Mega-files" and ensure clean architecture, `legion_core` must follow this structure:

```text
legion_core/src/
├── lib.rs                 # Main entry, registers all Plugin groups
├── common/                # Shared types (Enums like AttackType/ArmorType)
├── features/              # Feature-specific logic (SOLID Isolation)
│   ├── combat/            # Systems, components, and damage matrix
│   ├── economy/           # Gold/Lumber tracking and award logic
│   ├── pathfinding/       # FlowField/NavMesh logic
│   ├── waves/             # Wave loading and spawner logic
│   └── legions/           # Ability archetypes and fighter templates
```

## Shared Context (All Agents Must Know)

### Tech Stack
- **Language:** Rust (stable toolchain)
- **Engine:** Bevy 0.13+
- **Physics:** `avian` or `bevy_rapier` (decided by `bevy-setup` agent — document the choice in `AGENTS.md`)
- **Workspace:** Cargo workspace at `/legion_td`
- **No networking in MVP** — all sprints are local/single-player simulation

### Canonical Reference Files
Every agent must treat these files as read-only authoritative sources. Never contradict them.

| File | What It Defines |
|---|---|
| `01_legionRules.md` | All game constants, formulas, wave tables, damage matrix |
| `02_TECH_STACK_ARCHITECTURE.md` | DB schema, service architecture (reference only — Bevy replaces Go/Node) |
| `03_DEVELOPMENT_ROADMAP.md` | Phase checklist and deliverable list |
| `04_STARTER_CODE.md` | Combat logic reference (Go → translate to Rust/Bevy) |
| `05_ART_DIRECTION.md` | Visual style, legion colors, unit shapes, camera setup |
| `implementation_plan_md.resolved` | Sprint goals, workspace structure, collision requirements |

### Non-Negotiable Rules (Apply to All Agents)
1. **No invented constants.** Every numeric game value must be sourced from `01_legionRules.md`. If a value is unknown, use `None`/`null`/`0.0` with a `// TODO: source from w3u` comment.
2. **Server-authoritative economy.** All gold/lumber mutations go through `award_gold()` / `spend_gold()` — never direct field assignment. (From `01_legionRules.md §11`)
3. **Anti-maze enforcement.** Tower placement must validate that a path still exists from spawn to King. Reject if blocked. (From `01_legionRules.md §15`)
4. **ECS-first.** All game state lives in Bevy components. No global mutable state outside `Resource`s.
5. **Headless verifiable.** Every sprint must leave `headless_srvr` in a runnable state that produces loggable output.

---

## Execution Order & Handoff Protocol

### How to Run an Agent
1. Load the orchestrator (this file) first.
2. Load the target agent's `.md` file.
3. Load the context documents listed in that agent's **Context Documents** section.
4. Execute the **Task List** in order.
5. Verify all **Acceptance Criteria** before marking the sprint complete.
6. Write a handoff note to `AGENTS.md` (see below).

### AGENTS.md (Living Document)
Each agent appends to `/legion_td/AGENTS.md` when its sprint is complete:

```markdown
## Sprint N — [Agent ID] — COMPLETE

**Date:** YYYY-MM-DD
**Physics crate chosen:** avian | bevy_rapier  (Sprint 2 only)
**Acceptance criteria met:** all / partial (list failures)
**Known issues / TODOs for next agent:**
- [ ] Issue 1
- [ ] Issue 2
**Modules created:**
- `legion_core/src/components.rs`
- `legion_core/src/wave_data.rs`
- ...
```

---

## Dependency Graph

```
Sprint 1 (Foundation — COMPLETE)
    └── Sprint 2: bevy-setup
            └── Sprint 3: visual-prototype
                    └── Sprint 4: combat-sandbox
                            └── Sprint 5: economy-waves
                                    └── Sprint 6–7: legions-spells
                                            └── Future: Networking / Matchmaking
```

---

## Shared Module Map (Grows Each Sprint)

After each sprint, this map must be updated to reflect what exists in the codebase.

### After Sprint 2 (`bevy-setup`)
```
legion_core/src/
├── lib.rs              (exports all modules)
├── components.rs       (Fighter, Creep, King, PlayerEconomy, AttackType, ArmorType)
├── wave_data.rs        (WAVE_COUNT, WAVE_BOUNTY, WAVE_END_GOLD, etc.)
└── combat.rs           (damage_multiplier, calc_damage)

apps/headless_srvr/src/main.rs   (minimal Bevy app, GamePhase enum)
apps/web_client/src/main.rs      (stub)
```

### After Sprint 3 (`visual-prototype`)
```
legion_core/src/
└── pathfinding.rs      (FlowField or NavMesh, update_on_tower_change)

apps/web_client/src/
├── scene.rs            (setup_lane, setup_king, setup_camera)
├── spawn.rs            (spawn_wave — wave 1 only)
├── movement.rs         (move_creeps, boid_separation)
└── vfx.rs              (floating_gold_text)
```

### After Sprint 4 (`combat-sandbox`)
```
apps/web_client/src/
├── input.rs            (handle_build_input, handle_sell_input)
├── combat.rs           (fighter_attack_system, creep_leak_system)
├── phases.rs           (check_round_end, resolution_phase)
└── ui.rs               (fighter_palette_hud, king_hp_bar)
```

### After Sprint 5 (`economy-waves`)
```
data/
└── waves.json          (31 wave entries)

legion_core/src/
├── wave_data.rs        (+ WaveDatabase, WaveEntry, build_timer_secs, income_cap)
└── economy.rs          (FightersValue, distribute_income, award_gold, spend_gold)

apps/headless_srvr/src/
└── simulation.rs       (full 30-wave headless loop with --waves flag)
```

### After Sprint 6–7 (`legions-spells`)
```
data/
├── fighters.json       (all 15 legions, Nature/Undead/Goblin complete)
└── mercenaries.json    (stub entries)

legion_core/src/
├── abilities.rs        (all 11 ability components + systems)
└── legions.rs          (FighterDatabase, LegionData, FighterTemplate)

apps/web_client/src/
├── legion_select.rs    (legion picker screen)
└── upgrade.rs          (handle_upgrade_input)
```

---

## Quick Reference: Key Constants (from `01_legionRules.md`)

```rust
// Starting resources
STARTING_GOLD: u32   = 750
STARTING_LUMBER: u32 = 150
FOOD_CAP: u32        = 250

// Sell refunds
SELL_PERCENT: f32            = 0.50  // during combat
SELL_PERCENT_PREBATTLE: f32  = 0.90  // during preparation

// Max waves
MAX_WAVE: u8 = 30  // +1 Arena wave (31)

// Legions
NUM_LEGIONS: u8 = 15

// Build timer formula: 40 + (wave / 2) seconds
// Income cap formula: 0.025w³ + 0.05w² + 4w + 20

// Boss waves: 10, 20, 30 (3 creeps, Chaos attack)
// Air waves: 5, 13, 21, 29
// Arena battles at waves 10 and 20 (42s prep, 150s max)
```

---

## Failure Protocol
If an agent's acceptance criteria cannot be fully met:
1. Complete what is possible.
2. Document failures clearly in `AGENTS.md`.
3. Do not block the next sprint for minor failures — list them as TODOs.
4. Block the next sprint only if a core system is missing (e.g., pathfinding not working → Sprint 4 cannot proceed).

---

## Future Work (Post-MVP, Not Assigned to Any Agent)
From `implementation_plan_md.resolved`:
- Deterministic netcode / rollback networking
- Server orchestration
- Matchmaking
- Real `war3map.w3u` binary parser (for exact fighter HP/damage values)
- Arena mode (wave 31 cross-team fight — `01_legionRules.md §9`)


