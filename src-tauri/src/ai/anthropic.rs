use async_trait::async_trait;
use futures_util::StreamExt;
use serde_json::json;

use super::provider::*;

pub struct AnthropicProvider {
    client: reqwest::Client,
    api_key: String,
    model: String,
    base_url: String,
}

impl AnthropicProvider {
    pub fn new(api_key: &str, base_url: &str, model: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.to_string(),
            model: model.to_string(),
            base_url: base_url.to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> anyhow::Result<ChatCompletionResponse> {
        let system = messages
            .iter()
            .find(|m| m.role == "system")
            .map(|m| m.content.clone());

        let api_messages: Vec<_> = messages
            .iter()
            .filter(|m| m.role != "system")
            .map(|m| json!({ "role": m.role, "content": m.content }))
            .collect();

        let mut body = json!({
            "model": self.model,
            "max_tokens": max_tokens.unwrap_or(4096),
            "messages": api_messages,
            "temperature": temperature.unwrap_or(0.7),
        });

        if let Some(s) = system {
            body["system"] = json!(s);
        }

        let resp = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err = resp.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API error: {}", err);
        }

        let json: serde_json::Value = resp.json().await?;

        let content = json["content"]
            .as_array()
            .and_then(|blocks| {
                blocks
                    .iter()
                    .filter_map(|b| b["text"].as_str())
                    .collect::<Vec<_>>()
                    .join("")
                    .into()
            })
            .filter(|s: &String| !s.is_empty())
            .unwrap_or_default();

        let model = json["model"].as_str().unwrap_or(&self.model).into();

        Ok(ChatCompletionResponse { content, model })
    }

    async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        on_chunk: StreamCallback,
    ) -> anyhow::Result<ChatCompletionResponse> {
        let system = messages
            .iter()
            .find(|m| m.role == "system")
            .map(|m| m.content.clone());

        let api_messages: Vec<_> = messages
            .iter()
            .filter(|m| m.role != "system")
            .map(|m| json!({ "role": m.role, "content": m.content }))
            .collect();

        let mut body = json!({
            "model": self.model,
            "max_tokens": max_tokens.unwrap_or(4096),
            "messages": api_messages,
            "temperature": temperature.unwrap_or(0.7),
            "stream": true,
        });

        if let Some(s) = system {
            body["system"] = json!(s);
        }

        let resp = self
            .client
            .post(format!("{}/v1/messages", self.base_url))
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let err = resp.text().await.unwrap_or_default();
            anyhow::bail!("Anthropic API stream error: {}", err);
        }

        let mut stream = resp.bytes_stream();
        let mut full_content = String::new();
        let mut model = self.model.clone();
        let mut buffer = String::new();

        // 直接在字节流循环里回调
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            while let Some(pos) = buffer.find("\n\n") {
                let event_block = buffer[..pos].to_string();
                buffer = buffer[pos + 2..].to_string();

                for line in event_block.lines() {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if data == "[DONE]" {
                            continue;
                        }
                        if let Ok(val) = serde_json::from_str::<serde_json::Value>(data) {
                            if val["type"] == "content_block_delta" {
                                if let Some(text) = val["delta"]["text"].as_str() {
                                    full_content.push_str(text);
                                    on_chunk(text);
                                }
                            }
                            if let Some(m) = val["model"].as_str() {
                                model = m.to_string();
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