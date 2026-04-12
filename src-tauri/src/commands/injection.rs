use crate::services::injector::Injector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InjectionRequest {
    pub component_path: String,
    pub game_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InjectionResult {
    pub success: bool,
    pub message: String,
    pub backup_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupResult {
    pub success: bool,
    pub backup_path: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn start_injection(request: InjectionRequest) -> Result<InjectionResult, String> {
    let injector = Injector::new();
    let result = injector.start_injection(&request.component_path, &request.game_path).await;
    
    match result {
        Ok(backup_path) => Ok(InjectionResult {
            success: true,
            message: "注入成功".to_string(),
            backup_path: Some(backup_path),
        }),
        Err(e) => Ok(InjectionResult {
            success: false,
            message: e.to_string(),
            backup_path: None,
        }),
    }
}

#[tauri::command]
pub async fn backup_directories(game_path: String) -> Result<BackupResult, String> {
    let injector = Injector::new();
    let result = injector.backup_directories(&game_path).await;
    
    match result {
        Ok(backup_path) => Ok(BackupResult {
            success: true,
            backup_path: Some(backup_path),
            error: None,
        }),
        Err(e) => Ok(BackupResult {
            success: false,
            backup_path: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn wait_for_game_launch(timeout_secs: u64) -> Result<bool, String> {
    let injector = Injector::new();
    Ok(injector.wait_for_game_launch(timeout_secs).await)
}

#[tauri::command]
pub async fn copy_files_to_game(
    source_mods: String,
    source_config: String,
    source_resourcepacks: String,
    game_path: String,
) -> Result<InjectionResult, String> {
    let injector = Injector::new();
    let result = injector.copy_files_to_game(&source_mods, &source_config, &source_resourcepacks, &game_path).await;
    
    match result {
        Ok(_) => Ok(InjectionResult {
            success: true,
            message: "文件复制成功".to_string(),
            backup_path: None,
        }),
        Err(e) => Ok(InjectionResult {
            success: false,
            message: e.to_string(),
            backup_path: None,
        }),
    }
}
