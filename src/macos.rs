use rust_macios::appkit::{NSPasteboard, NSPasteboardTypeString};

pub fn get_string_from_clipboard() -> String {
    let pasteboad = NSPasteboard::general_pasteboard();
    let clipboard_item = unsafe { pasteboad.string_for_type(NSPasteboardTypeString.clone()) };
    clipboard_item.to_string()
}
