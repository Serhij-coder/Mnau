# Features - Mnau

## Overview

This document describes all implemented and in-development features of the Mnau game.

## Game Mechanics

### Movement System

**Player Controls**
- **W Key**: Move cat upward
- **A Key**: Move cat leftward
- **S Key**: Move cat downward
- **D Key**: Move cat rightward
- **Q Key**: Quit game

**Movement Properties**
- **Speed**: 500 pixels/second (frame-rate independent)
- **Size**: 100×100 pixels
- **Boundaries**: Screen edges
- **Animation**: Mouth toggles every 0.5 seconds

**Implementation** (src/cat.rs)
```rust
pub fn check_cat_movement(&mut self) {
    if is_key_down(KeyCode::D) {
        // Boundary checking prevents off-screen movement
        if self.get_edge(TopRight(Vec2::ZERO)).x + self.velocity * get_frame_time()
            > screen_width() {
            // Stop at edge
        } else {
            self.position.x += self.velocity * get_frame_time();
        }
    }
}
```

## Feature 1: Fish Collection ✅ IMPLEMENTED

### Overview
Core mechanic where players collect different fish types to earn points.

### Fish Types

| Type | Appearance | Points | Rarity |
|------|-----------|--------|--------|
| Green Fish | 🟢 Green sprite | +300 | Valuable |
| White Fish | ⚪ White sprite | +100 | Common |
| Skeleton Fish | 💀 Skeleton sprite | -200 | Penalty |

### Spawning Behavior
- **Initial State**: 0 fish — fish spawn continuously via timer
- **Interval**: Exponential decay from 5.0s → 1.5s floor (time-based difficulty scaling)
- **Distribution**: Random across full screen (`rand::gen_range(0, screen_width())`, `rand::gen_range(0, screen_height())`)
- **Variants**: Random selection (1/3 each type)

### Collection Mechanics

**Collision Detection** (src/fish.rs)
- Uses AABB (Axis-Aligned Bounding Box) collision
- Both cat and fish are 100×100 pixels
- Collision triggers immediate collection

```rust
pub fn check_collision(&self, cat_pos: Vec2) -> bool {
    const CAT_SIZE: f32 = 100.0;
    const FISH_SIZE: f32 = 100.0;
    
    let cat_right = cat_pos.x + CAT_SIZE;
    let cat_bottom = cat_pos.y + CAT_SIZE;
    let fish_right = self.position.x + FISH_SIZE;
    let fish_bottom = self.position.y + FISH_SIZE;
    
    cat_pos.x < fish_right && cat_right > self.position.x &&
    cat_pos.y < fish_bottom && cat_bottom > self.position.y
}
```

**Collection Process**
1. Cat touches fish collision boundary
2. Points calculated: `score += fish.variant.base_points()`
3. Fish removed from game world
4. Event logged to console

### Scoring System

**Score Tracking**
- **Data Type**: `i64` (supports negative scores)
- **Persistence**: Maintained throughout game session
- **Display**: Top-left corner, 50pt custom font
- **Updates**: Real-time on collection

**Score Calculation**
```rust
score += match fish.variant {
    FishType::Green => 300,
    FishType::White => 100,
    FishType::Skeleton => -200,
};
```

**Display** (src/main.rs)
```rust
draw_text_ex(
    &format!("Score: {}", score),
    10.0,
    60.0,
    TextParams {
        font: Some(&assets.font),
        font_size: 50,
        color: WHITE,
        ..Default::default()
    },
);
```

### Branch: feature/fish-collection
- **Status**: ✅ Merged to main
- **Location**: main branch
- **Features**: Initial fish spawning + collection + scoring

## Feature 2: Fish Respawn ✅ IMPLEMENTED

### Overview
Automatic fish spawning with time-based difficulty scaling — intervals decrease the longer you play.

### Spawning Mechanics

**Spawn Timer** (exponential decay)
- **Start**: 5.0 seconds between fish
- **Formula**: `fish_interval = 1.5 + 3.5 × e^(-elapsed / 50.0)`
- **Floor**: 1.5 seconds (approached asymptotically)
- **Spawn Count**: 1 fish per interval
- **Variants**: Random (1/3 probability each)

**Initial State**
- Game starts with **0 fish**
- First fish spawns after ~5 seconds
- Continuous spawning, accelerating over time

### Implementation Details (src/main.rs)

