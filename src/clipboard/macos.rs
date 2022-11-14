use std::slice;

use egui_extras::RetainedImage;
use rust_macios::{
    appkit::{NSPasteboard, NSPasteboardTypePNG, NSPasteboardTypeTIFF},
    foundation::NSData,
};

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

pub fn get_png_from_clipboard_for_macos() -> Option<RetainedImage> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        RetainedImage::from_image_bytes(
            "png_clipboard_bytes",
            nsdata_as_bytes(pasteboard.data_for_type(NSPasteboardTypePNG.clone())?),
        )
        .ok()
    }
}

pub fn get_tiff_from_clipboard_for_macos() -> Option<RetainedImage> {
    let pasteboard = NSPasteboard::general_pasteboard();
    unsafe {
        RetainedImage::from_image_bytes(
            "tiff_clipboard_bytes",
            nsdata_as_bytes(pasteboard.data_for_type(NSPasteboardTypeTIFF.clone())?),
        )
        .ok()
    }
}
