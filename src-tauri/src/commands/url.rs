use crate::services::url_opener::URLOpener;

#[tauri::command]
pub async fn open_url(url: String) -> Result<bool, String> {
    URLOpener::open_url(&url).await.map_err(|e| e.to_string())
}
