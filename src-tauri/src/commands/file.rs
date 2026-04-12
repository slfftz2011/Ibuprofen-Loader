use crate::services::file_processor::FileProcessor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopFileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessResult {
    pub success: bool,
    pub output_path: Option<String>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn scan_cop_files() -> Result<Vec<CopFileInfo>, String> {
    Ok(FileProcessor::scan_cop_files())
}

#[tauri::command]
pub async fn process_cop_file(file_path: String) -> Result<ProcessResult, String> {
    let result = FileProcessor::process_cop_file(&file_path).await;
    
    match result {
        Ok(output_path) => Ok(ProcessResult {
            success: true,
            output_path: Some(output_path),
            error: None,
        }),
        Err(e) => Ok(ProcessResult {
            success: false,
            output_path: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_components_directory() -> Result<String, String> {
    Ok(FileProcessor::get_components_directory())
}

#[tauri::command]
pub async fn check_or_create_components_dir() -> Result<bool, String> {
    Ok(FileProcessor::check_or_create_components_dir())
}
