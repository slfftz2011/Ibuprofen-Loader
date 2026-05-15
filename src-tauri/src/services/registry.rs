use anyhow::Result;
use tracing::{info, warn};
use winreg::enums::*;
use winreg::RegKey;

pub struct RegistryReader;

impl RegistryReader {
    /// Get Netease Minecraft download path from Windows Registry
    pub fn get_netease_download_path() -> Result<String> {
        // Try multiple registry keys that might contain the game path
        let keys_to_try = [
            (r"Software\Netease\MCLauncher", "DownloadPath")
        ];

        for (key_path, value_name) in &keys_to_try {
            if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER).open_subkey(key_path) {
                if let Ok(path) = hkcu.get_value::<String, _>(value_name) {
                    if !path.is_empty() {
                        info!("Found game path: {}", path);
                        return Ok(path);
                    }
                }
            }
        }

        // Also try HKEY_LOCAL_MACHINE
        let hklm_keys = [
            (r"Software\Netease\MCLauncher", "DownloadPath")
        ];

        for (key_path, value_name) in &hklm_keys {
            match RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(key_path) {
                Ok(hklm) => {
                    if let Ok(path) = hklm.get_value::<String, _>(value_name) {
                        if !path.is_empty() {
                            info!("Found game path (HKLM): {}", path);
                            return Ok(path);
                        }
                    }
                }
                Err(_) => {}
            }
        }

        warn!("Could not find Netease Minecraft installation path in registry");
        Err(anyhow::anyhow!("未找到网易我的世界安装目录"))
    }
}

