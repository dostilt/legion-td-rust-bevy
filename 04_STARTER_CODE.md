# Starter Code — First Steps
> This document contains the first concrete code to write. Each snippet is self-contained and runnable. Start here before touching anything else.

---

## Step 1 — Game Data JSON (Extract from JASS)

Run this Python script first. It creates the canonical data files every other system depends on.

```python
# scripts/extract_waves.py
# Run: python3 scripts/extract_waves.py > data/waves.json

import json

# Exact values from war3map.j — DO NOT CHANGE
WAVE_COUNT     = [0,36,45,40,36,36,36,30,36,45,3,54,45,45,26,36,45,35,45,36,3,36,48,36,35,45,36,36,18,30,3,15]
WAVE_BOUNTY    = [0,3,3,4,5,5,5,6,6,5,51,5,6,7,12,9,8,10,8,10,86,10,9,11,11,9,12,12,23,14,123,0]
WAVE_END_GOLD  = [0,11,12,13,14,16,18,20,23,26,30,35,40,45,50,55,60,70,80,90,100,110,120,130,140,150,160,170,180,190,200]
WAVE_ARMOR     = [0,"unarmored","unarmored","medium","heavy","light","fortified","light","medium","heavy","fortified",
                  "fortified","medium","light","medium","heavy","light","fortified","medium","light","heavy",
                  "light","medium","light","heavy","light","medium","heavy","fortified","heavy","fortified","unarmored"]
WAVE_ATTACK    = [0,"piercing","normal","normal","piercing","magic","siege","piercing","magic","normal","chaos",
                  "siege","piercing","magic","normal","normal","magic","siege","magic","piercing","chaos",
                  "piercing","normal","normal","siege","piercing","normal","normal","siege","normal","chaos","chaos"]
WAVE_IS_AIR    = {5,13,21,29}
WAVE_IS_BOSS   = {10,20,30}
WAVE_IS_RANGED = {4,8,12,16,20,24,28,29}

waves = []
for w in range(1, 32):
    waves.append({
        "wave": w,
        "count": WAVE_COUNT[w],
        "bounty_per_kill": WAVE_BOUNTY[w],
        "end_round_gold": WAVE_END_GOLD[w] if w <= 30 else 0,
        "armor_type": WAVE_ARMOR[w],
        "attack_type": WAVE_ATTACK[w],
        "is_air": w in WAVE_IS_AIR,
        "is_boss": w in WAVE_IS_BOSS,
        "is_ranged": w in WAVE_IS_RANGED,
        "build_timer_seconds": 40 + w // 2,
        "income_cap": round(0.025 * w**3 + 0.05 * w**2 + 4 * w + 20),
    })

print(json.dumps({"waves": waves}, indent=2))
```

---

## Step 2 — Go Game Server Skeleton

```go
// server/cmd/main.go
package main

import (
    "log"
    "net/http"
    "github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
    CheckOrigin: func(r *http.Request) bool { return true },
}

func main() {
    http.HandleFunc("/health", func(w http.ResponseWriter, r *http.Request) {
        w.WriteHeader(200)
        w.Write([]byte("ok"))
    })
    http.HandleFunc("/ws", handleWS)
    log.Println("Game server listening on :8080")
    log.Fatal(http.ListenAndServe(":8080", nil))
}

func handleWS(w http.ResponseWriter, r *http.Request) {
    conn, err := upgrader.Upgrade(w, r, nil)
    if err != nil { return }
    defer conn.Close()
    for {
        _, msg, err := conn.ReadMessage()
        if err != nil { break }
        log.Printf("recv: %s", msg)
        conn.WriteMessage(websocket.TextMessage, []byte(`{"type":"ack"}`))
    }
}
```

