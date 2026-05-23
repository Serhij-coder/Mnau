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

### Spawning Behavior (Configured)
- **Initial State**: 10 fish spawn randomly on game start
- **Respawn**: None (static placement)
- **Distribution**: Random across full screen
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

## Feature 2: Fish Respawn 🚀 IN DEVELOPMENT

### Overview
Automatic fish spawning mechanism with timed intervals instead of static initial placement.

### Spawning Mechanics

**Respawn Timer**
- **Interval**: 5 seconds
- **Implementation**: Frame-time accumulation
- **Spawn Count**: 1 fish per interval
- **Variants**: Random (1/3 probability each)

**Initial State**
- Game starts with **0 fish**
- First fish spawns after 5 seconds
- Continuous spawning throughout game

### Implementation Details (src/main.rs)

```rust
let mut fish_spawn_timer: f32 = 0.0;

loop {
    // Accumulate time
    fish_spawn_timer += get_frame_time();
    
    // Check if spawn interval reached
    if fish_spawn_timer >= 5.0 {
        fishes.push(Fish::new());
        fish_spawn_timer = 0.0;
    }
}
```

### Behavior
- No respawn of collected fish (they stay gone)
- Continuous new spawning every 5 seconds
- Random position each spawn
- Random type each spawn
- Creates ongoing gameplay challenge

### Branch: feature/fish-respawn
- **Status**: 🚀 In development
- **Location**: feature/fish-respawn branch
- **Changes**: Removes initial spawn loop, adds 5-second timer

### Testing Checklist
- [ ] Game starts with 0 fish
- [ ] First fish appears at ~5 seconds
- [ ] Second fish appears at ~10 seconds
- [ ] Fish can still be collected
- [ ] Scoring works correctly
- [ ] Score display persists

## Feature 3: Cars 🚀 IN DEVELOPMENT

### Overview
Dynamic obstacle system where cars spawn randomly and move across the screen, creating hazards for the player.

### Car Mechanics

**Spawn Behavior**
- **Interval**: 10 seconds
- **Spawn Location**: Random screen edge (off-screen)
  - Left edge: x = -100
  - Right edge: x = screen_width() + 100
  - Top edge: y = -100
  - Bottom edge: y = screen_height() + 100
- **Direction**: Toward opposite edge
- **Speed**: 300-500 pixels/second (randomized)

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
- Collision triggers immediate game exit

**Implementation** (src/main.rs)
```rust
for car in &cars {
    if car.check_collision(cat.position) {
        exit(0);  // Exit immediately
    }
}
```

**Behavior**
- No game over screen
- No fade transition
- Instant window close
- Simple, direct end game

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
- `new()` - Spawn from random edge with random velocity/direction
- `update()` - Update position and render
- `is_off_screen()` - Check if outside screen bounds
- `check_collision()` - AABB collision with cat

### Spawning Logic

```rust
let mut car_spawn_timer: f32 = 0.0;

loop {
    car_spawn_timer += get_frame_time();
    
    if car_spawn_timer >= 10.0 {
        cars.push(Car::new());
        car_spawn_timer = 0.0;
    }
    
    // Remove off-screen cars
    cars.retain(|car| !car.is_off_screen());
}
```

### Game Loop Integration

```rust
// Car updates and rendering
for car in &cars {
    car.update(&assets);
}

// Car collision checking
for car in &cars {
    if car.check_collision(cat.position) {
        exit(0);
    }
}
```

### Branch: feature/cars
- **Status**: 🚀 In development
- **Location**: feature/cars branch
- **New File**: src/car.rs
- **Changes**: Car spawning, movement, collision

### Testing Checklist
- [ ] First car spawns after ~10 seconds
- [ ] Car moves across screen
- [ ] Car exits after leaving screen
- [ ] New car spawns every 10 seconds
- [ ] Cat collision exits game immediately
- [ ] Score/fish collection works while cars active
- [ ] White and yellow cars appear

## Feature Roadmap

### Tier 1: Core Experience ✅
- [x] Player movement with controls
- [x] Fish collection and scoring
- [x] Screen boundary collision
- [x] Display score with custom font

### Tier 2: Gameplay Loop 🚀
- [ ] Fish respawn timer (feature/fish-respawn)
- [ ] Car obstacles (feature/cars)
- [ ] Multiple game sessions

### Tier 3: Polish 💡
- [ ] Game over screen
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

**Current** (feature/fish-collection)
- Static 10 fish
- No pressure (can collect at own pace)
- Good for learning controls

**With Respawn** (feature/fish-respawn)
- Continuous fish spawn
- Can accumulate large scores
- Moderate challenge

**With Cars** (feature/cars)
- Car danger every 10 seconds
- Time pressure
- Requires skill and awareness
- Game can end suddenly

### Tuning Parameters

Consider adjusting:
- Fish spawn interval (currently 5s)
- Car spawn interval (currently 10s)
- Car speed (currently 300-500)
- Point values (Green: 300, White: 100, Skeleton: -200)

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
