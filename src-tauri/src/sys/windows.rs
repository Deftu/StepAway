use std::process::Command;
use windows_sys::Win32::{
    System::Shutdown::LockWorkStation,
    UI::WindowsAndMessaging::{SendMessageW, HWND_BROADCAST, SC_MONITORPOWER, WM_SYSCOMMAND},
};

pub fn platform_sleep() {
    unsafe {
        SendMessageW(
            HWND_BROADCAST,
            WM_SYSCOMMAND,
            SC_MONITORPOWER as usize,
            2_isize,
        );
    }
}

pub fn platform_lock() {
    unsafe {
        LockWorkStation();
    }
}

pub fn platform_shutdown() {
    let _ = Command::new("shutdown").args(["/s", "/t", "0"]).status();
}