```go
// server/internal/game/state.go
package game

type Phase string
const (
    PhaseBuild  Phase = "build"
    PhaseCombat Phase = "combat"
    PhaseIncome Phase = "income"
)

type ArmorType string
const (
    ArmorUnarmored  ArmorType = "unarmored"
    ArmorLight      ArmorType = "light"
    ArmorMedium     ArmorType = "medium"
    ArmorHeavy      ArmorType = "heavy"
    ArmorFortified  ArmorType = "fortified"
)

type AttackType string
const (
    AttackNormal   AttackType = "normal"
    AttackPiercing AttackType = "piercing"
    AttackMagic    AttackType = "magic"
    AttackSiege    AttackType = "siege"
    AttackChaos    AttackType = "chaos"
)

// DamageMultiplier returns the multiplier from attack type vs armor type.
// Source: war3map.j Trig_Attack_Types_Actions / Trig_Armor_Types_Actions
var DamageMultiplier = map[AttackType]map[ArmorType]float64{
    AttackNormal:   {ArmorUnarmored: 1.00, ArmorLight: 1.00, ArmorMedium: 1.00, ArmorHeavy: 0.70, ArmorFortified: 0.70},
    AttackPiercing: {ArmorUnarmored: 1.00, ArmorLight: 1.50, ArmorMedium: 0.75, ArmorHeavy: 0.50, ArmorFortified: 0.35},
    AttackMagic:    {ArmorUnarmored: 1.00, ArmorLight: 1.25, ArmorMedium: 0.75, ArmorHeavy: 0.75, ArmorFortified: 0.35},
    AttackSiege:    {ArmorUnarmored: 1.00, ArmorLight: 0.50, ArmorMedium: 0.75, ArmorHeavy: 1.00, ArmorFortified: 1.50},
    AttackChaos:    {ArmorUnarmored: 1.00, ArmorLight: 1.00, ArmorMedium: 1.00, ArmorHeavy: 1.00, ArmorFortified: 1.00},
}

func CalcDamage(base float64, atk AttackType, def ArmorType) float64 {
    mult, ok := DamageMultiplier[atk][def]
    if !ok { return base }
    return base * mult
}

// IncomeCap returns the max income a player can earn at a given wave.
// Source: war3map.j Trig_Setup_Creep_Properties_Actions
func IncomeCap(wave int) int {
    w := float64(wave)
    return int(0.025*w*w*w + 0.05*w*w + 4*w + 20)
}

// SellRefund returns gold refunded when selling a fighter.
// Source: war3map.j Trig_Sell_Actions — udg_SellPercent = 0.50, prebattle = 0.90
func SellRefund(pointValue int, inRound bool) int {
    if !inRound {
        return int(float64(pointValue) * 0.90)
    }
    return int(float64(pointValue) * 0.50)
}

type Vec2 struct{ X, Y float64 }

type Fighter struct {
    ID          string
    UnitID      string     // from fighters.json
    OwnerID     string
    Pos         Vec2
    HP, MaxHP   float64
    Armor       float64
    ArmorType   ArmorType
    DamageMin   float64
    DamageMax   float64
    AttackSpeed float64    // seconds between attacks
    AttackRange float64
    AttackType  AttackType
    PointValue  int        // used for sell refund + income calc
    AttackTimer float64    // countdown to next attack
}

type Creep struct {
    ID         string
    HP, MaxHP  float64
    Armor      float64
    ArmorType  ArmorType
    AttackType AttackType
    Speed      float64
    Bounty     int
    PathIndex  int         // current waypoint index
    Pos        Vec2
}

type Player struct {
    ID      string
    TeamID  int
    Legion  string
    Gold    int
    Lumber  int
    Fighters []*Fighter
}

type Team struct {
    ID      int
    KingHP  int
    Players []*Player
}

type GameState struct {
    ID      string
    Wave    int
    Phase   Phase
    Teams   [2]*Team
    Creeps  []*Creep
    Tick    int64
}
```

---

## Step 3 — Combat Tick Loop

```go
// server/internal/game/combat.go
package game

import (
    "math"
    "math/rand"
    "time"
)

const TickRate = 20 // ticks per second
const TickDuration = time.Second / TickRate

func (g *GameState) RunCombatTick(dt float64) {
    g.tickFighters(dt)
    g.tickCreeps(dt)
    g.checkWaveEnd()
}

func (g *GameState) tickFighters(dt float64) {
    for _, team := range g.Teams {
        for _, p := range team.Players {
            for _, f := range p.Fighters {
                f.AttackTimer -= dt
                if f.AttackTimer > 0 { continue }
                target := g.nearestCreepInRange(f, team.ID)
                if target == nil { continue }
                f.AttackTimer = f.AttackSpeed
                dmgBase := f.DamageMin + rand.Float64()*(f.DamageMax-f.DamageMin)
                dmg := CalcDamage(dmgBase, f.AttackType, target.ArmorType)
                target.HP -= dmg
                if target.HP <= 0 {
                    g.onCreepDeath(target, p)
                }
            }
        }
    }
}

func (g *GameState) tickCreeps(dt float64, waypoints []Vec2) {
    alive := g.Creeps[:0]
    for _, c := range g.Creeps {
        if c.HP <= 0 { continue }
        if c.PathIndex >= len(waypoints) {
            // Reached King
            g.onCreepLeak(c)
            continue
        }
        target := waypoints[c.PathIndex]
        dx := target.X - c.Pos.X
        dy := target.Y - c.Pos.Y
        dist := math.Sqrt(dx*dx + dy*dy)
        if dist < 10 {
            c.PathIndex++
        } else {
            c.Pos.X += (dx / dist) * c.Speed * dt
            c.Pos.Y += (dy / dist) * c.Speed * dt
        }
        alive = append(alive, c)
    }
    g.Creeps = alive
}

func (g *GameState) onCreepDeath(c *Creep, killer *Player) {
    killer.Gold += c.Bounty
}

func (g *GameState) onCreepLeak(c *Creep) {
    // Find which team this lane belongs to and decrement king HP
    // For 1v1: lane side maps to team index
}

func (g *GameState) nearestCreepInRange(f *Fighter, teamID int) *Creep {
    var nearest *Creep
    nearestDist := math.MaxFloat64
    for _, c := range g.Creeps {
        dx := c.Pos.X - f.Pos.X
        dy := c.Pos.Y - f.Pos.Y
        dist := math.Sqrt(dx*dx + dy*dy)
        if dist <= f.AttackRange && dist < nearestDist {
            nearest = c
            nearestDist = dist
        }
    }
    return nearest
}

func (g *GameState) checkWaveEnd() {
    if len(g.Creeps) == 0 {
        g.Phase = PhaseIncome
        g.distributeIncome()
    }
}

func (g *GameState) distributeIncome() {
    // Stub: award end-round gold + player income
    // See 01_GAME_MECHANICS.md §4 for exact formula
}
```

