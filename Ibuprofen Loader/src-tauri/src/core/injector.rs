// Injector - Handle mod injection to game directory
use anyhow::Result;
// use crate::core::injector::Injector; - Removed invalid self-import
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

pub struct Injector {
    pub log_path: PathBuf,
    pub mods_dest: PathBuf,
    pub config_dest: PathBuf,
    pub resource_dest: PathBuf,
}

impl Injector {
    pub fn new() -> Self {
        let temp_dir = std::env::temp_dir();
        
        Self {
            log_path: temp_dir.join("netease_mod_injector_trigger.txt"),
            mods_dest: PathBuf::new(),
            config_dest: PathBuf::new(),
            resource_dest: PathBuf::new(),
        }
    }

    /// Set the game path and initialize destination directories
    pub fn set_game_path(&mut self, game_path: &str) {
        let path = Path::new(game_path);
        self.mods_dest = path.join("mods");
        self.config_dest = path.join("config");
        self.resource_dest = path.join("resourcepacks");
    }

    /// Create trigger log file
    pub fn create_trigger_file(&self) -> Result<()> {
        let content = "3401765#JuwLBFt";
        fs::write(&self.log_path, content)?;
        info!("Created trigger file: {}", self.log_path.display());
        Ok(())
    }

    /// Delete trigger log file
    pub fn delete_trigger_file(&self) -> Result<()> {
        if self.log_path.exists() {
            fs::remove_file(&self.log_path)?;
            info!("Deleted trigger file");
        }
        Ok(())
    }

    /// Wait for game to launch (trigger file deleted)
    pub async fn wait_for_game_launch(&self, timeout_secs: u64) -> bool {
        let check_interval = Duration::from_secs(1);
        let mut elapsed = 0u64;

        // First create the trigger file
        if let Err(e) = self.create_trigger_file() {
            warn!("Failed to create trigger file: {}", e);
        }

        while elapsed < timeout_secs {
            sleep(check_interval).await;
            elapsed += 1;

            if !self.log_path.exists() {
                info!("Game launched, trigger file deleted");
                return true;
            }
        }

        warn!("Timeout waiting for game launch");
        // Clean up trigger file on timeout
        self.delete_trigger_file().ok();
        false
    }

    /// Backup existing mods, config, and resourcepacks directories
    pub async fn backup_directories(&self, game_path: &str) -> Result<String> {
        let backup_dir = PathBuf::from(game_path).join("backup");
        
        // Create backup directory
        fs::create_dir_all(&backup_dir)?;

        let dirs_to_backup = ["mods", "config", "resourcepacks"];
        
        for dir_name in &dirs_to_backup {
            let source = PathBuf::from(game_path).join(dir_name);
            let dest = backup_dir.join(dir_name);
            
            if source.exists() {
                // Remove existing backup if any
                if dest.exists() {
                    fs::remove_dir_all(&dest).ok();
                }
                
                // Copy directory
                Self::copy_dir_recursive(&source, &dest)?;
                info!("Backed up {} to {}", dir_name, dest.display());
            }
        }

        Ok(backup_dir.to_string_lossy().to_string())
    }

    /// Copy files to game directory
    pub async fn copy_files_to_game(
        &self,
        source_mods: &str,
        source_config: &str,
        source_resourcepacks: &str,
        game_path: &str,
    ) -> Result<()> {
        let game_path = Path::new(game_path);
        
        // Copy mods
        let mods_dest = game_path.join("mods");
        if Path::new(source_mods).exists() {
            Self::copy_dir_recursive(Path::new(source_mods), &mods_dest)?;
            info!("Copied mods to {}", mods_dest.display());
        }

        // Copy config
        let config_dest = game_path.join("config");
        if Path::new(source_config).exists() {
            Self::copy_dir_recursive(Path::new(source_config), &config_dest)?;
            info!("Copied config to {}", config_dest.display());
        }

        // Copy resourcepacks
        let resource_dest = game_path.join("resourcepacks");
        if Path::new(source_resourcepacks).exists() {
            Self::copy_dir_recursive(Path::new(source_resourcepacks), &resource_dest)?;
            info!("Copied resourcepacks to {}", resource_dest.display());
        }

        Ok(())
    }

    /// Start the injection process
    pub async fn start_injection(&self, component_path: &str, game_path: &str) -> Result<String> {
        // First backup existing files
        let backup_path = self.backup_directories(game_path).await?;

        // Wait for game to launch
        let launched = self.wait_for_game_launch(60).await;
        
        if !launched {
            return Err(anyhow::anyhow!("等待游戏启动超时"));
        }

        // Copy files
        let component_path = Path::new(component_path);
        self.copy_files_to_game(
            &component_path.join("mods").to_string_lossy(),
            &component_path.join("config").to_string_lossy(),
            &component_path.join("resourcepacks").to_string_lossy(),
            game_path,
        ).await?;

        Ok(backup_path)
    }

    /// Recursively copy directory
    fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<()> {
        fs::create_dir_all(dest)?;
        
        for entry in walkdir::WalkDir::new(src)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let src_path = entry.path();
            let dest_path = dest.join(src_path.strip_prefix(src).unwrap());
            
            if src_path.is_dir() {
                fs::create_dir_all(&dest_path)?;
            } else {
                fs::copy(src_path, &dest_path)?;
            }
        }
        
        Ok(())
    }
}
