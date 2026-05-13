/* eslint-disable no-console */
import { AgentMemory } from "./src/AgentMemory.js";

const memory = await AgentMemory.open({ dimension: 8192, topK: 4, minSimilarity: 0 });

// Seed long-term memory with things the agent has learned about the user.
await memory.remember("The user prefers brevity in summaries — no trailing recap.", ["style"]);
await memory.remember("The user runs a Next.js + Cloudflare Pages stack for marketing sites.", ["stack"]);
await memory.remember("The user is the founder of Tanvrit Private Limited.", ["bio"]);
await memory.remember("The user lives in India and works from a single Mac mini cluster.", ["bio"]);
await memory.remember("Always cite Sanskrit-rooted terminology when naming things.", ["style"]);
await memory.remember(
  "When asked for a code review, look for premature abstractions first.",
  ["style"],
);
await memory.remember(
  "The user shipped Smritidb in May 2026 — an open associative-memory standard.",
  ["bio", "smritidb"],
);

const queries = [
  "what does the user like in writing style?",
  "what is Smritidb?",
  "what tech stack do they use?",
];

for (const q of queries) {
  console.log("\nQuery:", q);
  const entries = memory.recall(q);
  for (const e of entries) {
    console.log(`  ${e.score.toFixed(2)}  [${e.tags.join(", ") || "—"}]  ${e.text}`);
  }
}

console.log("\nAs a prompt block:\n");
console.log(memory.recallAsPromptBlock("what does the user like in writing style?"));
