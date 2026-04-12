use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::{info, warn};

#[derive(Debug, Serialize, Deserialize)]
pub struct ListJsonInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub files: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ListJson {
    name: Option<String>,
    version: Option<String>,
    author: Option<String>,
    description: Option<String>,
    files: Option<Vec<String>>,
}

pub struct JsonParser;

impl JsonParser {
    pub fn new() -> Self {
        Self
    }

    /// Load and parse list.json from the extracted component directory
    pub async fn load_list_json(&self, dir_path: &str) -> Result<ListJsonInfo> {
        let list_json_path = Path::new(dir_path).join("list.json");
        
        if !list_json_path.exists() {
            return Err(anyhow::anyhow!("list.json 文件不存在"));
        }

        let content = fs::read_to_string(&list_json_path)
            .context("无法读取 list.json")?;
        
        let json: ListJson = serde_json::from_str(&content)
            .context("无法解析 list.json")?;

        info!("Loaded list.json from: {}", list_json_path.display());

        Ok(ListJsonInfo {
            name: json.name.unwrap_or_else(|| "Unknown".to_string()),
            version: json.version.unwrap_or_else(|| "1.0.0".to_string()),
            author: json.author.unwrap_or_else(|| "Unknown".to_string()),
            description: json.description.unwrap_or_else(|| "".to_string()),
            files: json.files.unwrap_or_default(),
        })
    }

    /// Verify if a file exists in the component directory
    pub fn verify_file(&self, base_path: &str, relative_path: &str) -> bool {
        let full_path = Path::new(base_path).join(relative_path);
        let exists = full_path.exists();
        
        if !exists {
            warn!("File not found: {}", full_path.display());
        }
        
        exists
    }

    /// Verify all files listed in list.json exist
    pub async fn verify_all_files(&self, dir_path: &str) -> Result<Vec<String>> {
        let info = self.load_list_json(dir_path).await?;
        let mut missing = Vec::new();

        for file in &info.files {
            if !self.verify_file(dir_path, file) {
                missing.push(file.clone());
            }
        }

        if !missing.is_empty() {
            warn!("Missing files: {:?}", missing);
        }

        Ok(missing)
    }
}
