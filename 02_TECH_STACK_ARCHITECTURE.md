# Tech Stack, Architecture & Deployment Recommendation
> **Goal:** A standalone, browser-playable, multiplayer Legion TD clone with 3D aesthetics inspired by *The King is Watching*. This doc gives the recommended language, engine, backend, and infrastructure.

---

## 1. TL;DR — Recommended Stack

| Layer | Choice | Rationale |
|---|---|---|
| **Game Client** | **Godot 4 (GDScript / C#)** | Free, open-source, native 3D, exports to Web (WASM), perfect for indie |
| **Networking Protocol** | **WebSocket + custom binary protocol** | Browser-compatible, low latency for real-time |
| **Game Server** | **Go (Golang)** | High concurrency, tiny binaries, fast tick loops |
| **API / REST** | **Node.js + Express** or **FastAPI (Python)** | Auth, matchmaking, leaderboards |
| **Database** | **PostgreSQL** | Relational — fits game state, player stats, legion data |
| **Cache / Pub-Sub** | **Redis** | Phase timers, lobby state, pub-sub for game events |
| **Containerization** | **Docker** | One image per service |
| **Orchestration** | **Kubernetes (K8s)** | Scale game servers horizontally |
| **CDN / Static Assets** | **Cloudflare** or **AWS CloudFront** | Serve Godot WASM client, 3D models |
| **CI/CD** | **GitHub Actions** | Auto-build Docker images, deploy to K8s |

---

## 2. Client — Why Godot 4

**Godot 4** is the ideal choice for this project because:
- Exports to **WebAssembly** (WASM) — players open a browser tab, no install needed.
- Full **3D pipeline**: PBR materials, lighting, animation trees, particle systems — all needed for *The King is Watching* aesthetic.
- **GDScript** is Python-like, fast to prototype. Can mix **C#** for performance-critical systems (combat resolution).
- Built-in **multiplayer API** (but we recommend using raw WebSockets for a dedicated server model instead of Godot's peer-to-peer).
- Large asset import support: GLTF/GLB for 3D models, Blender pipeline is seamless.
- 100% free, MIT-licensed.

**Alternative considered:** Unity — rejected because of licensing issues and WebGL performance overhead. Unreal — rejected because it's too heavy for 2D-ish tower defense and WASM export is unofficial.

---

## 3. Game Server — Why Go

The game server is the **authoritative simulation** — it runs the actual combat loop, validates player actions, and broadcasts game state.

Go is chosen because:
- **Goroutines** make it trivial to run hundreds of simultaneous game rooms concurrently.
- **Low GC pauses** — important for a 30–60 tick/s combat loop.
- Compiles to a **tiny static binary** — ideal for Docker containers.
- Strong standard library for WebSocket (`gorilla/websocket` or stdlib `net/http`).
- Excellent tooling for profiling concurrent systems.

### Game Server Responsibilities
```
- Receive player actions (build fighter, sell fighter, send mercenary)
- Validate actions against game state (enough gold? in-lane position? valid phase?)
- Run combat simulation at fixed tick rate (e.g., 20 ticks/second)
- Broadcast authoritative game state deltas to all clients in the room
- Manage phase transitions (build → combat → income) with timers
- Persist match result to DB at game end
```

---

## 4. API Server — Why Node.js or FastAPI

A separate **REST API** handles everything non-real-time:

```
POST /auth/register
POST /auth/login
GET  /matchmaking/join
POST /matchmaking/create
GET  /players/:id/stats
GET  /legions
GET  /legions/:id/fighters
GET  /leaderboard
```

**Node.js + Express** is recommended if the team is more JS-native.
**FastAPI (Python)** is recommended if AI agents (Gemini) will also be generating endpoints — Python is the lingua franca of AI tooling.

---

## 5. Database Schema (PostgreSQL)

```sql
-- Players
CREATE TABLE players (
  id UUID PRIMARY KEY,
  username VARCHAR(32) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password_hash TEXT NOT NULL,
  elo INTEGER DEFAULT 1000,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Matches
CREATE TABLE matches (
  id UUID PRIMARY KEY,
  mode VARCHAR(16),          -- '1v1', '2v2', '4v4'
  started_at TIMESTAMP,
  ended_at TIMESTAMP,
  winner_team INTEGER
);

-- Match Players
CREATE TABLE match_players (
  match_id UUID REFERENCES matches(id),
  player_id UUID REFERENCES players(id),
  team INTEGER,
  legion VARCHAR(64),
  waves_survived INTEGER,
  gold_earned INTEGER,
  PRIMARY KEY (match_id, player_id)
);

-- Legions (static data, loaded at boot)
CREATE TABLE legions (
  id VARCHAR(64) PRIMARY KEY,
  name VARCHAR(128),
  description TEXT,
  theme VARCHAR(128)
);

-- Fighters (static data)
CREATE TABLE fighters (
  id VARCHAR(64) PRIMARY KEY,
  legion_id VARCHAR(64) REFERENCES legions(id),
  name VARCHAR(128),
  tier INTEGER,
  cost INTEGER,
  supply INTEGER,
  hp INTEGER,
  armor FLOAT,
  armor_type VARCHAR(32),
  damage_min INTEGER,
  damage_max INTEGER,
  attack_speed FLOAT,
  attack_range INTEGER,
  attack_type VARCHAR(32),
  abilities JSONB,
  upgrades_from VARCHAR(64),
  upgrades_to JSONB
);
```

---

## 6. Redis Usage

| Key Pattern | Purpose |
|---|---|
| `lobby:{id}` | Lobby state (players, ready status, legion choices) |
| `game:{id}:phase` | Current phase + time remaining |
| `game:{id}:state` | Serialized game state snapshot (for reconnect) |
| `game:{id}:actions` | Queue of unprocessed player actions |
| `leaderboard` | Sorted set by ELO |

---

## 7. Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                       BROWSER                           │
│              Godot 4 → WASM Client                      │
│         (Renders 3D, handles input, plays SFX)          │
└────────────────────┬────────────────────────────────────┘
                     │ WebSocket (game) + HTTPS (auth/api)
          ┌──────────▼────────────────────┐
          │    Kubernetes Ingress / LB     │
          └──────┬──────────────┬─────────┘
                 │              │
    ┌────────────▼───┐  ┌───────▼────────────┐
    │  Game Server   │  │   API Server        │
    │  (Go)          │  │   (Node/FastAPI)    │
    │  - Combat loop │  │   - Auth (JWT)      │
    │  - Phase mgr   │  │   - Matchmaking     │
    │  - WS hub      │  │   - Stats/Leaderb.  │
    └────────┬───────┘  └───────┬────────────┘
             │                  │
    ┌────────▼──────────────────▼────────────┐
    │              Redis                      │
    │  (Lobby state, phase timers, pub-sub)  │
    └────────────────────┬───────────────────┘
                         │
    ┌────────────────────▼───────────────────┐
    │            PostgreSQL                   │
    │  (Players, matches, legions, fighters) │
    └────────────────────────────────────────┘
```

---

## 8. Kubernetes Setup

### Namespaces
```yaml
namespaces:
  - legion-td-prod
  - legion-td-staging
```

### Deployments
```yaml
# Game Server — scales horizontally per player load
Deployment: game-server
  replicas: 2–10 (HPA based on active connections)
  image: ghcr.io/your-org/ltd-game-server:latest
  resources:
    requests: { cpu: "250m", memory: "128Mi" }
    limits:   { cpu: "1", memory: "512Mi" }

# API Server
Deployment: api-server
  replicas: 2
  image: ghcr.io/your-org/ltd-api:latest

# Redis
StatefulSet: redis
  replicas: 1 (or 3 for HA with Redis Sentinel)

# PostgreSQL
StatefulSet: postgres
  replicas: 1 (use managed DB in prod: AWS RDS / Supabase)
```

### Key K8s Configs
- **HPA** (Horizontal Pod Autoscaler) on game-server: scale on CPU and connection count.
- **PodDisruptionBudget**: ensure at least 1 game-server pod stays alive during rolling updates.
- **PersistentVolumes**: for PostgreSQL and Redis data.
- **Ingress** (nginx): route `/ws` to game-server, `/api` to api-server, `/` to CDN static files.

---

## 9. Docker Files

### Game Server (Go)
```dockerfile
FROM golang:1.22-alpine AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN go build -o game-server ./cmd/server

FROM alpine:3.19
COPY --from=builder /app/game-server /game-server
EXPOSE 8080
CMD ["/game-server"]
```

### API Server (Node.js)
```dockerfile
FROM node:20-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production
COPY . .
EXPOSE 3000
CMD ["node", "src/index.js"]
```

---

## 10. Recommended Dev Environment

```bash
# Local stack via Docker Compose (for development)
docker compose up
# Services:
#   - game-server:8080
#   - api-server:3000
#   - postgres:5432
#   - redis:6379
#   - godot-web-export served via nginx:8000
```

Use **Godot Editor** locally for client development. Export to Web and test against local Docker services.

---

## 11. Deployment Platforms (Options)

| Option | Cost | Complexity | Notes |
|---|---|---|---|
| **AWS EKS** | $$$  | High | Production-grade, scales to millions |
| **GCP GKE** | $$$  | High | Excellent for Gemini AI integration |
| **Hetzner Cloud + K3s** | $ | Medium | Best cost/performance for indie |
| **Render.com** | $$ | Low | Great for MVP, limited K8s control |
| **Railway.app** | $$ | Very Low | Fastest to deploy, for early prototypes |

**Recommendation for this project**: Start with **Railway or Render** for MVP, migrate to **Hetzner + K3s** (lightweight Kubernetes) when you have real player load. This saves ~70% cost vs AWS/GCP at indie scale.
