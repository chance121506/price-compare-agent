export interface ChatMessage {
  role: string;
  content: string;
}

export interface Product {
  id: string;
  name: string;
  platform: string;
  price: number;
  original_price?: number;
  specs: string;
  category: string;
  features: string[];
  rating?: number;
  review_count?: number;
  shipping?: number;
  link: string;
  match_type?: "exact" | "similar" | "alternative";
}

export interface AgentResult {
  products: Product[];
  recommendation: string;
}

export interface Conversation {
  id: string;
  title: string;
  lastMessage: string;
  createdAt: number;
  updatedAt: number;
}

export interface Message {
  key: string;
  role: "user" | "agent";
  content?: string;
  loading?: boolean;
  stepIndex?: number;
  error?: string;
  result?: AgentResult;
  streamingText?: string;
  thinkingText?: string; // 保存思考过程内容
}

export const WELCOME_MESSAGE: Message = {
  key: "welcome",
  role: "agent",
  content:
    "你好！我是比价助手 🛒\n\n" +
    "告诉我你想买什么，我帮你跨平台比价，找到最划算的选择。\n\n" +
    "💡 **提示**：你可以连续追问，比如\"把预算缩小到150\"、\"推荐性价比最高的\"，我会记住上下文。",
};