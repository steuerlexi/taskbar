import type { RunningApp, PinnedApp } from "../types";

interface AppIconProps {
  app: RunningApp | PinnedApp;
  isRunning: boolean;
  isActive: boolean;
  onClick: () => void;
  onContextMenu: (e: React.MouseEvent) => void;
}

export function AppIcon({ app, isRunning, isActive, onClick, onContextMenu }: AppIconProps) {
  return (
    <button
      onClick={onClick}
      onContextMenu={onContextMenu}
      className="relative flex flex-col items-center justify-center w-12 h-12 rounded-lg taskbar-icon group"
      title={app.name}
    >
      <div className="w-8 h-8 bg-white/20 rounded flex items-center justify-center text-xs font-medium text-white">
        {app.name.charAt(0)}
      </div>
      {(isRunning || isActive) && (
        <div
          className={`absolute bottom-0.5 h-0.5 rounded-full taskbar-indicator ${
            isActive ? "w-4 bg-[#0078D4]" : "w-1.5 bg-white/60"
          }`}
        />
      )}
    </button>
  );
}