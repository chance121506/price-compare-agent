# 🛒 跨平台比价智能体

基于大模型的桌面比价应用。输入自然语言，AI 自动在京东/淘宝间比价并给出推荐。

## 功能

- 🔍 **自然语言比价** — 输入"300 以内运动蓝牙耳机"，自动匹配推荐
- 📊 **价格可视化** — 结果表格 + ECharts 柱状图
- 🤖 **双模型兼容** — OpenAI 兼容格式 + Anthropic Claude，设置页一键切换
- 💬 **对话式交互** — ChatGPT 风格消息气泡，实时步骤进度
- ⚡ **即时生效** — 切换模型无需重启

## 技术栈

| 层 | 技术 |
|------|------|
| 桌面框架 | Tauri 2 |
| 后端 | Rust (tokio, async-openai, reqwest) |
| 前端 | React 19 + TypeScript |
| UI | Ant Design 5 + ECharts |

## 开发

```bash
# 安装依赖
pnpm install

# 启动开发服务器
cargo tauri dev

# 构建
cargo tauri build
```

首次运行需要在设置页（右上角齿轮）配置 API Key。支持 DeepSeek、Claude、Ollama 等任意兼容 OpenAI/Anthropic 格式的服务商。

## 文档

完整文档 → [pca-docs](https://badnuker.github.io/pca-docs/)

## 许可证

GPLv3
