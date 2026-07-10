use async_openai::{
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use async_trait::async_trait;
use futures_util::StreamExt;
use serde_json::Value;

use super::provider::*;

pub struct OpenAiCompatProvider {
    client: Client<OpenAIConfig>,
    api_key: String,
    model: String,
    base_url: String,
}

impl OpenAiCompatProvider {
    pub fn new(api_key: &str, base_url: &str, model: &str) -> Self {
        let config = OpenAIConfig::default()
            .with_api_base(base_url)
            .with_api_key(api_key);

        Self {
            client: Client::with_config(config),
            api_key: api_key.to_string(),
            model: model.to_string(),
            base_url: base_url.to_string(),
        }
    }

    fn build_messages(
        messages: &[ChatMessage],
    ) -> anyhow::Result<Vec<async_openai::types::chat::ChatCompletionRequestMessage>> {
        messages
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
            .collect::<Result<_, _>>()
            .map_err(Into::into)
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
        let api_messages = Self::build_messages(&messages)?;

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

    async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        on_chunk: StreamCallback,
    ) -> anyhow::Result<ChatCompletionResponse> {
        // 直接用 reqwest 做流式请求，在 HTTP 字节流循环里立即回调
        let api_messages: Vec<Value> = messages
            .iter()
            .map(|m| {
                serde_json::json!({ "role": m.role, "content": m.content })
            })
            .collect();

        let mut body = serde_json::json!({
            "model": self.model,
            "messages": api_messages,
            "stream": true,
        });

        if let Some(t) = temperature {
            body["temperature"] = serde_json::json!(t);
        }
        if let Some(mt) = max_tokens {
            body["max_tokens"] = serde_json::json!(mt);
        }

        let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
        let client = reqwest::Client::new();

        let resp = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let err = resp.text().await.unwrap_or_default();
            anyhow::bail!("API stream error ({}): {}", status, err);
        }

        let mut stream = resp.bytes_stream();
        let mut full_content = String::new();
        let mut model = self.model.clone();
        let mut buffer = String::new();

        // 核心：在这个循环里，每解析到文字就立刻回调 on_chunk
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            while let Some(pos) = buffer.find('\n') {
                let line = buffer[..pos].trim().to_string();
                buffer = buffer[pos + 1..].to_string();

                if line.is_empty() || line.starts_with(':') {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    if data == "[DONE]" {
                        continue;
                    }
                    if let Ok(val) = serde_json::from_str::<Value>(data) {
                        if let Some(m) = val["model"].as_str() {
                            model = m.to_string();
                        }
                        if let Some(choices) = val["choices"].as_array() {
                            for choice in choices {
                                if let Some(content) = choice["delta"]["content"].as_str() {
                                    full_content.push_str(content);
                                    // 立即回调 — 不经过任何 channel 或 spawn
                                    on_chunk(content);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(ChatCompletionResponse {
            content: full_content,
            model,
        })
    }
}
