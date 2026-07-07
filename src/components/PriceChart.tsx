import { useEffect, useRef } from "react";
import * as echarts from "echarts";
import type { Product } from "../types/product";

const COLORS = ["#f5222d", "#fa8c16", "#1890ff", "#52c41a"];

export default function PriceChart({ products }: { products: Product[] }) {
  const chartRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!chartRef.current) return;

    const chart = echarts.init(chartRef.current);
    const platforms = [...new Set(products.map((p) => p.platform))];
    const names = products.map((p) =>
      p.name.length > 8 ? p.name.slice(0, 8) + "..." : p.name
    );

    chart.setOption({
      tooltip: { trigger: "axis" },
      legend: { data: platforms, bottom: 0 },
      xAxis: {
        type: "category",
        data: names,
        axisLabel: { rotate: 30 },
      },
      yAxis: { type: "value", name: "价格 (¥)" },
      series: platforms.map((platform, i) => ({
        name: platform,
        type: "bar",
        data: products.map((p) => (p.platform === platform ? p.price : null)),
        itemStyle: { color: COLORS[i % COLORS.length] },
      })),
      grid: { bottom: 60, top: 20 },
    });

    return () => chart.dispose();
  }, [products]);

  return <div ref={chartRef} style={{ height: 400 }} />;
}
