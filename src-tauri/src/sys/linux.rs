use std::env;
use std::process::Command;

pub fn platform_sleep() {
    let session_type = env::var("XDG_SESSION_TYPE")
        .unwrap_or_default()
        .to_lowercase();

    if session_type == "wayland" {
        // Sway / wlroots
        if Command::new("swaymsg")
            .args(["output", "*", "dpms", "off"])
            .status()
            .is_ok()
        {
            return;
        }

        // Hyprland
        if Command::new("hyprctl")
            .args(["dispatch", "dpms", "off"])
            .status()
            .is_ok()
        {
            return;
        }

        // KDE Plasma (via qdbus)
        if Command::new("qdbus")
            .args([
                "org.kde.Solid.PowerManagement",
                "/org/kde/Solid/PowerManagement/Actions/DPMSControl",
                "org.kde.Solid.PowerManagement.Actions.DPMSControl.turnOff",
            ])
            .status()
            .is_ok()
        {
            return;
        }

        eprintln!("Unsupported Wayland compositor. Could not sleep displays.");
    } else {
        // Fallback for X11
        let _ = Command::new("xset").args(["dpms", "force", "off"]).status();
    }
}

pub fn platform_lock() {
    let _ = Command::new("loginctl").arg("lock-session").status();
}

pub fn platform_shutdown() {
    let _ = Command::new("systemctl").arg("poweroff").status();
}
