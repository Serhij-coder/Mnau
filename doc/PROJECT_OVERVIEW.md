# Project Overview - Mnau

## What is Mnau?

**Mnau** is a 2D game written in Rust using the Macroquad graphics library. It's a simple yet engaging interactive experience where players control a cat character in a 2D world filled with fish NPCs and obstacles.

## Project Goals

1. **Educational** - Learn Rust game development with Macroquad
2. **Modular** - Clean, well-organized code with separation of concerns
3. **Expandable** - Easy to add new features and game mechanics
4. **Cross-Platform** - Run on multiple operating systems via Macroquad

## Core Gameplay

### Player Experience
- **Character**: Control a cat character with keyboard (WASD)
- **Objective**: Collect fish to earn points
- **Challenge**: Avoid obstacles (cars) while collecting fish
- **Scoring**: Different fish types award different points

### Game Mechanics

#### Fish Collection
- **Variety**: Three fish types with different point values
  - 🟢 **Green Fish** - +300 points (valuable)
  - ⚪ **White Fish** - +100 points (common)
  - 💀 **Skeleton Fish** - -200 points (penalty)
- **Spawn Mechanism**: Fish spawn on a 5-second timer
- **Collision**: Touching a fish collects it and adds points to score

#### Obstacles
- **Cars**: Dangerous obstacles that spawn every 10 seconds
- **Collision**: Touching a car immediately ends the game
- **Movement**: Cars move across the screen from random directions

### Visual Elements
- **Custom Assets**: Sprite-based graphics (100x100 PNG images)
- **Font**: Custom TrueType font for UI text
- **Fullscreen**: Game runs in fullscreen mode
- **Animations**: Cat has mouth animation (toggles every 0.5 seconds)

## Current Features

### Implemented ✅
- Player cat with WASD controls and boundary collision
- Fish entities with random spawning
- Fish collection with point scoring
- Score display with custom font
- Basic collision detection (AABB)
- Keyboard controls and quit functionality

### In Development 🚀
- Fish respawn timer system (`feature/fish-respawn`)
- Car obstacles (`feature/cars`)

### Future Enhancements 💡
- Multiple game states (menu, game over, pause)
- Sound effects and music
- Enemy AI behaviors
- Power-ups and special items
- Different game modes
- Persistent high score tracking

## Technical Highlights

### Architecture
- **Modular Design**: Separate modules for each entity type (cat, fish, assets, utils)
- **Entity System**: Simple component-based approach with update/render pattern
- **State Management**: Enum-based state machines for type-safe state handling
- **Asset Management**: Concurrent asset loading with async/await

### Code Quality
- **Separation of Concerns**: Each module handles a specific responsibility
- **Collision Detection**: AABB collision system for accurate hit detection
- **Frame-Independent Movement**: Delta time-based movement for consistent behavior
- **Error Handling**: Result types for asset loading

### Performance
- Built in Rust for high performance
- Frame-rate independent updates
- Efficient sprite rendering via Macroquad

## Game Loop

The core game loop follows this pattern each frame:

```
1. Handle Input (WASD movement, Q quit)
2. Update Logic
   - Check fish spawning timer
   - Check car spawning timer
   - Detect collisions
   - Update entity positions
3. Render
   - Clear background
   - Draw all entities (fish, cars, cat)
   - Draw UI (score)
4. Wait for next frame
```

## Controls

| Key | Action |
|-----|--------|
| **W** | Move cat up |
| **A** | Move cat left |
| **S** | Move cat down |
| **D** | Move cat right |
| **Q** | Quit game |

## Repository Structure

```
Mnau/
├── src/                 # Rust source code
│   ├── main.rs         # Game loop and initialization
│   ├── cat.rs          # Player character implementation
│   ├── fish.rs         # Fish NPC implementation
│   ├── car.rs          # Car obstacle implementation (in development)
│   ├── textures.rs     # Asset management
│   └── utils.rs        # Utility functions
├── res/                # Game resources
│   ├── cat.png         # Cat sprite (mouth open)
│   ├── cat2.png        # Cat sprite (mouth closed)
│   ├── fish_*.png      # Fish sprites
│   ├── car_*.png       # Car sprites
│   └── font.otf        # Custom font
├── doc/                # Documentation
├── Cargo.toml          # Project manifest
└── .gitignore          # Git configuration
```

## Development Status

**Version**: 0.1.0

**Timeline**:
- Initial commit: ~2 weeks ago
- Latest stable: Fish collection system
- Current work: Fish respawn + cars feature branch development

**Team**: Individual contributor

## Why Rust?

- **Performance**: Zero-cost abstractions, compiled to native code
- **Safety**: Memory safety without garbage collection
- **Reliability**: Strong type system catches errors at compile time
- **Expressiveness**: Functional programming features for clean code

## Dependencies

- **macroquad** (0.4.14) - Simple 2D graphics and game development
  - Cross-platform window management
  - Sprite rendering
  - Input handling
  - Text rendering

## Getting Started

To get started with Mnau:

1. **Prerequisites**: Install Rust and Cargo
2. **Clone**: Get the project from repository
3. **Build**: `cargo build`
4. **Run**: `cargo run`
5. **Play**: Use WASD to move, Q to quit

For detailed instructions, see [SETUP_AND_BUILD.md](SETUP_AND_BUILD.md)

## Next Steps

- Review [ARCHITECTURE.md](ARCHITECTURE.md) to understand the codebase structure
- Check [FEATURES.md](FEATURES.md) for detailed feature documentation
- Start with [SETUP_AND_BUILD.md](SETUP_AND_BUILD.md) for development environment setup

---

**Happy coding!** 🐱🐟
