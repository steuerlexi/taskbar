import { useState, useEffect } from "react";

export interface MonitorInfo {
  name: string;
  x: number;
  y: number;
  width: number;
  height: number;
  scaleFactor: number;
}

export function useMonitors() {
  const [monitors, setMonitors] = useState<MonitorInfo[]>([]);

  useEffect(() => {
    // Monitor info is available via Tauri but for now we
    // just track the count from the backend
    setMonitors([]);
  }, []);

  return { monitors };
}