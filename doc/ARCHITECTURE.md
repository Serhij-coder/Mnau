# Architecture - Mnau

## System Design Overview

Mnau follows a classic **game loop architecture** with **modular entity design**. The system uses Macroquad as the rendering and input framework.

```
┌─────────────────────────────────────────────────────┐
│                   Game Loop (main.rs)               │
│  ┌────────────────────────────────────────────────┐ │
│  │  1. Input Handling (WASD, Q)                  │ │
│  │  2. Logic Update (timers, collisions)         │ │
│  │  3. Render (clear, draw, UI)                  │ │
│  │  4. Frame wait (async)                        │ │
│  └────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
         ↓                ↓                ↓
    ┌────────┐      ┌──────────┐    ┌──────────┐
    │  Cat   │      │   Fish   │    │   Car    │
    │Entity  │      │  Entity  │    │  Entity  │
    └────────┘      └──────────┘    └──────────┘
         ↓                ↓                ↓
    ┌──────────────────────────────────────────┐
    │         Assets & Textures                │
    │  (cat.png, fish_*.png, car_*.png)        │
    └──────────────────────────────────────────┘
```

## Project Structure

```
src/
├── main.rs          # Game loop orchestration
├── cat.rs           # Player character entity
├── fish.rs          # Fish NPC entity
├── car.rs           # Car obstacle entity (developing)
├── textures.rs      # Asset manager
└── utils.rs         # Utility functions
```

### Module Responsibilities

#### main.rs
**Purpose**: Game loop and initialization

**Responsibilities**:
- Initialize Macroquad window and fullscreen mode
- Seed RNG with system time (`srand(nanoseconds)`)
- Load custom font and assets
- Create and manage entity collections (Cat, Fish, Cars)
- Main game loop: input → logic → render
- Collision detection and scoring
- Dynamic difficulty scaling (exponential decay on spawn intervals and car speed)
- Game over screen with retry/quit
- Debug overlay toggled by F3 (FPS, timers, counts, positions)

**Key Variables**:
- `cat: Cat` - Player entity
- `fishes: Vec<Fish>` - Fish collection
- `cars: Vec<Car>` - Car collection
- `score: i64` - Player score
- `elapsed: f32` - Time since game start/retry (drives difficulty scaling)
- `fish_spawn_timer: f32` - Fish respawn timer
- `car_spawn_timer: f32` - Car spawn timer

#### cat.rs
**Purpose**: Player character implementation

**Structures**:
- `Cat` - Main cat entity
  - `position: Vec2` - Screen position (top-left corner)
  - `variant: CatType` - Animation state
  - `timer: f32` - Animation timer
  - `velocity: f32` - Movement speed (500 px/sec)
- `CatType` - Animation enum
  - `MouthOpen` - Open mouth sprite
  - `MouthClose` - Closed mouth sprite
- `CatEdge` - Boundary collision points
  - `TopLeft(Vec2)`, `TopRight(Vec2)`, `BotLeft(Vec2)`, `BotRight(Vec2)`

**Key Methods**:
- `new()` - Create cat at center, size 100x100
- `update(&mut self, assets)` - Update and render
- `check_cat_movement()` - Handle WASD input, boundary collision
- `check_cat_variant()` - Toggle mouth animation every 0.5s
- `get_edge()` - Get edge position for collision
- `set_by_edge()` - Position cat from edge point

**Features**:
- WASD keyboard controls (smooth movement)
- Screen boundary collision (predictive)
- Animated mouth (0.5s toggle interval)
- Fixed 100x100 size
- Frame-rate independent movement

#### fish.rs
**Purpose**: Fish NPC entities and mechanics

**Structures**:
- `Fish` - Fish entity
  - `position: Vec2` - Random spawn position
  - `variant: FishType` - Fish type (Green/White/Skeleton)
- `FishType` - Fish variant enum
  - `Green` - +300 points
  - `White` - +100 points
  - `Skeleton` - -200 points

