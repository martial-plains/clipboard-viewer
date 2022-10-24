#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;

#[cfg(target_os = "macos")]
mod macos;
