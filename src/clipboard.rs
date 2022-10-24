use std::fmt::Display;

use egui::{Response, Ui};

#[derive(Debug, Clone)]
pub enum ClipboardItem {
    None,
    Text(String),
    Url(String),
    FilePath(String),
}

impl Display for ClipboardItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClipboardItem::None => write!(f, "None"),
            ClipboardItem::Text(_) => write!(f, "Text"),
            ClipboardItem::Url(_) => write!(f, "URL"),
            ClipboardItem::FilePath(_) => write!(f, "File Path"),
        }
    }
}

impl ClipboardItem {
    pub fn as_egui_response(&self, ui: &mut Ui) -> Response {
        match self {
            ClipboardItem::None => {
                ui.horizontal_centered(|ui| ui.label("There is nothing in the current clipboard"))
                    .response
            }
            ClipboardItem::Text(string) => ui.label(string),
            ClipboardItem::FilePath(string) => ui.link(string),
            ClipboardItem::Url(string) => ui.link(string),
        }
    }
}
