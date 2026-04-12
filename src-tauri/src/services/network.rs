use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn};

pub struct NetworkChecker;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    LimitedAccess,
}

impl NetworkChecker {
    /// Check internet connection status
    pub async fn check_connection_status() -> ConnectionStatus {
        // Try to connect to multiple well-known websites
        let test_urls = [
            "https://www.google.com",
            "https://www.baidu.com", 
            "https://api.github.com",
        ];

        let client = match Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
        {
            Ok(c) => c,
            Err(_) => return ConnectionStatus::Disconnected,
        };

        let mut success_count = 0;
        for url in &test_urls {
            if client.get(*url).send().await.map(|_| ()).is_ok() {
                success_count += 1;
            }
        }

        match success_count {
            2.. => ConnectionStatus::Connected,
            1 => ConnectionStatus::LimitedAccess,
            _ => ConnectionStatus::Disconnected,
        }
    }

    /// Check if specific website is reachable
    pub async fn check_website_reachable(url: &str) -> Result<bool> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        let response = client.head(url).send().await?;
        
        let is_reachable = response.status().is_success() || response.status().is_redirection();
        
        if is_reachable {
            info!("✓ Website reachable: {}", url);
        } else {
            warn!("✗ Website unreachable ({}): {}", response.status(), url);
        }
        
        Ok(is_reachable)
    }
}
