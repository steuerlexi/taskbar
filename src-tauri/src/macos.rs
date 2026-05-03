use objc2_app_kit::NSWorkspace;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningApp {
    pub bundle_id: String,
    pub name: String,
    pub pid: i32,
    pub is_active: bool,
}

pub fn get_running_apps() -> Vec<RunningApp> {
    let workspace = NSWorkspace::sharedWorkspace();
    let apps = workspace.runningApplications();

    let mut result = Vec::new();
    let count = apps.count();

    for i in 0..count {
        let app = apps.objectAtIndex(i);
        let bundle_id = app
            .bundleIdentifier()
            .map(|s| s.to_string())
            .unwrap_or_default();
        let name = app
            .localizedName()
            .map(|s| s.to_string())
            .unwrap_or_default();

        if bundle_id.is_empty() {
            continue;
        }

        let pid = app.processIdentifier();
        let is_active = app.isActive();

        result.push(RunningApp {
            bundle_id,
            name,
            pid,
            is_active,
        });
    }

    result
}

pub fn launch_app(bundle_id: &str) -> bool {
    // Use `open -b` which is simpler than NSWorkspace launch API
    std::process::Command::new("open")
        .args(["-b", bundle_id])
        .spawn()
        .is_ok()
}

pub fn get_frontmost_app() -> Option<RunningApp> {
    let workspace = NSWorkspace::sharedWorkspace();
    let app = workspace.frontmostApplication()?;

    let bundle_id = app
        .bundleIdentifier()
        .map(|s| s.to_string())
        .unwrap_or_default();
    let name = app
        .localizedName()
        .map(|s| s.to_string())
        .unwrap_or_default();

    if bundle_id.is_empty() {
        return None;
    }

    let pid = app.processIdentifier();
    let is_active = app.isActive();

    Some(RunningApp {
        bundle_id,
        name,
        pid,
        is_active,
    })
}