# Legion TD — Core Game Mechanics Reference
> **For AI agents (Gemini / Antigravity):** This document is the authoritative source for Legion TD mechanics, extracted directly from `war3map.j` (the JASS source of Legion TD Mega 3.41). Every formula, constant, and system here is pulled from actual game code, not approximation. Read fully before generating any game logic.

---

## 1. High-Level Concept

Legion TD is a **cooperative tower-defense / arena hybrid** for 2–10 players (typically 2v2–4v4). Two teams compete on mirrored lanes. Each player defends their own lane while attacking the opponent by **sending mercenaries**. The last team with a living King wins.
**Genre:** Tower Defense (Team-based)
**Format:** 4v4 (8 players + 2 AI Kings + 2 AI Wave Spawners)
**Win Condition:** Destroy the enemy King before yours falls

Core loop:
```
Build Phase → Combat Phase → Income Phase → repeat (30 total waves + Arena at wave 31)
Each level follows a 4-phase cycle repeated 30 times:

1. **Preparation Phase** - Variable duration per level. Players build/upgrade units.
2. **Wave Spawn** - Creeps spawn from AI players (P10/P11) and march toward kings.
3. **Combat Phase** - Player fighters engage creeps. Surviving creeps damage the king.
4. **Resolution** - Gold rewards distributed: king kills, level completion, income.

Boss waves at levels 10, 20, and 30. Arena battles at levels 10 and 20.
```

---
## Teams & Players

| Role | Players | Notes |
|------|---------|-------|
| West Team | P1, P2, P3, P4 | Human players |
| East Team | P5, P6, P7, P8 | Human players |
| West King | P8 (Computer) | 'West Legion' |
| East King | P9 (Computer) | 'East Legion' |
| Wave Spawner West | P10 | Spawns creeps attacking West |
| Wave Spawner East | P11 | Spawns creeps attacking East |

---
## Game Modes

### Primary Modes (mutually exclusive)

| Code | Name | Description |
|------|------|-------------|
| `-ap` | A | ll Pick. All players can pick their race. This is the default mode. |
| `-ar` | A | ll Random. All players are given a random race. |
| `-sd` | S | ingle Draft. All players are given 2 random races to choose from. |
| `-hp` | H | ost Pick. Pick pick one race, everyone are given same race to host. |
| `-ph` | P | rophet Handpicked: Prophet with manual & incremental (up to 6) cost rerolls. |
| `-pr` | P | rophet Random: You have no re-rolls, every round your units are automatically re-rolled. Finishing waves rewards less gold. |

### Secondary Modes (combinable)

| Code | Name | Description |
|------|------|-------------|
| `-mm` | M | aster Mind. Restricted vision and limited scoreboard information. |
| `-mi` | M | irrored Rolls. Rolls are sync between counterpart players (Red with Yellow, Blue with Orange, ...) |
| `-ah` | A | uto Heals. Players no longer need to manually heal the king. |
| `-qg` | Q | uick Game. Removing levels 20 -> 30. |
| `-li` | L | imited Income. Pure lumber is disabled. |
| `-hg` | H | ourglass: You receive gold for enemy leaks. |
| `-gg` | G | ood Game: Enemies that reach king area give 50% gold when killed. |
| `-cb` | C | hange Builder: Enable change builder, player can change to difference build. |
| `-cc` | C | hallenge Champions: Champions can be manually challenged. |
| `-ac` | A | ll Champions: All waves above level 5 will spawn a Champion (except Boss Waves). |
| `-nc` | N | o Champions: There won't be any Champions in the game at all. |
| `-eq` | e | q spawn: 10 time more creep. |

**Ranked shortcut:** `-ranked` = `-pracahmiqg`

## Economy System

### Starting Resources

- **Gold:** 750
- **Lumber:** 150
- **Food Cap:** 250

### Gold Sources

| Source | Formula | Confidence |
|--------|---------|------------|
| King Kill Gold | gold = king_kill_count * 100 (capped) | 90% |
| Level Completion Gold | OO[level], doubled at boss waves (10, 20, 30) | 90% |
| Income per Level | BI[player_id] gold per level completion | 90% |
| Slave Trade Gold | EE gold from Slave Trade | 70% |

---


## 2. Map Layout & Legions

