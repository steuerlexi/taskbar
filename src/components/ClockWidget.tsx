import { useWidgets } from "../hooks/useWidgets";

export function ClockWidget() {
  const { clock } = useWidgets();

  return (
    <div className="flex flex-col items-end text-xs text-white/90 leading-tight">
      <span className="font-medium">{clock.time}</span>
      <span className="text-white/60">{clock.date}</span>
    </div>
  );
}