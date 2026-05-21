use serde_json::json;
use std::sync::Mutex;
use std::time::Duration;
use system_idle_time::get_idle_time;
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_store::StoreExt;

use crate::sys;

pub struct AppState {
    timeout_minutes: Mutex<u64>,
    action: Mutex<String>,
    is_paused: Mutex<bool>,
}

impl AppState {
    pub fn new(app: &AppHandle) -> Self {
        let mut timeout = 5;
        let mut action = "sleep".to_string();
        let mut is_paused = false;

        if let Ok(store) = app.store("settings.json") {
            if let Some(v) = store.get("timeout_minutes").and_then(|v| v.as_u64()) {
                timeout = v;
            }

            if let Some(v) = store.get("is_paused").and_then(|v| v.as_bool()) {
                is_paused = v;
            }

            if let Some(v) = store.get("action") {
                if let Some(s) = v.as_str() {
                    action = s.to_string();
                }
            }
        }

        Self {
            timeout_minutes: Mutex::new(timeout),
            action: Mutex::new(action),
            is_paused: Mutex::new(is_paused),
        }
    }
}

#[derive(serde::Serialize)]
pub struct SettingsPayload {
    timeout: u64,
    action: String,
    paused: bool,
}

#[tauri::command]
pub fn get_settings(state: tauri::State<AppState>) -> SettingsPayload {
    SettingsPayload {
        timeout: *state.timeout_minutes.lock().unwrap(),
        action: state.action.lock().unwrap().clone(),
        paused: *state.is_paused.lock().unwrap(),
    }
}

#[tauri::command]
pub fn update_settings(
    app: tauri::AppHandle,
    state: tauri::State<AppState>,
    timeout: u64,
    action: String,
    paused: bool,
) {
    *state.timeout_minutes.lock().unwrap() = timeout;
    *state.action.lock().unwrap() = action.clone();
    *state.is_paused.lock().unwrap() = paused;

    if let Ok(store) = app.store("settings.json") {
        store.set("timeout_minutes", json!(timeout));
        store.set("action", json!(action));
        store.set("is_paused", json!(paused));
        let _ = store.save();
    }
}

pub fn init_background_worker(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let mut last_idle = 0;

        loop {
            std::thread::sleep(Duration::from_secs(1));

            let state = app_handle.state::<AppState>();

            if *state.is_paused.lock().unwrap() {
                continue;
            }

            let idle_secs = match get_idle_time() {
                Ok(time) => time.as_secs(),
                Err(_) => 0,
            };

            let timeout_secs = *state.timeout_minutes.lock().unwrap() * 60;
            println!(
                "state: idle_secs={}, timeout_secs={}, last_idle={}",
                idle_secs, timeout_secs, last_idle
            );

            if idle_secs >= timeout_secs && last_idle < timeout_secs {
                let action = state.action.lock().unwrap().clone();
                match action.as_str() {
                    "sleep" => sys::sleep(),
                    "lock" => sys::lock(),
                    "lock_and_sleep" => {
                        sys::lock();
                        std::thread::sleep(Duration::from_secs(2));
                        sys::sleep();
                    }

                    "shutdown" => sys::shutdown(),
                    _ => {}
                }
            }

            if last_idle > 120 && idle_secs < 5 {
                let minutes_away = last_idle / 60;

                let _ = app_handle
                    .notification()
                    .builder()
                    .title("Welcome back!")
                    .body(format!("You were away for {} minutes.", minutes_away))
                    .show();
            }

            last_idle = idle_secs;
        }
    });
}
