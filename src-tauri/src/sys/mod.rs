#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

// Fallback for unsupported systems
#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn platform_sleep() {
    eprintln!("Display sleep is not supported on this OS.");
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn platform_lock() {
    eprintln!("Screen lock is not supported on this OS.");
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn platform_shutdown() {
    eprintln!("Screen lock is not supported on this OS.");
}

pub fn sleep() {
    platform_sleep();
}

pub fn lock() {
    platform_lock();
}

pub fn shutdown() {
    platform_shutdown();
}
