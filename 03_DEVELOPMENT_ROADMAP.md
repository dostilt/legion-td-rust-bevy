# Development Roadmap — Legion TD Remake
> Step-by-step planning from zero to a playable online game. Each phase has clear deliverables, estimated duration, and dependencies. This is designed to be consumed by Gemini/Antigravity agents as a task queue.

---

## Summary Timeline

```
Phase 0: Foundation          (2 weeks)  — Repo, tooling, local dev stack
Phase 1: Core Simulation     (4 weeks)  — Game server: waves, fighters, income
Phase 2: Client Prototype    (3 weeks)  — Godot 4 basic rendering + lane
Phase 3: Multiplayer         (4 weeks)  — WebSocket sync, 1v1 playable
Phase 4: Content Pass        (3 weeks)  — All 30 waves, 15 legions, mercs
Phase 5: Art & Polish        (4 weeks)  — 3D models, VFX, UI, audio
Phase 6: Online Launch       (2 weeks)  — Auth, matchmaking, deploy to cloud
Phase 7: Live Ops            (ongoing)  — Balance, new content, tournaments
```
**Total to online MVP: ~22 weeks**

---

## Phase 0 — Foundation (Weeks 1–2)

### Goal
Every team member (human or AI agent) can clone the repo, run `docker compose up`, and have the full stack running locally in under 5 minutes.

### Tasks

**0.1 — Repository Structure**
```
/
├── server/          # Go game server
├── api/             # Node.js REST API  
├── client/          # Godot 4 project
├── data/            # Game data JSONs (fighters, waves, legions)
├── infra/           # Docker, K8s manifests
├── docs/            # These .md files
└── docker-compose.yml
```

**0.2 — Docker Compose Stack**
Services: `game-server`, `api`, `postgres`, `redis`. All with health checks.

**0.3 — Data Pipeline**
Extract fighter/wave/mercenary data from `war3map.w3u` and `war3map.j` into structured JSON files under `/data`. This is the single source of truth for agents generating content.

```bash
# Run extraction script
python3 scripts/extract_w3u.py → data/fighters.json
python3 scripts/extract_waves.py → data/waves.json
python3 scripts/extract_mercs.py → data/mercenaries.json
```

**0.4 — CI/CD**
GitHub Actions: lint → test → build Docker image → push to registry on merge to main.

### Deliverables
- [ ] Monorepo with all services scaffolded
- [ ] `docker compose up` works end-to-end
- [ ] `/data/fighters.json`, `/data/waves.json`, `/data/mercenaries.json` populated
- [ ] CI pipeline green

---

## Phase 1 — Core Simulation (Weeks 3–6)

### Goal
A headless game server that can simulate a full 30-wave Legion TD game with correct mechanics, verifiable against the JASS source data.

### Tasks

**1.1 — Game State Model (Go)**
Implement all structs from `01_GAME_MECHANICS.md §12`:
```go
type GameState struct { Wave int; Phase Phase; Teams [2]Team }
type Player struct { Gold, Lumber int; Fighters []Fighter; Legion string }
type Team struct { KingHP int; Players []Player }
```

**1.2 — Wave Engine**
- Spawn `wave_count[w]` creeps at phase start
- Creep follows waypoints (hardcoded path per lane — 8 waypoints extracted from `war3map.j` region coords)
- Creep reaches King → decrement `Team.KingHP`
- All creeps dead → trigger `EndRound()`

**1.3 — Combat Resolution (tick loop)**
- 20 ticks/second fixed timestep
- Fighter auto-attacks nearest creep in range
- Apply attack type × armor type damage matrix (§6 of mechanics doc)
- Handle AoE splash, abilities, on-death effects

**1.4 — Income System**
Implement exactly:
```go
func EndRoundGold(wave int) int { return waveEndGoldTable[wave] }
func IncomeCap(wave int) float64 { return 0.025*pow(w,3) + 0.05*pow(w,2) + 4*w + 20 }
func SellRefund(unit Fighter, inRound bool) int {
    if !inRound { return int(float64(unit.PointValue) * 0.90) }
    return int(float64(unit.PointValue) * 0.50)
}
```

**1.5 — Unit Tests**
- Test income at waves 1, 10, 20, 30 against hardcoded expected values
- Test attack matrix for all 5×5 combos
- Test sell refund both cases
- Test wave_count array matches extracted data

**1.6 — Simulation CLI**
```bash
go run cmd/simulate/main.go --waves=30 --legion=nature --merc=kobold:3
# Output: wave-by-wave log of kills, leaks, gold earned
```

### Deliverables
- [ ] Game simulation passes all unit tests
- [ ] CLI can simulate full 30-wave game
- [ ] Wave data matches `war3map.j` constants exactly

