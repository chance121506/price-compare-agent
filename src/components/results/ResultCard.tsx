import { Tag, Button, Drawer, Row, Col, Tooltip } from "antd";
import {
  ShoppingOutlined,
  StarOutlined,
  LinkOutlined,
  ExportOutlined,
  TrophyOutlined,
} from "@ant-design/icons";
import { useState, useMemo } from "react";
import { openUrl } from "@tauri-apps/plugin-opener";
import { message } from "antd";
import type { Product } from "@/types/product";

const MATCH_CONFIG: Record<string, { color: string; label: string; bg: string }> = {
  exact: { color: "#3fb950", label: "Exact match", bg: "rgba(63,185,80,0.1)" },
  similar: { color: "#58a6ff", label: "Similar", bg: "rgba(88,166,255,0.1)" },
  alternative: { color: "#d29922", label: "Alternative", bg: "rgba(210,153,34,0.1)" },
};

const PLATFORM_STYLE: Record<string, { color: string; bg: string; icon: string }> = {
  京东: { color: "#f85149", bg: "rgba(248,81,73,0.1)", icon: "🔴" },
  淘宝: { color: "#d29922", bg: "rgba(210,153,34,0.1)", icon: "🟠" },
  拼多多: { color: "#e55354", bg: "rgba(229,83,84,0.1)", icon: "🔺" },
  苏宁: { color: "#f0883e", bg: "rgba(240,136,62,0.1)", icon: "🟡" },
  天猫: { color: "#db61a2", bg: "rgba(219,97,162,0.1)", icon: "🐱" },
};

function MatchBadge({ type }: { type?: string }) {
  const cfg = MATCH_CONFIG[type || ""] ?? {
    color: "#6e7681",
    label: "Unknown",
    bg: "rgba(110,118,129,0.1)",
  };
  return (
    <span
      style={{
        display: "inline-block",
        fontSize: 10,
        padding: "1px 8px",
        borderRadius: 20,
        background: cfg.bg,
        color: cfg.color,
        fontWeight: 500,
      }}
    >
      {cfg.label}
    </span>
  );
}

function handleOpenLink(link: string | undefined | null) {
  if (!link || link.trim() === "") {
    message.warning("此商品暂无购买链接（AI 生成的价格仅供参考）");
    return;
  }
  openUrl(link);
}
function scoreProduct(p: Product, minPrice: number, maxPrice: number): number {
  const priceScore = maxPrice > minPrice ? 1 - (p.price - minPrice) / (maxPrice - minPrice) : 1;
  const ratingScore = (p.rating ?? 3) / 5;
  return Math.round((priceScore * 0.7 + ratingScore * 0.3) * 100);
}

