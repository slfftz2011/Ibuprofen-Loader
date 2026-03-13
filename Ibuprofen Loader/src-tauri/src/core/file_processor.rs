// File Processor - Handle COP files and component operations
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
// Removed unused imports: Read, Write
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use walkdir::WalkDir;
use zip::ZipArchive;

use super::registry::RegistryReader;

#[derive(Debug, Serialize, Deserialize)]
// CopFileInfo moved to commands::file for IPC serialization

pub struct FileProcessor;

impl FileProcessor {
    /// Get the components directory path
    pub fn get_components_directory() -> String {
        // Try to get from game path first
        if let Ok(game_path) = RegistryReader::get_netease_download_path() {
            let components_dir = PathBuf::from(&game_path).join("components");
            if components_dir.exists() {
                return components_dir.to_string_lossy().to_string();
            }
        }
        
        // Fallback to app directory
        let app_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));
        
        let components_dir = app_dir.join("components");
        components_dir.to_string_lossy().to_string()
    }

    /// Check or create components directory
    pub fn check_or_create_components_dir() -> bool {
        let components_dir = Self::get_components_directory();
        let dir = Path::new(&components_dir);
        if !dir.exists() {
            if let Err(e) = fs::create_dir_all(dir) {
                warn!("Failed to create components directory: {}", e);
                return false;
            }
        }
        true
    }

    /// Scan all COP files in the components directory
    pub fn scan_cop_files() -> Vec<crate::commands::file::CopFileInfo> {
        let components_dir = Self::get_components_directory();
        let path = Path::new(&components_dir);
        
        let mut files = Vec::new();
        
        if !path.exists() {
            warn!("Components directory does not exist: {}", components_dir);
            return files;
        }

        for entry in WalkDir::new(path)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let file_path = entry.path();
            if file_path.is_file() {
                let extension = file_path.extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("");
                
                if extension.eq_ignore_ascii_case("cop") || 
                   extension.eq_ignore_ascii_case("zip") {
                    let metadata = fs::metadata(file_path).ok();
                    let size = metadata.map(|m| m.len()).unwrap_or(0);
                    
                    files.push(crate::commands::file::CopFileInfo {
                        name: file_path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string(),
                        path: file_path.to_string_lossy().to_string(),
                        size,
                    });
                }
            }
        }
        
        info!("Found {} COP files", files.len());
        files
    }

    /// Process a COP file (extract to temp directory)
    pub async fn process_cop_file(file_path: &str) -> Result<String> {
        let path = Path::new(file_path);
        
        if !path.exists() {
            return Err(anyhow::anyhow!("文件不存在: {}", file_path));
        }

        // Get the file name without extension
        let file_stem = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("无效的文件名"))?;

        // Create temp extraction directory
        let temp_dir = PathBuf::from(&Self::get_components_directory())
            .join(file_stem);
        
        if temp_dir.exists() {
            fs::remove_dir_all(&temp_dir).ok();
        }
        fs::create_dir_all(&temp_dir)?;

        // Handle COP file (which is just a renamed ZIP)
        let extension = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let zip_path = if extension.eq_ignore_ascii_case("cop") {
            // Rename COP to ZIP temporarily
            let temp_zip = temp_dir.with_extension("zip");
            fs::copy(path, &temp_zip)?;
            temp_zip
        } else {
            path.to_path_buf()
        };

        // Extract ZIP
        let file = fs::File::open(&zip_path)
            .context("无法打开压缩文件")?;
        let mut archive = ZipArchive::new(file)
            .context("无法解析ZIP文件")?;
        
        archive.extract(&temp_dir)
            .context("无法解压文件")?;

        // Clean up temp zip if we created one
        if extension.eq_ignore_ascii_case("cop") {
            fs::remove_file(&zip_path).ok();
        }

        info!("Extracted COP file to: {}", temp_dir.display());
        Ok(temp_dir.to_string_lossy().to_string())
    }
}
