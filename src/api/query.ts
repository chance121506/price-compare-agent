import { invoke } from "@tauri-apps/api/core";
import type { AgentResult, ChatMessage } from "@/types/product";

export async function searchProducts(
  question: string,
  history: ChatMessage[] = [],
): Promise<AgentResult> {
  return invoke<AgentResult>("search_products", { question, history });
}