---

## Step 4 — Godot 4 WebSocket Client

```gdscript
# client/scripts/GameClient.gd
extends Node

signal game_state_received(state: Dictionary)
signal phase_changed(phase: String, timer: float)
signal game_over(winner_team: int)

var socket := WebSocketPeer.new()
var connected := false

func connect_to_server(url: String) -> void:
    socket.connect_to_url(url)

func _process(_delta: float) -> void:
    socket.poll()
    match socket.get_ready_state():
        WebSocketPeer.STATE_OPEN:
            if not connected:
                connected = true
                print("Connected to game server")
            while socket.get_available_packet_count() > 0:
                _handle_packet(socket.get_packet())
        WebSocketPeer.STATE_CLOSED:
            if connected:
                connected = false
                print("Disconnected")

func _handle_packet(data: PackedByteArray) -> void:
    var msg = JSON.parse_string(data.get_string_from_utf8())
    if msg == null: return
    match msg.get("type", ""):
        "game_state":   emit_signal("game_state_received", msg.payload)
        "phase_change": emit_signal("phase_changed", msg.payload.phase, msg.payload.timer)
        "game_over":    emit_signal("game_over", msg.payload.winner_team)

func send_place_fighter(fighter_id: String, x: float, y: float) -> void:
    _send({ "type": "place_fighter", "payload": { "fighter_id": fighter_id, "x": x, "y": y } })

func send_sell_fighter(unit_id: String) -> void:
    _send({ "type": "sell_fighter", "payload": { "unit_id": unit_id } })

func send_mercenary(merc_id: String, target_lane: int) -> void:
    _send({ "type": "send_mercenary", "payload": { "merc_id": merc_id, "target_lane": target_lane } })

func _send(msg: Dictionary) -> void:
    socket.send_text(JSON.stringify(msg))
```

```gdscript
# client/scripts/LaneRenderer.gd
extends Node3D

@export var fighter_scene: PackedScene
@export var creep_scene: PackedScene

var placed_fighters := {}   # unit_id -> Node3D
var active_creeps := {}     # unit_id -> Node3D

func apply_game_state(state: Dictionary) -> void:
    _sync_fighters(state.get("fighters", []))
    _sync_creeps(state.get("creeps", []))

func _sync_fighters(fighters: Array) -> void:
    var seen := {}
    for f in fighters:
        seen[f.id] = true
        if not placed_fighters.has(f.id):
            var node = fighter_scene.instantiate()
            add_child(node)
            placed_fighters[f.id] = node
        placed_fighters[f.id].position = Vector3(f.x, 0, f.y)
    for id in placed_fighters.keys():
        if not seen.has(id):
            placed_fighters[id].queue_free()
            placed_fighters.erase(id)

func _sync_creeps(creeps: Array) -> void:
    var seen := {}
    for c in creeps:
        seen[c.id] = true
        if not active_creeps.has(c.id):
            var node = creep_scene.instantiate()
            add_child(node)
            active_creeps[c.id] = node
        active_creeps[c.id].position = Vector3(c.x, 0, c.y)
    for id in active_creeps.keys():
        if not seen.has(id):
            active_creeps[id].queue_free()
            active_creeps.erase(id)
```

---

## Step 5 — Docker Compose

```yaml
# docker-compose.yml
version: "3.9"
services:
  game-server:
    build: ./server
    ports: ["8080:8080"]
    environment:
      REDIS_URL: redis:6379
      DB_URL: postgres://ltd:ltd@postgres:5432/ltd
    depends_on: [redis, postgres]

  api:
    build: ./api
    ports: ["3000:3000"]
    environment:
      DB_URL: postgres://ltd:ltd@postgres:5432/ltd
      JWT_SECRET: change_me_in_prod
    depends_on: [postgres]

  client:
    image: nginx:alpine
    ports: ["8000:80"]
    volumes:
      - ./client/export/web:/usr/share/nginx/html:ro

  postgres:
    image: postgres:16-alpine
    environment:
      POSTGRES_USER: ltd
      POSTGRES_PASSWORD: ltd
      POSTGRES_DB: ltd
    volumes:
      - pgdata:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    volumes:
      - redisdata:/data

volumes:
  pgdata:
  redisdata:
```

---

## Step 6 — Run Order for Agents

When executing this project from scratch, always follow this order:

1. `python3 scripts/extract_waves.py > data/waves.json`
2. `docker compose up postgres redis` — wait for healthy
3. `cd api && npm install && npm run migrate` — create DB schema
4. `docker compose up` — bring full stack up
5. Open `http://localhost:8000` — verify client loads
6. `cd server && go test ./...` — verify all unit tests pass
7. Begin implementing Phase 1 tasks from `03_DEVELOPMENT_ROADMAP.md`
