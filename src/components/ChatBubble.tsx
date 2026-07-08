import { Alert, Typography } from "antd";
import { RobotOutlined, UserOutlined } from "@ant-design/icons";
import type { AgentResult } from "../types/product";
import ResultTable from "./ResultTable";
import PriceChart from "./PriceChart";
import StepsBar from "./StepsBar";

interface Props {
  role: "user" | "agent";
  content?: string;
  loading?: boolean;
  stepIndex?: number;
  error?: string;
  result?: AgentResult;
}

export default function ChatBubble({ role, content, loading, stepIndex = -1, error, result }: Props) {
  const isUser = role === "user";

  return (
    <div style={{ display: "flex", justifyContent: isUser ? "flex-end" : "flex-start", marginBottom: 16 }}>
      {!isUser && (
        <div style={{ marginRight: 10, fontSize: 24, color: "#6366f1" }}>
          <RobotOutlined />
        </div>
      )}
      <div style={{ maxWidth: "90%", minWidth: isUser ? undefined : "50%" }}>
        <Typography.Text type="secondary" style={{ fontSize: 12, marginBottom: 4, display: "block" }}>
          {isUser ? "你" : "比价助手"}
        </Typography.Text>
        <div
          style={{
            background: isUser ? "#6366f1" : "#fff",
            color: isUser ? "#fff" : "inherit",
            padding: "12px 16px",
            borderRadius: 12,
            border: isUser ? "none" : "1px solid #f0f0f0",
            boxShadow: "0 1px 3px rgba(0,0,0,0.08)",
          }}
        >
          {/* 用户消息 */}
          {isUser && <div>{content}</div>}

          {/* Agent: 加载中 */}
          {!isUser && loading && (
            <StepsBar current={stepIndex} />
          )}

          {/* Agent: 错误 */}
          {!isUser && error && (
            <Alert type={error.includes("补充") ? "warning" : "error"} message={error} showIcon />
          )}

          {/* Agent: 结果 */}
          {!isUser && result && result.products.length > 0 && (
            <div>
              {result.recommendation && (
                <Alert type="success" message={result.recommendation} showIcon style={{ marginBottom: 12 }} />
              )}
              <ResultTable data={result.products} />
              <div style={{ marginTop: 12 }}>
                <PriceChart products={result.products} />
              </div>
            </div>
          )}

          {/* Agent: 空结果 */}
          {!isUser && result && result.products.length === 0 && (
            <Alert type="info" message="未找到匹配的商品，试试换个说法？" showIcon />
          )}

          {/* Agent: 纯文本（欢迎消息等） */}
          {!isUser && content && !loading && !error && !result && (
            <div style={{ whiteSpace: "pre-line" }}>{content}</div>
          )}
        </div>
      </div>
      {isUser && (
        <div style={{ marginLeft: 10, fontSize: 24, color: "#6366f1" }}>
          <UserOutlined />
        </div>
      )}
    </div>
  );
}
