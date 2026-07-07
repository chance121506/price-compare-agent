import { useState } from "react";
import { Layout, Typography, Space, Alert, Steps } from "antd";
import { listen } from "@tauri-apps/api/event";
import SearchBox from "./components/SearchBox";
import ResultTable from "./components/ResultTable";
import PriceChart from "./components/PriceChart";
import { searchProducts } from "./api/query";
import type { AgentResult } from "./types/product";
import "./App.css";

const { Header, Content } = Layout;

const STEP_LABELS = ["理解需求", "筛选商品", "比价分析", "生成推荐"];

interface StepPayload {
  index: number;
  label: string;
}

export default function App() {
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<AgentResult | null>(null);
  const [stepIndex, setStepIndex] = useState(-1);
  const [error, setError] = useState<string | null>(null);

  const handleSearch = async (question: string) => {
    setLoading(true);
    setError(null);
    setResult(null);
    setStepIndex(0);

    // 监听后端实时推送的步骤事件
    const unlistenStep = listen<StepPayload>("agent-step", (event) => {
      setStepIndex(event.payload.index);
    });
    const unlistenErr = listen<string>("agent-step-error", (event) => {
      setError(event.payload);
      setStepIndex(-1);
    });

    try {
      const res = await searchProducts(question);
      setResult(res);
    } catch (e) {
      if (!error) setError(String(e));
    } finally {
      setLoading(false);
      unlistenStep.then((fn) => fn());
      unlistenErr.then((fn) => fn());
    }
  };

  const stepsItems = STEP_LABELS.map((title) => ({ title }));

  return (
    <Layout style={{ minHeight: "100vh" }}>
      <Header
        style={{
          display: "flex",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <Typography.Title level={3} style={{ color: "#fff", margin: 0 }}>
          🛒 跨平台比价智能体
        </Typography.Title>
      </Header>
      <Content style={{ padding: "32px 40px", maxWidth: 1000, margin: "0 auto" }}>
        <Space direction="vertical" size="large" style={{ width: "100%" }}>
          <div style={{ textAlign: "center" }}>
            <SearchBox onSearch={handleSearch} loading={loading} />
          </div>

          {stepIndex >= 0 && (
            <Steps
              size="small"
              current={stepIndex}
              status={error ? "error" : result ? "finish" : "process"}
              items={stepsItems}
            />
          )}

          {error && (
            <Alert
              type={error.includes("补充") ? "warning" : "error"}
              message={error}
              showIcon
              closable
              onClose={() => setError(null)}
            />
          )}

          {result && result.products.length > 0 && (
            <>
              {result.recommendation && (
                <Alert
                  type="success"
                  message="推荐建议"
                  description={result.recommendation}
                  showIcon
                />
              )}
              <ResultTable data={result.products} />
              <PriceChart products={result.products} />
            </>
          )}

          {result && result.products.length === 0 && (
            <Alert type="info" message="未找到匹配的商品，试试换个说法？" showIcon />
          )}
        </Space>
      </Content>
    </Layout>
  );
}
