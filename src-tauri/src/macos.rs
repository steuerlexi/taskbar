use objc2_app_kit::NSWorkspace;
use objc2_foundation::NSString;
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
        let bundle_id = unsafe { app.bundleIdentifier() };
        let name = unsafe { app.localizedName() };
        let pid = unsafe { app.processIdentifier() };
        let is_active = unsafe { app.isActive() };

        let bundle_id_str = bundle_id
            .map(|s| unsafe { s.to_string() })
            .unwrap_or_default();
        let name_str = name
            .map(|s| unsafe { s.to_string() })
            .unwrap_or_default();

        if bundle_id_str.is_empty() {
            continue;
        }

        result.push(RunningApp {
            bundle_id: bundle_id_str,
            name: name_str,
            pid,
            is_active,
        });
    }

    result
}

pub fn launch_app(bundle_id: &str) -> bool {
    let workspace = unsafe { NSWorkspace::sharedWorkspace() };
    let ns_bundle_id = NSString::from_str(bundle_id);

    match unsafe {
        workspace.launchAppWithBundleIdentifier_options_additionalEventParamDescriptor_launchIdentifier_(
            &ns_bundle_id,
            objc2_app_kit::NSWorkspaceLaunchOptions::NSWorkspaceLaunchDefault,
            None,
            None,
        )
    } {
        Some(_) => true,
        None => false,
    }
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
    let pid = unsafe { app.processIdentifier() };
    let is_active = unsafe { app.isActive() };

    if bundle_id.is_empty() {
        return None;
    }

    Some(RunningApp {
        bundle_id,
        name,
        pid,
        is_active,
    })
}