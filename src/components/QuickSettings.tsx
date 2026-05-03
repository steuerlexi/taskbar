import { useWidgets } from "../hooks/useWidgets";

export function QuickSettings() {
  const { battery } = useWidgets();

  return (
    <div className="flex items-center gap-2 text-white/70 text-xs">
      <span title={battery.charging ? "Charging" : "Battery"}>
        {battery.percent}%
      </span>
    </div>
  );
}