/// TypeScript interfaces for Ibuprofen Loader

export interface CopFileInfo {
  name: string;
  path: string;
  size: number;
}

export interface ListJsonInfo {
  name: string;
  version: string;
  author: string;
  description: string;
  files: string[];
}

export interface GamePathResult {
  path?: string;
  error?: string;
}

export type ConnectionStatus = {
  status: 'checking' | 'connected' | 'disconnected' | 'limited';
  message: string;
};

export interface WebsiteCheckResult {
  reachable: boolean;
  error?: string;
}

export interface InjectionStatus {
  progress: number; // 0-100
  message: string;
  done: boolean;
  error?: string;
  backupPath?: string;
}

// App settings (minimal for loader)
export interface AppSettings {
  componentsDir: string;
  autoBackup: boolean;
  manualGamePath?: string;
  theme: 'light' | 'dark' | 'auto';
}

