import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { RunningApp, PinnedApp, AppConfig } from "../types";

export function useApps() {
  const [runningApps, setRunningApps] = useState<RunningApp[]>([]);
  const [pinnedApps, setPinnedApps] = useState<PinnedApp[]>([]);
  const [frontmostApp, setFrontmostApp] = useState<string | null>(null);

  useEffect(() => {
    invoke<RunningApp[]>("get_running_apps").then(setRunningApps);
    invoke<AppConfig>("get_config").then((config) => setPinnedApps(config.pinned_apps));
    invoke<RunningApp | null>("get_frontmost_app").then((app) => {
      if (app) setFrontmostApp(app.bundle_id);
    });

    const unlistenRunning = listen<RunningApp[]>("running-apps-update", (event) => {
      setRunningApps(event.payload);
    });

    const unlistenFrontmost = listen<RunningApp>("frontmost-app-changed", (event) => {
      setFrontmostApp(event.payload.bundle_id);
    });

    return () => {
      unlistenRunning.then((f) => f());
      unlistenFrontmost.then((f) => f());
    };
  }, []);

  const launchApp = async (bundleId: string) => {
    await invoke("launch_app", { bundleId });
  };

  return { runningApps, pinnedApps, frontmostApp, launchApp };
}