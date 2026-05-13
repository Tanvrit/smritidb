// A small framework-agnostic long-term-memory adapter for LLM agents.
//
// Drop this next to whatever orchestration layer you use (LangChain.js,
// LlamaIndex.ts, Mastra, OpenAI Assistants, a custom loop, etc.). The
// adapter is intentionally tiny — it exposes `remember`, `recall`, and
// `consolidate` so the agent loop never has to think about hypervectors.
//
// Conceptually:
//   - Every fact / message / observation goes in via `remember(text)`.
//   - Before responding, the agent calls `recall(prompt)` to retrieve
//     related memories and prepends them to the prompt.
//   - Periodically (e.g. at session end) the agent calls `consolidate()`
//     to let Smritidb bind frequently-co-accessed memories closer together.

import {
  Smritidb,
  bundle,
  encodeString,
  fsAdapter,
  openPersistentStore,
  persistStore,
  type Hypervector,
  type StorageAdapter,
} from "@tanvrit/smritidb";

/**
 * Bag-of-words encoder. Text is normalised, split on whitespace, each word
 * mapped to its own hypervector and bundled. This is the simplest encoder
 * that yields genuine partial-cue retrieval (substrings of the same content
 * recover the original). For higher fidelity, plug in a real embedding model
 * and use `encodeEmbedding` instead.
 */
function bagOfWords(text: string, dim: number): Hypervector {
  const words = text
    .toLowerCase()
    .replace(/[^a-z0-9 ]+/g, " ")
    .split(/\s+/)
    .filter((w) => w.length > 2);
  if (words.length === 0) return encodeString(text, dim);
  return bundle(words.map((w) => encodeString(`word:${w}`, dim)));
}

export interface MemoryEntry {
  readonly id: string;
  readonly text: string;
  readonly score: number;
  readonly tags: readonly string[];
  readonly accessCount: number;
}

export interface AgentMemoryConfig {
  dimension?: number;
  /** Storage adapter; defaults to ephemeral in-memory. */
  adapter?: StorageAdapter;
  /** Auto-persist after every `remember()` call. */
  autoPersist?: boolean;
  /** How many memories to surface per recall. */
  topK?: number;
  /** Floor for similarity (filters obvious non-matches before ranking). */
  minSimilarity?: number;
}

export class AgentMemory {
  readonly #store: Smritidb;
  readonly #adapter?: StorageAdapter;
  readonly #autoPersist: boolean;
  readonly #topK: number;
  readonly #minSim: number;

  private constructor(
    store: Smritidb,
    adapter: StorageAdapter | undefined,
    autoPersist: boolean,
    topK: number,
    minSim: number,
  ) {
    this.#store = store;
    this.#adapter = adapter;
    this.#autoPersist = autoPersist;
    this.#topK = topK;
    this.#minSim = minSim;
  }

  static async open(config: AgentMemoryConfig = {}): Promise<AgentMemory> {
    const dimension = config.dimension ?? 10_000;
    const store = config.adapter
      ? await openPersistentStore({ dimension, adapter: config.adapter })
      : new Smritidb({ dimension });
    return new AgentMemory(
      store,
      config.adapter,
      config.autoPersist ?? true,
      config.topK ?? 6,
      config.minSimilarity ?? 0.55,
    );
  }

  /** Add a fact / message / observation to long-term memory. */
  async remember(text: string, tags: readonly string[] = []): Promise<string> {
    const key = bagOfWords(text, this.#store.dimension);
    const item = this.#store.put(key, text, { tags: [...tags] });
    if (this.#autoPersist && this.#adapter) {
      await persistStore(this.#store, this.#adapter);
    }
    return item.id;
  }

  /** Recall memories relevant to a query. Returns text + similarity score. */
  recall(query: string, opts: { topK?: number; minSimilarity?: number } = {}): MemoryEntry[] {
    const cue = bagOfWords(query, this.#store.dimension);
    const hits = this.#store.recall(cue, {
      topK: opts.topK ?? this.#topK,
      minSimilarity: opts.minSimilarity ?? this.#minSim,
    });
    const decoder = new TextDecoder();
    return hits.map((m) => ({
      id: m.item.id,
      text: decoder.decode(m.item.value),
      score: m.similarity,
      tags: m.item.tags,
      accessCount: m.item.accessCount,
    }));
  }

  /** Render the top-K relevant memories as a system-style prompt block. */
  recallAsPromptBlock(query: string, title = "Relevant memories"): string {
    const entries = this.recall(query);
    if (entries.length === 0) return "";
    const body = entries
      .map((e, i) => `[${i + 1}] (relevance ${e.score.toFixed(2)})  ${e.text}`)
      .join("\n");
    return `${title}:\n${body}`;
  }

  /** Forget a specific memory by id. */
  forget(id: string): boolean {
    return this.#store.delete(id);
  }

  /** Hebbian compaction — frequently co-recalled memories are bound closer. */
  async consolidate(): Promise<void> {
    this.#store.consolidate();
    if (this.#adapter) {
      await persistStore(this.#store, this.#adapter);
    }
  }

  /** Current memory count. */
  size(): number {
    return this.#store.size();
  }
}

/**
 * Convenience: open a memory backed by a file on disk.
 *
 *     const memory = await openFileBackedMemory("/var/lib/myagent/memory.kmf");
 *     await memory.remember("The user prefers brevity in summaries.");
 *     const relevant = memory.recall("summarise this PR");
 */
export async function openFileBackedMemory(
  path: string,
  config: Omit<AgentMemoryConfig, "adapter"> = {},
): Promise<AgentMemory> {
  return AgentMemory.open({ ...config, adapter: fsAdapter(path) });
}