---

## Phase 2 — Client Prototype (Weeks 7–9)

### Goal
Godot 4 client renders a single lane with placeholder 3D boxes as fighters/creeps, playable in browser (WASM export).

### Tasks

**2.1 — Godot Project Setup**
- Godot 4.x project, export target: Web (WASM)
- Directory structure:
  ```
  client/
  ├── scenes/
  │   ├── Lane.tscn       # Main game scene
  │   ├── Fighter.tscn    # Fighter node
  │   ├── Creep.tscn      # Creep node
  │   └── HUD.tscn        # Gold/lumber/timer UI
  ├── scripts/
  │   ├── GameClient.gd   # WebSocket connection
  │   ├── LaneRenderer.gd # Renders game state
  │   └── InputHandler.gd # Click-to-place fighters
  └── assets/             # Placeholder meshes
  ```

**2.2 — Lane Geometry**
- Rectangular lane (8×40 units in world space)
- 8 waypoint markers (invisible, used for creep path)
- Build zone (first half of lane) highlighted

**2.3 — HUD**
- Top bar: Wave number, timer countdown, Gold, Lumber
- Bottom: Fighter palette (clickable buttons to place)
- Multiboard-style scoreboard (placeholder data)

**2.4 — Local State Machine (offline mode)**
Play without server for iteration speed:
```gdscript
enum Phase { BUILD, COMBAT, INCOME }
var current_phase: Phase = Phase.BUILD
var wave: int = 1
var gold: int = 200
```

**2.5 — WASM Export**
- Export to `client/export/web/`
- Serve via nginx container in docker compose
- Accessible at `http://localhost:8000`

### Deliverables
- [ ] Godot project opens, WASM export works
- [ ] Can place placeholder fighters in lane
- [ ] Creeps walk path from spawn to King
- [ ] Build timer counts down
- [ ] HUD shows correct gold/lumber

---

## Phase 3 — Multiplayer (Weeks 10–13)

### Goal
Two players can play a real 1v1 game over WebSocket, fully synchronized.

### Tasks

**3.1 — WebSocket Protocol**
Define message types (JSON-encoded):
```typescript
// Client → Server
{ type: "place_fighter",   payload: { fighter_id: string; x: number; y: number } }
{ type: "sell_fighter",    payload: { unit_id: string } }
{ type: "send_mercenary",  payload: { merc_id: string; target_lane: number } }
{ type: "ready" }

// Server → Client
{ type: "game_state",      payload: GameStateSnapshot }
{ type: "phase_change",    payload: { phase: string; timer: number } }
{ type: "unit_died",       payload: { unit_id: string; killer_id: string } }
{ type: "gold_update",     payload: { amount: number; reason: string } }
{ type: "game_over",       payload: { winner_team: number } }
```

**3.2 — Lobby System**
- `POST /api/lobbies` — create lobby, returns `lobby_id`
- `GET /api/lobbies/:id` — poll lobby state
- WebSocket `ws://host/game/:lobby_id` — join game

**3.3 — Server Authority**
All game state lives on server. Client only sends actions. Server validates, simulates, and broadcasts `game_state` every tick (50ms). Client interpolates between states.

**3.4 — State Reconciliation**
Client predicts fighter placement locally for snappy feel. Server confirms or rejects. On rejection, client rolls back.

**3.5 — Integration Test**
Write a bot client in Go that:
1. Connects to server
2. Places 3 fighters per wave
3. Sends 1 merc per wave
4. Plays all 30 waves
5. Verifies final gold balance matches expected

### Deliverables
- [ ] Two browser tabs can play 1v1 to completion
- [ ] No desync over 30 waves (integration test passes)
- [ ] Server correctly enforces sell %, income cap, anti-cheat

---

## Phase 4 — Content Pass (Weeks 14–16)

### Goal
All 15 Legions, all 30 waves, all mercenaries implemented with real stats.

### Tasks

**4.1 — Fighter Data**
Parse `war3map.w3u` binary format to extract all fighter stats:
- HP, armor, damage, attack speed, range, attack type, armor type
- Gold cost, point value (sell value), supply cost
- Upgrade chains (from/to unit IDs)
- Abilities (mapped from `war3map.w3a`)

Export to `data/fighters.json`:
```json
{
  "legions": {
    "nature": {
      "name": "Nature",
      "builder": "u003",
      "fighters": [
        { "id": "h0XY", "name": "Treant", "tier": 1, "cost": 120, "point_value": 120,
          "hp": 350, "armor": 0, "armor_type": "light", "damage": [18,24],
          "attack_speed": 1.5, "attack_range": 100, "attack_type": "normal",
          "upgrades_to": ["h0XZ", "h0XW"] }
      ]
    }
  }
}
```

