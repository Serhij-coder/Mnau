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
- **Interval**: exponential decay `1.5 + 1.0 × e^(-elapsed / 50)`, starts at 2.5s, floor at 1.5s
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
- **Double car**: after 30s elapsed, 2 cars spawn per interval instead of 1
- **Triple car**: after 60s elapsed, 3 cars spawn per interval

### Difficulty Scaling
- **Spawn interval**: exponential decay `3.0 + 2.0 × e^(-elapsed / 55)`, starts at 5.0s, floor at 3.0s
- **Speed range**: starts at 450–650 px/s, caps at 600–800 px/s over time

### Collision
- AABB collision with cat (both 100×100) triggers game over screen
- Game over shows final score with retry/quit buttons

## Difficulty Curve

| Elapsed | Fish interval | Car interval | Car speed | Cars/spawn |
|---------|--------------|-------------|-----------|------------|
| 0s | 2.5s | 5.0s | 450–650 | 1 |
| 30s | 2.1s | 4.2s | 509–709 | 2 |
| 60s | 1.8s | 3.7s | 545–745 | 3 |
| 90s | 1.7s | 3.4s | 567–767 | 3 |
| 120s+ | ~1.5s | ~3.0s | ~600–800 | 3 |

## Timer

- Integer seconds display (e.g. `123s`), right-aligned in top-right corner
- Driven by `elapsed` — same counter used for difficulty scaling
- Resets to 0 on retry along with all game state
- Same style as score: 50pt custom font, white color

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
