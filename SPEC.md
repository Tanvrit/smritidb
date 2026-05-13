# Kanerva Specification v0.1.0-draft

**Status:** DRAFT — Phase 0. Subject to change until the v0.1.0 tag. After v1.0.0, breaking changes require a major version bump.

**Purpose:** This document is the *contract* every Kanerva implementation must satisfy. The reference TypeScript implementation (Phase 1) and every downstream binding (Phase 2+) are downstream of this file. Behavior disagreements between an implementation and this document are bugs in the implementation, not the spec.

---

## 0. Document conventions

- "MUST", "SHOULD", "MAY" follow [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119) semantics.
- "The substrate" refers to the in-memory representation of a Kanerva store.
- "An implementation" refers to any conformant runtime (core-ts, core-rs, kanerva-py, etc.).
- "KMF" (Kanerva Memory Format) refers to the persistent wire format defined in §8.

---

## 1. Mathematical substrate

Kanerva is built on **binary hyperdimensional computing (HDC)** with a sparse-distributed-memory backing store. The choice of binary (rather than bipolar or real-valued) optimizes for cross-platform reproducibility, cheap operations, and direct mapping to bitwise SIMD.

### 1.1 Hypervectors

A **hypervector** is a vector in `{0, 1}^D`.

- `D` (dimension) is fixed per store. Recommended values: `D = 10000` (default), `D = 8192` (SIMD-aligned), `D = 16384` (high-fidelity).
- Implementations MUST reject `D < 1024` and SHOULD warn for `D > 65536`.
- Hypervectors are stored as packed bit-arrays: `ceil(D / 8)` bytes, MSB-first within each byte.

### 1.2 Core operations

All operations are deterministic given identical inputs.

| Op | Symbol | Definition | Properties |
|---|---|---|---|
| **Bundle** (superposition) | `⊕` | Element-wise majority over a multiset of hypervectors, ties broken by a deterministic tiebreaker (§1.3). | Approximately preserves similarity to each input. |
| **Bind** | `⊗` | Element-wise XOR. | Self-inverse: `(a ⊗ b) ⊗ b = a`. Distributes over bundle. |
| **Permute** | `Π_k` | Cyclic bit rotation by `k` positions. | Used to encode order and protect bindings from interference. |
| **Similarity** | `sim` | `1 - hamming(a, b) / D`, in `[0, 1]`. | `sim(a, a) = 1`. Random pair: `sim ≈ 0.5`. |

### 1.3 Tiebreaker

The majority operation for an even-sized bundle has ties. The tiebreaker MUST be deterministic and identical across implementations:

> For each bit position with an exact tie, the result bit is `H(D || index || count)[0]` where `H` is BLAKE3 and `||` is byte concatenation. `index` is the bit position (u32 LE), `count` is the bundle multiplicity (u32 LE).

This is overkill for correctness but is required for **bit-exact cross-binding reproducibility**, which the KMF wire format depends on.

### 1.4 Random hypervector generation

`randomHV(seed)` MUST produce identical output across implementations:

> Expand `BLAKE3(seed)` with the XOF construction to produce `ceil(D / 8)` bytes. Interpret bytes as MSB-first bits.

`seed` is a 32-byte value derived from the user-provided key (UTF-8 encoded) via `BLAKE3(key)`.

---

## 2. Data model

### 2.1 Item

An **item** is the unit of storage:

```
Item := {
  id:        UUIDv7              # generated on put if not provided
  key:       Hypervector         # 'address' in associative space
  value:     bytes                # opaque payload (≤ 16 MiB default cap)
  tags:      [string]            # arbitrary string labels
  metadata:  map<string, scalar> # arbitrary scalar metadata
  createdAt: u64                  # unix millis
  accessCount: u32               # incremented on each recall hit
  lastAccessedAt: u64            # unix millis
}
```

- `key` is derived from the user-provided semantic key (see §3).
- `value` is treated as opaque bytes by the substrate. Encoding is the caller's responsibility.
- Implementations MUST enforce the value cap; the default is 16 MiB, configurable to 256 MiB. Larger values belong in a blob store; pair the blob's CAS hash with Kanerva.

### 2.2 Store

A **store** is the persistent container:

