#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::ClipboardViewerApp;

mod clipboard;

mod utils;
