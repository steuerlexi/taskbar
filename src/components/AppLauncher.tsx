import { useState } from "react";
import { AppIcon } from "./AppIcon";
import { ContextMenu } from "./ContextMenu";
import type { RunningApp, PinnedApp } from "../types";

interface AppLauncherProps {
  pinnedApps: PinnedApp[];
  runningApps: RunningApp[];
  frontmostApp: string | null;
  onLaunch: (bundleId: string) => void;
}

interface ContextMenuState {
  x: number;
  y: number;
  bundleId: string;
  name: string;
  isRunning: boolean;
}

export function AppLauncher({
  pinnedApps,
  runningApps,
  frontmostApp,
  onLaunch,
}: AppLauncherProps) {
  const runningBundleIds = new Set(runningApps.map((a) => a.bundle_id));
  const [contextMenu, setContextMenu] = useState<ContextMenuState | null>(null);

  const handleContextMenu = (
    e: React.MouseEvent,
    bundleId: string,
    name: string,
    isRunning: boolean
  ) => {
    e.preventDefault();
    setContextMenu({ x: e.clientX, y: e.clientY, bundleId, name, isRunning });
  };

  return (
    <div className="flex items-center gap-1">
      {pinnedApps.map((app) => (
        <AppIcon
          key={app.bundle_id}
          app={app}
          isRunning={runningBundleIds.has(app.bundle_id)}
          isActive={frontmostApp === app.bundle_id}
          onClick={() => onLaunch(app.bundle_id)}
          onContextMenu={(e) =>
            handleContextMenu(e, app.bundle_id, app.name, runningBundleIds.has(app.bundle_id))
          }
        />
      ))}

      {runningApps
        .filter((app) => !pinnedApps.some((p) => p.bundle_id === app.bundle_id))
        .map((app) => (
          <AppIcon
            key={app.bundle_id}
            app={app}
            isRunning={true}
            isActive={frontmostApp === app.bundle_id}
            onClick={() => onLaunch(app.bundle_id)}
            onContextMenu={(e) =>
              handleContextMenu(e, app.bundle_id, app.name, true)
            }
          />
        ))}

      {contextMenu && (
        <ContextMenu
          x={contextMenu.x}
          y={contextMenu.y}
          bundleId={contextMenu.bundleId}
          name={contextMenu.name}
          isRunning={contextMenu.isRunning}
          onClose={() => setContextMenu(null)}
        />
      )}
    </div>
  );
}