**4.2 — Mercenary Data**
From the purchase/warp triggers and `war3map.w3u`, extract all mercs:
```json
{
  "mercenaries": [
    { "id": "h0M1", "name": "Kobold", "lumber_cost": 3, "bounty": 2,
      "hp": 120, "armor_type": "unarmored", "speed": 270 }
  ]
}
```

**4.3 — Wave Creep Data**
Map the 31 unit type IDs from `udg_Level_UnitType[]` to actual creep stats via `war3map.w3u`.

**4.4 — Ability System**
Implement the ability types found in `Trig_Period_Ability_Actions`:
- Frost Aura (slow nearby enemies)
- Heal Aura (regen nearby allies)
- Splash damage (area attack)
- Mana Shield (absorb damage with mana)
- Berserk (attack speed boost on low HP)
- Raise Dead (spawn skeletons on kill)
- Triple Attack (3-hit burst)
- Mark Target (debuff enemy)
- Guardian Spirit (on-death revive)
- Biotoxin (poison DoT)
- Catastophe (AoE nuke ability)

### Deliverables
- [ ] All 15 Legions playable with real stats
- [ ] All 30 wave types spawn with correct creep types
- [ ] All merc units purchasable and functional
- [ ] Ability system handles all 11 ability archetypes

---

## Phase 5 — Art & Polish (Weeks 17–20)

> See `05_ART_DIRECTION.md` for full art specification. This phase implements it.

### Tasks

**5.1 — 3D Models**
- Commission/create Blender models for:
  - 1 Builder per faction (15 total) — stylized humanoid, isometric-friendly
  - 3–5 fighters per faction (45–75 total) — priority: tier 1 and 2
  - 10 creep types
  - 3 boss types
  - King model (can reuse `King.mdx` as reference)
- Export as GLB for Godot import

**5.2 — VFX**
- Godot GPUParticles for:
  - Projectile trails (arrow, magic bolt, cannon shell)
  - Death explosions (per armor type)
  - Gold coin pop on kill
  - Build/sell dust cloud
  - Aura rings for buff abilities

**5.3 — Audio**
- SFX: attack sounds (per attack type), death sounds, wave start horn, income chime, King death
- Music: 2 tracks — Build phase (ambient), Combat phase (tense)
- All sounds: royalty-free or original

**5.4 — UI Polish**
- Fighter card (hover tooltip): portrait, stats, ability description
- Upgrade arrow overlay on lane fighters
- Wave progress bar
- King HP bar (team-shared)
- Merc shop panel with lumber cost display

**5.5 — Map Environment**
- Stylized terrain (The King is Watching aesthetic: parchment/map look)
- Lane boundaries with wall/fence models
- King platform at lane end

### Deliverables
- [ ] All tier 1 fighters have custom 3D models
- [ ] All 10 core VFX implemented
- [ ] Audio system plays correct sounds for each event
- [ ] UI is polished and readable

---

## Phase 6 — Online Launch (Weeks 21–22)

### Tasks

**6.1 — Auth System**
- `POST /api/auth/register` (username, email, password)
- `POST /api/auth/login` → JWT token
- Token passed in WebSocket handshake header

**6.2 — Matchmaking**
Simple queue-based:
```
Player clicks "Find Match" → POST /api/matchmaking/join
Server pairs two players → creates Game room → notifies both via SSE
Players connect to WebSocket → game starts
```

**6.3 — Leaderboard**
- ELO rating system (±32 per match, scaled by opponent ELO delta)
- `GET /api/leaderboard?limit=100`
- Stored in PostgreSQL, cached in Redis sorted set

**6.4 — Deployment**
- Push Docker images to container registry
- Deploy to Hetzner Cloud (2× CX21 VMs for game servers, 1× for DB/Redis)
- Nginx reverse proxy with SSL (Let's Encrypt)
- Domain + CDN for static WASM client

**6.5 — Monitoring**
- Prometheus metrics from game server (active games, tick latency, player count)
- Grafana dashboard
- Uptime alerting (PagerDuty or Discord webhook)

### Deliverables
- [ ] Players can register and log in
- [ ] Matchmaking pairs players automatically
- [ ] ELO updates after each match
- [ ] Game accessible at public URL with SSL
- [ ] Monitoring dashboard live

---

## Phase 7 — Live Ops (Ongoing)

- Balance patches (tune fighter stats, wave bounty, income curve)
- New Legion releases (1 new Legion per month)
- Tournament system (bracket support, prize tracking)
- Replay system (using `Team-OZE/replays-wasm` as reference for format)
- Mobile-friendly UI improvements
- Ranked season system (seasonal ELO resets)
