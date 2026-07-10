use crate::models::product::ParsedIntent;

/// 构建联网搜索 prompt — 引导 LLM 扮演比价搜索引擎
pub fn build_search_prompt(user_input: &str, intent: &ParsedIntent) -> String {
    format!(
        r#"你是一个专业的电商比价搜索引擎。请扮演搜索引擎的角色，从你的训练数据中检索京东、淘宝、拼多多、苏宁、天猫等平台的最新商品信息，为用户提供准确的价格对比。

## 用户需求
- 商品类型: {}
- 品牌偏好: {}
- 预算范围: {} ~ {}
- 功能要求: {}
- 使用场景: {}

原始输入: {}

## 任务要求
1. 搜索 3-6 款最匹配的商品，**必须覆盖至少 2 个不同平台**（京东/淘宝/拼多多/苏宁/天猫等）
2. 价格尽量接近当前市场真实价格
3. 必须为每款商品标注 **rating**（1-5分，基于口碑）和 **review_count**（评价数量）
4. 为每个商品标注 match_type：
   - "exact" — 完全匹配用户需求的同款商品
   - "similar" — 规格相近的替代品
   - "alternative" — 预算/功能略有差异但值得参考
5. 按价格从低到高排序
6. recommendation 字段留空即可（后续流式生成）

## 输出格式（纯 JSON，不要 markdown 标记）
```json
{{
  "products": [
    {{
      "id": "唯一ID",
      "name": "商品全名（含品牌型号）",
      "platform": "京东|淘宝|拼多多|苏宁|天猫",
      "price": 数字价格,
      "original_price": 原价或null,
      "specs": "核心规格参数（简明扼要）",
      "category": "商品类别",
      "features": ["功能标签1", "功能标签2", "功能标签3"],
      "rating": 评分1-5的数字,
      "review_count": 评价数量（数字）,
      "shipping": 运费（数字，包邮填0）,
      "link": "商品在对应平台的搜索直达链接，格式如 https://search.jd.com/Search?keyword=商品名 或 https://s.taobao.com/search?q=商品名，不可为空",
      "match_type": "exact|similar|alternative"
    }}
  ],
  "recommendation": ""
}}
```"#,
        intent.product_name.as_deref().unwrap_or("未知"),
        intent.brand.as_deref().unwrap_or("无偏好"),
        intent.budget_min
            .map_or("不限".into(), |v| format!("¥{}", v)),
        intent.budget_max
            .map_or("不限".into(), |v| format!("¥{}", v)),
        intent.features.join("、"),
        intent.usage_scenario.as_deref().unwrap_or("通用"),
        user_input,
    )
}