**Key Methods**:
- `new()` - Spawn at random position with random type
- `update(&self, assets)` - Render fish
- `base_points(&self)` - Get point value
- `check_collision(&self, cat_pos)` - AABB collision detection

**Features**:
- Continuous spawning via timer (exponential decay interval)
- Three distinct variants with different values
- Static position (no movement)
- Spawns with 100px margin from screen edges
- AABB collision detection (100x100)

#### car.rs
**Purpose**: Car obstacle entities

**Structures**:
- `Car` - Car entity
  - `position: Vec2` - Current position (top-left corner)
  - `velocity: f32` - Movement speed
  - `direction: Vec2` - Normalized direction vector
  - `is_white: bool` - Texture variant
- `CarSide` - Spawn side enum
  - `Left`, `Right`, `Top`, `Bottom`

**Key Methods**:
- `new(min_velocity, max_velocity)` - Spawn from random edge with dynamic speed range
- `update(&mut self, assets)` - Update position and render
- `is_off_screen(&self)` - Check if off-screen
- `check_collision(&self, cat_pos)` - AABB collision detection

**Features**:
- Spawn from random edges (off-screen)
- Move in straight line across screen
- Frame-rate independent velocity
- Speed increases over time (exponential decay, 300-500 → 600-800)
- Two texture variants (white/yellow)
- Off-screen cleanup

#### textures.rs
**Purpose**: Asset management and loading

**Structure**:
- `Assets` - Container for all game textures and fonts
  - Textures: `cat`, `cat2`, `fish_*`, `car_*`
  - Font: `font`

**Key Methods**:
- `load(font: Font)` - Async asset loading

**Features**:
- Concurrent texture loading
- Error handling with descriptions
- Singleton-like pattern

#### utils.rs
**Purpose**: Utility functions

**Functions**:
- `load_screen(font)` - Animated loading screen with progress bar
- `game_over_screen(font, score)` - Game over UI with retry/quit buttons, returns `GameOverAction`

**Structures**:
- `GameOverAction` - Enum for game over choices (`Retry`, `Quit`, `None`)

## Design Patterns

### 1. Game Loop Pattern
Classic game loop: Input → Logic → Render → Wait

```rust
loop {
    // Input
    handle_input();
    
    // Logic
    update_positions();
    check_collisions();
    
    // Render
    clear_background();
    draw_entities();
    
    // Wait
    next_frame().await;
}
```

### 2. Entity Pattern
Each entity (Cat, Fish, Car) is a struct with:
- State (position, variant)
- Methods (new, update, collision)

```rust
impl Cat {
    pub fn new() -> Self { ... }
    pub fn update(&mut self, assets) { ... }
}
```

### 3. State Machine Pattern
Enums represent discrete states:
- `CatType` - Animation states
- `FishType` - Fish variants with associated values
- `CarSide` - Spawn sides

### 4. Component Pattern
Assets act as a shared component:
- Loaded once at startup
- Passed to entities for rendering
- Centralized resource management

### 5. Timer Pattern
Accumulate frame time for events:

```rust
timer += get_frame_time();
if timer >= 5.0 {
    spawn_entity();
    timer = 0.0;
}
```

## Collision Detection

**Algorithm**: AABB (Axis-Aligned Bounding Box)

All entities use 100x100 bounding boxes:

```rust
pub fn check_collision(pos1: Vec2, pos2: Vec2) -> bool {
    const SIZE: f32 = 100.0;
    
    let right1 = pos1.x + SIZE;
    let bottom1 = pos1.y + SIZE;
    let right2 = pos2.x + SIZE;
    let bottom2 = pos2.y + SIZE;
    
    pos1.x < right2 && right1 > pos2.x &&
    pos1.y < bottom2 && bottom1 > pos2.y
}
```

**Usage**:
- Fish collection: Cat collides with Fish → add points
- Car collision: Cat collides with Car → show game over screen → retry/quit

## Data Flow

