use egui::{ColorImage, TextureHandle, TextureOptions};
use std::path::Path;

use crate::loader;

/// Information about the currently loaded image.
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub format: String,
    pub size_mb: f64,
}

/// The image viewer state — display, zoom, pan.
pub struct Viewer {
    /// Texture handle for the currently loaded image (egui GPU texture).
    pub texture_id: Option<TextureHandle>,
    /// Metadata about the loaded image.
    pub image_info: Option<ImageInfo>,
    /// Current zoom level (1.0 = 100%).
    pub zoom: f32,
    /// Whether to fit the image to the window.
    pub zoom_fit: bool,
    /// Pan offset in pixels.
    pan: egui::Vec2,
    /// Maximum memory budget for decoded images (500 MB).
    max_memory: u64,
    /// Min/max zoom bounds.
    min_zoom: f32,
    max_zoom: f32,
    zoom_step: f32,
}

impl Default for Viewer {
    fn default() -> Self {
        Self {
            texture_id: None,
            image_info: None,
            zoom: 1.0,
            zoom_fit: true,
            pan: egui::Vec2::ZERO,
            max_memory: 500 * 1024 * 1024, // 500 MB
            min_zoom: 0.01,
            max_zoom: 50.0,
            zoom_step: 0.1,
        }
    }
}

impl Viewer {
    /// Load an image from disk and upload it as a GPU texture.
    pub fn load(&mut self, path: &Path) {
        let result = loader::decode_image(path, self.max_memory);
        match result {
            Ok((image, info)) => {
                // Upload to GPU — will be handled in egui context in the real impl
                // For now, store metadata; texture upload happens in ui()
                self.image_info = Some(info);
                // TODO: actually create egui ColorImage and upload to TextureHandle
                let _ = image; // consumed when texture is created
                self.zoom_fit = true; // Reset zoom on new image
            }
            Err(e) => {
                tracing::warn!("Failed to load {}: {e}", path.display());
                self.image_info = None;
            }
        }
    }

    /// Render the image in the given UI region.
    /// Returns the response for interaction (scroll zoom, drag swipe).
    pub fn ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let available = ui.available_size();

        if let Some(ref texture) = self.texture_id {
            if let Some(ref info) = self.image_info {
                // Calculate display size
                let img_size = egui::Vec2::new(info.width as f32, info.height as f32);

                let display_size = if self.zoom_fit {
                    let scale = (available.x / img_size.x)
                        .min(available.y / img_size.y)
                        .min(1.0);
                    img_size * scale
                } else {
                    img_size * self.zoom
                };

                // Center the image in available space
                let offset = (available - display_size) * 0.5 + self.pan;

                ui.put(
                    egui::Rect::from_min_size(
                        ui.min_rect().min + offset,
                        display_size,
                    ),
                    egui::Image::new(texture).fit_to_exact_size(display_size),
                );
            }
        } else {
            // No image loaded — placeholder
            ui.centered_and_justified(|ui| {
                ui.label("No image");
            });
        }

        // Return the full area as interactive for scrolling/dragging
        ui.interact(ui.max_rect(), ui.next_auto_id(), egui::Sense::click_and_drag())
    }

    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom + self.zoom_step).min(self.max_zoom);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom - self.zoom_step).max(self.min_zoom);
    }
}
