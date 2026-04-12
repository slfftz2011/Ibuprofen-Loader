use crate::services::registry::RegistryReader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GamePathResult {
    pub path: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn get_netease_download_path() -> Result<GamePathResult, String> {
    let result = RegistryReader::get_netease_download_path();
    
    match result {
        Ok(path) => Ok(GamePathResult {
            path: Some(path),
            error: None,
        }),
        Err(e) => Ok(GamePathResult {
            path: None,
            error: Some(e.to_string()),
        }),
    }
}
