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
    let workspace = unsafe { NSWorkspace::sharedWorkspace() };
    let apps = unsafe { workspace.runningApplications() };

    let mut result = Vec::new();
    let count = unsafe { apps.count() };

    for i in 0..count {
        let app = unsafe { apps.objectAtIndex(i) };
        let bundle_id = unsafe { app.bundleIdentifier() }
            .map(|s| unsafe { s.to_string() })
            .unwrap_or_default();
        let name = unsafe { app.localizedName() }
            .map(|s| unsafe { s.to_string() })
            .unwrap_or_default();

        if bundle_id.is_empty() {
            continue;
        }

        let pid = unsafe { app.processIdentifier() };
        let is_active = unsafe { app.isActive() };

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
    // Use `open -b` which is simpler and avoids objc2-app-kit API uncertainty
    std::process::Command::new("open")
        .args(["-b", bundle_id])
        .spawn()
        .is_ok()
}

pub fn get_frontmost_app() -> Option<RunningApp> {
    let workspace = unsafe { NSWorkspace::sharedWorkspace() };
    let app = unsafe { workspace.frontmostApplication() }?;

    let bundle_id = unsafe { app.bundleIdentifier() }
        .map(|s| unsafe { s.to_string() })
        .unwrap_or_default();
    let name = unsafe { app.localizedName() }
        .map(|s| unsafe { s.to_string() })
        .unwrap_or_default();

    if bundle_id.is_empty() {
        return None;
    }

    let pid = unsafe { app.processIdentifier() };
    let is_active = unsafe { app.isActive() };

    Some(RunningApp {
        bundle_id,
        name,
        pid,
        is_active,
    })
}