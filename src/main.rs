//! pictura — A fast, minimal image viewer for Linux.
//!
//! Module overview:
//! - `app`: Main loop, window management, fullscreen toggle
//! - `viewer`: Image display, zoom, pan, fit-to-window
//! - `loader`: File I/O, format detection, image decoding
//! - `config`: TOML config parsing, slideshow timer settings
//! - `navigation`: Ordered file list, next/prev logic, slideshow

mod app;
mod config;
mod loader;
mod navigation;
mod viewer;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

/// A fast, minimal image viewer for Linux.
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Directory or image file to open
    path: Option<String>,

    /// Start slideshow immediately with given interval in seconds
    #[arg(short, long)]
    slideshow: Option<u64>,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("pictura=info")),
        )
        .init();

    let args = Args::parse();
    let config = config::load()?;

    tracing::info!("pictura v{} starting", env!("CARGO_PKG_VERSION"));

    let path = args.path.unwrap_or_else(|| ".".into());
    let files = loader::scan_directory(&path)?;

    if files.is_empty() {
        anyhow::bail!("No supported images found in '{}'", path);
    }

    tracing::info!("Found {} image(s)", files.len());

    let nav = navigation::FileList::new(files);

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("pictura"),
        ..Default::default()
    };

    eframe::run_native(
        "pictura",
        native_options,
        Box::new(|_cc| Ok(Box::new(app::PicturaApp::new(nav, config)))),
    )
    .map_err(|e| anyhow::anyhow!("GUI error: {e}"))?;

    Ok(())
}
