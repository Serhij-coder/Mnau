# Architecture — Mnau

## System Design

Macroquad game loop: Input → Logic → Render → Frame wait. Modular entity design (Cat, Fish, Car) with shared Assets.

```
src/
├── main.rs          # Game loop, difficulty scaling, collision
├── cat.rs           # Player (WASD, animation)
├── fish.rs          # Collectible NPCs
├── car.rs           # Obstacle entities
├── textures.rs      # Asset loading
└── utils.rs         # Load screen, game over screen
```

## Modules

### main.rs — Game Loop
- Initializes Macroquad fullscreen, seeds RNG, loads font/assets
- Tracks `elapsed` time for exponential difficulty scaling
- Handles fish/car spawning, collision detection, scoring
- Renders game world: starry background → entities → score → timer → debug overlay
- Game over loop with retry/quit; retry resets all state including elapsed

### cat.rs — Player
- `Cat` struct: `position` (top-left), `velocity` (500 px/s), `CatType` animation
- WASD movement with predictive screen boundary collision
- Mouth open/close animation toggles every 0.5s
- Size: 100×100

### fish.rs — Collectibles
- `Fish` struct: `position` (top-left), `variant` (Green/White/Skeleton)
- Spawns continuously via timer with 100px margin from screen edges
- Point values: Green +300, White +100, Skeleton -200
- Static position, AABB collision (100×100)

### car.rs — Obstacles
- `Car` struct: `position` (top-left), `velocity`, `direction`, `is_white` (texture variant)
- `CarSide`: Left, Right, Top, Bottom — spawns from random edges 100px off-screen
- `new(min_velocity, max_velocity)` — dynamic speed range
- Moves in a straight line, cleaned up when off-screen (threshold: ±150px)
- AABB collision with cat triggers game over

### textures.rs — Asset Loading
- `Assets` struct holds all textures (`cat`, `cat2`, `fish_*`, `car_*`) and `font`
- `load(font)` loads textures concurrently with error handling

### utils.rs — UI Screens
- `load_screen(font)` — animated loading progress bar
- `game_over_screen(font, score)` — retry/quit buttons, returns `GameOverAction`

## Design Patterns

- **Game Loop**: input → logic → render → `next_frame().await`
- **Entity Pattern**: structs with `new()` and `update(&self/&mut self, assets)`
- **State Machine**: enums (`CatType`, `FishType`, `CarSide`)
- **Timer Pattern**: accumulate `get_frame_time()` until threshold, then spawn/reset

## Difficulty Scaling

Exponential decay driven by `elapsed` (seconds since game start/retry):

```
fish_interval = max(1.5, 1.5 + 1.0 × e^(-elapsed / 50))     # 2.5s → 1.5s floor
car_interval  = max(3.0, 3.0 + 2.0 × e^(-elapsed / 55))     # 5.0s → 3.0s floor
car_speed_min = min(600, 600 - 150 × e^(-elapsed / 60))      # 450 → 600 cap
car_speed_max = min(800, 800 - 150 × e^(-elapsed / 60))      # 650 → 800 cap
car_double_threshold = 30.0                                   # 2 cars/spawn after 30s
car_triple_threshold = 60.0                                   # 3 cars/spawn after 60s
```

## Collision Detection

**AABB** with 100×100 bounding boxes. Both cat and entities use top-left-based coordinates:

```
right = pos.x + 100; bottom = pos.y + 100
Collision: pos1.x < right2 && right1 > pos2.x && pos1.y < bottom2 && bottom1 > pos2.y
```

- Fish collision → add points, remove fish
- Car collision → trigger game over screen
