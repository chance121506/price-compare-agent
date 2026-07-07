use tauri::{AppHandle, State};
use crate::agent::orchestrator::AgentOrchestrator;
use crate::models::product::AgentResult;

#[tauri::command]
pub async fn search_products(
    app_handle: AppHandle,
    orchestrator: State<'_, AgentOrchestrator>,
    question: String,
) -> Result<AgentResult, String> {
    orchestrator.run(&app_handle, &question).await.map_err(|e| e.to_string())
}
