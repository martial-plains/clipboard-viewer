use egui::{Response, Ui};

use egui_extras::RetainedImage;

pub use libclipboard::Clipboard;
use pdf::file::File;

use crate::utils::open_with_default;

pub mod macos;

pub enum EguiClipboardItem {
    Text(String),
    FilePath(String),
    Url(String),
    Png(RetainedImage),
    Tiff(RetainedImage),
    Html(String),
    UnicodeText(String),
    Rtf(String),
    Rtfd(String),
    Pdf(File<Vec<u8>>),
}

impl std::fmt::Debug for EguiClipboardItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text(_) => f.debug_tuple("Text").finish(),
            Self::FilePath(_) => f.debug_tuple("FilePath").finish(),
            Self::Url(_) => f.debug_tuple("Url").finish(),
            Self::Png(_) => f.debug_tuple("Png").finish(),
            Self::Tiff(_) => f.debug_tuple("Tiff").finish(),
            Self::Html(_) => f.debug_tuple("Html").finish(),
            Self::UnicodeText(_) => f.debug_tuple("UnicodeText").finish(),
            Self::Rtf(_) => f.debug_tuple("Rtf").finish(),
            Self::Rtfd(_) => f.debug_tuple("Rtfd").finish(),
            Self::Pdf(_) => f.debug_tuple("Pdf").finish(),
        }
    }
}

impl EguiClipboardItem {
    pub fn get_clipboard_item(clipboard: &Clipboard) -> Option<EguiClipboardItem> {
        let item = clipboard.get_item()?;

        Some(match item {
            libclipboard::ClipboardItem::Html(html) => Self::Html(html),
            libclipboard::ClipboardItem::Text(text) => Self::Text(text),
            libclipboard::ClipboardItem::UnicodeText(text) => Self::UnicodeText(text),
            libclipboard::ClipboardItem::Rtf(rtf) => Self::Rtf(rtf),
            libclipboard::ClipboardItem::Rtfd(rtfd) => Self::Rtfd(rtfd),
            libclipboard::ClipboardItem::Url(url) => Self::Url(url),
            libclipboard::ClipboardItem::FilePath(file_path) => Self::FilePath(file_path),
            libclipboard::ClipboardItem::Png(png) => {
                cfg_if::cfg_if! {
                    if #[cfg(target_os = "macos")] {
                        Self::Png(macos::get_png_from_clipboard_for_macos()?)
                    } else {
                        return None
                    }
                }
            }
            libclipboard::ClipboardItem::Tiff(tiff) => {
                cfg_if::cfg_if! {
                    if #[cfg(target_os = "macos")] {
                        Self::Tiff(macos::get_tiff_from_clipboard_for_macos()?)
                    } else {
                        return None
                    }
                }
            }
            libclipboard::ClipboardItem::Pdf(pdf) => Self::Pdf(pdf),
        })
    }
}

impl EguiClipboardItem {
    pub fn as_egui_response(&self, ctx: &egui::Context, ui: &mut Ui) -> Response {
        match self {
            Self::Text(string) => ui.label(string),
            Self::FilePath(string) => {
                ui.scope(|ui| {
                    if ui.link(string).clicked() {
                        open_with_default(string)
                    }
                })
                .response
            }
            Self::Url(string) => {
                ui.scope(|ui| {
                    if ui.link(string).clicked() {
                        open_with_default(string)
                    }
                })
                .response
            }
            Self::Png(image) => image.show_max_size(ui, ctx.available_rect().size()),
            Self::Tiff(image) => image.show_max_size(ui, ctx.available_rect().size()),
            Self::Html(html) => ui.label(html),
            Self::UnicodeText(_) => todo!(),
            Self::Rtf(_) => todo!(),
            Self::Rtfd(_) => todo!(),
            Self::Pdf(_) => todo!(),
        }
    }
}
