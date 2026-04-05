# Art Direction — "The King is Watching" Aesthetic
> This document defines the complete visual identity for the Legion TD remake. All artists, AI image generators, and 3D modelers should treat this as the definitive style guide.

---

## 1. Reference: The King is Watching

*The King is Watching* is a 2D strategy/tower defense game with a distinct visual style characterized by:
- **Hand-drawn medieval manuscript / illuminated parchment** aesthetic
- **Flat, stylized 3D** with visible brushstroke-like textures
- Warm sepia/golden tones for UI, contrasted with vivid unit colors
- Chunky, readable silhouettes — units are immediately identifiable at small sizes
- A "living map" feeling — the world looks like it's drawn on an old battle map

---

## 2. Core Visual Pillars

### Pillar 1: The Living Parchment World
The game world looks like a **top-view military battle map drawn on aged parchment**. Terrain is drawn with ink outlines, watercolor fills. Lane boundaries look like fortification walls sketched by a cartographer.

### Pillar 2: Chunky Stylized 3D Units
Units are **3D but highly stylized** — exaggerated proportions, big heads, stubby limbs. Think *Clash of Clans* meets medieval manuscript illustration. No photorealism. High contrast, readable from above.

### Pillar 3: Warm + Saturated Color Palette
- Background/terrain: warm tan, aged paper, sepia (#C8A97E, #A07850, #8B6040)
- Lane zones: slightly cooler parchment (#B0C4A0 for grass, #C0A878 for dirt)
- UI chrome: deep burgundy, gold leaf, dark ink (#4A0E0E, #C8941A, #1A0F08)
- Fighter colors: vivid per-legion palette (see §5)
- Creeps: desaturated grey/green — they are the threat, not the heroes

### Pillar 4: Ink & Stamp UI
All UI elements look **stamped or hand-lettered** onto parchment. Buttons have ink-blot edges. Numbers are in a serif font styled after medieval numerals. Icons look like wax seals.

---

## 3. Camera & Perspective

- **Isometric 3D** — camera at 45° angle, ~30° elevation
- Fixed camera rotation (no free rotation, to preserve map readability)
- Zoom in/out allowed (mouse wheel)
- Lane runs vertically on screen (top = enemy spawn, bottom = King)

---

## 4. Terrain & Environment

### Lane
- Width: ~8 world units, visible as a dirt/stone path
- Edge decoration: wooden palisade walls, ink-drawn fortification lines
- Surface: worn cobblestone or dirt path with parchment-paper texture

### King Platform
- Elevated stone dais with a throne
- The King unit sits on it with a crown and scepter
- HP orbs or a heart counter displayed prominently above the King
- When HP drops: cracks appear on throne, King becomes visually distressed

### Background
- Beyond the lanes: aged parchment with faded map markings
- Stylized trees (flat painted look), compass rose in corner
- Cartographic latitude/longitude lines faintly visible

### Lighting
- Warm directional sunlight from upper-left
- Soft ambient — no harsh shadows
- Fighter auras cast subtle colored light pools on ground

---

## 5. Legion Color Identities

Each of the 15 Legions has a primary color used for:
- Builder unit clothing/banner
- Fighter unit accent color (trim, glow)
- UI highlight when player selects that legion

| Legion | Primary Color | Secondary | Feel |
|---|---|---|---|
| Nature | Forest Green #3A7A1A | Bark Brown #5C3A1E | Organic, ancient |
| Shadow | Deep Purple #3A0F5C | Midnight Blue #0F0F3A | Mysterious, dark |
| Mech | Steel Blue #4A6A8A | Copper #A05A2A | Industrial, steam |
| Goblin | Sickly Green #6A8A1A | Rust Orange #9A4A1A | Chaotic, scrappy |
| Element | Cyan #1A8A9A | Ember Red #9A2A1A | Elemental contrast |
| Beast | Amber #9A6A1A | Blood Red #7A1A1A | Primal, fierce |
| Ghost | Ice Blue #8AAABB | White Mist #DDEEFF | Ethereal, cold |
| Demi-Human | Warm Tan #8A6A3A | Royal Blue #1A3A8A | Noble, hybrid |
| Marine | Navy #1A2A6A | Brass #9A7A2A | Military, disciplined |
| Elf | Leaf Gold #8A9A1A | Silver #AAAAAA | Graceful, ancient |
| Arctic | Ice White #CCEEFF | Deep Blue #1A3A5A | Cold, sparse |
| Paladin | Holy Gold #C8941A | White #F0F0E0 | Sacred, radiant |
| Prophet | Bone White #DDD8C0 | Violet #5A1A8A | Mystical, cryptic |
| Orc | War Green #3A5A1A | Blood Red #7A1A1A | Brutal, warlike |
| Undead | Grave Gray #4A4A4A | Corpse Green #2A5A1A | Decaying, dark |

---

## 6. Unit Design Language

### Builder Hero
- **Shape**: Humanoid, 60% head, 40% body — very top-heavy, cartoon proportions
- **Size**: 1.5× fighter scale — clearly the unit you control
- **Details**: Legion-themed clothing, a glowing tool (hammer, staff, wrench per faction)
- **Animation**: Idle bob, build gesture (swing/cast toward placement point)

### Fighter Units (Towers)
- **Shape language by tier:**
  - Tier 1: Round/soft shapes — approachable, cheap feel
  - Tier 2: More angular, confident stance
  - Tier 3: Imposing, detailed, larger silhouette
  - Legendary: Unique, immediately recognizable, particle effects
- **All fighters:** Chunky base/pedestal they stand on (makes placement feel grounded)
- **Ranged fighters:** Always hold a visible weapon (bow, staff, cannon)
- **Melee fighters:** Weapon drawn, aggressive forward lean
- **Aura fighters:** Constant ambient glow in legion color

### Creeps
- **Consistent silhouette language per armor type:**
  - Unarmored: Thin, fast-looking, robe/cloak
  - Light: Leather-clad, agile
  - Medium: Chainmail, steady
  - Heavy: Plate armor, slow-moving, hulking
  - Fortified: Stone/metal shell, extremely blocky
- **All creeps:** Greyed-out color palette — they are threats, not protagonists
- **Boss creeps:** 3× normal scale, unique model, glowing eyes

### Mercenaries (sent units)
- Hybrid style: slightly brighter than creeps, have a faction banner on their back
- Show which team sent them (small team color flag)

---

## 7. VFX Style

### Attack Effects
- **Projectiles**: Leave an ink-trail. Arrow = black ink streak. Magic bolt = colored calligraphy stroke. Cannon ball = stone sphere with dust puff.
- **Hit effects**: Ink splatter (like a stamp impact), not sci-fi sparks
- **Melee hit**: Single bold stroke, like a brushstroke slash

### Death Effects
- Small units: Crumple and dissolve into ink droplets
- Large units: Break apart into fragments, brief dust cloud
- Boss death: Slow, dramatic — unit cracks, glows bright, then shatters

### Aura Effects
- Rings of calligraphic swirls in legion color
- Gentle pulse on the ground around the unit
- Subtle particle drift upward (ink motes, leaves, sparks depending on legion)

### Build/Sell
- Build: Unit rises from ground with a "stamp" animation (like a wax seal being pressed)
- Sell: Unit dissolves into gold coin burst, small ink blot left on ground

### Wave Start
- **Camera shakes slightly**
- **Ink-drawn arrow** sweeps across the screen from spawn zone to King
- Text overlay: "WAVE 7 — HEAVY ARMOR" in stylized medieval font

---

## 8. UI Design

### HUD Layout
```
┌─────────────────────────────────────────────────────┐
│  [WAVE 7]  ⏱ 00:38  ♦ 420 Gold  🪵 15 Lumber       │
│                                           [⚔ MERC]  │
├─────────────────────────────────────────────────────┤
│                                                     │
│                    [LANE VIEW]                      │
│                                                     │
├─────────────────────────────────────────────────────┤
│  [Fighter Palette — 6 tier-1 slots + 2 tier-2]     │
│  King HP: ♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥♥ (25 hearts)   │
└─────────────────────────────────────────────────────┘
```

### Fighter Tooltip Card
Looks like a torn playing card / medieval trading card:
```
┌──────────────────┐
│  [Unit Portrait] │  ← painted illustration style
│  TREANT          │  ← bold serif name
│  ──────────────  │
│  HP:    350      │
│  DMG:   18-24    │
│  ATK:   Normal   │
│  ARM:   Light    │
│  ──────────────  │
│  🌿 Root Aura    │  ← ability with icon
│  ──────────────  │
│  Cost: 120g      │
│  Sell: 60g       │
└──────────────────┘
```

### Scoreboard
Styled like a parchment ledger:
- Player names in left column (with legion icon)
- Fighter Value, Income, Kills, Mercs in columns
- Enemy team data visible but greyed out in MM mode

### Fonts
- **Headers/Wave Text**: A serif font inspired by medieval woodblock printing (e.g., "MedievalSharp" or "Cinzel")
- **Numbers/Stats**: Clean but slightly textured serif — readable at small sizes
- **Flavor text**: Italic serif, slightly faded

---

## 9. 3D Model Technical Specs (for Blender artists)

- **Export format**: GLB (GLTF Binary) for Godot 4 import
- **Poly budget per unit:**
  - Builder: 2,000–3,500 tris
  - Fighter Tier 1: 800–1,200 tris
  - Fighter Tier 2: 1,200–2,000 tris
  - Fighter Tier 3+: 2,000–3,500 tris
  - Creep: 500–900 tris
  - Boss: 2,000–4,000 tris
- **Texture**: Single 512×512 or 1024×1024 atlas per unit, PBR roughness/metallic
- **Rig**: Simple armature (4–8 bones) for idle/attack/death animations
- **Animations required per unit:**
  - `idle` (loop, 2s)
  - `attack` (1 shot, 0.5–0.8s)
  - `death` (1 shot, 1.2s)
  - `build` (Builder only, 1s)

---

## 10. Priority Asset List (Build in This Order)

### Immediate (Phase 5 start)
1. Terrain tile (parchment lane) — needed for every screenshot
2. King unit model — centerpiece of the game
3. 1 Builder hero (Nature legion) — needed for playtest videos
4. 5 Tier-1 fighters (Nature legion) — enough for a playable demo
5. 5 basic creep models (one per armor type)

### Short-term (Month 2)
6. 3 Boss creep models
7. 3 complete Legions (Goblin, Undead, Paladin) — visual diversity
8. Core VFX (projectiles, hit sparks, death)
9. UI chrome (buttons, card frames, header bar)

### Full content (Month 3–4)
10. Remaining 12 Legions
11. All mercenary units
12. Arena zone environment
13. King ability VFX (lightning bolt, blizzard, etc.)
14. Full audio set

---

## 11. Mood Board Keywords (for AI Image Generation)

When prompting image generation tools for concept art, use combinations of:

```
"medieval illuminated manuscript"
"isometric tower defense"
"chunky stylized 3D characters"
"parchment map aesthetic"
"ink and watercolor game art"
"The King is Watching game style"
"clash of clans proportions"
"medieval fantasy strategy"
"top-down war map"
"bold silhouette game characters"
"warm sepia color palette"
"stylized not realistic"
```

Example prompt for a Nature legion fighter:
> *"Chunky stylized 3D game character, treant/ent tree creature, isometric tower defense art style, medieval illuminated manuscript aesthetic, forest green and brown palette, bold black outlines, parchment texture background, The King is Watching game art style, high contrast, 512×512 icon"*
