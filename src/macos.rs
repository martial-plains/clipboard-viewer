use rust_macios::{
    appkit::{
        NSPasteboard, NSPasteboardTypeFileURL, NSPasteboardTypeHTML, NSPasteboardTypeString,
        NSPasteboardTypeURL,
    },
    foundation::NSString,
};

use crate::clipboard::ClipboardItem;

#[derive(Debug)]
enum PasteType {
    URL,
    Color,
    FileURL,
    Font,
    HTML,
    MultipleTextSelection,
    PDF,
    PNG,
    RTF,
    RTFD,
    Ruler,
    Sound,
    String,
    TabularText,
    TIFF,
}

impl From<NSString> for PasteType {
    fn from(value: NSString) -> Self {
        match value.to_string().as_str() {
            "public.url" => PasteType::URL,
            "com.apple.cocoa.pasteboard.color" => PasteType::Color,
            "public.file-url" => PasteType::FileURL,
            "com.apple.cocoa.pasteboard.character-formatting" => PasteType::Font,
            "public.html" => PasteType::HTML,
            "com.apple.cocoa.pasteboard.multiple-text-selection" => {
                PasteType::MultipleTextSelection
            }
            "com.adobe.pdf" => PasteType::PDF,
            "public.png" => PasteType::PNG,
            "public.rtf" => PasteType::RTF,
            "com.apple.flat-rtfd" => PasteType::RTFD,
            "com.apple.cocoa.pasteboard.paragraph-formatting" => PasteType::Ruler,
            "com.apple.cocoa.pasteboard.sound" => PasteType::Sound,
            "public.utf8-plain-text" => PasteType::String,
            "public.utf8-tab-separated-values-text" => PasteType::TabularText,
            "public.tiff" => PasteType::TIFF,
            _ => PasteType::String,
        }
    }
}

pub fn get_url_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeURL.clone())?
                .to_string(),
        )
    }
}

pub fn get_file_url_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeFileURL.clone())?
                .to_string(),
        )
    }
}

pub fn get_string_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeString.clone())?
                .to_string(),
        )
    }
}

pub fn get_html_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeHTML.clone())?
                .to_string(),
        )
    }
}

pub fn get_clipboard_item() -> Option<ClipboardItem> {
    let pasteboard = NSPasteboard::general_pasteboard();
    let pasteboard_item = pasteboard.types();
    let content_type = pasteboard.available_type_from_array(pasteboard_item?)?;

    let pastetype: PasteType = content_type.into();

    Some(match pastetype {
        PasteType::URL => ClipboardItem::Url(get_url_from_clipboard()?),
        PasteType::Color => todo!(),
        PasteType::FileURL => ClipboardItem::FilePath(get_file_url_from_clipboard()?),
        PasteType::Font => todo!(),
        PasteType::HTML => ClipboardItem::Text(get_html_from_clipboard()?),
        PasteType::MultipleTextSelection => todo!(),
        PasteType::PDF => todo!(),
        PasteType::PNG => todo!(),
        PasteType::RTF => todo!(),
        PasteType::RTFD => todo!(),
        PasteType::Ruler => todo!(),
        PasteType::Sound => todo!(),
        PasteType::String => ClipboardItem::Text(get_string_from_clipboard()?),
        PasteType::TabularText => todo!(),
        PasteType::TIFF => todo!(),
    })
}