export function ResultCard({ products }: { products: Product[] }) {
  const minPrice = Math.min(...products.map((p) => p.price));
  const maxPrice = Math.max(...products.map((p) => p.price));
  const [selectedProduct, setSelectedProduct] = useState<Product | undefined>();
  const [drawerOpen, setDrawerOpen] = useState(false);

  // Find best value product
  const bestValue = useMemo(() => {
    return products
      .map((p) => ({ product: p, score: scoreProduct(p, minPrice, maxPrice) }))
      .sort((a, b) => b.score - a.score)[0];
  }, [products, minPrice, maxPrice]);

  if (products.length === 0) return null;

  return (
    <>
      {/* Summary bar */}
      <div
        style={{
          display: "flex",
          alignItems: "center",
          gap: 12,
          padding: "10px 14px",
          background: "var(--bg-overlay)",
          border: "1px solid var(--border-default)",
          borderRadius: 8,
          marginBottom: 10,
          flexWrap: "wrap",
        }}
      >
        <TrophyOutlined style={{ color: "var(--accent-orange)", fontSize: 16 }} />
        <span style={{ fontSize: 13, fontWeight: 600, color: "var(--text-primary)" }}>
          最佳性价比:{" "}
          <span style={{ color: "var(--accent-green)" }}>{bestValue?.product.name}</span>
        </span>
        <span style={{ fontSize: 12, color: "var(--text-secondary)" }}>
          {bestValue?.product.platform} · ¥{bestValue?.product.price} · 性价比评分 {bestValue?.score}分
        </span>
        <div style={{ flex: 1 }} />
        <span style={{ fontSize: 11, color: "var(--text-tertiary)" }}>
          {products.length} 款商品 · 价格区间 ¥{minPrice}-¥{maxPrice}
        </span>
      </div>

      {/* Product cards */}
      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        {products.map((product) => {
          const isLowest = product.price === minPrice;
          const isBestValue = bestValue?.product.id === product.id;
          const ps = PLATFORM_STYLE[product.platform] ?? {
            color: "#6e7681",
            bg: "rgba(110,118,129,0.1)",
            icon: "🏪",
          };

          return (
            <div
              key={product.id}
              className="product-card"
              style={{
                borderColor: isLowest
                  ? "rgba(63,185,80,0.3)"
                  : "var(--border-default)",
              }}
            >
              {/* Badges row */}
              <div style={{ position: "absolute", top: -8, right: 12, display: "flex", gap: 6, zIndex: 1 }}>
                {isLowest && (
                  <span
                    style={{
                      background: "var(--accent-green)",
                      color: "#000",
                      fontSize: 10,
                      fontWeight: 600,
                      padding: "1px 8px",
                      borderRadius: 10,
                    }}
                  >
                    🏆 Best price
                  </span>
                )}
                {isBestValue && !isLowest && (
                  <span
                    style={{
                      background: "var(--accent-orange)",
                      color: "#000",
                      fontSize: 10,
                      fontWeight: 600,
                      padding: "1px 8px",
                      borderRadius: 10,
                    }}
                  >
                    ⭐ Best value
                  </span>
                )}
              </div>

              <div
                style={{
                  display: "flex",
                  justifyContent: "space-between",
                  alignItems: "flex-start",
                }}
              >
                {/* Left side */}
                <div style={{ flex: 1, minWidth: 0 }}>
                  <div
                    style={{
                      display: "flex",
                      alignItems: "center",
                      gap: 6,
                      marginBottom: 5,
                    }}
                  >
                    <Tag
                      style={{
                        background: ps.bg,
                        color: ps.color,
                        border: "none",
                        borderRadius: 4,
                        fontSize: 11,
                        fontWeight: 600,
                        margin: 0,
                      }}
                    >
                      {ps.icon} {product.platform}
                    </Tag>
                    <MatchBadge type={product.match_type} />
                  </div>

                  <Tooltip
                    title={product.name}
                    placement="topLeft"
                    overlayStyle={{ maxWidth: 400 }}
                    overlayInnerStyle={{
                      background: "#21262d",
                      color: "#c9d1d9",
                      fontSize: 13,
                      padding: "8px 12px",
                      border: "1px solid #30363d",
                      borderRadius: 6,
                      wordBreak: "break-word",
                    }}
                    getPopupContainer={() => document.body}
                    destroyTooltipOnHide
                    mouseEnterDelay={0.3}
                  >
                    <div
                      style={{
                        color: "var(--text-primary)",
                        fontSize: 14,
                        fontWeight: 600,
                        marginBottom: 3,
                        overflow: "hidden",
                        textOverflow: "ellipsis",
                        whiteSpace: "nowrap",
                        cursor: "default",
                        display: "inline-block",
                        maxWidth: "100%",
                      }}
                    >
                      <ShoppingOutlined
                        style={{
                          color: "var(--text-tertiary)",
                          marginRight: 6,
                          fontSize: 12,
                        }}
                      />
                      {product.name}
                    </div>
                  </Tooltip>

                  <div
                    style={{
                      color: "var(--text-tertiary)",
                      fontSize: 12,
                      marginBottom: 3,
                      fontFamily: "var(--font-mono)",
                    }}
                  >
                    {product.specs}
                  </div>

                  {/* Rating */}
                  {product.rating && (
                    <div
                      style={{
                        display: "flex",
                        alignItems: "center",
                        gap: 4,
                        fontSize: 12,
                        color: "var(--accent-orange)",
                      }}
                    >
                      <StarOutlined />
                      <span style={{ fontWeight: 500 }}>{product.rating}</span>
                      {product.review_count && (
                        <span style={{ color: "var(--text-tertiary)", fontWeight: 400 }}>
                          ({product.review_count} reviews)
                        </span>
                      )}
                    </div>
                  )}
                </div>

                {/* Right side — price + shipping */}
                <div
                  style={{
                    textAlign: "right",
                    flexShrink: 0,
                    marginLeft: 16,
                  }}
                >
                  <div
                    style={{
                      fontSize: 22,
                      fontWeight: 700,
                      color: isLowest ? "var(--accent-green)" : "var(--accent-orange)",
                      lineHeight: 1.2,
                      fontFamily: "var(--font-mono)",
                      letterSpacing: "-0.5px",
                    }}
                  >
                    ¥{product.price}
                  </div>
                  {product.original_price && (
                    <div
                      style={{
                        fontSize: 12,
                        color: "var(--text-tertiary)",
                        textDecoration: "line-through",
                        fontFamily: "var(--font-mono)",
                      }}
                    >
                      ¥{product.original_price}
                    </div>
                  )}
                  {product.shipping !== undefined && product.shipping !== null && (
                    <div
                      style={{
                        fontSize: 10,
                        color: product.shipping === 0 ? "var(--accent-green)" : "var(--text-tertiary)",
                        fontFamily: "var(--font-mono)",
                      }}
                    >
                      {product.shipping === 0 ? "📦 包邮" : `运费 ¥${product.shipping}`}
                    </div>
                  )}
                  <span
                    onClick={(e) => {
                      e.preventDefault();
                      handleOpenLink(product.link);
                    }}
                    style={{
                      color: "var(--text-link)",
                      fontSize: 11,
                      display: "inline-flex",
                      alignItems: "center",
                      gap: 4,
                      marginTop: 4,
                      textDecoration: "none",
                      cursor: "pointer",
                    }}
                  >
                    <LinkOutlined /> Buy
                  </span>
                </div>
              </div>

              {/* Feature tags */}
              {product.features.length > 0 && (
                <div
                  style={{
                    display: "flex",
                    gap: 4,
                    flexWrap: "wrap",
                    marginTop: 8,
                  }}
                >
                  {product.features.slice(0, 5).map((feat, i) => (
                    <span
                      key={i}
                      style={{
                        fontSize: 10,
                        padding: "1px 7px",
                        borderRadius: 10,
                        background: "var(--bg-subtle)",
                        color: "var(--text-secondary)",
                        fontFamily: "var(--font-mono)",
                      }}
                    >
                      {feat}
                    </span>
                  ))}
                </div>
              )}

              {/* Expand button */}
              <div style={{ marginTop: 8 }}>
                <Button
                  type="text"
                  size="small"
                  onClick={() => {
                    setSelectedProduct(product);
                    setDrawerOpen(true);
                  }}
                  style={{
                    color: "var(--text-link)",
                    fontSize: 11,
                    padding: 0,
                    height: "auto",
                    fontWeight: 500,
                  }}
                >
                  View details →
                </Button>
              </div>
            </div>
          );
        })}
      </div>

      {/* Detail drawer */}
      {selectedProduct && (
        <Drawer
          title={
            <span style={{ color: "var(--text-primary)", fontSize: 14, fontWeight: 600 }}>
              Product details
            </span>
          }
          onClose={() => setDrawerOpen(false)}
          open={drawerOpen}
          width={Math.min(480, window.innerWidth - 32)}
        >
          <Row gutter={[14, 14]}>
            <Col span={24}>
              <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                Name
              </div>
              <div
                style={{
                  fontSize: 14,
                  marginTop: 4,
                  color: "var(--text-primary)",
                  wordBreak: "break-word",
                  lineHeight: 1.6,
                  fontWeight: 500,
                }}
              >
                {selectedProduct.name}
              </div>
            </Col>
            <Col span={12}>
              <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                Platform
              </div>
              <div style={{ marginTop: 4 }}>
                <Tag
                  color={PLATFORM_STYLE[selectedProduct.platform]?.color ?? "#6e7681"}
                  style={{ borderRadius: 4, fontWeight: 500, margin: 0 }}
                >
                  {PLATFORM_STYLE[selectedProduct.platform]?.icon} {selectedProduct.platform}
                </Tag>
              </div>
            </Col>
            <Col span={12}>
              <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                Price
              </div>
              <div
                style={{
                  fontSize: 18,
                  marginTop: 4,
                  fontWeight: 700,
                  color: "var(--accent-green)",
                  fontFamily: "var(--font-mono)",
                }}
              >
                ¥{selectedProduct.price}
              </div>
            </Col>
            {selectedProduct.original_price && (
              <Col span={12}>
                <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                  Original price
                </div>
                <div
                  style={{
                    fontSize: 14,
                    marginTop: 4,
                    textDecoration: "line-through",
                    color: "var(--text-tertiary)",
                    fontFamily: "var(--font-mono)",
                  }}
                >
                  ¥{selectedProduct.original_price}
                </div>
              </Col>
            )}
            {selectedProduct.rating && (
              <Col span={12}>
                <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                  Rating
                </div>
                <div style={{ fontSize: 14, marginTop: 4, color: "var(--accent-orange)", fontWeight: 600 }}>
                  ⭐ {selectedProduct.rating}
                  {selectedProduct.review_count && (
                    <span style={{ fontWeight: 400, color: "var(--text-tertiary)", fontSize: 12 }}>
                      {" "}
                      ({selectedProduct.review_count})
                    </span>
                  )}
                </div>
              </Col>
            )}
            <Col span={24}>
              <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                Specifications
              </div>
              <div
                style={{
                  fontSize: 13,
                  marginTop: 4,
                  color: "var(--text-secondary)",
                  wordBreak: "break-word",
                  lineHeight: 1.7,
                }}
              >
                {selectedProduct.specs}
              </div>
            </Col>
            {selectedProduct.features && selectedProduct.features.length > 0 && (
              <Col span={24}>
                <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                  Features
                </div>
                <div style={{ fontSize: 12, marginTop: 4, display: "flex", flexWrap: "wrap", gap: 4 }}>
                  {selectedProduct.features.map((f) => (
                    <Tag
                      key={f}
                      style={{
                        background: "var(--bg-subtle)",
                        color: "var(--text-secondary)",
                        border: "1px solid var(--border-default)",
                        borderRadius: 4,
                      }}
                    >
                      {f}
                    </Tag>
                  ))}
                </div>
              </Col>
            )}
            <Col span={24}>
              <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                Match type
              </div>
              <div style={{ marginTop: 4 }}>
                <MatchBadge type={selectedProduct.match_type} />
              </div>
            </Col>
            {selectedProduct.shipping !== undefined && selectedProduct.shipping !== null && (
              <Col span={24}>
                <div style={{ fontSize: 11, color: "var(--text-tertiary)", fontWeight: 500 }}>
                  Shipping
                </div>
                <div
                  style={{
                    fontSize: 14,
                    marginTop: 4,
                    color: "var(--text-primary)",
                    fontFamily: "var(--font-mono)",
                  }}
                >
                  {selectedProduct.shipping === 0 ? "📦 Free shipping" : `¥${selectedProduct.shipping}`}
                </div>
              </Col>
            )}
            <Col span={24}>
              <Button
                type="primary"
                icon={<ExportOutlined />}
                onClick={() => {
                  handleOpenLink(selectedProduct.link);
                }}
                style={{
                  width: "100%",
                  height: 36,
                  fontSize: 13,
                  fontWeight: 500,
                  borderRadius: 4,
                }}
              >
                Open product page
              </Button>
            </Col>
          </Row>
        </Drawer>
      )}
    </>
  );
}
