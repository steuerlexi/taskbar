use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PinnedApp {
    pub bundle_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClockConfig {
    pub enabled: bool,
    pub format: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
    pub clock: ClockConfig,
    pub battery: BatteryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppearanceConfig {
    pub theme: String,
    pub accent_color: String,
    pub opacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub position: String,
    pub height: u32,
    pub show_on_all_monitors: bool,
    pub pinned_apps: Vec<PinnedApp>,
    pub widgets: WidgetConfig,
    pub appearance: AppearanceConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            position: "bottom".to_string(),
            height: 56,
            show_on_all_monitors: true,
            pinned_apps: vec![
                PinnedApp {
                    bundle_id: "com.apple.Safari".to_string(),
                    name: "Safari".to_string(),
                },
                PinnedApp {
                    bundle_id: "com.apple.Terminal".to_string(),
                    name: "Terminal".to_string(),
                },
                PinnedApp {
                    bundle_id: "com.apple.finder".to_string(),
                    name: "Finder".to_string(),
                },
            ],
            widgets: WidgetConfig {
                clock: ClockConfig {
                    enabled: true,
                    format: "HH:mm".to_string(),
                },
                battery: BatteryConfig { enabled: true },
            },
            appearance: AppearanceConfig {
                theme: "auto".to_string(),
                accent_color: "#0078D4".to_string(),
                opacity: 0.85,
            },
        }
    }
}

fn config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("taskbar")
}

fn config_path() -> PathBuf {
    config_dir().join("config.json")
}

pub fn load_config() -> AppConfig {
    let path = config_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => AppConfig::default(),
        }
    } else {
        let config = AppConfig::default();
        save_config(&config);
        config
    }
}

pub fn save_config(config: &AppConfig) {
    let dir = config_dir();
    let _ = fs::create_dir_all(&dir);
    let path = config_path();
    let content = serde_json::to_string_pretty(config).unwrap_or_default();
    let _ = fs::write(path, content);
}