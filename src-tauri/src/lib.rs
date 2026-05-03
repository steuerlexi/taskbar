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

            let monitors: Vec<_> = match app.available_monitors() {
                Ok(m) => m.into_iter().collect(),
                Err(e) => {
                    eprintln!("Failed to get monitors: {e}");
                    return Ok(());
                }
            };

            if monitors.is_empty() {
                eprintln!("No monitors found");
                return Ok(());
            }

            let taskbar_height: f64 = 56.0;

            for (i, monitor) in monitors.iter().enumerate() {
                let label = format!("taskbar-{}", i);
                let scale = monitor.scale_factor();
                let size = monitor.size();
                let position = monitor.position();

                // Compute logical coordinates for panel placement
                let logical_width = size.width as f64 / scale;
                let logical_x = position.x as f64 / scale;
                let logical_y = position.y as f64 / scale + size.height as f64 / scale - taskbar_height;

                if i == 0 {
                    // First window already exists from Tauri config — convert it to a panel
                    let window = match app.get_webview_window("taskbar-0") {
                        Some(w) => w,
                        None => {
                            eprintln!("Failed to get primary window");
                            continue;
                        }
                    };

                    // Position the window first (while it's still a normal window)
                    let y = position.y + size.height as i32 - (taskbar_height * scale) as i32;
                    let _ = window.set_position(tauri::PhysicalPosition::new(position.x, y));
                    let _ = window.set_size(tauri::PhysicalSize::new(size.width, (taskbar_height * scale) as u32));

                    let panel = match window.to_panel::<TaskbarPanel>() {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("Failed to convert window to panel: {e}");
                            continue;
                        }
                    };
                    panel.set_alpha_value(1.0);
                    panel.set_has_shadow(false);
                    panel.set_opaque(false);
                    panel.show();
                } else {
                    // Create additional panels for other monitors using PanelBuilder
                    let panel = match PanelBuilder::<_, TaskbarPanel>::new(app.handle(), &label)
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
                            logical_width,
                            taskbar_height,
                        )))
                        .position(tauri::Position::Logical(tauri::LogicalPosition::new(
                            logical_x,
                            logical_y,
                        )))
                        .build()
                    {
                        Ok(p) => p,
                        Err(e) => {
                            eprintln!("Failed to build panel for monitor {i}: {e}");
                            continue;
                        }
                    };
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