use std::process::Command;

pub fn platform_sleep() {
    let _ = Command::new("pmset").arg("displaysleepnow").status();
}

pub fn platform_lock() {
    let _ = Command::new("osascript")
        .args([
            "-e",
            "tell application \"System Events\" to keystroke \"q\" using {control down, command down}",
        ])
        .status();
}

pub fn platform_shutdown() {
    let _ = Command::new("osascript")
        .args(["-e", "tell app \"System Events\" to shut down"])
        .status();
}
