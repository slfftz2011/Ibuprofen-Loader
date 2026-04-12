import type { AppSettings } from "../types";

const STORAGE_KEY = "ibuprofen.settings";

const DEFAULT_SETTINGS: AppSettings = {
  componentsDir: "",
  autoBackup: true,
  manualGamePath: "",
  theme: "auto",
};

function parseSettings(raw: string | null): AppSettings {
  if (!raw) return DEFAULT_SETTINGS;
  try {
    const parsed = JSON.parse(raw) as Partial<AppSettings>;
    return {
      ...DEFAULT_SETTINGS,
      ...parsed,
    };
  } catch {
    return DEFAULT_SETTINGS;
  }
}

export async function get(): Promise<AppSettings> {
  return parseSettings(localStorage.getItem(STORAGE_KEY));
}

export async function save(settings: AppSettings): Promise<AppSettings> {
  localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  return settings;
}

export async function reset(): Promise<AppSettings> {
  localStorage.removeItem(STORAGE_KEY);
  return get();
}

export async function exportJson(): Promise<string> {
  return JSON.stringify(await get(), null, 2);
}

export async function importJson(json: string): Promise<AppSettings> {
  const parsed = JSON.parse(json) as AppSettings;
  if (typeof parsed !== "object" || parsed === null) {
    throw new Error("无效的设置 JSON");
  }
  const settings = {
    ...DEFAULT_SETTINGS,
    ...parsed,
  };
  await save(settings);
  return settings;
}