```rust
let fish_interval = (1.5 + 3.5 * (-elapsed / 50.0).exp()).max(1.5);

fish_spawn_timer += get_frame_time();
if fish_spawn_timer >= fish_interval {
    fishes.push(Fish::new());
    fish_spawn_timer = 0.0;
}
```

### Difficulty Curve

| Elapsed | Fish interval |
|---------|--------------|
| 0s | 5.0s |
| 30s | 3.8s |
| 60s | 2.6s |
| 90s | 2.0s |
| 120s+ | ~1.5s (floor) |

### Branch: feature/fish-respawn
- **Status**: ✅ Merged to main
- **Location**: main branch

## Feature 3: Cars ✅ IMPLEMENTED

### Overview
Dynamic obstacle system where cars spawn randomly and move across the screen, creating hazards for the player.

### Car Mechanics

**Spawn Behavior** (exponential decay)
- **Start**: 10.0 seconds between cars
- **Formula**: `car_interval = 3.0 + 7.0 × e^(-elapsed / 55.0)`
- **Floor**: 3.0 seconds (approached asymptotically)
- **Spawn Location**: Random screen edge (100px off-screen)
  - Left edge: x = -100
  - Right edge: x = screen_width() + 100
  - Top edge: y = -100
  - Bottom edge: y = screen_height() + 100
- **Direction**: Toward opposite edge
- **Speed**: Exponential decay from 300–500 → 600–800 over time
  - Formula: `car_speed = rand(600 - 300×e^(-elapsed/60), 800 - 300×e^(-elapsed/60))`

**Movement**
- Linear path across screen
- Frame-rate independent velocity
- Automatic cleanup when off-screen

### Car Types

| Variant | Color | Texture |
|---------|-------|---------|
| Standard | White | car_white.png |
| Alternate | Yellow | car_yellow.png |

**Texture Properties**
- Size: 100×100 pixels (matches other entities)
- Random selection: 50/50 chance

### Collision & Game Over

**Car-Cat Collision**
- Uses AABB collision detection
- Both entities are 100×100
- Collision triggers game over screen

**Game Over Screen**
- Displays "GAME OVER" text and final score
- "RETRY" button: resets all state (cars, fish, score, elapsed, cat position)
- "QUIT" button: exits the game
- Uses mouse input for button interaction

**Implementation** (src/main.rs)
```rust
if game_over {
    loop {
        let action = utils::game_over_screen(&assets.font, score).await;
        match action {
            GameOverAction::Retry => {
                fishes.clear(); fish_spawn_timer = 0.0;
                cars.clear(); car_spawn_timer = 0.0;
                cat = Cat::new(); score = 0; elapsed = 0.0;
                break;
            }
            GameOverAction::Quit => { exit(0); }
            GameOverAction::None => { }
        }
    }
}
```

### Car Module Structure (src/car.rs)

**Car Struct**
```rust
pub struct Car {
    pub position: Vec2,        // Current position
    pub velocity: f32,         // Speed (px/sec)
    pub direction: Vec2,       // Normalized direction
    pub is_white: bool,        // Texture variant
}
```

**CarSide Enum** (for spawn logic)
```rust
pub enum CarSide {
    Left,
    Right,
    Top,
    Bottom,
}
```

**Methods**
- `new(min_velocity, max_velocity)` - Spawn from random edge with velocity range
- `update()` - Update position and render
- `is_off_screen()` - Check if outside screen bounds
- `check_collision()` - AABB collision with cat

### Spawning Logic

```rust
let car_interval = (3.0 + 7.0 * (-elapsed / 55.0).exp()).max(3.0);
let car_speed_min = (600.0 - 300.0 * (-elapsed / 60.0).exp()).min(600.0);
let car_speed_max = (800.0 - 300.0 * (-elapsed / 60.0).exp()).min(800.0);

car_spawn_timer += get_frame_time();
if car_spawn_timer >= car_interval {
    cars.push(Car::new(car_speed_min, car_speed_max));
    car_spawn_timer = 0.0;
}

// Remove off-screen cars
cars.retain(|car| !car.is_off_screen());
```

### Branch: feature/cars
- **Status**: ✅ Merged to main
- **Location**: main branch

## Feature Roadmap

### Tier 1: Core Experience ✅
- [x] Player movement with controls
- [x] Fish collection and scoring
- [x] Screen boundary collision
- [x] Display score with custom font

### Tier 2: Gameplay Loop ✅
- [x] Fish respawn timer (feature/fish-respawn)
- [x] Car obstacles (feature/cars)
- [x] Multiple game sessions (retry functionality)