From `Trig_Setup_Tower_Properties_Actions` the game has exactly **15 Legions** (`udg_numRaces = 15`), organized into 3 groups:
```
Group A (0–5):  Hybrid, Beast, Mech, Nature, Shadow, Element
Group B (6–11): Ghost, Demi-Human, Marine, Elf, Goblin, Arctic
Group C (12–15): Paladin, Prophet, Orc, Undead
```
Each Legion has its own Builder unit type (`udg_BuilderType[0..15]`).

**Random Draft (RD) mode combos:**
- RD1 = Group A + Group B
- RD2 = Group A + Group C
- RD3 = Group B + Group C

---

## 3. Wave System — Confirmed Data

### Level Progression

| Level | Boss | Arena | Notes |
|-------|------|-------|-------|
| 1 |  |  |  |
| 2 |  |  |  |
| 3 |  |  |  |
| 4 |  |  |  |
| 5 |  |  |  |
| 6 |  |  |  |
| 7 |  |  |  |
| 8 |  |  |  |
| 9 |  |  |  |
| 10 | Yes | Yes | Arena battle: 42s prep, 150s max duration |
| 11 |  |  |  |
| 12 |  |  |  |
| 13 |  |  |  |
| 14 |  |  |  |
| 15 |  |  |  |
| 16 |  |  |  |
| 17 |  |  |  |
| 18 |  |  |  |
| 19 |  |  |  |
| 20 | Yes | Yes | Arena battle: 42s prep, 150s max duration |
| 21 |  |  |  |
| 22 |  |  |  |
| 23 |  |  |  |
| 24 |  |  |  |
| 25 |  |  |  |
| 26 |  |  |  |
| 27 |  |  |  |
| 28 |  |  |  |
| 29 |  |  |  |
| 30 | Yes |  | Final boss wave |


**Max level:** `udg_MaxLevel_Integer = 30` + 1 special Arena wave (level 31).

### Wave Counts (actual creeps spawned per wave — x1 mode)
From `udg_LevelWaveCountList` (comma-separated, index = wave number):
```
Wave: 01  02  03  04  05  06  07  08  09  10
Count:36  45  40  36  36  36  30  36  45  03

Wave: 11  12  13  14  15  16  17  18  19  20
Count:54  45  45  26  36  45  35  45  36  03

Wave: 21  22  23  24  25  26  27  28  29  30  31
Count:36  48  36  35  45  36  36  18  30  03  15
```
Waves 10, 20, 30 have only **3 creeps** — these are **Boss waves**.

### Creep Bounty Per Kill (gold)
From `udg_LevelBountyList`:
```
W01:3  W02:3  W03:4  W04:5  W05:5  W06:5  W07:6  W08:6  W09:5  W10:51
W11:5  W12:6  W13:7  W14:12 W15:9  W16:8  W17:10 W18:8  W19:10 W20:86
W21:10 W22:9  W23:11 W24:11 W25:9  W26:12 W27:12 W28:23 W29:14 W30:123
W31:0
```
Boss waves (10, 20, 30) give 51, 86, 123 gold per kill.

### Creep Armor Type by Wave
From `udg_DEF*List`:
- **Unarmored**: waves 1, 2, 31
- **Light**: 5, 7, 10, 13, 16, 19, 21, 25
- **Medium**: 3, 8, 12, 14, 18, 24, 27
- **Heavy**: 4, 9, 15, 20, 23, 26, 29
- **Fortified**: 6, 11, 17, 22, 28, 30
- **Air units**: waves 5, 13, 21, 29
- **Boss waves**: 10, 20, 30

### Creep Attack Type by Wave
From `udg_ATK*List`:
- **Piercing**: 1, 4, 7, 12, 19, 21, 25
- **Normal**: 2, 3, 9, 14, 15, 23, 26, 27
- **Magic**: 5, 8, 13, 16, 18, 24, 29
- **Siege**: 6, 11, 17, 22, 28
- **Chaos**: 10, 20, 30, 31 ← Boss waves and Arena use Chaos

### Ranged Creeps (attack from range)
Waves 4, 8, 12, 16, 20, 24, 28, 29.

### Build Phase Timer
From `Trig_Setup_Creep_Properties_Actions`:
```
TimeToPrepare[wave] = 40 + (wave / 2)   // seconds
```
Wave 1 = 40s, Wave 2 = 41s, Wave 10 = 45s, Wave 30 = 55s.

