use crate::services::json_parser::JsonParser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListJsonInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyResult {
    pub valid: bool,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn load_list_json(path: String) -> Result<ListJsonInfo, String> {
    let parser = JsonParser::new();
    let result = parser.load_list_json(&path).await;
    
    match result {
        Ok(info) => Ok(ListJsonInfo {
            name: info.name,
            version: info.version,
            author: info.author,
            description: info.description,
            files: info.files,
        }),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn verify_file_exists(base_path: String, relative_path: String) -> Result<VerifyResult, String> {
    let parser = JsonParser::new();
    let valid = parser.verify_file(&base_path, &relative_path);
    
    Ok(VerifyResult {
        valid,
        error: None,
    })
}