### Tier 3: Polish 🚀
- [x] Game over screen
- [ ] Main menu
- [ ] High score tracking
- [ ] Sound effects
- [ ] Background music

### Tier 4: Advanced 💡
- [ ] Power-ups
- [ ] Different game modes
- [ ] AI fish behaviors
- [ ] Level progression
- [ ] Particle effects

## Asset Management

### Textures

| Asset | Size | Type | Purpose |
|-------|------|------|---------|
| cat.png | 100×100 | PNG | Cat (mouth open) |
| cat2.png | 100×100 | PNG | Cat (mouth closed) |
| fish_green.png | 100×100 | PNG | Green fish |
| fish_white.png | 100×100 | PNG | White fish |
| fish_skeleton.png | 100×100 | PNG | Skeleton fish |
| car_white.png | 100×100 | PNG | White car |
| car_yellow.png | 100×100 | PNG | Yellow car |

### Font

| Asset | Type | Purpose |
|-------|------|---------|
| font.otf | TrueType | Score display, UI text |

**Font Usage**
- Size: 50pt for score
- Color: White
- Position: Top-left (10px, 60px)
- Default font for other text

### Asset Loading (src/textures.rs)

```rust
pub struct Assets {
    pub cat: Texture2D,
    pub cat2: Texture2D,
    pub fish_green: Texture2D,
    pub fish_white: Texture2D,
    pub fish_skeleton: Texture2D,
    pub car_white: Texture2D,
    pub car_yellow: Texture2D,
    pub font: Font,
}
```

## Performance Characteristics

### Memory Usage
- **Cat**: ~500 bytes
- **Per Fish**: ~500 bytes
- **Per Car**: ~500 bytes
- **Textures**: ~1-2 MB (GPU memory)
- **Font**: ~100 KB

### CPU Usage
- **Frame Rate**: 60 FPS target
- **Collision Checks**: O(n) where n = number of entities
- **Spawn Timers**: O(1)
- **Rendering**: Macroquad handles optimization

### Storage
- **Executable**: ~10-15 MB (release build)
- **Assets**: ~800 KB
- **Total**: ~15-20 MB

## Gameplay Balance

### Difficulty Progression

Difficulty scales **exponentially** with game time. All spawn intervals and car speeds use `elapsed` (time since last retry):

| Elapsed | Fish interval | Car interval | Car speed |
|---------|--------------|-------------|-----------|
| 0s | 5.0s | 10.0s | 300–500 |
| 30s | 3.8s | 7.9s | 390–590 |
| 60s | 2.6s | 5.8s | 480–680 |
| 90s | 2.0s | 4.3s | 545–745 |
| 120s+ | ~1.5s (floor) | ~3.0s (floor) | ~600–800 (cap) |

The rate of change is fastest at the start and gradually tapers — there is no sudden difficulty spike. Resetting via retry sets `elapsed` back to 0.

### Tuning Parameters

| Parameter | Starting value | Formula | Floor/Cap |
|-----------|---------------|---------|-----------|
| Fish spawn interval | 5.0s | `1.5 + 3.5 × e^(-elapsed/50)` | 1.5s |
| Car spawn interval | 10.0s | `3.0 + 7.0 × e^(-elapsed/55)` | 3.0s |
| Car speed min | 300 | `600 - 300 × e^(-elapsed/60)` | 600 |
| Car speed max | 500 | `800 - 300 × e^(-elapsed/60)` | 800 |
| Green fish points | +300 | — | — |
| White fish points | +100 | — | — |
| Skeleton fish points | -200 | — | — |

## Controls Summary

| Key | Action | Feature |
|-----|--------|---------|
| **W** | Move up | Feature 1+ |
| **A** | Move left | Feature 1+ |
| **S** | Move down | Feature 1+ |
| **D** | Move right | Feature 1+ |
| **Q** | Quit | Feature 1+ |

*Note: No other inputs currently active*

## Future Enhancements

### Potential Features
- Mouse controls as alternative
- Pause functionality
- Difficulty levels
- Fish AI (swimming patterns)
- Particle effects on collection
- Screen shake on car collision
- Combo multipliers
- Special fish types

### Potential Improvements
- Configuration file for tunables
- Settings menu
- Replay system
- Leaderboard
- Procedural level generation
- Mobile support

---

For implementation details, see [ARCHITECTURE.md](ARCHITECTURE.md)  
For setup instructions, see [SETUP_AND_BUILD.md](SETUP_AND_BUILD.md)
