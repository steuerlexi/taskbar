use serde::Serialize;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize)]
pub struct ClockData {
    pub time: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatteryData {
    pub percent: i32,
    pub charging: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct WidgetUpdate {
    pub clock: ClockData,
    pub battery: BatteryData,
}

pub fn start_widget_updates(app_handle: AppHandle) {
    let ah = app_handle.clone();
    std::thread::spawn(move || {
        loop {
            let now = chrono::Local::now();
            let update = WidgetUpdate {
                clock: ClockData {
                    time: now.format("%H:%M").to_string(),
                    date: now.format("%d.%m.%Y").to_string(),
                },
                battery: get_battery_data(),
            };
            let _ = ah.emit("widget-update", &update);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}

fn get_battery_data() -> BatteryData {
    let output = std::process::Command::new("pmset")
        .arg("getbattinfo")
        .output();

    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mut percent = 100;
            let mut charging = false;

            for line in stdout.lines() {
                if line.contains("Battery") && line.contains('%') {
                    if let Some(p) = line.split_whitespace().find(|s| s.ends_with('%')) {
                        percent = p.trim_end_matches('%').parse().unwrap_or(100);
                    }
                }
                if line.contains("AC") && line.contains("attached") {
                    charging = true;
                }
            }

            BatteryData { percent, charging }
        }
        Err(_) => BatteryData {
            percent: 100,
            charging: true,
        },
    }
}