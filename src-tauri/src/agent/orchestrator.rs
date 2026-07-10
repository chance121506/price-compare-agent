use std::sync::Arc;
use tauri::Emitter;

use crate::ai::provider::{ChatMessage, LlmProvider, StreamCallback};
use crate::models::product::AgentResult;

use super::intent::parse_intent;
use super::tools;

const STEPS: &[&str] = &["解析意图", "联网搜索", "生成推荐"];

#[derive(Clone, serde::Serialize)]
struct StepEvent {
    index: usize,
    label: String,
}

pub struct AgentOrchestrator {
    llm: Arc<dyn LlmProvider>,
}

impl AgentOrchestrator {
    pub fn new(llm: Arc<dyn LlmProvider>) -> Self {
        Self { llm }
    }

    fn emit_step(&self, app_handle: &tauri::AppHandle, index: usize) {
        let _ = app_handle.emit(
            "agent-step",
            StepEvent {
                index,
                label: STEPS[index].into(),
            },
        );
    }

    /// Emit a short thinking text to the frontend to show progress
    fn emit_thought(&self, app_handle: &tauri::AppHandle, text: &str) {
        let _ = app_handle.emit("agent-stream-chunk", text.to_string());
    }

    /// Build conversation context for follow-up questions — key for multi-turn
    fn build_context(history: &[ChatMessage], current: &str) -> String {
        if history.is_empty() {
            return current.to_string();
        }
        let mut ctx = String::from("## 对话历史（用于理解上下文和追问）\n");
        for msg in history {
            match msg.role.as_str() {
                "user" => ctx.push_str(&format!("用户: {}\n", msg.content)),
                "assistant" | "system" => {
                    // Extract key info: product category, budget, recommended product names
                    let summary: String = if msg.content.len() > 400 {
                        // Try to extract the most relevant parts
                        let short = &msg.content[..400];
                        format!("{}…", short)
                    } else {
                        msg.content.clone()
                    };
                    ctx.push_str(&format!("助手回复: {}\n", summary));
                }
                _ => {}
            }
        }
        ctx.push_str(&format!(
            "\n## 当前输入（用户的最新追问，请结合历史进行意图补全）\n{current}\n\n重要提示：如果用户追问没有明确重复商品类型，必须从对话历史中推断！"
        ));
        ctx
    }

    pub async fn run(
        &self,
        app_handle: &tauri::AppHandle,
        user_input: &str,
        history: Vec<ChatMessage>,
    ) -> anyhow::Result<AgentResult> {
        // ----- Step 0: 解析意图（包含上下文）-----
        self.emit_step(app_handle, 0);
        self.emit_thought(app_handle, "🔍 正在分析你的需求…\n");

        let contextual_input = Self::build_context(&history, user_input);

        // Use tokio::time::timeout to prevent hangs
        let intent = tokio::time::timeout(
            std::time::Duration::from_secs(25),
            parse_intent(&self.llm, &contextual_input),
        )
        .await
        .map_err(|_| anyhow::anyhow!("意图解析超时，请检查网络或 API 配置后重试"))??;

        if let Some(_follow_up) = intent.missing_info() {
            let msg = format!(
                "请补充以下信息以获取更准确的比价结果：{}\n\n💡 你可以直接告诉我，比如: 预算200以内, 需要降噪功能等",
                intent.missing_fields.join(", ")
            );
            let _ = app_handle.emit("agent-step-error", msg.clone());
            return Err(anyhow::anyhow!(msg));
        }

        // ----- Step 1: 联网搜索商品 -----
        self.emit_step(app_handle, 1);
        self.emit_thought(
            app_handle,
            &format!(
                "🛒 正在为你搜索 {}…\n",
                intent.product_name.as_deref().unwrap_or("相关商品")
            ),
        );

        let search_prompt = tools::build_search_prompt(user_input, &intent);

        let resp = tokio::time::timeout(
            std::time::Duration::from_secs(40),
            self.llm.chat(
                vec![
                    ChatMessage {
                        role: "system".into(),
                        content: "你是一个精确的 JSON 输出引擎。只输出 JSON，不输出任何其他文字。".into(),
                    },
                    ChatMessage {
                        role: "user".into(),
                        content: search_prompt,
                    },
                ],
                Some(0.3),
                Some(4096),
            ),
        )
        .await
        .map_err(|_| anyhow::anyhow!("商品搜索超时，请重试或换个说法"))??;

        // 清洗 LLM 返回的 JSON
        let cleaned = resp
            .content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        #[derive(serde::Deserialize)]
        struct SearchResponse {
            products: Vec<crate::models::product::Product>,
            #[serde(default)]
            recommendation: String,
        }

        let search_result: SearchResponse = serde_json::from_str(cleaned).map_err(|e| {
            anyhow::anyhow!(
                "LLM 返回数据解析失败: {}。原始内容前 200 字: {}",
                e,
                &cleaned[..cleaned.len().min(200)]
            )
        })?;

        if search_result.products.is_empty() {
            let _ = app_handle.emit("agent-step-error", "未能搜索到匹配的商品，请换个说法试试？");
            return Err(anyhow::anyhow!("未能搜索到匹配的商品"));
        }

        // ----- Step 2: 流式生成推荐 + 总结 -----
        self.emit_step(app_handle, 2);
        self.emit_thought(app_handle, "💡 正在为你生成购物建议…\n\n");

        let products_json = serde_json::to_string(&search_result.products).unwrap_or_default();

        let rec_prompt = format!(
            r#"根据以下比价结果，给用户一个完整的购物建议。请严格按以下格式输出：

## 📋 总结
用 1-2 句话概括本次比价的核心发现（如：共找到X款商品，价格范围从¥X到¥Y，最推荐哪一款）。

## 💰 推荐理由
用 2-4 句话详细说明推荐的理由，重点分析性价比和适合场景。

## ⚡ 省钱小贴士
如果有的话，给一个省钱建议（如关注促销节点、考虑二手、搭配优惠券等）。

用户需求: {} (预算 {} ~ {})
功能要求: {}

匹配商品:
{}

请用简洁实用的中文回答。"#,
            intent.product_name.as_deref().unwrap_or("未知"),
            intent.budget_min
                .map_or("不限".into(), |v| format!("¥{}", v)),
            intent.budget_max
                .map_or("不限".into(), |v| format!("¥{}", v)),
            intent.features.join("、"),
            products_json,
        );

        let app = app_handle.clone();
        let on_chunk: StreamCallback = Arc::new(move |chunk: &str| {
            let _ = app.emit("agent-stream-chunk", chunk.to_string());
        });

        let stream_result = self
            .llm
            .chat_stream(
                vec![
                    ChatMessage {
                        role: "system".into(),
                        content: "你是一个专业的电商比价顾问。用简洁实用的中文回答，按格式输出总结、推荐理由和省钱贴士。".into(),
                    },
                    ChatMessage {
                        role: "user".into(),
                        content: rec_prompt,
                    },
                ],
                Some(0.5),
                Some(512),
                on_chunk,
            )
            .await?;

        // 优先用流式推荐文本
        let recommendation = if !stream_result.content.is_empty() {
            stream_result.content
        } else {
            search_result.recommendation
        };

        Ok(AgentResult {
            products: search_result.products,
            recommendation,
        })
    }
}
