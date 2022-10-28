#[cfg(target_os = "macos")]
use crate::clipboard::{Clipboard, ClipboardItem};
use crate::utils::open_web_link;

#[derive(Default)]
struct Window {
    always_on_top: bool,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ClipboardViewerApp {
    #[serde(skip)]
    window: Window,

    #[cfg(target_os = "macos")]
    #[serde(skip)]
    clipboard_item: Option<ClipboardItem>,
}

impl ClipboardViewerApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Self {
            window: Default::default(),
            #[cfg(target_os = "macos")]
            clipboard_item: super::get_clipboard_item(),
        }
    }
}

impl eframe::App for ClipboardViewerApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        #[cfg(target_os = "macos")]
        {
            if Clipboard::has_changed() {
                self.clipboard_item = super::get_clipboard_item();
            }
        }

        let Self {
            window,
            #[cfg(target_os = "macos")]
            clipboard_item,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                ui.menu_button("Window", |ui| {
                    if ui
                        .checkbox(&mut window.always_on_top, "Stay in Front")
                        .clicked()
                    {
                        frame.set_always_on_top(window.always_on_top);
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("Report issue").clicked() {
                        open_web_link(
                            "https://github.com/a-isaiahharvey/clipboard-viewer/issues/new",
                        );
                    }
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    #[cfg(target_os = "macos")]
                    {
                        if let Some(clipboard_item) = clipboard_item {
                            clipboard_item.as_egui_response(ctx, ui);
                        }
                    }
                });
            // The central panel the region left after adding TopPanel's and SidePanel's
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(target_os = "macos")]
                {
                    if let Some(clipboard_item) = clipboard_item {
                        ui.label(format!("Clipboard contents: {}", clipboard_item));
                    }
                }
            });
        });

        ctx.request_repaint();
    }
}
