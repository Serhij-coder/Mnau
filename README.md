# Mnau

A 2D arcade game built with Rust and Macroquad. Control a cat, collect fish for points, and dodge cars to survive as long as you can.

## Play

Download the latest release from [GitHub Releases](https://github.com/anomalyco/Mnau/releases):

| Platform | File | Run |
|----------|------|-----|
| Linux | `Mnau-x86_64.AppImage` | `chmod +x Mnau-x86_64.AppImage && ./Mnau-x86_64.AppImage` |
| Windows | `Mnau.exe` | double-click |

## Build from Source

```bash
cargo run --release
```

For packaging cross-platform builds:

```bash
./build.sh --linux     # AppImage
./build.sh --windows   # .exe
./build.sh --all       # both
```

## Documentation

See [`doc/`](doc/) for setup, architecture, features, and controls.