# Setup and Build Guide - Mnau

## Quick Start

### Prerequisites
- Rust 1.95.0 or later
- Cargo (comes with Rust)
- Git

### Installation & Running

```bash
# Clone the repository
git clone <repository-url>
cd Mnau

# Build the project
cargo build

# Run the game
cargo run

# Run in release mode (optimized)
cargo run --release
```

## System Requirements

### Minimum Requirements
- **OS**: Linux, macOS, or Windows
- **RAM**: 256 MB
- **Disk**: 100 MB (including dependencies)
- **Display**: 1024x768 or higher

### Development Requirements
- Rust toolchain (1.95.0+)
- Cargo package manager
- Git for version control
- Text editor or IDE (VSCode recommended)

## Installing Rust

### Linux/macOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Windows
Download from https://rustup.rs/ and run the installer

### Verify Installation
```bash
rustc --version
cargo --version
```

## Project Setup

### Clone Repository
```bash
git clone <repository-url>
cd Mnau
```

### Verify Structure
```bash
ls -la
# Should show: src/, res/, doc/, Cargo.toml, .gitignore, etc.
```

### Initial Build
```bash
cargo build
# First build takes longer due to dependency compilation
```

## Building the Project

### Debug Build
```bash
cargo build
# Output: target/debug/Mnau
```

**Pros**: Faster compilation, detailed error messages  
**Cons**: Slower runtime, larger binary

### Release Build
```bash
cargo build --release
# Output: target/release/Mnau
```

**Pros**: Optimized, ~10x faster gameplay  
**Cons**: Longer compilation time

### Check Only (No Binary)
```bash
cargo check
# Fast syntax check without generating binary
```

## Running the Game

### Standard Run
```bash
cargo run
```

### Release Run (Recommended for Playing)
```bash
cargo run --release
```

### Direct Execution
```bash
# After building
./target/debug/Mnau        # Debug version
./target/release/Mnau      # Release version (Linux/macOS)
./target\release\Mnau.exe  # Release version (Windows)
```

## Development Workflow

### 1. Making Changes
```bash
# Edit code in src/
vim src/main.rs

# Check syntax
cargo check

# If errors, fix them
# If OK, proceed to test
```

### 2. Testing Changes
```bash
# Run with changes
cargo run

# Test game mechanics manually
# Use WASD to move, Q to quit
```

### 3. Committing Changes
```bash
# Stage changes
git add src/

# Commit with message
git commit -m "Description of changes"

# Push to repository
git push origin branch-name
```

## Working with Branches

### View Branches
```bash
git branch
# Current branch marked with *
```

### Create New Branch
```bash
git checkout -b feature/your-feature-name
# Creates and switches to new branch
```

### Switch Branches
```bash
git checkout feature/fish-collection
# Switches to existing branch
```

### Merge Branches
```bash
git checkout main
git merge feature/your-feature
```

### Delete Branch
```bash
git branch -d feature/old-feature
```

## Managing Dependencies

### View Dependencies
```bash
# Check Cargo.toml for listed dependencies
cat Cargo.toml
```

### Current Dependencies
```
macroquad = "0.4.14"  # Graphics framework
```

### Add New Dependency
```bash
cargo add package-name
```

### Update Dependencies
```bash
cargo update
```

## Cleaning Up

### Remove Build Artifacts
```bash
cargo clean
# Removes target/ directory
# Useful if build is corrupted
```

### Rebuild from Scratch
```bash
cargo clean
cargo build
```

## Common Issues and Solutions

### Issue: "rustc not found"
**Solution**: Install Rust or add to PATH
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: "failed to load texture"
**Solution**: Run from project root, not src/
```bash
cd /path/to/Mnau
cargo run
```

### Issue: Compilation takes too long
**Solution**: Use incremental compilation or check only
```bash
cargo check  # Faster than build
```

### Issue: Game window doesn't appear (Linux)
**Solution**: Install graphics libraries
```bash
# Ubuntu/Debian
sudo apt-get install libxrandr-dev libxinerama-dev libxcursor-dev libxi-dev

# Fedora
sudo dnf install libxrandr-devel libxinerama-devel libxcursor-devel libxi-devel
```

### Issue: Game running slow
**Solution**: Use release build
```bash
cargo run --release  # ~10x faster than debug
```

## Development Tools

