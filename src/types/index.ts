export interface RunningApp {
  bundle_id: string;
  name: string;
  pid: number;
  is_active: boolean;
}

export interface PinnedApp {
  bundle_id: string;
  name: string;
}

export interface AppConfig {
  position: string;
  height: number;
  show_on_all_monitors: boolean;
  pinned_apps: PinnedApp[];
  widgets: {
    clock: { enabled: boolean; format: string };
    battery: { enabled: boolean };
  };
  appearance: {
    theme: string;
    accent_color: string;
    opacity: number;
  };
}

export interface ClockData {
  time: string;
  date: string;
}

export interface BatteryData {
  percent: number;
  charging: boolean;
}

export interface WidgetUpdate {
  clock: ClockData;
  battery: BatteryData;
}

export interface AppEvent {
  event_type: string;
  bundle_id: string;
  name: string;
}