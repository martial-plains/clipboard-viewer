#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Clipboard Viewer",
        native_options,
        Box::new(|cc| Box::new(clipboard_viewer::ClipboardViewerApp::new(cc))),
    );
}
