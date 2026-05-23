# Setup and Build — Mnau

## Prerequisites

- Rust 1.95.0+ (install via [rustup.rs](https://rustup.rs))
- Git
- Linux: `sudo apt install libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev`

## Quick Start

```bash
git clone <repository-url>
cd Mnau
cargo run --release
```

## Build Commands

| Task | Command |
|------|---------|
| Build (debug) | `cargo build` |
| Build (release) | `cargo build --release` |
| Run | `cargo run` |
| Run (optimized) | `cargo run --release` |
| Check syntax | `cargo check` |
| Format code | `cargo fmt` |
| Lint code | `cargo clippy` |

## Controls

| Key | Action |
|-----|--------|
| W/A/S/D | Move cat |
| Q | Quit |
| F3 | Toggle debug overlay |

## Common Issues

**"failed to load texture"** — Run from project root:
```bash
cd /path/to/Mnau
cargo run
```

**Game running slow** — Use release build:
```bash
cargo run --release
```

**Game window doesn't appear (Linux)** — Install system deps:
```bash
# Ubuntu/Debian
sudo apt install libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev
# Fedora
sudo dnf install libxrandr-devel libxinerama-devel libxcursor-devel libxi-devel
```