---

## 4. Economy — Exact Formulas

### 4.1 End-Round Gold (wave completion bonus)
From `udg_Level_EndRoundGold` (extracted verbatim):
```
W01:11  W02:12  W03:13  W04:14  W05:16  W06:18  W07:20  W08:23  W09:26  W10:30
W11:35  W12:40  W13:45  W14:50  W15:55  W16:60  W17:70  W18:80  W19:90  W20:100
W21:110 W22:120 W23:130 W24:140 W25:150 W26:160 W27:170 W28:180 W29:190 W30:200
```

### 4.2 Income (PlayerIncome)
`udg_PlayerIncome[playerID]` is set each wave based on `FightersValue / RecommendValue` ratio. The income is distributed in `Trig_End_Round_Func029Func001A` along with the end-round gold.

### 4.3 Income Cap Formula (exact from source)
```jass
udg_Temp_Real = I2R(wave)
income_cap = (wave³ × 0.025) + (wave² × 0.05) + (wave × 4) + 20
```
In plain math:
```
income_cap[w] = 0.025w³ + 0.05w² + 4w + 20
```
Examples: Wave 1 = 24, Wave 10 = 69, Wave 20 = 200, Wave 30 = 830.

### 4.4 Recommended Fighter Value (gold worth of fighters to maintain income)
From `udg_RecommendValue[]` (exact per-wave values):
```
W01:250    W02:350    W03:500    W04:650    W05:800
W06:1000   W07:1200   W08:1450   W09:1600   W10:1850
W11:2050   W12:2400   W13:2700   W14:3100   W15:3500
W16:4000   W17:4500   W18:5000   W19:5500   W20:6000
W21:6500   W22:7100   W23:7700   W24:8500   W25:9500
W26:10600  W27:11800  W28:13000  W29:14000  W30:15000
```
This is the benchmark the game uses for score calculation and income efficiency.

### 4.5 Sell Percentage
From `Trig_Setup_Tower_Properties_Actions`:
```jass
udg_SellPercent = 0.50   // 50% refund
```
**Exception:** Fighters sold during build phase (not in-round) with special condition get 90% refund (`0.90`). During round: 50% of `GetUnitPointValue()`.

The sell logic also adjusts `PLAYER_STATE_GOLD_GATHERED` to track net spent gold correctly.

### 4.6 Computer Gold (shared pool for AI players)
Player(8) and Player(9) are the "computer" team gold pools. At end of round, their accumulated gold is split equally among living team players (`/ CountPlayersInForce`). Then it's zeroed: `SetPlayerStateBJ(Player(8/9), PLAYER_STATE_RESOURCE_GOLD, 0)`.

---

## 5. Fighter (Tower) Value System

From `Trig_Value_Fighters_Actions` and `Trig_Calculate_Score_Func001A`:

```
FightersValue[player]    = sum of GetUnitPointValue() for all placed fighters
FightersValueSummon[player] = sum of GetUnitPointValue() for all summoned units
```

**Score formula** (used for leaderboard, not income directly):
```
r1 = (FightersValue[id] / RecommendValue[wave]) × builderModifier
     where builderModifier = clamp(100 + (5 - timesChangedBuilder) × 10, 100, 150)

r2 = (PlayerIncome[id] / EndRoundGold[wave]) × 100

TotalScore[id] += r1 + r2
```

---

## 6. Attack Type vs Armor Type Matrix

From `Trig_Attack_Types_Actions` and `Trig_Armor_Types_Actions` (displayed to players as help):

| | Unarmored | Light | Medium | Heavy | Fortified |
|---|---|---|---|---|---|
| **Normal** | 1.00 | 1.00 | 1.00 | 0.70 | 0.70 |
| **Piercing** | 1.00 | 1.50 | 0.75 | 0.50 | 0.35 |
| **Magic** | 1.00 | 1.25 | 0.75 | 0.75 | 0.35 |
| **Siege** | 1.00 | 0.50 | 0.75 | 1.00 | 1.50 |
| **Chaos** | 1.00 | 1.00 | 1.00 | 1.00 | 1.00 |

> Chaos (boss waves 10, 20, 30, 31) ignores all armor reductions.

---

## 7. King System

