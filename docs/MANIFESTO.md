# The Smritidb Manifesto

*Associative memory is the missing layer of the modern computing stack. This is the open standard that fills it.*

---

## 1. The unstated assumption

Every storage system you have ever used is built on the same unstated assumption: **the question you ask later will look exactly like the address you wrote earlier.**

- A filesystem will return the bytes at `/var/log/auth.log` only if you ask for `/var/log/auth.log` — byte-for-byte.
- A key-value store will return your row only if you remember the key you wrote it under.
- A relational database will find your row only if your `WHERE` clause exactly matches a value you stored.
- A content-addressed store (Git, IPFS) will only return your blob if you have the exact hash.
- Even a vector database — closer than the others — separates the *exact* document store from the *similarity* index, and asks you to maintain both.

This assumption made perfect sense in 1970. Disks were spinning rust; addressing was the entire game. We built a fifty-year stack on top of it.

Then it stopped making sense. The data we store now is **meaning-shaped**: text, embeddings, sensor traces, agent state, user intent. The questions we ask are **partial, fuzzy, compositional, time-decaying**. We have spent the last decade gluing vector indexes onto byte-addressed storage and calling the result "AI infrastructure."

It works. It is also the *least* native abstraction we could have picked.

## 2. What biology already solved

The human brain stores roughly a petabit of information on twenty watts of power and retrieves any of it from a partial cue in milliseconds. It does this without addresses, without exact matching, without a separation between storage and computation, and without losing the entire item when a chunk of substrate dies.

DNA — a four-base molecular tape — stores the recipe for an entire organism in archival-grade durable form. Its redundancy and error correction are built into the chemistry. Its addressing is not numeric; you find a gene by the motif at its boundary, the way you find a sentence in a book by reading the words around it.

Two completely different mechanisms. Two completely different scales. One thing in common: **content and address are the same substance**. There is no separate index. The data *is* the index.

We have known how to mathematize this since 1988. Pentti Kanerva's *Sparse Distributed Memory* and the subsequent literature on **Hyperdimensional Computing** and **Vector Symbolic Architectures** describe a substrate in which:

- Items are encoded as high-dimensional random vectors (typically 10,000 dimensions).
- "Storing" is **superposition** — adding the vector to a running sum.
- "Recalling" is **similarity** — finding the stored vector nearest to a cue.
- "Composing" is **binding** — combining two vectors into a third that can be unbound back to either.
- Losing 30% of the substrate degrades *all* items by 30%, not 30% of items by 100%.

The math has been quietly waiting in academic journals for nearly four decades. The hardware to run it efficiently — wide SIMD, neuromorphic chips, memristor crossbars — has arrived.

Nobody has shipped it as infrastructure.

## 3. The shape of what is missing

Look at the layers of a modern stack:

| Layer | What it stores | How you ask |
|---|---|---|
| Filesystem | Bytes | Path |
| Key-value (Redis, RocksDB) | Bytes | Exact key |
| Relational (Postgres) | Rows | Predicate match |
| Document (Mongo) | JSON | Path query |
| Object store (S3) | Blobs | Exact key |
| Content-addressed (IPFS, Git) | Bytes | Exact hash |
| Vector DB (Pinecone, Qdrant) | Vectors + bytes | k-NN over a separate index |
| **Associative memory** | **Anything** | **Partial cue, by similarity, with composition** |

The last row is empty. It has been empty for fifty years. Every team that wants it builds a half-hearted version on top of a vector DB, a Redis cache, and a pile of hand-rolled glue code. The result is fragile, expensive, and locked into a specific vendor's pipeline.

Smritidb fills the row.

## 4. Three properties, all required

A system has to deliver all three of these together, or the abstraction collapses back into the previous layer:

### 4.1 Fuzzy content-addressing

Look up data by *similarity*, not by exact hash. A partial cue — half a sentence, a near-duplicate image, an embedding, a structured query — retrieves the closest stored item. This is the operation a vector database has bolted on as a side feature. Here it is the *only* primitive.

### 4.2 Holographic distribution

Every item is spread across the entire substrate. There is no block where "item #4271" lives. Lose half the substrate and you lose zero specific items — every item gets a little fuzzier. This is genuinely new for a storage system. RAID gives you fault tolerance at the block level; holographic storage gives you fault tolerance at the *meaning* level.

