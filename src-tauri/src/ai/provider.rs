use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub content: String,
    pub model: String,
}

/// 流式 chunk 回调：每收到一段文字就调用
pub type StreamCallback = Arc<dyn Fn(&str) + Send + Sync>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// 普通（非流式）调用
    async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
    ) -> anyhow::Result<ChatCompletionResponse>;

    /// 流式调用：每收到一个 chunk 就调用 on_chunk
    async fn chat_stream(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<u32>,
        on_chunk: StreamCallback,
    ) -> anyhow::Result<ChatCompletionResponse>;
}