// TypeScript types for Netease Mod Injector

// Registry types
export interface GamePathResult {
  path: string | null;
  error: string | null;
}

// Network types
export interface ConnectionStatus {
  status: 'connected' | 'disconnected' | 'limited_access';
  message: string;
}

export interface WebsiteCheckResult {
  reachable: boolean;
  error: string | null;
}

// File processor types
export interface CopFileInfo {
  name: string;
  path: string;
  size: number;
}

export interface ProcessResult {
  success: boolean;
  output_path: string | null;
  error: string | null;
}

// JSON parser types
export interface ListJsonInfo {
  name: string;
  version: string;
  author: string;
  description: string;
  files: string[];
}

export interface VerifyResult {
  valid: boolean;
  error: string | null;
}

// Injector types
export interface InjectionRequest {
  component_path: string;
  game_path: string;
}

export interface InjectionResult {
  success: boolean;
  message: string;
  backup_path: string | null;
}

export interface BackupResult {
  success: boolean;
  backup_path: string | null;
  error: string | null;
}

// System types
export interface AppVersion {
  version: string;
  name: string;
}

// UI State types
export interface AppState {
  gamePath: string | null;
  networkStatus: ConnectionStatus | null;
  components: CopFileInfo[];
  selectedComponent: CopFileInfo | null;
  isLoading: boolean;
  error: string | null;
  currentView: 'home' | 'components' | 'inject' | 'settings';
}
