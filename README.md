# pictura

A fast, minimal image viewer for Linux. Pure Rust, zero system dependencies — one command to build, one command to run.

## Features

- **PNG & JPEG** support (baseline + progressive, magic-byte detection)
- **Zoom** in/out with scroll wheel, fit-to-window toggle
- **Navigate** with arrow keys or mouse drag (swipe left/right)
- **Slideshow** with configurable interval in seconds
- **Fullscreen** mode (F11 toggles chrome)
- **Memory-safe** — refuses images over 500MB decoded size

## Quick Start

```bash
# 1. Clone
git clone https://github.com/alexoutest2000-del/pictura.git
cd pictura

# 2. Build
cargo build --release

# 3. Run
./target/release/pictura ~/Pictures
```

That's it. No `apt install`, no system libraries, no C toolchain needed.

## Requirements

| What | Minimum |
|---|---|
| Rust | 1.75+ (install: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh`) |
| OS | Linux with X11 or Wayland |
| Disk | ~200MB for build artifacts |

No other dependencies. The binary bundles everything.

## Installing Rust (if needed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Restart your shell or run: source "$HOME/.cargo/env"
rustc --version  # should print 1.75+ or higher
```

## Usage

```bash
# Open a directory (scans recursively)
pictura ~/Pictures

# Open a specific image
pictura ~/Pictures/photo.png

# Start slideshow immediately (5 second interval)
pictura --slideshow 5 ~/Pictures

# Open current directory
pictura
```

## Keybindings

| Key | Action |
|---|---|
| `←` `→` | Previous / Next image |
| `Scroll ↑↓` | Zoom in / out |
| `F` | Fit to window (reset zoom) |
| `F11` | Toggle fullscreen |
| `Space` | Start / Stop slideshow |
| `H` | Show keybindings overlay |
| `Esc` | Close overlay / Quit |

## Configuration

On first run, pictura creates `~/.config/pictura/config.toml` with defaults:

```toml
[slideshow]
interval_seconds = 5      # 1–86400

[viewer]
zoom_step = 0.1           # Zoom multiplier per scroll tick
background_color = "#1a1a1a"
fit_on_open = true        # Fit image to window on load
```

Edit the file and restart — no recompilation needed.

## Development

```bash
# Check + lint
cargo check
cargo clippy -- -D warnings
cargo fmt --check

# Run tests
cargo test

# Run benchmarks (decode speed)
cargo bench

# Watch mode (auto-rebuild on changes)
cargo watch -x run
```

## Troubleshooting

**"cargo: command not found"**
→ Rust is not installed. See [Installing Rust](#installing-rust-if-needed) above.

**"No supported images found"**
→ The directory has no PNG or JPEG files (case-insensitive: .png, .jpg, .jpeg). Try a different directory.

**"Image too large"**
→ The image exceeds the 500MB decoded memory budget. This is a safety limit.

**Window opens but is blank/black**
→ The first image may have failed to decode. Check terminal output for error messages.

**"error: linker 'cc' not found"**
→ You need a C linker for some Rust crates:
```bash
sudo apt install build-essential   # Debian/Ubuntu
sudo dnf install gcc               # Fedora
```

**Wayland issues**
→ egui uses winit which supports both. If you get a blank window on Wayland, set:
```bash
export WINIT_UNIX_BACKEND=x11
```

## Project Structure

```
src/
├── main.rs        # Entry point, CLI args, init
├── app.rs         # Main loop, panels, keybindings, slideshow timer
├── viewer.rs      # Image display, zoom, pan, GPU texture management
├── loader.rs      # File scanning, format detection, image decoding
├── config.rs      # TOML config with defaults
└── navigation.rs  # Ordered file list with wrap-around cursor
```

## License

MIT — see [LICENSE](LICENSE)
