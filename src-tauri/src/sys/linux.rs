use std::env;
use std::process::Command;

fn execute_first_available(commands: &[&[&str]]) -> Result<(), String> {
    for cmd_parts in commands {
        if let Some((executable, args)) = cmd_parts.split_first() {
            match Command::new(executable).args(args).status() {
                Ok(status) if status.success() => {
                    return Ok(());
                }
                _ => {
                    // Either it doesn't exist or it failed because the daemon isn't running.
                    // We just swallow the error and try the next one in the array.
                    continue;
                }
            }
        }
    }

    Err("No compatible power management backend found in current environment".into())
}

pub fn platform_sleep() {
    let _ = execute_first_available(&[
        // Standard systemd
        &["systemctl", "suspend"],
        // elogind
        &["loginctl", "suspend"],
        // XFCE / generic DBus fallback
        &[
            "dbus-send",
            "--system",
            "--print-reply",
            "--dest=org.freedesktop.login1",
            "/org/freedesktop/login1",
            "org.freedesktop.login1.Manager.Suspend",
            "boolean:true",
        ],
    ]);
}

pub fn platform_lock() {
    let _ = execute_first_available(&[
        // systemd / elogind session lock
        &["loginctl", "lock-session"],
        // Cross-desktop standard (requires xdg-utils)
        &["xdg-screensaver", "lock"],
        // GNOME specific
        &["gnome-screensaver-command", "-l"],
        // KDE specific
        &[
            "qdbus",
            "org.freedesktop.ScreenSaver",
            "/ScreenSaver",
            "Lock",
        ],
        // Barebones X11 window managers (i3, sway, dwm)
        &["slock"],
        &["i3lock"],
    ]);
}

pub fn platform_shutdown() {
    let _ = execute_first_available(&[
        // Standard systemd
        &["systemctl", "poweroff"],
        // elogind
        &["loginctl", "poweroff"],
        // Classic POSIX (Might require root/sudo depending on the distro's polkit rules)
        &["shutdown", "-h", "now"],
    ]);
}