### Recommended IDE/Editor
- **VSCode** (recommended)
  - Install "Rust Analyzer" extension
  - Provides real-time error checking

- **IntelliJ IDEA**
  - Community or Ultimate edition
  - Rust plugin available

- **Vim/Neovim**
  - With rust.vim plugin
  - Lightweight option

### Useful Cargo Commands

```bash
cargo build              # Compile in debug mode
cargo build --release   # Compile optimized
cargo run               # Build and run
cargo check             # Fast syntax check
cargo test              # Run tests
cargo clean             # Remove build artifacts
cargo doc --open        # Generate and open documentation
cargo fmt               # Format code
cargo clippy            # Lint for improvements
```

### Using Cargo Extensions
```bash
# Install cargo-watch (rebuilds on file change)
cargo install cargo-watch
cargo watch -x run

# Install cargo-expand (see macro expansions)
cargo install cargo-expand
cargo expand
```

## Testing

### Manual Testing
```bash
cargo run

# Test controls:
# W - Move up
# A - Move left
# S - Move down
# D - Move right
# Q - Quit game
```

### Unit Tests (if implemented)
```bash
cargo test
```

### Build Validation
```bash
cargo check
cargo build
```

## Performance Profiling

### Debug Build Performance
```bash
cargo build
time ./target/debug/Mnau
```

### Release Build Performance
```bash
cargo build --release
time ./target/release/Mnau
```

## Code Quality

### Format Code
```bash
cargo fmt
```

### Check for Issues
```bash
cargo clippy
```

### Generate Documentation
```bash
cargo doc --open
```

## Version Control Best Practices

### Before Committing
```bash
# Check what changed
git status
git diff

# Only stage intended files
git add src/main.rs src/cat.rs

# Review staged changes
git diff --staged
```

### Commit Messages
```bash
# Good
git commit -m "feat: add fish collection system"
git commit -m "fix: correct cat boundary collision"

# Avoid
git commit -m "update"
git commit -m "fix stuff"
```

### Pushing Changes
```bash
git push origin branch-name
# Always push to feature branches first
# Merge to main only after review
```

## Documentation

### Generate Docs
```bash
cargo doc --no-deps --open
# Opens generated Rust documentation
```

### Writing Comments
```rust
/// Public API documentation
/// This appears in generated docs
pub fn my_function() {}

// Implementation comment
// Only visible in source code
```

## Next Steps

After setup:

1. **Play the Game**: `cargo run --release`
2. **Read Architecture**: See [ARCHITECTURE.md](ARCHITECTURE.md)
3. **Explore Code**: Check `src/` directory
4. **Make Changes**: Follow the workflow above
5. **Test Features**: Manual testing via game

## Environment Configuration

### Optional: Update Rust
```bash
rustup update
```

### Optional: Use Nightly
```bash
rustup install nightly
rustup override set nightly
```

### Optional: Set Edition
Already set in Cargo.toml:
```toml
[package]
edition = "2024"
```

## Getting Help

### Rust Documentation
```bash
rustup doc --open
```

### Macroquad Documentation
- https://docs.rs/macroquad/
- https://github.com/not-fl3/macroquad

### Cargo Help
```bash
cargo --help
cargo build --help
```

## Troubleshooting

### Check Current Branch
```bash
git status
# Shows current branch and uncommitted changes
```

### View Git Log
```bash
git log --oneline -10
# Shows recent commits
```

### Reset to Last Commit
```bash
git reset --hard HEAD
# Discards all changes
```

## CI/CD Considerations

### Pre-commit Checklist
```bash
cargo check      # Must pass
cargo fmt        # Code formatting
cargo clippy     # Linting
```

### Before Merging
- Ensure `cargo check` passes
- Manual testing complete
- Code follows project style
- Documentation updated

---

## Quick Reference

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Run | `cargo run` |
| Run (optimized) | `cargo run --release` |
| Check syntax | `cargo check` |
| Format code | `cargo fmt` |
| Lint code | `cargo clippy` |
| Clean build | `cargo clean` |
| New branch | `git checkout -b feature/name` |
| Commit | `git commit -m "message"` |
| Push | `git push origin branch` |

---

For architecture details, see [ARCHITECTURE.md](ARCHITECTURE.md)  
For feature details, see [FEATURES.md](FEATURES.md)