### Fish Collection Flow
```
Cat moves → Check collision with each Fish
  → Fish collision detected
  → Get fish.variant.base_points()
  → score += points
  → Remove fish from Vec
  → Log event
```

### Scoring System
```
score: i64 (persistent throughout game)
  Green Fish: +300
  White Fish: +100
  Skeleton Fish: -200
  Display: Top-left, 50pt custom font
```

### Spawning System

**Fish Spawning** (exponential decay):
```
fish_interval = max(1.5, 1.5 + 3.5 * e^(-elapsed / 50.0))

fish_spawn_timer += get_frame_time()
if fish_spawn_timer >= fish_interval {
    fishes.push(Fish::new())
    fish_spawn_timer = 0.0
}
```

**Car Spawning** (exponential decay):
```
car_interval = max(3.0, 3.0 + 7.0 * e^(-elapsed / 55.0))
car_speed_min = min(600.0, 600.0 - 300.0 * e^(-elapsed / 60.0))
car_speed_max = min(800.0, 800.0 - 300.0 * e^(-elapsed / 60.0))

car_spawn_timer += get_frame_time()
if car_spawn_timer >= car_interval {
    cars.push(Car::new(car_speed_min, car_speed_max))
    car_spawn_timer = 0.0
}
```

## Memory Management

### Stack Allocation
- `cat: Cat` - Single entity on stack
- `score: i64` - Score tracking on stack
- Timers: `f32` values on stack

### Heap Allocation
- `fishes: Vec<Fish>` - Dynamic collection
- `cars: Vec<Car>` - Dynamic collection
- `Assets` - Texture/font resources

### Lifetime Management
- Entities own their state
- Assets passed by reference (`&Assets`)
- No circular references or complex lifetimes
- Simple ownership model

## Performance Considerations

### Frame Timing
- Uses `get_frame_time()` for delta timing
- Frame-rate independent movement
- 60 FPS target (Macroquad default)

### Collision Detection
- O(n) per frame for fish collection checks
- O(n) per frame for car collisions
- Linear with number of entities
- AABB provides fast collision checks

### Rendering
- Macroquad handles batching internally
- Each frame: clear + draw sprites + draw text
- No complex rendering pipeline

### Asset Loading
- Concurrent async loading at startup
- Textures cached in Assets struct
- No per-frame loading

## Development Guidelines

### Adding New Features

1. **Create New Module** (if needed)
   - Follow entity pattern
   - Implement `new()` and `update()` methods
   - Keep logic contained

2. **Update main.rs**
   - Add to imports
   - Create collection (Vec or single)
   - Add to game loop

3. **Implement Collision** (if interactive)
   - Add `check_collision()` method
   - Handle in main game loop
   - Update scoring/state as needed

4. **Add Timer** (if spawning entities)
   - Add timer variable
   - Accumulate frame time
   - Check threshold and spawn

### Code Style

- Use Rust conventions (snake_case, PascalCase)
- Prefer `const` for fixed values
- Use `impl` blocks for organization
- Document public methods with `///` comments
- Keep methods small and focused

### Testing

- Manual testing via `cargo run`
- Check `cargo build` for compilation
- Verify collision detection visually
- Test edge cases (screen boundaries)

## Future Architecture Considerations

### Potential Improvements
- Event system for game state changes
- UI manager for menus/HUD
- Particle system for effects
- Sound manager
- Input mapping system
- Configuration file support

### Scalability
- Current design suitable for small games
- Would need ECS for complex projects
- Consider state machines for complex logic
- Asset streaming for larger games

---

## Quick Reference

| Concept | Implementation |
|---------|-----------------|
| **Movement** | Delta time × velocity |
| **Collision** | AABB with 100x100 boxes |
| **Rendering** | Macroquad sprite drawing |
| **Input** | Macroquad KeyCode checking |
| **Timing** | Frame time accumulation |
| **Entities** | Structs with methods |
| **Assets** | Loaded once, shared via reference |

---

For implementation details of specific systems, see [FEATURES.md](FEATURES.md)