```
Store := {
  spec_version: "0.1.0"
  dimension:    u32
  created_at:   u64
  items:        Set<Item>
  cleanup:      CleanupMemory  # see §4
  config:       StoreConfig
}
```

---

## 3. The user-facing API (canonical surface)

Every binding MUST expose this surface. Idiomatic naming per language is allowed (`put` in TS, `put` in Rust, `put` in Python, `put` in Kotlin — keep it the same), but the semantics MUST match.

```
open(config)              -> Store
put(store, key, value, opts?) -> Item
recall(store, cue, opts?) -> [Match]
bind(a, b)                -> Hypervector
unbind(a, b)              -> Hypervector
encode(input)             -> Hypervector
consolidate(store, opts?) -> ConsolidationReport
snapshot(store, sink)     -> KmfSnapshot
restore(source)           -> Store
delete(store, id)         -> bool
```

### 3.1 `put`

```
put(store, key, value, { tags?, metadata?, id? })
```

- `key` MAY be: a string (encoded per §3.5), a `Hypervector`, or an embedding (a `float[]`, encoded per §3.6).
- Stores the item; returns the canonical `Item`.
- If `id` is supplied and already exists, behavior is **upsert** (the existing item's `key`, `value`, `tags`, `metadata` are replaced; `createdAt` is preserved; `lastAccessedAt` is updated).

### 3.2 `recall`

```
recall(store, cue, { topK = 10, minSimilarity = 0.5, filter? }) -> [Match]
```

Where `Match := { item, similarity }`.

- `cue` MAY be the same types as a `put` key.
- Results are sorted by `similarity` descending.
- `filter` is an optional predicate over `tags` and `metadata` applied **before** similarity ranking.
- Each match increments the item's `accessCount` and updates `lastAccessedAt`.

### 3.3 `bind` / `unbind`

```
bind(a, b)   := a XOR b      # commutative, associative
unbind(a, b) := a XOR b      # same op; the name signals intent
```

Used to encode role-filler pairs:

```
let subject  = encode("subject")
let cat      = encode("cat")
let bound    = bind(subject, cat)
// later
let filler   = unbind(bound, subject)   // ≈ cat (subject to noise)
```

### 3.4 `consolidate`

Performs Hebbian compaction (§5). MUST be idempotent on a substrate with no recent access activity.

### 3.5 String encoding

```
encode(string s) := randomHV(BLAKE3("str:" || s))
```

### 3.6 Embedding encoding

```
encode(float[] v) := levelHV(v)
```

Where `levelHV` performs **thermometer encoding** with `L = 100` levels per dimension and **random projection** down to `D` bits. The exact algorithm is normative and lives in §A.1 of this document (appendix). The intent: two embeddings with cosine similarity `s` should produce hypervectors with `sim ≈ s`.

### 3.7 Errors

Every binding MUST surface these error kinds:

- `DimensionMismatch` — operating on hypervectors of different `D`.
- `ValueTooLarge` — payload exceeds configured cap.
- `NotFound` — id-based lookups when the item is absent.
- `CorruptSnapshot` — KMF validation failed.
- `UnsupportedSpecVersion` — KMF spec_version is newer than the runtime supports.

Mapping to language-native error types (Result in Rust, exceptions in JS/Python/Kotlin/Swift) is the binding's choice, but the *kinds* and their *semantics* are normative.

---

## 4. Cleanup memory

The cleanup memory is the **content-addressable index** over stored items. Given a noisy hypervector, it returns the nearest stored hypervectors and their similarities.

### 4.1 Required interface

```
cleanup.search(cue: Hypervector, k: u32) -> [{ id, similarity }]
```

### 4.2 Implementation guidance

- Phase 1 (TS reference): **brute-force linear scan**, parallelized via TypedArrays. Acceptable up to `~100k` items at `D = 10000`.
- Phase 2+ (Rust): hybrid — brute force below a threshold, LSH or learned index above.
- All implementations MUST produce identical top-`k` results for any given `(cue, k, substrate state)`. Tiebreakers on equal similarity are by `id` lexicographic ascending.

This identity-of-results requirement is what makes cross-binding test suites possible.

---

## 5. Hebbian consolidation

Periodically (or on demand via `consolidate()`), the substrate reshapes itself:

### 5.1 Co-activation tracking

For each pair of items `(a, b)` that have both been hit by `recall` within a sliding window `W` (default: 1000 recalls), increment a co-activation counter `c(a, b)`.

### 5.2 Binding pull

When `c(a, b) > threshold_pull` (default: 32), modify `a.key` and `b.key` to be *slightly more similar* by flipping a deterministically-chosen subset of disagreeing bits. Flip count is bounded so the keys move by at most `0.02` in `sim` per consolidation pass.

### 5.3 Cold summarization

Items with `lastAccessedAt` older than `T_cold` (default: 30 days) AND `accessCount < N_cold` (default: 3) are eligible for **bundling**: the substrate may replace `n` cold items with a single bundled hypervector that retains approximate similarity to each. The original `value` payloads are moved to a `kanerva://attic/<bundle-id>` sub-store from which they can be cheaply re-instantiated on a near-hit.

### 5.4 Determinism

`consolidate()` MUST be deterministic given identical substrate state and identical co-activation history. This makes consolidation replayable from a KMF snapshot + access log.

---

## 6. Persistence adapters

Adapters implement the following minimal interface:

```
loadVectorTable() -> KmfTable     # full state at last snapshot
appendUpdate(op)                  # append to the WAL since last snapshot
snapshot(table) -> handle         # atomic write of a new full snapshot
```

Reference adapters (Phase 4):

| Adapter | Platform | Notes |
|---|---|---|
| `memory`     | all      | Ephemeral, no persistence. |
| `indexeddb`  | browser  | Snapshot + WAL in two object stores. |
| `sqlite`     | native   | WAL in a journal table; snapshot in a blob. |
| `fs`         | server   | Append-only log + periodic snapshot file. |
| `s3`         | cloud    | Snapshot in one object; WAL in chunked objects keyed by epoch. |

Adapters are *not* part of the spec's hot path. The spec defines what `loadVectorTable` returns and what `appendUpdate` accepts; the implementation is the adapter's business.

---

## 7. Configuration

```
StoreConfig := {
  dimension:      u32   = 10000
  valueCapBytes:  u32   = 16 * 1024 * 1024
  backend:        "memory" | "indexeddb" | "sqlite" | "fs" | "s3" | custom adapter
  consolidation: {
    enabled:           bool = true
    intervalMs:        u32  = 60_000
    coldDays:          u32  = 30
    coldMinAccess:     u32  = 3
    pullThreshold:     u32  = 32
  }
  recall: {
    defaultTopK:       u32  = 10
    defaultMinSim:     f32  = 0.5
  }
  serialization: {
    kmf_compression:  "none" | "zstd" = "zstd"
    kmf_compress_level: u8   = 3
  }
}
```

---

## 8. KMF — Kanerva Memory Format (open wire format)

KMF is the **persistent, implementation-independent** wire format for Kanerva substrates. It is positioned for the same standards path as Apache Parquet and Apache Iceberg: a canonical layout that any conformant implementation can read and write.

### 8.1 Goals

- **Implementation-independent**: a snapshot written by core-rs MUST be readable byte-for-byte by core-ts and vice versa.
- **Streaming-friendly**: vectors are stored in chunked, column-major segments to allow partial loads.
- **Versioned**: every snapshot carries a spec version; readers reject newer majors.
- **Compressible**: hypervector blocks are zstd-compressible to ~30% of raw size on typical data.
- **Verifiable**: every block is BLAKE3-checksummed; corruption is detectable.

### 8.2 File structure (v0.1.0)

```
+----------------------------------------------------------+
| Magic            "KMF\x00"                4 bytes        |
| Spec version     "0.1.0"                  6 bytes        |
| Header offset    u64 LE                   8 bytes        |
+----------------------------------------------------------+
| ... data blocks ...                                      |
+----------------------------------------------------------+
| Header (JSON-encoded, zstd-compressed)                   |
|   { dimension, item_count, created_at,                   |
|     index: [{ kind, offset, length, blake3 }] }          |
+----------------------------------------------------------+
| Trailer magic    "FMK\x00"                4 bytes        |
+----------------------------------------------------------+
```

Block kinds:

| Kind | Contents |
|---|---|
| `hv_block`   | `n × ceil(D/8)` packed hypervectors. |
| `meta_block` | `n` rows of `{ id, tags, metadata, createdAt, accessCount, lastAccessedAt }` in MessagePack. |
| `value_block` | `n` value payloads, length-prefixed. |
| `attic_block` | Cold-summary entries (see §5.3). |

Detailed byte layouts live in `docs/kmf-v0.1.0.md` (to be authored in Phase 1 alongside the reference reader/writer).

### 8.3 Conformance

A conformant KMF reader MUST:

1. Reject files where major version > supported major.
2. Verify the trailer magic before trusting any offsets.
3. Verify each block's BLAKE3 against the header before deserializing.
4. Produce a substrate whose `recall` output matches any other conformant implementation **bit-exactly** for the same query.

The cross-implementation test suite in `tests/conformance/` is the canonical check.

### 8.4 Standardization path

After v1.0.0 of the spec, we will:

1. Publish KMF as an [IETF Independent Submission](https://www.rfc-editor.org/about/independent/).
2. Open a [W3C Community Group](https://www.w3.org/community/groups/) for ongoing evolution.
3. License the spec document under CC-BY-4.0 (the reference implementations are Apache-2.0).

---

## 9. Cross-binding conformance

The single source of truth for "did I implement the spec correctly?" is the **conformance corpus** in `tests/conformance/`:

- A set of fixed seeds, inputs, and expected hypervectors.
- A set of operations (puts, recalls, consolidations) and their expected outcomes.
- A set of KMF golden files that round-trip.

Every binding MUST run the corpus in CI. A binding that does not pass is not Kanerva.

---

## 10. Versioning

- **Spec version** (this document): semver. Major bumps allow wire-format breakage; minor bumps add backward-compatible fields.
- **Implementation version** (each package): independent semver; must declare which spec version it implements.
- **KMF format version**: tied to spec version. KMF readers accept their own major plus all earlier.

---

## 11. Open questions (resolve before v0.1.0 freeze)

1. **Default `D`**: 8192 (SIMD-aligned, 1 KiB per vector) vs. 10000 (literature default, awkward alignment). *Leaning 8192.*
2. **Bipolar vs binary**: re-evaluate after the Phase 0 notebook. Binary is the working assumption.
3. **`metadata` value types**: strict scalar (string/int/float/bool) or allow nested? *Leaning strict.*
4. **Cleanup memory hot index**: ship LSH from day one, or start brute-force and add LSH in v0.2.0? *Leaning brute-force-first.*
5. **`encode` for byte strings**: hash-based (current §3.5) or content-shingle-based (for partial-byte-cue support)? *Open.*
6. **Multi-tenancy**: an explicit `namespace` field in `Item`, or compose tags? *Leaning explicit namespace because permissions hang off it.*

Each open question is tracked as a `spec` issue.

---

## Appendix A — Normative algorithms

### A.1 Embedding-to-hypervector encoding

```
function levelHV(embedding: float[]):
  let v = bits(D)                 # all zeros
  for i in 0..len(embedding):
    let lvl = clamp(round((embedding[i] + 1) * (L - 1) / 2), 0, L - 1)
    let proj_seed = "lvl:" || i || ":" || lvl
    v = v XOR randomHV(BLAKE3(proj_seed))
  return v
```

This is the binary thermometer-and-projection scheme from Imani et al. (2017), adapted for our `randomHV`.

### A.2 BLAKE3-XOF expansion

We use BLAKE3 in XOF mode, calling the standard `blake3_xof(seed, out, out_len)` from the reference C/Rust libraries. JS implementations may use the WebAssembly build of `@noble/hashes/blake3`.

---

## Appendix B — References

- Kanerva, P. (1988). *Sparse Distributed Memory*. MIT Press.
- Plate, T. (1995). Holographic reduced representations. *IEEE TNN*, 6(3).
- Kanerva, P. (2009). Hyperdimensional computing. *Cognitive Computation*, 1(2).
- Imani, M., et al. (2017). Voicehd: Hyperdimensional computing for efficient speech recognition. *ICRC*.
- Schlegel, K., et al. (2022). A comparison of vector symbolic architectures. *Artificial Intelligence Review*.
- Kleyko, D., et al. (2023). A survey on hyperdimensional computing aka vector symbolic architectures. *ACM Computing Surveys*.

---

*Phase 0 deliverable: this document, locked, alongside the validation notebook in `notebooks/phase0_hdc_validation.ipynb`.*
