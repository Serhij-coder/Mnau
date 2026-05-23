# Mnau - Documentation

Welcome to the Mnau project documentation. This folder contains comprehensive guides for understanding and developing the Mnau 2D game.

## Documentation Structure

### Core Documentation
- **[PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md)** - High-level project description, goals, and features
- **[ARCHITECTURE.md](ARCHITECTURE.md)** - System design, module structure, and design patterns
- **[SETUP_AND_BUILD.md](SETUP_AND_BUILD.md)** - Installation, build instructions, and development environment setup
- **[FEATURES.md](FEATURES.md)** - Detailed feature descriptions and implementation details

### Quick Links
- [Getting Started](SETUP_AND_BUILD.md#quick-start)
- [Project Structure](ARCHITECTURE.md#project-structure)
- [Game Mechanics](FEATURES.md#game-mechanics)
- [Branching Strategy](#branching-strategy)

## Branching Strategy

The project uses feature branch development:

- **main** - Stable, production-ready code
- **feature/*** - Feature development branches
  - `feature/fish-collection` - Fish collection and scoring system
  - `feature/fish-respawn` - Automatic fish spawning mechanism
  - `feature/cars` - Car obstacles and collision system

## Key Technologies

- **Language**: Rust (Edition 2024)
- **Graphics Framework**: Macroquad 0.4.14
- **Build System**: Cargo
- **Platform**: Cross-platform (Linux, macOS, Windows via Macroquad)

## Quick Navigation

For different purposes, start here:

- **I want to play the game** → [SETUP_AND_BUILD.md](SETUP_AND_BUILD.md#quick-start)
- **I want to understand how it works** → [ARCHITECTURE.md](ARCHITECTURE.md)
- **I want to add a new feature** → [ARCHITECTURE.md](ARCHITECTURE.md#development-guidelines)
- **I want to understand game mechanics** → [FEATURES.md](FEATURES.md)

## Contributing

When adding new features:
1. Create a new branch from `main`: `git checkout -b feature/your-feature`
2. Follow the code style and patterns documented in [ARCHITECTURE.md](ARCHITECTURE.md)
3. Update relevant documentation
4. Test thoroughly before requesting merge

## Project Status

**Current Version**: 0.1.0

**Active Branches**:
- `main` - Stable with fish collection, car obstacles, difficulty scaling, game over screen
- `feature/cars` - Merged to main

---

For questions or issues, refer to the specific documentation files or examine the source code in `src/`.
