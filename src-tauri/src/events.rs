use tauri::{AppHandle, Emitter};

pub fn start_listening(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let mut last_frontmost: Option<String> = None;

        loop {
            if let Some(app) = crate::macos::get_frontmost_app() {
                if last_frontmost.as_ref() != Some(&app.bundle_id) {
                    last_frontmost = Some(app.bundle_id.clone());
                    let _ = app_handle.emit("frontmost-app-changed", &app);
                }
            }

            let running = crate::macos::get_running_apps();
            let _ = app_handle.emit("running-apps-update", &running);

            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });
}