use crate::config::Config;
use crate::navigation::FileList;
use crate::viewer::Viewer;
use std::time::Instant;

/// Main application state.
pub struct PicturaApp {
    nav: FileList,
    viewer: Viewer,
    config: Config,
    fullscreen: bool,
    slideshow_active: bool,
    slideshow_last_advance: Option<Instant>,
    show_help: bool,
}

impl PicturaApp {
    pub fn new(nav: FileList, config: Config) -> Self {
        Self {
            nav,
            viewer: Viewer::default(),
            config,
            fullscreen: false,
            slideshow_active: false,
            slideshow_last_advance: None,
            show_help: false,
        }
    }

    fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        self.fullscreen = !self.fullscreen;
        ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.fullscreen));
    }

    fn toggle_slideshow(&mut self) {
        self.slideshow_active = !self.slideshow_active;
        if self.slideshow_active {
            self.slideshow_last_advance = Some(Instant::now());
        }
    }

    fn advance_image(&mut self) {
        if let Some(path) = self.nav.next() {
            self.viewer.load(path);
        }
        self.slideshow_last_advance = Some(Instant::now());
    }

    fn prev_image(&mut self) {
        if let Some(path) = self.nav.previous() {
            self.viewer.load(path);
        }
        // Reset slideshow timer on manual navigation
        self.slideshow_last_advance = Some(Instant::now());
    }

    /// Update slideshow timer — advances image if interval has elapsed.
    fn tick_slideshow(&mut self) {
        if !self.slideshow_active {
            return;
        }
        let interval = self.config.slideshow_interval();
        if let Some(last) = self.slideshow_last_advance {
            if last.elapsed().as_secs() >= interval {
                self.advance_image();
            }
        } else {
            self.slideshow_last_advance = Some(Instant::now());
        }
    }

    /// Build the top bar with controls and status.
    fn top_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Position counter
                ui.label(format!(
                    "{} / {}",
                    self.nav.current_index() + 1,
                    self.nav.len()
                ));

                // Image info
                if let Some(ref info) = self.viewer.image_info {
                    ui.separator();
                    ui.label(format!(
                        "{} — {}×{} | {:.1} MB",
                        info.format, info.width, info.height, info.size_mb,
                    ));
                }

                // Zoom level
                ui.separator();
                let zoom_label = if self.viewer.zoom_fit {
                    "Fit".to_string()
                } else {
                    format!("{:.0}%", self.viewer.zoom * 100.0)
                };
                ui.label(format!("Zoom: {}", zoom_label));

                // Slideshow status
                if self.slideshow_active {
                    ui.separator();
                    let elapsed = self
                        .slideshow_last_advance
                        .map(|t| t.elapsed().as_secs())
                        .unwrap_or(0);
                    let remain = self.config.slideshow_interval().saturating_sub(elapsed);
                    ui.label(format!("Slideshow: {}s", remain));
                }

                // Right-aligned controls
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        let play_icon = if self.slideshow_active { "⏸" } else { "▶" };
                        if ui.button(play_icon).clicked() {
                            self.toggle_slideshow();
                        }
                        if ui.button("⛶").clicked() {
                            self.toggle_fullscreen(ctx);
                        }
                        if ui.button("ⓘ").clicked() {
                            self.show_help = !self.show_help;
                        }
                    },
                );
            });
        });
    }

    /// Build the central image display area.
    fn central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let response = self.viewer.ui(ui);

            // Scroll to zoom
            if response.hovered() {
                let scroll = ctx.input(|i| i.smooth_scroll_delta);
                if scroll.y > 0.0 {
                    self.viewer.zoom_in();
                } else if scroll.y < 0.0 {
                    self.viewer.zoom_out();
                }
            }

            // Drag to swipe
            if response.dragged() {
                let drag = response.drag_delta();
                if drag.x > 150.0 {
                    self.prev_image();
                } else if drag.x < -150.0 {
                    self.advance_image();
                }
            }
        });
    }

    /// Help overlay (keybindings).
    fn help_overlay(&mut self, ctx: &egui::Context) {
        egui::Window::new("Keybindings")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.heading("Keybindings");
                ui.separator();
                ui.label("← →        Previous / Next image");
                ui.label("Scroll      Zoom in / out");
                ui.label("F           Fit to window");
                ui.label("F11         Toggle fullscreen");
                ui.label("Space       Start / stop slideshow");
                ui.label("H           Show / hide this help");
                ui.label("Esc / Q     Quit");
                ui.separator();
                ui.label(format!(
                    "Slideshow interval: {}s",
                    self.config.slideshow_interval()
                ));
                ui.label("Config: ~/.config/pictura/config.toml");
                ui.separator();
                if ui.button("Close").clicked() {
                    self.show_help = false;
                }
            });
    }
}

impl eframe::App for PicturaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Upload pending texture (first frame or after navigation)
        self.viewer.upload_pending(ctx);

        // Slideshow timer tick
        self.tick_slideshow();

        // Request repaint for slideshow countdown
        if self.slideshow_active {
            ctx.request_repaint_after(std::time::Duration::from_millis(250));
        }

        // Keyboard input
        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowRight) {
                self.advance_image();
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.prev_image();
            }
            if i.key_pressed(egui::Key::F11) {
                self.toggle_fullscreen(ctx);
            }
            if i.key_pressed(egui::Key::Space) {
                self.toggle_slideshow();
            }
            if i.key_pressed(egui::Key::F) {
                self.viewer.reset_zoom();
            }
            if i.key_pressed(egui::Key::Escape) {
                if self.show_help {
                    self.show_help = false;
                } else {
                    // frame.close() — close the window
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
            if i.key_pressed(egui::Key::H) {
                self.show_help = !self.show_help;
            }
        });

        // UI panels
        if !self.fullscreen {
            self.top_bar(ctx);
        }
        self.central_panel(ctx);
        if self.show_help {
            self.help_overlay(ctx);
        }
    }
}
