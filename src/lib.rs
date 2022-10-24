#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

mod clipboard;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::get_clipboard_item;
