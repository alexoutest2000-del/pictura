use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration, loaded from ~/.config/pictura/config.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub slideshow: SlideshowConfig,

    #[serde(default)]
    pub viewer: ViewerConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SlideshowConfig {
    /// Interval between slides in seconds (1–86400).
    #[serde(default = "default_slideshow_interval")]
    pub interval_seconds: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ViewerConfig {
    /// Zoom multiplier per scroll step.
    #[serde(default = "default_zoom_step")]
    pub zoom_step: f32,

    /// Background color in hex (e.g., "#1a1a1a").
    #[serde(default = "default_background")]
    pub background_color: String,

    /// Fit image to window when opening a new file.
    #[serde(default = "default_true")]
    pub fit_on_open: bool,
}

fn default_slideshow_interval() -> u64 {
    5
}
fn default_zoom_step() -> f32 {
    0.1
}
fn default_background() -> String {
    "#1a1a1a".into()
}
fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            slideshow: SlideshowConfig {
                interval_seconds: default_slideshow_interval(),
            },
            viewer: ViewerConfig {
                zoom_step: default_zoom_step(),
                background_color: default_background(),
                fit_on_open: default_true(),
            },
        }
    }
}

impl Config {
    pub fn slideshow_interval(&self) -> u64 {
        self.slideshow
            .interval_seconds
            .clamp(1, 86400)
    }
}

/// Load config from the standard location, or return defaults.
pub fn load() -> anyhow::Result<Config> {
    let config_path = config_path()?;

    if config_path.exists() {
        let content = std::fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    } else {
        // Return defaults — write them out for the user to customize
        let default = Config::default();
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&config_path, toml::to_string_pretty(&default)?)?;
        Ok(default)
    }
}

fn config_path() -> anyhow::Result<PathBuf> {
    let base = dirs_next().ok_or_else(|| {
        anyhow::anyhow!("Could not determine config directory")
    })?;
    Ok(base.join("pictura").join("config.toml"))
}

fn dirs_next() -> Option<PathBuf> {
    // ~/.config on Linux
    std::env::var("XDG_CONFIG_HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var("HOME")
                .ok()
                .map(|h| PathBuf::from(h).join(".config"))
        })
}
