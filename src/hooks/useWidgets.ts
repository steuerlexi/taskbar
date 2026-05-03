import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import type { WidgetUpdate } from "../types";

export function useWidgets() {
  const [clock, setClock] = useState({ time: "--:--", date: "" });
  const [battery, setBattery] = useState({ percent: 100, charging: true });

  useEffect(() => {
    const unlisten = listen<WidgetUpdate>("widget-update", (event) => {
      setClock(event.payload.clock);
      setBattery(event.payload.battery);
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  return { clock, battery };
}