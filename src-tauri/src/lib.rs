mod agent;
mod ai;
mod commands;
mod config;
mod models;

use agent::orchestrator::AgentOrchestrator;
use ai::anthropic::AnthropicProvider;
use ai::openai_compat::OpenAiCompatProvider;
use ai::provider::LlmProvider;
use commands::settings::Settings;
use std::sync::Arc;
use tokio::sync::RwLock;

pub fn create_provider(settings: &Settings) -> Arc<dyn LlmProvider> {
    match settings.llm_provider.as_str() {
        "anthropic" => Arc::new(AnthropicProvider::new(
            &settings.llm_api_key,
            &settings.llm_base_url,
            &settings.llm_model,
        )),
        _ => Arc::new(OpenAiCompatProvider::new(
            &settings.llm_api_key,
            &settings.llm_base_url,
            &settings.llm_model,
        )),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    let config = config::AppConfig::from_env();

    let settings = Settings {
        llm_provider: config.llm_provider,
        llm_api_key: config.llm_api_key,
        llm_base_url: config.llm_base_url,
        llm_model: config.llm_model,
    };

    let orchestrator = AgentOrchestrator::new(create_provider(&settings));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(RwLock::new(orchestrator)))
        .invoke_handler(tauri::generate_handler![
            commands::query::search_products,
            commands::settings::get_settings,
            commands::settings::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
