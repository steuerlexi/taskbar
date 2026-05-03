import { AppLauncher } from "./AppLauncher";
import { SystemTray } from "./SystemTray";
import { useApps } from "../hooks/useApps";

export function Taskbar() {
  const { runningApps, pinnedApps, frontmostApp, launchApp } = useApps();

  return (
    <div
      data-tauri-drag-region
      className="h-full w-full flex items-center justify-between taskbar-glass select-none"
    >
      <div className="w-4" />

      <div data-tauri-drag-region={false}>
        <AppLauncher
          pinnedApps={pinnedApps}
          runningApps={runningApps}
          frontmostApp={frontmostApp}
          onLaunch={launchApp}
        />
      </div>

      <SystemTray />
    </div>
  );
}