# Setup and Build — Mnau

## Prerequisites

- Rust 1.95.0+ (install via [rustup.rs](https://rustup.rs))
- Git

### System deps (required for building on Linux)

```bash
# Arch
sudo pacman -S libxrandr libxinerama libxcursor libxi

# Ubuntu/Debian
sudo apt install libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev

# Fedora
sudo dnf install libxrandr-devel libxinerama-devel libxcursor-devel libxi-devel
```

### Cross-compilation deps (optional, for Windows builds)

```bash
# Arch
sudo pacman -S mingw-w64-gcc

# Ubuntu/Debian
sudo apt install mingw-w64

# Then add the target:
rustup target add x86_64-pc-windows-gnu
```

### AppImage deps (optional, for Linux packaging)

Requires `curl` and FUSE (`sudo pacman -S fuse2` / `sudo apt install libfuse2`). The `build.sh` script downloads appimagetool automatically.

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

### Release packaging via `build.sh`

```bash
./build.sh --linux     # → build/Mnau-x86_64.AppImage
./build.sh --windows   # → build/windows/Mnau.exe
./build.sh --all       # both
```

Clears `build/` before each run. Textures are embedded in the binaries — no `res/` folder needed at runtime.

## Controls

| Key | Action |
|-----|--------|
| W/A/S/D | Move cat |
| Q | Quit |
| F3 | Toggle debug overlay |

## Common Issues

**Game running slow** — Use release build:
```bash
cargo run --release
```

**Game window doesn't appear (Linux)** — Install system deps (see Prerequisites above).

**Windows build fails** — Install `mingw-w64-gcc` and run `rustup target add x86_64-pc-windows-gnu`.

**AppImage won't run** — Install FUSE: `sudo pacman -S fuse2` or `sudo apt install libfuse2`.
