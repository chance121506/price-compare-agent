use serde::{Deserialize, Serialize};

/// 商品数据（对应 data/products.json）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub platform: String,
    pub price: f64,
    #[serde(default)]
    pub original_price: Option<f64>,
    pub specs: String,
    pub category: String,
    pub features: Vec<String>,
    #[serde(default)]
    pub rating: Option<f64>,
    #[serde(default)]
    pub review_count: Option<u32>,
    #[serde(default)]
    pub shipping: Option<f64>,
    pub link: String,
    /// LLM 匹配后的类型：exact / similar / alternative
    #[serde(default)]
    pub match_type: Option<String>,
}

/// LLM 意图解析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedIntent {
    pub product_name: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub budget_min: Option<f64>,
    pub budget_max: Option<f64>,
    pub features: Vec<String>,
    pub usage_scenario: Option<String>,
    pub is_complete: bool,
    pub missing_fields: Vec<String>,
}

impl ParsedIntent {
    /// 如果信息不完整，生成追问文案
    pub fn missing_info(&self) -> Option<String> {
        if self.is_complete || self.missing_fields.is_empty() {
            return None;
        }
        Some(format!(
            "请补充以下信息以获取更准确的比价结果：{}",
            self.missing_fields.join("、")
        ))
    }
}

/// Agent 返回给前端的最终结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub products: Vec<Product>,
    pub recommendation: String,
}