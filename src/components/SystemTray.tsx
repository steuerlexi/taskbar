import { ClockWidget } from "./ClockWidget";
import { QuickSettings } from "./QuickSettings";

export function SystemTray() {
  return (
    <div className="flex items-center gap-3 pr-3" data-tauri-drag-region={false}>
      <QuickSettings />
      <ClockWidget />
    </div>
  );
}