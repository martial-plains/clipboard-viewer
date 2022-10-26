use std::slice;

use egui_extras::RetainedImage;
use rust_macios::{
    appkit::{
        NSPasteboard, NSPasteboardTypeColor, NSPasteboardTypeFileURL, NSPasteboardTypeFont,
        NSPasteboardTypeHTML, NSPasteboardTypePDF, NSPasteboardTypePNG, NSPasteboardTypeRTF,
        NSPasteboardTypeRTFD, NSPasteboardTypeRuler, NSPasteboardTypeSound, NSPasteboardTypeString,
        NSPasteboardTypeTIFF, NSPasteboardTypeTabularText, NSPasteboardTypeURL,
    },
    foundation::{NSData, NSString},
};

use crate::clipboard::ClipboardItem;

static mut CHANGE_COUNT: i64 = 0;

#[derive(Debug)]
enum PasteType {
    Url,
    Color,
    FileURL,
    Font,
    Html,
    MultipleTextSelection,
    Pdf,
    Png,
    Rtf,
    Rtfd,
    Ruler,
    Sound,
    String,
    TabularText,
    Tiff,
}

impl From<NSString> for PasteType {
    fn from(value: NSString) -> Self {
        match value.to_string().as_str() {
            "public.url" => PasteType::Url,
            "com.apple.cocoa.pasteboard.color" => PasteType::Color,
            "public.file-url" => PasteType::FileURL,
            "com.apple.cocoa.pasteboard.character-formatting" => PasteType::Font,
            "public.html" => PasteType::Html,
            "com.apple.cocoa.pasteboard.multiple-text-selection" => {
                PasteType::MultipleTextSelection
            }
            "com.adobe.pdf" => PasteType::Pdf,
            "public.png" => PasteType::Png,
            "public.rtf" => PasteType::Rtf,
            "com.apple.flat-rtfd" => PasteType::Rtfd,
            "com.apple.cocoa.pasteboard.paragraph-formatting" => PasteType::Ruler,
            "com.apple.cocoa.pasteboard.sound" => PasteType::Sound,
            "public.utf8-plain-text" => PasteType::String,
            "public.utf8-tab-separated-values-text" => PasteType::TabularText,
            "public.tiff" => PasteType::Tiff,
            _ => PasteType::String,
        }
    }
}

fn get_url_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeURL.clone())?
                .to_string(),
        )
    }
}

fn get_file_url_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeFileURL.clone())?
                .to_string(),
        )
    }
}

fn get_string_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeString.clone())?
                .to_string(),
        )
    }
}

fn get_ruler_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeRuler.clone())?
                .to_string(),
        )
    }
}

fn get_sound_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeSound.clone())?
                .to_string(),
        )
    }
}

fn get_font_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeFont.clone())?
                .to_string(),
        )
    }
}

fn get_color_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeColor.clone())?
                .to_string(),
        )
    }
}

fn get_rtf_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeRTF.clone())?
                .to_string(),
        )
    }
}

fn get_rtfd_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeRTFD.clone())?
                .to_string(),
        )
    }
}

fn get_tabular_text_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeTabularText.clone())?
                .to_string(),
        )
    }
}

fn get_multiple_text_selection_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeTabularText.clone())?
                .to_string(),
        )
    }
}

fn get_html_from_clipboard() -> Option<String> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        Some(
            pasteboard
                .string_for_type(NSPasteboardTypeHTML.clone())?
                .to_string(),
        )
    }
}

fn get_png_from_clipboard() -> Option<RetainedImage> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        RetainedImage::from_image_bytes(
            "png_clipboard_bytes",
            nsdata_as_bytes(pasteboard.data_for_type(NSPasteboardTypePNG.clone())?),
        )
        .ok()
    }
}

fn get_tiff_from_clipboard() -> Option<RetainedImage> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        RetainedImage::from_image_bytes(
            "tiff_clipboard_bytes",
            nsdata_as_bytes(pasteboard.data_for_type(NSPasteboardTypeTIFF.clone())?),
        )
        .ok()
    }
}

fn get_pdf_from_clipboard() -> Option<RetainedImage> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        RetainedImage::from_image_bytes(
            "pdf_clipboard_bytes",
            nsdata_as_bytes(pasteboard.data_for_type(NSPasteboardTypePDF.clone())?),
        )
        .ok()
    }
}

pub fn get_clipboard_item() -> Option<ClipboardItem> {
    let pasteboard = NSPasteboard::general_pasteboard();
    let pasteboard_item = pasteboard.types();
    let content_type = pasteboard.available_type_from_array(pasteboard_item?)?;

    let pastetype: PasteType = content_type.into();

    Some(match pastetype {
        PasteType::Url => ClipboardItem::Url(get_url_from_clipboard()?),
        PasteType::Color => ClipboardItem::Text(get_color_from_clipboard()?),
        PasteType::FileURL => ClipboardItem::FilePath(get_file_url_from_clipboard()?),
        PasteType::Font => ClipboardItem::Text(get_font_from_clipboard()?),
        PasteType::Html => ClipboardItem::Text(get_html_from_clipboard()?),
        PasteType::MultipleTextSelection => {
            ClipboardItem::Text(get_multiple_text_selection_from_clipboard()?)
        }
        PasteType::Pdf => ClipboardItem::Tiff(get_pdf_from_clipboard()?),
        PasteType::Png => ClipboardItem::Png(get_png_from_clipboard()?),
        PasteType::Rtf => ClipboardItem::Text(get_rtf_from_clipboard()?),
        PasteType::Rtfd => ClipboardItem::Text(get_rtfd_from_clipboard()?),
        PasteType::Ruler => ClipboardItem::Text(get_ruler_from_clipboard()?),
        PasteType::Sound => ClipboardItem::Text(get_sound_from_clipboard()?),
        PasteType::String => ClipboardItem::Text(get_string_from_clipboard()?),
        PasteType::TabularText => ClipboardItem::Text(get_tabular_text_from_clipboard()?),
        PasteType::Tiff => ClipboardItem::Tiff(get_tiff_from_clipboard()?),
    })
}

fn nsdata_as_bytes<'bytes>(nsdata: NSData) -> &'bytes [u8] {
    let ptr = nsdata.bytes();

    // The bytes pointer may be null for length zero
    let (ptr, len) = if ptr.is_null() {
        (0x1 as *const u8, 0)
    } else {
        (ptr as *const u8, nsdata.length())
    };

    unsafe { slice::from_raw_parts(ptr, len as usize) }
}

pub fn has_clipboard_changed() -> bool {
    let pasteboard = NSPasteboard::general_pasteboard();

    unsafe {
        if CHANGE_COUNT != pasteboard.change_count() {
            CHANGE_COUNT = pasteboard.change_count();
            log::debug!("Clipboard item has changed");
            true
        } else {
            false
        }
    }
}
