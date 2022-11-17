use std::{collections::HashMap, fmt::format, fs, io::Read};

use egui::{Response, RichText, Ui};

use egui_extras::RetainedImage;

use image::io::Reader;
pub use libclipboard::Clipboard;
use pdf::{build::PageBuilder, file::File};

use crate::utils::open_with_default;

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
            libclipboard::ClipboardItem::Png(mut png) => {
                cfg_if::cfg_if! {
                    if #[cfg(target_os = "macos")] {
                            let mut bytes = Vec::new();
                            png.read_to_end(&mut bytes).ok()?;
                            Self::Png( RetainedImage::from_image_bytes(
                                "png_clipboard_bytes",
                                &bytes,
                            )
                            .ok()?)

                    } else {
                        return None
                    }
                }
            }
            libclipboard::ClipboardItem::Tiff(mut tiff) => {
                cfg_if::cfg_if! {
                    if #[cfg(target_os = "macos")] {
                        let mut bytes = vec![];
                            tiff.read_to_end(&mut bytes).ok()?;
                            Self::Png( RetainedImage::from_image_bytes(
                                "png_clipboard_bytes",
                                &bytes,
                            )
                            .ok()?)
                    } else {
                        return None
                    }
                }
            }
            libclipboard::ClipboardItem::Pdf(mut pdf) => {
                let mut bytes = vec![];
                pdf.read_to_end(&mut bytes).ok()?;
                Self::Pdf(File::from_data(bytes).ok()?)
            }
            libclipboard::ClipboardItem::RawBytes(_) => todo!(),
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
            Self::UnicodeText(string) => ui.label(string),
            Self::Rtf(rtf) => ui.label(rtf),
            Self::Rtfd(rtfd) => ui.label(rtfd),
            Self::Pdf(pdf) => {
                ui.scope(|ui| {
                    if let Some(ref info) = pdf.trailer.info_dict {
                        ui.label(format!("{:#?}", info));
                        let title = info.get("Title").and_then(|p| p.to_string_lossy().ok());
                        let author = info.get("Author").and_then(|p| p.to_string_lossy().ok());

                        match (title, author) {
                            (Some(title), None) => ui.heading(RichText::new(title).heading()),
                            (None, Some(author)) => {
                                ui.heading(RichText::new(author).heading().small_raised())
                            }
                            (Some(title), Some(author)) => {
                                ui.scope(|ui| {
                                    ui.heading(RichText::new(title).heading());
                                    ui.heading(RichText::new(author).heading().small_raised());
                                })
                                .response
                            }
                            _ => ui.label("PDF"),
                        };
                    }
                })
                .response
            }
        }
    }
}
