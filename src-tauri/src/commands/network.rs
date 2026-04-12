use crate::services::network::{NetworkChecker, ConnectionStatus as CoreConnectionStatus};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub status: String, 
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteCheckResult {
    pub reachable: bool,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn check_connection_status() -> Result<ConnectionStatus, String> {
    let status = NetworkChecker::check_connection_status().await;
    
    let (status_str, message) = match status {
        CoreConnectionStatus::Connected => ("connected".to_string(), "网络已连接".to_string()),
        CoreConnectionStatus::Disconnected => ("disconnected".to_string(), "网络未连接".to_string()),
        CoreConnectionStatus::LimitedAccess => ("limited_access".to_string(), "网络访问受限".to_string()),
    };
    
    Ok(ConnectionStatus { status: status_str, message })
}

#[tauri::command]
pub async fn check_website_reachable(url: String) -> Result<WebsiteCheckResult, String> {
    let result: Result<bool, _> = NetworkChecker::check_website_reachable(&url).await;
    
    match result {
        Ok(reachable) => Ok(WebsiteCheckResult {
            reachable,
            error: None,
        }),
        Err(e) => Ok(WebsiteCheckResult {
            reachable: false,
            error: Some(e.to_string()),
        }),
    }
}
