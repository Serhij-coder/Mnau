#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/build"
APPDIR="$BUILD_DIR/Mnau.AppDir"
APPIMAGETOOL_URL="https://github.com/AppImage/appimagetool/releases/download/continuous/appimagetool-x86_64.AppImage"

info()  { echo -e "\033[1;34m[INFO]\033[0m $*"; }
error() { echo -e "\033[1;31m[ERROR]\033[0m $*" >&2; exit 1; }

build_linux() {
    info "Building Linux binary..."
    cargo build --release --target x86_64-unknown-linux-gnu

    info "Creating AppDir..."
    mkdir -p "$APPDIR"/usr/bin
    mkdir -p "$APPDIR"/usr/share/applications
    mkdir -p "$APPDIR"/usr/share/icons/hicolor/256x256/apps

    cp target/x86_64-unknown-linux-gnu/release/Mnau "$APPDIR/usr/bin/Mnau"
    chmod +x "$APPDIR/usr/bin/Mnau"

    cp res/cat.png "$APPDIR/mnau.png"
    cp res/cat.png "$APPDIR/usr/share/icons/hicolor/256x256/apps/mnau.png"

    cat > "$APPDIR/mnau.desktop" << 'EOF'
[Desktop Entry]
Name=Mnau
Comment=2D arcade game - collect fish, avoid cars!
Exec=Mnau
Icon=mnau
Type=Application
Categories=Game;
Terminal=false
EOF

    cat > "$APPDIR/AppRun" << 'APPRUN'
#!/bin/bash
SELF=$(readlink -f "$0")
HERE=${SELF%/*}
cd "$HERE" || exit 1
exec "$HERE/usr/bin/Mnau" "$@"
APPRUN
    chmod +x "$APPDIR/AppRun"

    info "Downloading appimagetool..."
    APPIMAGETOOL="$BUILD_DIR/.appimagetool"
    if [ ! -f "$APPIMAGETOOL" ]; then
        curl -sL "$APPIMAGETOOL_URL" -o "$APPIMAGETOOL"
        chmod +x "$APPIMAGETOOL"
    fi

    info "Extracting appimagetool..."
    TOOL_DIR="$BUILD_DIR/.appimagetool_extracted"
    rm -rf "$TOOL_DIR"
    mkdir -p "$TOOL_DIR"
    cd "$TOOL_DIR"
    "$APPIMAGETOOL" --appimage-extract >/dev/null 2>&1 || true
    cd "$SCRIPT_DIR"

    info "Creating AppImage..."
    ARCH=x86_64 "$TOOL_DIR/squashfs-root/AppRun" "$APPDIR" "$BUILD_DIR/Mnau-x86_64.AppImage"

    rm -rf "$APPDIR" "$TOOL_DIR"
    info "Linux build complete: build/Mnau-x86_64.AppImage"
}

build_windows() {
    info "Building Windows binary..."
    cargo build --release --target x86_64-pc-windows-gnu

    mkdir -p "$BUILD_DIR/windows"
    cp target/x86_64-pc-windows-gnu/release/Mnau.exe "$BUILD_DIR/windows/Mnau.exe"

    info "Windows build complete: build/windows/Mnau.exe"
}

usage() {
    echo "Usage: $0 [--linux] [--windows] [--all]"
    echo ""
    echo "  --linux    Build Linux AppImage"
    echo "  --windows  Build Windows .exe"
    echo "  --all      Build both"
    exit 1
}

LINUX=0
WINDOWS=0

if [ $# -eq 0 ]; then
    usage
fi

while [ $# -gt 0 ]; do
    case "$1" in
        --linux)   LINUX=1 ;;
        --windows) WINDOWS=1 ;;
        --all)     LINUX=1; WINDOWS=1 ;;
        -h|--help) usage ;;
        *)         error "Unknown option: $1" ;;
    esac
    shift
done

info "Clearing build directory..."
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR"

[ "$LINUX" -eq 1 ]   && build_linux
[ "$WINDOWS" -eq 1 ] && build_windows

info "Done!"