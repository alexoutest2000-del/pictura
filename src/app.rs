use crate::config::Config;
use crate::navigation::FileList;
use crate::viewer::Viewer;

/// Main application state.
pub struct PicturaApp {
    nav: FileList,
    viewer: Viewer,
    config: Config,
    fullscreen: bool,
    slideshow_active: bool,
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
            show_help: false,
        }
    }

    fn toggle_fullscreen(&mut self, ctx: &egui::Context) {
        self.fullscreen = !self.fullscreen;
        // egui/eframe fullscreen toggle
        let viewport = egui::ViewportCommand::Fullscreen(self.fullscreen);
        ctx.send_viewport_cmd(viewport);
    }

    fn toggle_slideshow(&mut self) {
        self.slideshow_active = !self.slideshow_active;
    }

    fn next_image(&mut self) {
        if let Some(path) = self.nav.next() {
            self.viewer.load(&path);
        }
    }

    fn prev_image(&mut self) {
        if let Some(path) = self.nav.previous() {
            self.viewer.load(&path);
        }
    }

    fn load_current(&mut self) {
        if let Some(path) = self.nav.current_path() {
            self.viewer.load(path);
        }
    }

    /// Build the top bar with controls and status.
    fn top_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{} / {}",
                    self.nav.current_index() + 1,
                    self.nav.len()
                ));

                if let Some(ref info) = self.viewer.image_info {
                    ui.separator();
                    ui.label(format!(
                        "{} — {}×{} | {:.1} MB",
                        info.format,
                        info.width,
                        info.height,
                        info.size_mb,
                    ));
                }

                ui.separator();
                ui.label(format!("Zoom: {:.0}%", self.viewer.zoom * 100.0));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⏯").clicked() {
                        self.toggle_slideshow();
                    }
                    if ui.button("⛶").clicked() {
                        self.toggle_fullscreen(ctx);
                    }
                    if ui.button("?").clicked() {
                        self.show_help = !self.show_help;
                    }
                });
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

            // Click/drag area for swipe
            if response.hovered() {
                let drag = response.drag_delta();
                if drag.x > 100.0 {
                    self.prev_image();
                } else if drag.x < -100.0 {
                    self.next_image();
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
                ui.label("← →    Previous / Next image");
                ui.label("Scroll  Zoom in / out");
                ui.label("F       Fit to window");
                ui.label("F11     Toggle fullscreen");
                ui.label("Space   Start / stop slideshow");
                ui.label("?       Show / hide this help");
                ui.label("Esc     Quit");
                ui.separator();
                if ui.button("Close").clicked() {
                    self.show_help = false;
                }
            });
    }
}

impl eframe::App for PicturaApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Load current image on first frame
        if self.viewer.texture_id.is_none() && self.nav.current_path().is_some() {
            self.load_current();
        }

        // Slideshow timer
        if self.slideshow_active {
            let interval = self.config.slideshow_interval();
            let now = std::time::Instant::now();
            // Simple elapsed tracking — in real impl, store last_advance in state
            ctx.request_repaint_after(std::time::Duration::from_secs(interval));
            // TODO: advance on timer tick (store last_advance: Instant)
        }

        // Keyboard input
        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowRight) {
                self.next_image();
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
                self.viewer.zoom_fit = !self.viewer.zoom_fit;
            }
            if i.key_pressed(egui::Key::Escape) {
                if self.show_help {
                    self.show_help = false;
                } else {
                    frame.close();
                }
            }
            if i.key_pressed(egui::Key::Question) {
                self.show_help = !self.show_help;
            }
        });

        // UI panels
        if !self.fullscreen {
            self.top_bar(ctx, frame);
        }
        self.central_panel(ctx);
        if self.show_help {
            self.help_overlay(ctx);
        }
    }
}
