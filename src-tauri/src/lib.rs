use tauri::{AppHandle, Manager};

mod state;
mod sys;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(state::AppState::new(app.handle()));
            state::init_background_worker(app.handle().clone());
            start_minimized(app.handle());
            enable_devtools(app.handle());
            tray::create_tray(app)?;
            prevent_window_termination(app.handle());

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            sleep,
            lock,
            lock_and_sleep,
            shutdown,
            state::get_settings,
            state::update_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn start_minimized(app: &AppHandle) {
    let args: Vec<String> = std::env::args().collect();

    if !args.contains(&"--minimized".to_string()) {
        if let Some(window) = app.get_webview_window("main") {
            window.show().unwrap();
            window.set_focus().unwrap();
        }
    }
}

fn enable_devtools(app: &AppHandle) {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--devtools".to_string()) {
        if let Some(window) = app.get_webview_window("main") {
            window.open_devtools();
        }
    }
}

fn prevent_window_termination(app: &AppHandle) {
    if let Some(main_window) = app.get_webview_window("main") {
        let window_clone = main_window.clone();

        main_window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                window_clone.hide().unwrap();
            }
        });
    }
}

#[tauri::command]
fn sleep() {
    sys::sleep();
}

#[tauri::command]
fn lock() {
    sys::lock();
}

#[tauri::command]
fn lock_and_sleep() {
    sys::lock();
    std::thread::sleep(std::time::Duration::from_secs(2));
    sys::sleep();
}

#[tauri::command]
fn shutdown() {
    sys::shutdown();
}