- Two Kings: Left King (team A) and Right King (team B), both on neutral player slots.
- King HP is tracked via `udg_LKingLowHPCounter` / `udg_RKingLowHPCounter`.
- King has abilities granted each wave via `Trig_Give_LKing_Abilities` / `Trig_Give_RKing_Abilities`.
- King upgrades exist: purchased from a shop that unlocks abilities (HP, armor, damage boosts).
- The King **auto-upgrades** at certain wave thresholds (`Trig_King_AutoUpgrades`).
- King autocasts are real abilities (Blizzard, etc.) that fire during combat.
- Win/loss triggers: `Trig_Defeat_L` (left team dies) and `Trig_Defeat_R` (right team dies), checked when King HP = 0.


### King Upgrades

| ID | Name | Max Level | Gold Base | Gold/Level |
|-----|------|-----------|----------|------------|
| R000 | King Hit Points (1) | 75 | ? | ? |
| R001 | King Attack (1) | 30 | ? | ? |
| R002 | King Regeneration (1) | 35 | ? | ? |
| R006 | King Armor (1) | 35 | ? | ? |
| R007 | King Attack Speed (1) | 35 | ? | ? |
| R008 | King HP Modifier | 1 | ? | ? |

---

## 8. Mercenary System

From `Trig_Purchase_Summon_Conditions` and `Trig_Warp_Summons_Actions`:

- Mercenaries are bought during **build phase only** (spell is disabled in `Trig_Disable_Mercenary_spell` at round start).
- They are paid for with **lumber**.
- Mercs are stored in `udg_Summons_UnitGroup` until wave start.
- At wave start, `Trig_Warp_Summons` teleports them into the **opponent's lane** alongside the creep wave.
- Cross-lane sending (warp to mid): handled by `Trig_New_WarpGroup_Warp_Mids` — mercs can be split between opponent lanes.
- Mercs give **bounty gold** when killed (the defender earns it).
- "Hire Group" (`udg_Hire_Group`) also contributes to `FightersValueSummon`.

---

## 9. Arena Mode (Wave 31)

Triggered by `Trig_Level_31_Initiate_Actions` and `Trig_Spawn_Arena_Mode_Actions`:

- After wave 30, both teams have **30 seconds** to prepare (not the normal timer).
- All fighters are warped into a shared **Arena zone** (`Trig_Warp_Fighter_Arena`).
- East fighters vs West fighters fight directly.
- Winner determined by which side's fighters survive (`Trig_End_Arena_Battle_Handler`).
- The winning team gets gold bonuses distributed to all its players.
- A "Winner Walk" animation plays for surviving units.

---

## 10. Game Modes (from source)

From `Trig_Mode_*_Actions` triggers, confirmed modes:
- **AP** (All Pick) — default, each player picks any Legion
- **SD** (Single Draft) — each player gets 3 random Legions to pick from
- **AR** (All Random) — Legions assigned randomly
- **HP** (Hard Pick) — pick from a limited pool
- **MM** (Mind Match) — income hidden from enemy team
- **SM** (Solo Mode) — 1v1 variant
- **HG** (Hardcore Gold) — reduced income
- **GG** (Good Game) — all units revealed
- **NM** (Night Mode) — visibility changes
- **CB** (Challenge Build) — restrictive building rules
- **X3** (Triple spawn) — 3× the creep count per wave

---

## 11. Anti-Cheat Systems

The map includes explicit anti-cheat for both resources:
- `Trig_Anti_Cheat_LUMBER` — monitors lumber for impossible gains
- `Trig_Anti_Cheat_GOLD` — monitors gold thresholds per wave
- `Trig_Smart_Anti_Lumber_Cheat` — smarter lumber cheat detection

> **Note for implementation:** Any reimplementation must have **server-authoritative resource tracking**. Never trust client-reported gold/lumber values.

---

## 12. Creep Pathing System

Creeps follow **region-based waypoints**, not free pathfinding:
- `Trig_Creep_Pathing_1_2`, `_3_4`, `_5_6`, `_7_8` — per-lane path segments
- `Trig_Creep_Pathing_West_To_King` / `East_To_King` — final approach
- `Trig_PERIOD_Antistuck` — runs periodically to detect stuck creeps and teleport them forward through `udg_WarpGroup` waypoints

Creeps that enter the King zone (`gg_rct_LKing` / `gg_rct_RKing`) trigger `Trig_Guard_LKing` / `Trig_Guard_RKing` which command the King to attack them.

