use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::RwLock;

use crate::agent::orchestrator::AgentOrchestrator;
use crate::create_provider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub llm_provider: String,
    pub llm_api_key: String,
    pub llm_base_url: String,
    pub llm_model: String,
}

fn settings_path(app_handle: &tauri::AppHandle) -> std::path::PathBuf {
    app_handle
        .path()
        .app_data_dir()
        .unwrap_or_default()
        .join("settings.json")
}

#[tauri::command]
pub fn get_settings(app_handle: tauri::AppHandle) -> Settings {
    let path = settings_path(&app_handle);

    if let Ok(json) = fs::read_to_string(&path) {
        if let Ok(settings) = serde_json::from_str::<Settings>(&json) {
            return settings;
        }
    }

    Settings {
        llm_provider: std::env::var("LLM_PROVIDER").unwrap_or_else(|_| "openai".into()),
        llm_api_key: std::env::var("LLM_API_KEY").unwrap_or_default(),
        llm_base_url: std::env::var("LLM_BASE_URL").unwrap_or_else(|_| "https://api.deepseek.com".into()),
        llm_model: std::env::var("LLM_MODEL").unwrap_or_else(|_| "deepseek-v4-flash".into()),
    }
}

#[tauri::command]
pub async fn save_settings(
    app_handle: tauri::AppHandle,
    settings: Settings,
) -> Result<(), String> {
    let path = settings_path(&app_handle);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;

    // 重建 orchestrator，立即生效
    let new_orch = AgentOrchestrator::new(create_provider(&settings));

    let state = app_handle.state::<Arc<RwLock<AgentOrchestrator>>>();
    let mut orch = state.write().await;
    *orch = new_orch;

    Ok(())
}
