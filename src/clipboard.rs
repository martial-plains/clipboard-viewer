use std::fmt::Display;

use egui::{Response, Ui};
use egui_extras::RetainedImage;

#[cfg(target_os = "macos")]
use crate::macos::has_clipboard_changed;

pub enum ClipboardItem {
    Text(String),
    Url(String),
    FilePath(String),
    Png(RetainedImage),
    Tiff(RetainedImage),
}

impl Display for ClipboardItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardItem::Text(_) => write!(f, "Text"),
            ClipboardItem::Url(_) => write!(f, "URL"),
            ClipboardItem::FilePath(_) => write!(f, "File Path"),
            ClipboardItem::Png(_) => write!(f, "PNG"),
            ClipboardItem::Tiff(_) => write!(f, "TIFF"),
        }
    }
}

impl ClipboardItem {
    pub fn as_egui_response(&self, ctx: &egui::Context, ui: &mut Ui) -> Response {
        match self {
            ClipboardItem::Text(string) => ui.label(string),
            ClipboardItem::FilePath(string) => ui.link(string),
            ClipboardItem::Url(string) => ui.link(string),
            ClipboardItem::Png(image) => image.show_max_size(ui, ctx.available_rect().size()),
            ClipboardItem::Tiff(image) => image.show_max_size(ui, ctx.available_rect().size()),
        }
    }
}

pub struct Clipboard {
    items: Vec<ClipboardItem>,
}

impl Clipboard {
    #[cfg(target_os = "macos")]
    pub fn has_changed() -> bool {
        has_clipboard_changed()
    }
}
