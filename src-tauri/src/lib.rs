mod commands;
mod config;
mod events;
mod macos;
mod widgets;

pub use commands::AppState;
pub use config::load_config;

use std::sync::Mutex;
use tauri::{Manager, WebviewWindow};
use tauri_nspanel::{cocoa::appkit::NSWindowCollectionBehavior, WebviewWindowExt};

pub fn run() {
    let app_config = load_config();

    tauri::Builder::default()
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            config: Mutex::new(app_config),
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_running_apps,
            commands::get_frontmost_app,
            commands::launch_app,
            commands::quit_app,
            commands::show_in_finder,
            commands::get_config,
            commands::update_config,
        ])
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let monitors: Vec<_> = app.available_monitors().unwrap().into_iter().collect();

            for (i, monitor) in monitors.iter().enumerate() {
                if i == 0 {
                    // First window already exists from config
                    let window: WebviewWindow = app.get_webview_window("taskbar-0").unwrap();
                    setup_panel(&window);
                    position_window_on_monitor(&window, &monitor);
                    let panel = window.to_panel().unwrap();
                    panel.show();
                } else {
                    // Create additional windows for other monitors
                    let label = format!("taskbar-{}", i);
                    let window = tauri::WebviewWindowBuilder::new(
                        app,
                        &label,
                        tauri::WebviewUrl::App("index.html".into()),
                    )
                    .title("Taskbar")
                    .decorations(false)
                    .transparent(true)
                    .always_on_top(true)
                    .skip_taskbar(true)
                    .visible(false)
                    .resizable(false)
                    .build()?;

                    setup_panel(&window);
                    position_window_on_monitor(&window, &monitor);
                    let panel = window.to_panel().unwrap();
                    panel.show();
                }
            }

            // Start event listeners
            let app_handle = app.handle().clone();
            events::start_listening(app_handle);

            let ah_widgets = app.handle().clone();
            widgets::start_widget_updates(ah_widgets);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup_panel(window: &WebviewWindow) {
    let panel = window.to_panel().unwrap();

    const NS_FLOAT_WINDOW_LEVEL: i32 = 4;
    panel.set_level(NS_FLOAT_WINDOW_LEVEL);

    const NS_WINDOW_STYLE_MASK_NON_ACTIVATING_PANEL: i32 = 1 << 7;
    panel.set_style_mask(NS_WINDOW_STYLE_MASK_NON_ACTIVATING_PANEL);

    panel.set_collection_behaviour(
        NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
            | NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
    );
}

fn position_window_on_monitor(window: &WebviewWindow, monitor: &tauri::Monitor) {
    let size = monitor.size();
    let position = monitor.position();
    let scale = monitor.scale_factor();
    let taskbar_height: f64 = 56.0;

    let y = position.y + size.height as i32 - (taskbar_height * scale) as i32;

    let _ = window.set_position(tauri::PhysicalPosition::new(position.x, y));
    let _ = window.set_size(tauri::PhysicalSize::new(size.width, (taskbar_height * scale) as u32));
}