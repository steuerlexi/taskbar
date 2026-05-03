use crate::config::AppConfig;
use crate::macos;
use objc2_app_kit::NSWorkspace;
use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub config: Mutex<AppConfig>,
}

#[tauri::command]
pub fn get_running_apps() -> Vec<macos::RunningApp> {
    macos::get_running_apps()
}

#[tauri::command]
pub fn get_frontmost_app() -> Option<macos::RunningApp> {
    macos::get_frontmost_app()
}

#[tauri::command]
pub fn launch_app(bundle_id: String) -> bool {
    macos::launch_app(&bundle_id)
}

#[tauri::command]
pub fn quit_app(bundle_id: String) -> bool {
    let apps = crate::macos::get_running_apps();
    if let Some(app) = apps.iter().find(|a| a.bundle_id == bundle_id) {
        let workspace = NSWorkspace::sharedWorkspace();
        let running_apps = workspace.runningApplications();
        let count = running_apps.count();
        for i in 0..count {
            let running_app = running_apps.objectAtIndex(i);
            if running_app.processIdentifier() == app.pid {
                let _ = running_app.terminate();
                return true;
            }
        }
    }
    false
}

#[tauri::command]
pub fn show_in_finder(app_name: String) {
    let _ = std::process::Command::new("open")
        .args(["-R", &format!("/Applications/{}.app", app_name)])
        .spawn();
}

#[tauri::command]
pub fn get_config(state: State<AppState>) -> AppConfig {
    state.config.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_config(state: State<AppState>, config: AppConfig) {
    let mut current = state.config.lock().unwrap();
    *current = config.clone();
    crate::config::save_config(&config);
}