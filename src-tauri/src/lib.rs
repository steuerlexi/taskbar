mod commands;
mod config;
mod events;
mod macos;
mod widgets;

pub use commands::AppState;
pub use config::load_config;

use std::sync::Mutex;
use tauri::{Manager, WebviewUrl};
use tauri_nspanel::{
    tauri_panel, CollectionBehavior, PanelBuilder, PanelLevel, StyleMask, WebviewWindowExt,
};

// Define the taskbar panel class
tauri_panel! {
    panel!(TaskbarPanel {
        config: {
            can_become_key_window: false,
            can_become_main_window: false,
            becomes_key_only_if_needed: true,
            is_floating_panel: true
        }
    })
}

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
                let label = format!("taskbar-{}", i);
                let scale = monitor.scale_factor();
                let taskbar_height: f64 = 56.0;

                if i == 0 {
                    // First window already exists from Tauri config
                    let window: tauri::WebviewWindow = app.get_webview_window("taskbar-0").unwrap();

                    // Position the window before converting to panel
                    position_window_on_monitor(&window, &monitor, taskbar_height);

                    let panel = window.to_panel::<TaskbarPanel>().unwrap();
                    panel.set_alpha_value(1.0);
                    panel.set_has_shadow(false);
                    panel.set_opaque(false);
                    panel.show();
                } else {
                    // Create additional panels for other monitors using PanelBuilder
                    let panel = PanelBuilder::<_, TaskbarPanel>::new(app.handle(), &label)
                        .url(WebviewUrl::App("index.html".into()))
                        .title("Taskbar")
                        .level(PanelLevel::Floating)
                        .style_mask(StyleMask::empty().borderless())
                        .collection_behavior(
                            CollectionBehavior::new()
                                .can_join_all_spaces()
                                .full_screen_auxiliary(),
                        )
                        .size(tauri::Size::Logical(tauri::LogicalSize::new(
                            monitor.size().width as f64 / scale,
                            taskbar_height,
                        )))
                        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
                            monitor.position().x as f64 / scale,
                            monitor.position().y as f64 / scale
                                + monitor.size().height as f64 / scale
                                - taskbar_height,
                        )))
                        .build()?;
                    panel.set_alpha_value(1.0);
                    panel.set_has_shadow(false);
                    panel.set_opaque(false);
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

fn position_window_on_monitor(window: &tauri::WebviewWindow, monitor: &tauri::Monitor, taskbar_height: f64) {
    let size = monitor.size();
    let position = monitor.position();
    let scale = monitor.scale_factor();

    let y = position.y + size.height as i32 - (taskbar_height * scale) as i32;

    let _ = window.set_position(tauri::PhysicalPosition::new(position.x, y));
    let _ = window.set_size(tauri::PhysicalSize::new(size.width, (taskbar_height * scale) as u32));
}