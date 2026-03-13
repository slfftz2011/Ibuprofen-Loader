// Tauri command wrappers
import { invoke } from "@tauri-apps/api/core";
import type {
  GamePathResult,
  ConnectionStatus,
  WebsiteCheckResult,
  CopFileInfo,
  ProcessResult,
  ListJsonInfo,
  VerifyResult,
  InjectionRequest,
  InjectionResult,
  BackupResult,
  AppVersion,
} from "../types";

// Registry commands
export async function getNeteaseDownloadPath(): Promise<GamePathResult> {
  return invoke("get_netease_download_path");
}

// Network commands
export async function checkConnectionStatus(): Promise<ConnectionStatus> {
  return invoke("check_connection_status");
}

export async function checkWebsiteReachable(url: string): Promise<WebsiteCheckResult> {
  return invoke("check_website_reachable", { url });
}

// File commands
export async function scanCopFiles(): Promise<CopFileInfo[]> {
  return invoke("scan_cop_files");
}

export async function processCopFile(filePath: string): Promise<ProcessResult> {
  return invoke("process_cop_file", { filePath });
}

export async function getComponentsDirectory(): Promise<string> {
  return invoke("get_components_directory");
}

export async function checkOrCreateComponentsDir(): Promise<boolean> {
  return invoke("check_or_create_components_dir");
}

// JSON commands
export async function loadListJson(path: string): Promise<ListJsonInfo> {
  return invoke("load_list_json", { path });
}

export async function verifyFileExists(
  basePath: string,
  relativePath: string
): Promise<VerifyResult> {
  return invoke("verify_file_exists", { basePath, relativePath });
}

// Injector commands
export async function startInjection(request: InjectionRequest): Promise<InjectionResult> {
  return invoke("start_injection", { request });
}

export async function backupDirectories(gamePath: string): Promise<BackupResult> {
  return invoke("backup_directories", { gamePath });
}

export async function waitForGameLaunch(timeoutSecs: number): Promise<boolean> {
  return invoke("wait_for_game_launch", { timeoutSecs });
}

export async function copyFilesToGame(
  sourceMods: string,
  sourceConfig: string,
  sourceResourcepacks: string,
  gamePath: string
): Promise<InjectionResult> {
  return invoke("copy_files_to_game", {
    sourceMods,
    sourceConfig,
    sourceResourcepacks,
    gamePath,
  });
}

// URL commands
export async function openUrl(url: string): Promise<boolean> {
  return invoke("open_url", { url });
}

// System commands
export async function getAppVersion(): Promise<AppVersion> {
  return invoke("get_app_version");
}
