# Features — Mnau

## Movement

- **WASD** keyboard controls, 500 px/sec, frame-rate independent
- **Screen boundary collision** (predictive, stops at edges)
- **Cat size**: 100×100, mouth animation toggles every 0.5s

## Fish Collection

### Fish Types

| Type | Points |
|------|--------|
| Green | +300 |
| White | +100 |
| Skeleton | -200 |

### Spawning
- **Starts with 0 fish**, continuous timer-based spawning
- **Interval**: exponential decay `1.5 + 3.5 × e^(-elapsed / 50)`, starts at 5.0s, floor at 1.5s
- **Distribution**: random, 100px margin from screen edges
- **Variants**: 1/3 chance each type

### Collection
- AABB collision (100×100 boxes, top-left based)
- Points added/subtracted on collision, fish removed from game

## Car Obstacles

### Spawn & Behavior
- Spawns from 4 random edges (Left/Right/Top/Bottom), 100px off-screen
- Moves straight across the screen at frame-rate-independent velocity
- Two variants: white and yellow (50/50 chance)

### Difficulty Scaling
- **Spawn interval**: exponential decay `3.0 + 7.0 × e^(-elapsed / 55)`, starts at 10.0s, floor at 3.0s
- **Speed range**: starts at 300–500 px/s, caps at 600–800 px/s over time

### Collision
- AABB collision with cat (both 100×100) triggers game over screen
- Game over shows final score with retry/quit buttons

## Difficulty Curve

| Elapsed | Fish interval | Car interval | Car speed |
|---------|--------------|-------------|-----------|
| 0s | 5.0s | 10.0s | 300–500 |
| 30s | 3.8s | 7.9s | 390–590 |
| 60s | 2.6s | 5.8s | 480–680 |
| 90s | 2.0s | 4.3s | 545–745 |
| 120s+ | ~1.5s | ~3.0s | ~600–800 |

## Scoring

- `i64` score, persists until game over or retry
- Displayed top-left, 50pt custom font, white color
- Retry resets score to 0 along with all game state

## Controls

| Key | Action |
|-----|--------|
| W/A/S/D | Move cat |
| Q | Quit game |
| F3 | Toggle debug overlay |

## Debug Screen

Toggle with **F3**. Top-right overlay with dark semi-transparent background:

- FPS, elapsed time, score
- Fish spawn interval + count alive
- Car spawn interval + count alive
- Car speed range
- Cat screen position
