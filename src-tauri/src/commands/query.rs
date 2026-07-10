use std::sync::Arc;
use tauri::{AppHandle, State};
use tokio::sync::RwLock;

use crate::agent::orchestrator::AgentOrchestrator;
use crate::ai::provider::ChatMessage;
use crate::models::product::AgentResult;

#[tauri::command]
pub async fn search_products(
    app_handle: AppHandle,
    orchestrator: State<'_, Arc<RwLock<AgentOrchestrator>>>,
    question: String,
    history: Vec<ChatMessage>,
) -> Result<AgentResult, String> {
    let orch = orchestrator.read().await;
    orch.run(&app_handle, &question, history)
        .await
        .map_err(|e| e.to_string())
}