# agent-memory

A tiny framework-agnostic long-term-memory adapter for LLM agents, built on `@tanvrit/smritidb`. ~150 lines.

Drop it next to your orchestration layer (LangChain.js, LlamaIndex.ts, Mastra, OpenAI Assistants, a custom loop, …). The adapter exposes:

```ts
const memory = await AgentMemory.open({ adapter: fsAdapter("./memory.kmf") });
await memory.remember("The user prefers brevity in summaries.", ["style"]);
const block = memory.recallAsPromptBlock("draft a release note");
// Prepend `block` to your prompt before calling the model.
```

It deliberately doesn't pull in any LLM-vendor SDK — `AgentMemory.recall()` returns plain `{ text, score }` records. You decide how to thread them into your prompt.

## Demo

```bash
cd examples/agent-memory
pnpm install
pnpm demo
```

Prints recall results across three queries against a small seeded memory.

## Why this isn't a vector DB plugin

The three load-bearing Smritidb properties — fuzzy content-addressing, holographic distribution, Hebbian consolidation — are exactly what agent memory needs: partial-cue retrieval, graceful degradation under substrate loss, and use-pattern-driven reshaping. Smritidb is purpose-built for this shape of workload, not retrofitted to it.

See [the manifesto](https://smritidb.com/manifesto) for the long version.

## License

Apache-2.0.