---

## 13. Key Global Variables (Implementation Reference)

```typescript
// From war3map.j globals block
const MAX_WAVE = 30;              // udg_MaxLevel_Integer
const NUM_LEGIONS = 15;           // udg_numRaces
const SELL_PERCENT = 0.50;        // udg_SellPercent (50% refund)
const SELL_PERCENT_PREBATTLE = 0.90; // 90% if sold before round starts

// Per-wave arrays (index 1..30)
wave_count[w]        // creep spawn count
wave_bounty[w]       // gold per kill
wave_end_gold[w]     // completion bonus
wave_timer[w]        // build phase seconds = 40 + w/2
wave_income_cap[w]   // 0.025w³ + 0.05w² + 4w + 20
recommend_value[w]   // target fighter gold value

// Per-player state
fighters_value[p]       // sum of placed fighter point values
fighters_value_summon[p]// sum of summoned unit values
player_income[p]        // income per round
player_kills[p]         // total creep kills
total_score[p]          // cumulative score
```

---

## 14. Source Files in the ZIP

| File | Contents |
|---|---|
| `war3map.j` | 24,355 lines of JASS — **all game logic** |
| `war3map.w3u` | Unit data (fighters, creeps, builders) — binary format |
| `war3map.w3a` | Ability data — binary format |
| `war3map.wts` | String table (all UI text, TRIGSTR_#### references) |
| `war3map.wtg` | Trigger GUI data (mirrors .j for WE display) |
| `war3map.w3i` | Map info (name, author, players) |
| `war3map.w3e` | Terrain data |
| `war3map.doo` / `war3mapUnits.doo` | Doodads and unit placement |
| `King.mdx` | 3D model for the King unit |
| `GoblinAssaultTank.mdx` etc. | 3D models for Goblin legion fighters |

---

## 15. Collision & Pathfinding Requirements (Custom Overrides)

Unlike standard WC3 region-based straight-line walking, this implementation enforces **active collision**:
1. **Creep Collision:** Creeps collide with one another rather than stacking perfectly on top of each other. This causes swarming and requires flocking/avoidance behaviors (like boids or physics colliders).
2. **Tower Collision:** Player-built fighters act as physical obstacles. Creeps must path around them dynamically. If a player builds a wall of towers, the creeps must navigate around the wall to reach the King. 
*(Note: Care must be taken to prevent "mazing" exploits where creeps are completely blocked from reaching the King, typically solved by preventing placement if it completely blocks the path, or making towers attackable by creeps if blocked).*

---

## 16. Gameplay Visual & Pacing Observations (From Video Sample)

A review of classic WC3 Legion TD gameplay reveals crucial pacing and UI requirements for our Bevy implementation:

**1. The Wave Spawning & Swarm Mechanics:**
- Creeps spawn in thick, semi-organized clusters. They exhibit minor avoidance behavior (boids) to prevent perfectly overlapping. 
- When fighters engage the wave, creeps wrap around the frontline units. This creates a distinct "combat frontline" rather than 1-on-1 duels, emphasizing the importance of tanks holding the line so ranged fighters can fire from the back.

**2. Player UI & Interaction:**
- The player acts primarily as an overseer (a builder unit in WC3, but mechanically a disembodied cursor). 
- Players are actively researching upgrades at their base structure (like the "Town" building where Lumberjack training and King upgrades are spammed). This needs an easy-to-use HUD in Bevy to prevent interrupting lane management.
- There is a noticeable "Action Speed" — combat happens fast. A wave of 36 creeps engages 10-15 fighters and resolves within 15-20 seconds. The Bevy simulation must support very rapid health decrementing, visual feedback (floating text/blood effects), and fast-traveling projectiles.

**3. Visual Feedback Cues:**
- **Floating Gold:** Every single kill pops up yellow gold text (e.g., `+5`). This immediate micro-reward loop is visually essential.
- **Aura & Spell Indicators:** Visual halos exist around buffed units (like those affected by "Blessing of Encouragement"). The ECS needs to handle attaching particle emitters to units who possess active `.aura` or `.buff` components.
- **Leaking Consequence:** When creeps bypass the player's fighters, the HUD clearly alerts players who "leaked." These surviving creeps march directly to the central King area, and the King physically engages them in combat. The King is not just a health bar; he is an active boss unit.