use async_openai::{
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use async_trait::async_trait;

use super::provider::*;

/// OpenAI 兼容格式的通用 Provider
/// 支持 DeepSeek / Ollama / vLLM / 通义千问 等任何兼容 OpenAI API 的服务
pub struct OpenAiCompatProvider {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OpenAiCompatProvider {
    /// `base_url` — API 地址
    /// `api_key`  — API Key
    /// `model`    — 模型名
    pub fn new(base_url: &str, api_key: &str, model: &str) -> Self {
        let config = OpenAIConfig::default()
            .with_api_base(base_url)
            .with_api_key(api_key);

        Self {
            client: Client::with_config(config),
            model: model.to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiCompatProvider {
    async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> anyhow::Result<ChatCompletionResponse> {
        let api_messages: Vec<_> = messages
            .iter()
            .map(|m| match m.role.as_str() {
                "system" => ChatCompletionRequestSystemMessageArgs::default()
                    .content(m.content.clone())
                    .build()
                    .map(Into::into),
                "assistant" => ChatCompletionRequestAssistantMessageArgs::default()
                    .content(m.content.clone())
                    .build()
                    .map(Into::into),
                _ => ChatCompletionRequestUserMessageArgs::default()
                    .content(m.content.clone())
                    .build()
                    .map(Into::into),
            })
            .collect::<Result<_, _>>()?;

        let mut request = CreateChatCompletionRequestArgs::default();
        request.model(self.model.clone()).messages(api_messages);

        if let Some(t) = temperature {
            request.temperature(t);
        }
        if let Some(mt) = max_tokens {
            request.max_completion_tokens(mt);
        }

        let response = self.client.chat().create(request.build()?).await?;

        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        Ok(ChatCompletionResponse {
            content,
            model: response.model,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_openai_compat_chat() {
        dotenvy::dotenv().ok();
        let base_url = std::env::var("LLM_BASE_URL").expect("请先在 .env 中设置 LLM_BASE_URL");
        let api_key = std::env::var("LLM_API_KEY").expect("请先在 .env 中设置 LLM_API_KEY");
        let model = std::env::var("LLM_MODEL").expect("请先在 .env 中设置 LLM_MODEL");

        let provider = OpenAiCompatProvider::new(&base_url, &api_key, &model);

        let messages = vec![ChatMessage {
            role: "user".into(),
            content: "你好，请用一句话介绍你自己".into(),
        }];

        let resp = provider.chat(messages, Some(0.7), Some(256)).await.unwrap();

        println!("模型: {}", resp.model);
        println!("回复: {}", resp.content);
        assert!(!resp.content.is_empty());
    }
}
