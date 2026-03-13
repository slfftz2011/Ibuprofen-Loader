// URL Opener - Open URLs in default browser
use anyhow::Result;
use tracing::info;

pub struct URLOpener;

impl URLOpener {
    /// Open a URL in the default browser
    pub async fn open_url(url: &str) -> Result<bool> {
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            
            let result = Command::new("cmd")
                .args(["/C", "start", "", url])
                .spawn();
            
            match result {
                Ok(_) => {
                    info!("Opened URL: {}", url);
                    Ok(true)
                }
                Err(e) => {
                    Err(anyhow::anyhow!("无法打开URL: {}", e))
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            
            let result = Command::new("open")
                .arg(url)
                .spawn();
            
            match result {
                Ok(_) => {
                    info!("Opened URL: {}", url);
                    Ok(true)
                }
                Err(e) => {
                    Err(anyhow::anyhow!("无法打开URL: {}", e))
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            
            let result = Command::new("xdg-open")
                .arg(url)
                .spawn();
            
            match result {
                Ok(_) => {
                    info!("Opened URL: {}", url);
                    Ok(true)
                }
                Err(e) => {
                    Err(anyhow::anyhow!("无法打开URL: {}", e))
                }
            }
        }
    }
}
