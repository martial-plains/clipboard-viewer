use std::process::Command;

pub fn open_with_default(path: &str) {
    let mut command = if cfg!(target_os = "windows") {
        Command::new("start")
    } else if cfg!(target_os = "macos") {
        Command::new("open")
    } else if cfg!(target_os = "linux") {
        Command::new("xdg-open")
    } else {
        return;
    };

    command
        .arg(path)
        .output()
        .expect("falied to execute process");
}
