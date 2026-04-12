import { invoke } from "@tauri-apps/api/core";
import type { CopFileInfo, ListJsonInfo, GamePathResult, ConnectionStatus, WebsiteCheckResult } from "../types";

// Core invoke helper
async function tauriInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  return await invoke<T>(command, { ...args });
}

// Registry
export async function getGamePath(): Promise<GamePathResult> {
  return tauriInvoke('get_netease_download_path');
}

// Network
export async function checkConnectionStatus(): Promise<ConnectionStatus> {
  return tauriInvoke('check_connection_status');
}

export async function checkWebsiteReachable(url: string): Promise<WebsiteCheckResult> {
  return tauriInvoke('check_website_reachable', { url });
}

// File Processor
export async function scanCopFiles(): Promise<CopFileInfo[]> {
  return tauriInvoke('scan_cop_files');
}

export async function processCopFile(filePath: string): Promise<{ success: boolean; outputPath?: string; error?: string }> {
  try {
    const result = await tauriInvoke<string>('process_cop_file', { filePath });
    return { success: true, outputPath: result };
  } catch (error) {
    return { success: false, error: error instanceof Error ? error.message : 'Unknown error' };
  }
}

export async function checkComponentsDir(): Promise<boolean> {
  return tauriInvoke('check_or_create_components_dir');
}

// JSON Parser
export async function loadListJson(dirPath: string): Promise<ListJsonInfo> {
  return tauriInvoke('load_list_json', { dirPath });
}

// Injection
export async function backupDirectories(gamePath: string): Promise<string> {
  return tauriInvoke('backup_directories', { gamePath });
}

export async function waitForGameLaunch(timeout: number): Promise<boolean> {
  return tauriInvoke('wait_for_game_launch', { timeout });
}

export async function copyFilesToGame(modsSrc: string, configSrc: string, resourcepacksSrc: string, gamePath: string): Promise<void> {
  return tauriInvoke('copy_files_to_game', { modsSrc, configSrc, resourcepacksSrc, gamePath });
}

export async function startInjection(componentPath: string, gamePath: string): Promise<{ backupPath: string }> {
  return tauriInvoke('start_injection', { componentPath, gamePath });
}

// URL
export async function openUrl(url: string): Promise<void> {
  return tauriInvoke('open_url', { url });
}

// System
export async function getAppVersion(): Promise<string> {
  return tauriInvoke('get_app_version');
}

