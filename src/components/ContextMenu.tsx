import { invoke } from "@tauri-apps/api/core";

interface ContextMenuProps {
  x: number;
  y: number;
  bundleId: string;
  name: string;
  isRunning: boolean;
  onClose: () => void;
}

export function ContextMenu({ x, y, bundleId, name, isRunning, onClose }: ContextMenuProps) {
  const menuY = y - 200;

  return (
    <>
      <div className="fixed inset-0 z-40" onClick={onClose} />
      <div
        className="fixed z-50 bg-gray-800/95 backdrop-blur-md rounded-lg border border-white/10 shadow-2xl py-1 min-w-48"
        style={{ left: x, top: menuY }}
      >
        <button
          className="w-full text-left px-4 py-2 text-sm text-white/90 hover:bg-white/10"
          onClick={() => {
            invoke("launch_app", { bundleId });
            onClose();
          }}
        >
          Open
        </button>
        {isRunning && (
          <button
            className="w-full text-left px-4 py-2 text-sm text-white/90 hover:bg-white/10"
            onClick={() => {
              invoke("quit_app", { bundleId });
              onClose();
            }}
          >
            Quit
          </button>
        )}
        <button
          className="w-full text-left px-4 py-2 text-sm text-white/90 hover:bg-white/10"
          onClick={() => {
            invoke("show_in_finder", { appName: name });
            onClose();
          }}
        >
          Show in Finder
        </button>
      </div>
    </>
  );
}