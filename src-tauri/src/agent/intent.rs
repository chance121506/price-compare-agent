use std::sync::Arc;

use crate::ai::provider::{ChatMessage, LlmProvider};
use crate::models::product::ParsedIntent;

/// 调用 LLM 解析用户意图，提取商品名/品牌/预算/功能需求
/// 支持从对话历史中推断上下文（用于追问场景）
pub async fn parse_intent(
    llm: &Arc<dyn LlmProvider>,
    user_input: &str,
) -> anyhow::Result<ParsedIntent> {
    let prompt = format!(
        r#"你是一个电商比价助手。分析用户的购买需求，提取关键信息，返回纯 JSON（不要 markdown 标记）。

规则：
1. 只要有商品类型（product_name），就可以进行比价，is_complete 应为 true
2. brand、model、budget 都是可选的辅助信息，用户没提就填 null，不算缺失
3. 只有当用户输入完全无法判断要买什么时，is_complete 才为 false
4. **重要**：如果上下文中有对话历史，用户可能在追问/缩小范围，必须从历史中补全商品类型等信息！
   例如：历史中提到"300以内蓝牙耳机"，用户追问"把预算缩小到150" → product_name 应为"蓝牙耳机"，budget_max 应为150

返回格式：
{{
  "product_name": "商品类型（如蓝牙耳机、手机）",
  "brand": "偏好品牌（可为 null）",
  "model": "具体型号（可为 null）",
  "budget_min": 最低预算数字或 null,
  "budget_max": 最高预算数字或 null,
  "features": ["功能要求列表"],
  "usage_scenario": "使用场景（可为 null）",
  "is_complete": true 或 false,
  "missing_fields": ["缺少的关键字段"]
}}

用户输入（含上下文）：{user_input}"#
    );

    let resp = llm
        .chat(
            vec![
                ChatMessage {
                    role: "system".into(),
                    content: "你是一个精确的 JSON 输出引擎。只输出 JSON，不输出任何其他内容。"
                        .into(),
                },
                ChatMessage {
                    role: "user".into(),
                    content: prompt,
                },
            ],
            Some(0.1),
            Some(500),
        )
        .await?;

    // 清洗可能包裹的 ```json ... ```
    let cleaned = resp
        .content
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    Ok(serde_json::from_str(cleaned)?)
}
