const STEPS = ["理解需求", "筛选商品", "比价分析", "生成推荐"];

interface Props {
  current: number;
}

export default function StepsBar({ current }: Props) {
  return (
    <div style={{ display: "flex", alignItems: "flex-start" }}>
      {STEPS.map((label, i) => (
        <div key={i} style={{ display: "flex", alignItems: "flex-start", flex: i < STEPS.length - 1 ? 1 : undefined }}>
          {/* 圆点 + 标签 */}
          <div style={{ display: "flex", flexDirection: "column", alignItems: "center", flexShrink: 0 }}>
            <div
              style={{
                width: 12,
                height: 12,
                borderRadius: "50%",
                background: i <= current ? "#1677ff" : "#fff",
                border: `2px solid ${i <= current ? "#1677ff" : "#d9d9d9"}`,
                transition: "all 0.3s",
              }}
            />
            <span
              style={{
                fontSize: 11,
                marginTop: 4,
                color: i <= current ? "#1677ff" : "#999",
                whiteSpace: "nowrap",
              }}
            >
              {label}
            </span>
          </div>
          {/* 连接线 — 对齐圆点顶部 */}
          {i < STEPS.length - 1 && (
            <div style={{ flex: 1, margin: "0 6px", paddingTop: 5 }}>
              <div
                style={{
                  borderTop: `2px ${i < current ? "solid" : "dashed"} ${i < current ? "#1677ff" : "#d9d9d9"}`,
                  transition: "all 0.3s",
                }}
              />
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
