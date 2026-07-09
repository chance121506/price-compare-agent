import { useEffect, useRef, useState } from "react";
import { Layout, Typography, Input, Button, App as AntApp } from "antd";
import { SendOutlined } from "@ant-design/icons";
import { listen } from "@tauri-apps/api/event";
import ChatBubble from "./components/ChatBubble";
import SettingsDrawer from "./components/SettingsDrawer";
import { searchProducts } from "./api/query";
import type { AgentResult } from "./types/product";
import "./App.css";

const { Header, Content, Footer } = Layout;

interface StepPayload {
  index: number;
  label: string;
}

interface Message {
  key: string;
  role: "user" | "agent";
  content?: string;
  loading?: boolean;
  stepIndex?: number;
  error?: string;
  result?: AgentResult;
}

export default function App() {
  const [messages, setMessages] = useState<Message[]>([
    { key: "welcome", role: "agent", content: "你好！我是比价助手 🛒\n告诉我你想买什么，我帮你跨平台比价，找到最划算的选择。" },
  ]);
  const [input, setInput] = useState("");
  const [sending, setSending] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    scrollRef.current?.scrollTo({ top: scrollRef.current.scrollHeight, behavior: "smooth" });
  }, [messages]);

  const handleSend = async () => {
    const question = input.trim();
    if (!question || sending) return;

    setInput("");
    setSending(true);

    const userKey = Date.now().toString();
    const agentKey = (Date.now() + 1).toString();

    setMessages((prev) => [
      ...prev,
      { key: userKey, role: "user", content: question },
      { key: agentKey, role: "agent", loading: true, stepIndex: 0 },
    ]);

    const unlistenStep = listen<StepPayload>("agent-step", (event) => {
      setMessages((prev) =>
        prev.map((m) => (m.key === agentKey ? { ...m, stepIndex: event.payload.index } : m))
      );
    });
    const unlistenErr = listen<string>("agent-step-error", (event) => {
      setMessages((prev) =>
        prev.map((m) => (m.key === agentKey ? { ...m, loading: false, error: event.payload } : m))
      );
    });

    try {
      const res = await searchProducts(question);
      setMessages((prev) =>
        prev.map((m) => (m.key === agentKey ? { ...m, loading: false, result: res } : m))
      );
    } catch (e) {
      setMessages((prev) =>
        prev.map((m) =>
          m.key === agentKey ? { ...m, loading: false, error: String(e) } : m
        )
      );
    } finally {
      setSending(false);
      unlistenStep.then((fn) => fn());
      unlistenErr.then((fn) => fn());
    }
  };

  return (
    <AntApp>
      <Layout style={{ height: "100vh", display: "flex", flexDirection: "column" }}>
        <Header style={{ display: "flex", alignItems: "center", justifyContent: "center", flexShrink: 0 }}>
          <Typography.Title level={3} style={{ color: "#fff", margin: 0, flex: 1 }}>
            🛒 跨平台比价智能体
          </Typography.Title>
          <SettingsDrawer />
        </Header>
        <Content
          ref={scrollRef}
          style={{ flex: 1, overflow: "auto", padding: "20px 40px", background: "#f5f5f5" }}
        >
          <div style={{ maxWidth: 800, margin: "0 auto" }}>
            {messages.map((msg) => (
              <ChatBubble
                key={msg.key}
                role={msg.role}
                content={msg.content}
                loading={msg.loading}
                stepIndex={msg.stepIndex}
                error={msg.error}
                result={msg.result}
              />
            ))}
          </div>
        </Content>
        <Footer style={{ flexShrink: 0, padding: "12px 40px", background: "#fff", borderTop: "1px solid #f0f0f0" }}>
          <div style={{ maxWidth: 800, margin: "0 auto", display: "flex", gap: 8 }}>
            <Input
              size="large"
              placeholder="输入你想买的商品，例如：找一款300以内适合运动的蓝牙耳机"
              value={input}
              onChange={(e: React.ChangeEvent<HTMLInputElement>) => setInput(e.target.value)}
              onPressEnter={handleSend}
              disabled={sending}
            />
            <Button type="primary" size="large" icon={<SendOutlined />} loading={sending} onClick={handleSend}>
              发送
            </Button>
          </div>
        </Footer>
      </Layout>
    </AntApp>
  );
}
