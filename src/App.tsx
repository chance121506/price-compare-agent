import { useState } from "react";
import { Layout, Typography, Space, Alert, Steps } from "antd";
import SearchBox from "./components/SearchBox";
import ResultTable from "./components/ResultTable";
import PriceChart from "./components/PriceChart";
import { searchProducts } from "./api/query";
import type { AgentResult, AgentStep } from "./types/product";
import "./App.css";

const { Header, Content } = Layout;

export default function App() {
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<AgentResult | null>(null);
  const [steps, setSteps] = useState<AgentStep[]>([]);
  const [error, setError] = useState<string | null>(null);

  const handleSearch = async (question: string) => {
    setLoading(true);
    setError(null);
    setResult(null);
    setSteps([{ step: "正在理解需求...", status: "running" }]);

    try {
      const res = await searchProducts(question);
      setResult(res);
      setSteps(res.steps);
    } catch (e) {
      setError(String(e));
      setSteps([]);
    } finally {
      setLoading(false);
    }
  };

  const stepsIndex = steps.filter((s) => s.status === "done").length;

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

          {steps.length > 0 && (
            <Steps
              size="small"
              current={stepsIndex}
              status={steps.some((s) => s.status === "error") ? "error" : "process"}
              items={steps.map((s) => ({
                title: s.step.replace("正在", "").replace("...", ""),
              }))}
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