### 4.3 Hebbian consolidation

Items that are accessed together get pulled closer together in the substrate. Items that go untouched get summarized and eventually forgotten. The index is not static; it reshapes itself based on how the data is actually used. This collapses three things that are usually separate — write-amplification-aware tiering, query optimization, and forgetting — into a single self-organizing process.

Drop any one and you have something else. All three together is a category.

## 5. Why now

Three things are true at once for the first time:

1. **The math is settled.** SDM (1988), HRR (1995), HDC (2009), modern surveys (2023+). The properties are well-characterized. The failure modes are known. There are no fringe-physics claims to defend.

2. **The hardware caught up.** Wide CPU SIMD makes 10,000-dimensional vector ops cheap. GPUs make them trivial. Neuromorphic accelerators (Intel Loihi, IBM TrueNorth) and memristor crossbars are tuned for exactly these operations.

3. **The applications begged for it.** Every LLM agent on earth needs long-term memory. Every "find similar" feature needs sub-second fuzzy retrieval. Every cache wants to summarize cold entries. Every personalization system wants soft, time-decaying state. All of these are working around the absence of the layer this manifesto describes.

The fifty-year gap between Smritidb's paper and an open, productized implementation is not because the idea was wrong. It is because the idea was early.

## 6. What we are building

**Smritidb** is:

- An **open specification** of an associative memory substrate, written so multiple implementations can interoperate.
- An **open wire format** (the *Smritidb Memory Format*, KMF) for persisted substrates, on the same standards path as Parquet, Iceberg, and Arrow.
- A **reference implementation** in TypeScript (Phase 1), Rust + Wasm (Phase 2), and native bindings for Python, Kotlin Multiplatform, and Swift (Phase 3) — one core, every platform an app developer might target.
- A **public benchmark suite** with honest numbers against vector databases, embedded caches, and existing SDM implementations.
- A **hardware-agnostic** runtime: CPU is the floor; GPU is the optimization; neuromorphic is the long horizon.

It is not:

- A vendor. The point is the open standard. We will run a hosted offering eventually because someone has to, but the spec and reference impls will always be the canonical artifacts.
- A vector database. We do similarity, but as a *consequence* of being a memory — not as the headline.
- A transactional system. ACID is great. Use Postgres for ACID.
- A wet-lab biology product. The biology is inspiration, not implementation.

## 7. What revolution actually looks like

Revolutions in infrastructure are usually quiet. Nobody noticed when the relational model replaced hierarchical databases until they had been using SQL for ten years. Nobody noticed when columnar storage replaced row-oriented warehouses until Snowflake was a hundred-billion-dollar company.

The Smritidb bet is small, sharp, and load-bearing:

> *In five years, every nontrivial application will have an associative memory layer. The standard will be open or it will be closed. We are building it open.*

If we are right, the layer is permanent — in the same way the relational layer is permanent, regardless of which vendor wins.

If we are wrong, we will have produced the cleanest open-source implementation of one of the most elegant ideas in late-20th-century computer science. That is a fine consolation.

## 8. The invitation

We are at the spec stage. The most valuable contribution right now is **adversarial reading**: tell us where the abstraction leaks, which use cases break it, which property we will be tempted to compromise away when it gets hard.

If you have a use case that the current stack handles badly — agent memory, fuzzy cache, soft personalization, semantic deduplication, anything where "the thing I want to retrieve isn't exactly what I stored" — open an issue and describe it. That is the corpus the API is being designed against.

If you work on neuromorphic hardware, memristor designs, or hyperdimensional accelerators — we want a hardware target list for the reference Rust core. The math maps to your substrate; let's make it official.

If you are a researcher in SDM/HDC/VSA — we want your name on the citations file, your eyes on the spec, and your papers in the bibliography.

The reference implementations are Apache-2.0 licensed (with the explicit patent grant). The wire format will be CC-BY-4.0 once frozen. The hosted service, when it ships, will exist in service of the spec, not the other way around.

---

*Smritidb is for anyone who has ever written `embedding = model.embed(x); index.upsert(id, embedding); db.set(id, x)` and thought: this can't be the final form.*

*It isn't.*
