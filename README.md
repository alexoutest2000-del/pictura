# pictura

A fast, minimal image viewer for Linux. Written in Rust.

![pictura](docs/screenshot.png) <!-- TODO: add screenshot -->

## Features

- **PNG & JPEG** support (baseline + progressive)
- **Zoom** in/out with scroll wheel, fit-to-window toggle
- **Navigate** with arrow keys or mouse drag (swipe)
- **Slideshow** with configurable interval (in seconds)
- **Fullscreen** mode (F11 toggles chrome)
- **Zero system dependencies** — pure Rust, single binary

## Installation

### From crates.io
```bash
cargo install pictura
```

### From AppImage
Download the latest `.AppImage` from [Releases](https://github.com/user/pictura/releases), make executable, run.

### From source
```bash
git clone https://github.com/user/pictura.git
cd pictura
cargo build --release
./target/release/pictura
```

## Usage

```bash
# Open a directory
pictura ~/Pictures

# Open a specific image
pictura ~/Pictures/photo.png

# Start slideshow immediately
pictura --slideshow 5 ~/Pictures
```

## Keybindings

| Key | Action |
|---|---|
| ← → | Previous / next image |
| Scroll up/down | Zoom in/out |
| F | Fit to window |
| F11 | Toggle fullscreen |
| Space | Start/stop slideshow |
| ? | Show keybindings overlay |
| Esc / Q | Quit |

## Configuration

Config file: `~/.config/pictura/config.toml`

```toml
[slideshow]
interval_seconds = 5

[viewer]
zoom_step = 0.1
background_color = "#1a1a1a"
fit_on_open = true
```

## Development

```bash
cargo build
cargo test
cargo run -- ~/Pictures
```

## License

MIT — see [LICENSE](LICENSE)
