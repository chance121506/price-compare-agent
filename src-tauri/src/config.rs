#[derive(Clone)]
pub struct AppConfig {
    pub llm_provider: String,
    pub llm_api_key: String,
    pub llm_base_url: String,
    pub llm_model: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            llm_provider: std::env::var("LLM_PROVIDER").unwrap_or_else(|_| "openai".into()),
            llm_api_key: std::env::var("LLM_API_KEY").unwrap_or_default(),
            llm_base_url: std::env::var("LLM_BASE_URL").unwrap_or_else(|_| "https://api.deepseek.com".into()),
            llm_model: std::env::var("LLM_MODEL").unwrap_or_else(|_| "deepseek-v4-flash".into()),
        }
    }
}
