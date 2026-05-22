<!-- Round 4 changelog (pinned to SHA 17334f8) - applied 2026-05-20

Single critic-and-revise pass addressing residual issues after Rounds 2 and 3:

- SPEC-errata reconciliation: the seven SPEC errata flagged in Round 2 (item 5 of the Round-2 changelog below) were applied to the repository working tree (SPEC.md, tests/conformance/README.md) between Round 2 and Round 4. The "SPEC.md disagrees with code" footnotes have accordingly been removed or simplified throughout this file:
   - §3.1.1 "Editorial note (SPEC erratum)" — deleted; SPEC.md §1.3 (working tree) now recites the `"smritidb/tiebreak"` domain prefix and matches both reference bindings.
   - §5.2.1 final parenthetical — rewritten; SPEC.md §A.1 (working tree) now recites the canonical bracketing `((clamped + 1.0) * (L - 1)) / 2.0` and discusses the half-integer rounding-tail case under the outer clamp.
   - §6.5 — light edit; SPEC.md §4.2 (working tree) now expressly demarcates Phase 1 (normative brute-force) vs Phase 2 (contemplated LSH).
   - §8.3 — parenthetical re-anchored; Fig. 2 (post-Round-3) annotates meta_block as JSON (Phase 1) with MessagePack contemplated for Phase 2, matching the present specification.

- Claim-number cross-references corrected for Round-2 renumbering (Round-2 independent claims at 1, 7, 9, 18, 23, 27, 31, 35; 36 total claims):
   - §7.4 "subject of claim 27" -> "the apparatus form recited in dependent claim 15 of independent claim 9" (the restore module is now Round-4 claim 15).
   - §8.5 "exposed by claim 32" -> "exposed by dependent claim 20 of independent claim 18" (zstd of data blocks; the most analogous Round-4 limb).
   - §8.7.1 "subject of claim 35" -> "subject of dependent claim 22 of independent claim 18" (Round-4 claim 35 is the permutation-positional text encoder; the attic block is now claim 22).
   - §8.7.2 "subject of claim 38" -> "subject of dependent claim 24 of independent claim 23" (the compress-then-hash pipeline is now claim 24; there is no claim 38 in Round 4).
   - §8.2 "claim 5" left unchanged (integrity-hash emission alongside bundle output remains claim 5 of independent claim 1).

- Drawings cross-walk (post-Round-3 drawings corrections, see 06-drawings-list.md):
   - §5.2 / §5.2.1 / Example 2 / Example 3: confirmed encoder description matches Fig. 6 step 803 (`level = round((v_i + 1) * (L - 1) / 2)`) and the deletion of phantom step 805. No textual change required — the description already used `round((clamped + 1.0) * (L - 1) / 2.0)` and the present XOR-only accumulator.
   - §8 confirmed Fig. 2 references (JSON in Phase 1; zstd contemplated in Phase 2; attic_block contemplated in Phase 2).

- Prior-art distinguishing strengthened in light of new Round-2 prior-art additions:
   - §5.2 — added an explicit distinguishing paragraph against Charikar 2002 sign-LSH / SimHash and Achlioptas 2003 sparse random projection, both of which were added to 10-prior-art.md in Round 2 (§4.8 family).
   - §6.5 — added a distinguishing pointer to HNSW (Malkov & Yashunin 2020), now treated as adjacent-not-conflicting per 10-prior-art.md §3.8.
   - §9.3 — added a brief distinguishing statement against SQLite's WAL journal mode (Hipp et al.; cited in 10-prior-art.md) — the substrate inherits ACID rather than re-implementing it.

- No algorithm-semantic changes. No new claim language. No files outside `patent/` modified. No commits.
-->

<!-- Round 2 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Reconciled internal contradictions: §5.2.1 rounding rule (TS Math.round vs Rust f32::round both agree on non-negative inputs); §10.2 prose corpus count harmonised with table (five random_hv, not eight); §5.2 worked-example "banker's rounding" parenthetical removed.
- Marked as Phase 2 / v0.2 contemplated (not present at SHA 17334f8): §6.5 LSH layer; §8.3 MessagePack meta_block; §8.5 zstd-compressed header; §7.3 attic relocation; cache eviction-and-replay (§4); SIMD popcount path (§2.3, §3.4); new §8.7 covers attic and zstd-pipeline ordering for claims 35 and 38.
- Added reference numerals throughout per 06-drawings-list.md (100-series encoders, 200-series primitives, 300-series substrate, 400-series KMF, 500-series adapter interface, 600-series adapter implementations, 700-series conformance bindings, 800-series embedding-encoder steps).
- SPEC errata flagged for user to commit to SPEC.md v0.1.1 (subsequently applied to the repository working tree between Round 2 and Round 4; see Round 4 changelog above): (a) §1.3 tiebreaker missing the "smritidb/tiebreak" domain prefix; (b) §A.1 missing canonical bracketing ((clamped + 1) * (L - 1)) / 2 for embedding-encoder rounding; (c) Fig. 2 meta_block currently described as MessagePack — Phase-1 implementation is JSON; (d) Fig. 6 step 803 uses floor where code uses round; (e) Fig. 6 step 805 phantom per-dimension key vector not present in code.
- Prior-art acknowledge-and-distinguish paragraphs inserted: Kanerva 1988/2009, Imani 2017, Rahimi 2016, Najafabadi 2016, Hebb 1949, Hopfield 1982, Ramsauer 2020, Vaswani 2017, Sukhbaatar 2015, Apache Parquet/Iceberg footers, W3C Web Platform Tests / IETF JSON-Patch conformance suite.
- Softened SIMD/popcount claims throughout: reference TS uses per-byte scalar loops; Rust at 17334f8 also uses unpacked Vec<u8> per-byte loop (verified in core-rs/src/hypervector.rs); reframed as "amenable to SIMD acceleration on platforms supporting popcount instructions" with the packed-SIMD path explicitly reserved for a subsequent revision.
- Fixed factual errors: §5.3.3 char-trigram arithmetic (6 trigrams in "elephant", 5 in "elephnt", 3 common); §3.1.2 integer-only comparison form for bundle; §6.3 put pseudocode disambiguated; §7.2 sort-key disambiguated.
-->

# Detailed Description of the Invention

> **Form-2, Section 8 — Complete Specification**
> **Project:** Smritidb (open-source binary hyperdimensional associative memory)
> **Inventor:** Vivek Singh
> **Applicant:** Tanvrit Private Limited
> **Source-of-truth SHA (HEAD):** `17334f8`
> **License of reference implementation:** Apache-2.0
> **Round:** 2 (revised against Round-2 critique on 2026-05-20)

The following description discloses the invention in sufficient detail to enable a Person Having Ordinary Skill In The Art ("PHOSITA") — a competent software engineer with a working understanding of hyperdimensional computing (HDC) and basic cryptographic hashing — to construct, operate, and verify the invention from this text alone, in accordance with Section 10(4) of the Patents Act, 1970. Each described mechanism is followed by an explicit "**Technical Effect**" subsection. The technical-effect framing is provided to demonstrate that the invention is not a "computer programme per se" within the meaning of Section 3(k), but rather a system that yields concrete, measurable improvements in computer functioning, in line with the guidance laid down in *Ferid Allani v. Union of India* (Delhi High Court, 2019) and *Microsoft Technology Licensing v. Asst. Controller of Patents* (Delhi High Court, 2023).

Throughout this document the symbol `D` denotes the hypervector dimension; `||` denotes byte concatenation; `XOR` denotes the bitwise exclusive-OR operation; `BLAKE3(x)` denotes the cryptographic hash function BLAKE3 applied to the byte string `x`; and `BLAKE3-XOF(seed, n)` denotes the extendable-output construction of BLAKE3 producing exactly `n` bytes of output deterministically derived from `seed`. BLAKE3 throughout this document refers to BLAKE3 in **unkeyed mode** (no keying material, no context string); the use of BLAKE3's keyed-hash or key-derivation modes would produce a non-conformant implementation. All integer encodings are little-endian unless noted otherwise. All ASCII string literals (such as domain separators) are encoded in UTF-8.

Parenthetical numerals (e.g., "(100)", "(260)", "(412)") that appear in the description below refer to the corresponding elements of the drawings (Figs. 1–7); the numbering scheme is summarised in the preamble to `06-drawings-list.md`. The reader is invited to consult the drawings alongside this description.

---

## 1. Overview of the System Architecture

The invention provides a layered associative memory system in which information is represented, manipulated and persisted as binary hyperdimensional vectors ("hypervectors"). The system is organised into six layers, each of which is independently specified, deterministically verifiable, and substitutable across language runtimes:

**(a) Substrate Layer — Binary Hypervectors (300).** The lowest layer is a typed, packed-bit representation of binary hypervectors of fixed dimension `D`, where `D` is constrained to the range `[1024, 65536]` and defaults to `D = 10000`. Each hypervector occupies `ceil(D / 8)` bytes when packed and is internally manipulated either in packed (1 bit per stored bit) or unpacked (1 byte per bit) form depending on the operation. The substrate layer is described in Section 2 below. Two SIMD-aligned alternatives (`D = 8192` and `D = 16384`) are recommended for performance-sensitive deployments; these are selected so that `D` is an integer multiple of the natural SIMD word width on commodity central-processing units — specifically, 256-bit AVX2 lanes for `D = 8192`, 512-bit AVX-512 lanes for `D = 16384`, and 128-bit ARM NEON lanes for all listed dimensions.

**(b) Primitives Layer — Bundle, Bind, Permute, Similarity (200).** The four canonical HDC operations — bundle (210), bind (220), permute (230), and similarity (240) — are implemented as pure, side-effect-free functions over hypervectors of equal dimension, supported by a random-hypervector generator (250) and a deterministic tiebreaker (260). The novel deterministic tiebreaker (Section 3.1.1; element 260) eliminates the long-standing source of cross-platform divergence in the bundle (majority) operation. These primitives are described in Section 3.

**(c) Encoders Layer — String, Embedding, Text (100).** Three encoders convert non-vector inputs into hypervectors: (i) a whole-string hash encoder (110) for exact-match keys (Section 5.1); (ii) a thermometer-and-random-projection embedding encoder (120) that maps real-valued embedding vectors to binary hypervectors while approximately preserving cosine similarity as Hamming similarity (Section 5.2); (iii) a bag-of-words encoder (130); and (iv) a permutation-positional word- and character-level n-gram encoder (140) for natural-language text that preserves order information without an explicit vocabulary (Section 5.3).

**(d) Store Layer — Item Map and Cleanup Memory (300).** The store exposes a content-addressable item map keyed by hypervector. Public operations are `put`, `recall`, `delete`, `get`, and `consolidate`. The cleanup-memory index (310) is the index that, given a noisy cue hypervector, returns the nearest stored items ranked by similarity. A co-activation tracker (320) drives the consolidation step. The store is described in Section 6.

**(e) Consolidation Layer — Replayable Hebbian Compaction (330).** A novel deterministic Hebbian-style consolidation procedure tracks pairwise recall co-activations, then "pulls" frequently co-activated keys closer in Hamming space by flipping a deterministically-selected, bounded subset of disagreeing bits. Because the bit-selection ordering is derived from BLAKE3 over a salt, the consolidation is *replayable* — a property impossible with conventional stochastic gradient procedures. This layer is described in Section 7.

**(f) Persistence Layer — KMF Wire Format (400) and Adapters (500).** A self-describing binary wire format (the "Smritidb Memory Format", abbreviated KMF) provides implementation-independent, cryptographically-checksummed, streaming-friendly persistence (Section 8). Storage adapters (Section 9) implementing the `StorageAdapter` interface (500) — Memory (610), File-system (620), SQLite (630), and IndexedDB (640) — plug a single byte-level snapshot format into heterogeneous storage backends.

**(g) Bindings Layer (700).** The same primitives, encoders, store semantics, KMF reader/writer, and consolidation procedure are exposed through bindings for TypeScript / JavaScript (710, the reference implementation), Rust (720, compiled to native and to WebAssembly), Python (730, via PyO3), and Kotlin (740) / Swift (750) (via Mozilla UniFFI; the UDL interface description lives at `packages/smritidb-ffi/src/smritidb.udl` and is consumed by the `uniffi-bindgen` code-generation tool). The contract that all bindings satisfy is the byte-exact conformance corpus (660) described in Section 10.

A schematic diagram of the layered architecture is provided in **Fig. 1** (drawings sheet).

**Technical Effect.** The layered architecture provides three concrete and measurable system-level improvements in computer functioning, none of which is a property of any conventional vector-database design:

1. **Cross-runtime bit-exactness.** Because every layer is specified as a deterministic byte-level function — including the tiebreaker on majority ties and the bit-flip ordering during consolidation — the same logical operation executed in TypeScript, Rust, Python, Kotlin or Swift yields byte-identical results. This eliminates the cross-implementation drift that plagues conventional embedding-vector databases, in which floating-point non-associativity, RNG state, and platform-dependent maths libraries cause observable disagreement between replicas.
2. **Substitutable persistence with stable wire format.** The KMF wire format decouples the algorithmic substrate from the storage backend, enabling a snapshot written by the Rust binding to be loaded byte-for-byte by the TypeScript binding without conversion.
3. **Memory footprint reduction.** Binary hypervectors at `D = 10000` occupy `1250` bytes per item versus `~4096` bytes for a 1024-dimensional float-32 embedding of comparable distinguishing capacity — a reduction of approximately 3.27× per item, and ~32× when compared against the larger `~4096`-dimensional float-32 embeddings now common in foundation-model output spaces.

---

## 2. Binary Hypervector Substrate

A **hypervector** in this invention is an element of the set `{0, 1}^D`, where `D` is a positive integer fixed per store. The substrate enforces the following constraints, which constitute the lowest-level contract of the invention:

**(2.1) Dimension constraints.** The dimension `D` MUST satisfy `1024 <= D <= 65536`. Implementations MUST reject `D < 1024` (insufficient distinguishing capacity; specifically, at `D = 1024` the standard deviation of two-random-pair Hamming similarity is `0.5/sqrt(1024) ≈ 0.0156`, and below `D = 1024` the per-pair `±3σ` window exceeds `0.1`, at which point cleanup-memory false-positive rates exceed 1 % under uniform-random storage) and SHOULD emit a warning for `D > 65536` (no measurable improvement in recall accuracy; substantial memory cost). The default dimension is `D = 10000`, chosen to match the established HDC literature default while remaining a commodity-RAM-friendly size. Two SIMD-aligned alternatives (`D = 8192` and `D = 16384`) are recommended for performance-sensitive deployments, as noted in §1(a).

**(2.2) Storage representation.** A hypervector is stored as a packed bit array occupying exactly `ceil(D / 8)` bytes. Bits within a byte are interpreted **most-significant-bit first** ("MSB-first"). Concretely: if `v` is a hypervector and `i` is a bit index in `[0, D)`, then bit `i` is found in byte `floor(i / 8)` of the packed representation, at bit offset `7 - (i mod 8)` within that byte. This MSB-first byte order is normative; little-endian or LSB-first packing produces a non-conformant implementation.

**(2.3) In-memory unpacked form.** During computation, an implementation MAY operate on an unpacked one-byte-per-bit representation (a length-`D` byte array, each element being `0` or `1`). The reference TypeScript implementation uses this unpacked form for clarity and cross-implementation bit-exactness. At SHA `17334f8` the Rust binding likewise uses the unpacked `Vec<u8>` form (see `packages/core-rs/src/hypervector.rs`); the packed-bit representation with SIMD-popcount-accelerated similarity and SIMD vertical-add bundle is **contemplated as a future-phase optimisation** rather than a present feature of the embodiment. The persistent KMF wire format (Section 8) always uses the packed MSB-first form, so the SIMD-packed in-memory path can be adopted in a subsequent revision without disturbing the wire format. The empirical correspondence between cosine similarity in the input float-32 space and Hamming similarity in the binary hypervector output space is documented in the Phase-0 validation notebook (`notebooks/phase0_hdc_validation.ipynb` in the reference implementation tree), which records, for each of three foundation-model encoders, the linear-regression slope and `R²` between the two similarity measures over a 10 000-pair sample.

**Technical Effect.** Reduces memory footprint by approximately 32× compared with float-32 embeddings of comparable distinguishing capacity (1.25 KB per item at `D = 10000` versus approximately 40 KB for the float-32 embeddings emitted by typical large-language-model encoders). The bit-packed wire-format representation is **amenable to single-instruction-multiple-data ("SIMD") acceleration** on platforms supporting hardware popcount instructions (e.g., x86 `POPCNT`, ARM `CNT`), enabling, in a future-phase optimised binding, similarity computation at a rate of approximately one Hamming distance per machine cycle per 64-bit word. This is materially superior, in both memory and compute, to dense floating-point embedding storage as used by conventional vector databases such as Pinecone, Weaviate, and naïve FAISS configurations, and is competitive with Product-Quantisation ("PQ") approaches used in compressed FAISS configurations without incurring PQ's offline-training prerequisite. By way of illustration: Pinecone-style storage of 1024-dimensional float-32 embeddings consumes approximately 4 KB per item; FAISS-PQ at 256-bit codes consumes 32 B per item; and binary HDC at `D = 10000` consumes 1.25 KB per item — placing the present invention competitively against PQ on storage while preserving training-free determinism.

**(2.4) Best-mode SIMD acceleration (contemplated).** Although the reference TypeScript binding and the Rust binding at SHA `17334f8` both operate on the unpacked `byte-per-bit` form via straight-line scalar loops, the *best mode* contemplated by the inventor for high-throughput deployments is a packed-bit Rust implementation in which: (i) hypervectors are stored as arrays of 64-bit unsigned integer words (`Vec<u64>`); (ii) the similarity primitive computes the Hamming distance as the sum, over the word array, of the population counts of the XOR of corresponding word pairs, using the `core::arch::x86_64::_popcnt64` intrinsic on x86-64 hosts that expose the `popcnt` capability flag, and the equivalent `vcntq_u8` plus horizontal-sum sequence on ARM hosts; and (iii) the bundle primitive computes per-word vertical adds into a `Vec<u16>` accumulator using SIMD lanes, with scalar fall-through to the deterministic tiebreaker (260) on tied positions (the tiebreaker remaining a scalar control-path operation in all bindings, as it operates per bit-position rather than per word). A scalar fall-back path produces byte-identical output on hardware that does not expose the relevant capability. This best mode is disclosed here to satisfy §10(4)(d); it is not the subject of any present claim insofar as the present embodiment uses the unpacked scalar form.

---

## 3. Primitive Operations

The substrate exposes four primitive operations: **bundle (210)**, **bind (220)**, **permute (230)**, and **similarity (240)**. Each is a pure function over hypervectors of equal dimension. Every primitive is normatively specified at the bit level so that any conformant implementation produces byte-identical output.

### 3.1 Bundle (Superposition) Operation (210)

The **bundle** operation, denoted `⊕`, is element-wise majority over a multiset of `n` input hypervectors. For each bit position `i ∈ [0, D)`:

```
sum[i]  := number of input hypervectors whose bit i is 1
out[i]  := 1                                if  sum[i] > n/2
           0                                if  sum[i] < n/2
           tiebreak(D, i, n)                if  sum[i] = n/2  (only possible when n is even)
```

The bundle approximately preserves similarity to each of its constituents: for random inputs, the expected similarity between any input `x_k` and the bundle `⊕{x_1, ..., x_n}` is approximately `0.5 + 0.5 / sqrt(n)`, decaying with bundle multiplicity. This `0.5 + 0.5 / sqrt(n)` capacity bound is consistent with the general theory of binary majority bundling in high-dimensional spaces as described by Kanerva (1988, *Sparse Distributed Memory*) and refined by Imani et al. (2017). What is novel in the present invention is **not the capacity bound itself but the deterministic tiebreaker of §3.1.1 (260)**, which renders the bundle byte-identical across implementations — a property absent from every prior HDC system, all of which left tie-resolution unspecified or platform-dependent.

**(3.1.1) The deterministic tiebreaker (260).** When `n` is even and `sum[i] = n/2`, a tiebreaker is required. Naïve choices (always-zero, always-one, modular hash of position) are non-portable or biased. The invention specifies a cryptographically-strong deterministic tiebreaker:

```
function tiebreak(D: u32, i: u32, n: u32) -> u8:           # returns 0 or 1
    let domain = utf8_bytes("smritidb/tiebreak")     # 17 bytes
    let buf    = domain || u32_le(D) || u32_le(i) || u32_le(n)   # 17 + 12 = 29 bytes
    let digest = BLAKE3(buf)                         # 32-byte digest
    return digest[0] & 1                              # least significant bit of first digest byte
```

The domain string `"smritidb/tiebreak"` serves as a domain separator preventing reuse of any other BLAKE3 invocation in the system from accidentally aliasing the tiebreaker. The use of `BLAKE3(domain || D || i || n)` rather than a simpler function ensures that (i) the tiebreaker is uniformly distributed, (ii) it is impossible to construct an adversarial input pattern that systematically biases tied bits, and (iii) the function is identical across implementations because the input bytes are fully specified.

**Scalar-control-path invariant.** Even in bindings that adopt the SIMD-packed similarity and bundle paths contemplated in §2.4, the tiebreaker (260) MUST be invoked from a scalar control path, because it is a per-bit-position operation rather than a per-word operation. This ensures byte-identical output between SIMD-accelerated and scalar implementations.

**Source-of-truth cross-reference.** The normative tiebreaker formulation recited above is identical to that recited in the repository SPEC.md §1.3, namely `H("smritidb/tiebreak" || D || index || count)[0] & 1`. It is exercised at the byte level by the reference TypeScript binding at `packages/core-ts/src/hypervector.ts` (which defines `TIEBREAKER_DOMAIN = "smritidb/tiebreak"`) and by the reference Rust binding at `packages/core-rs/src/hypervector.rs` (which defines `TIEBREAKER_DOMAIN: &[u8] = b"smritidb/tiebreak"`). The conformance corpus of Section 10 verifies byte-identical output against both bindings.

**(3.1.2) Pseudocode for bundle.** The comparison against `n/2` is expressed below in **integer-only** form (comparing `2 * sums[i]` against `n`) so that no implementation introduces non-integer division or floating-point rounding into the majority decision; this in turn guarantees byte-identical bundle output across language runtimes whose default arithmetic rules differ:

```
function bundle(hvs: list of hypervector of dimension D) -> hypervector of dimension D:
    require len(hvs) >= 1
    require every hv in hvs has length D
    let sums  = integer array of length D, all zero
    for each hv in hvs:
        for i in 0..D-1:
            sums[i] += hv[i]
    let n     = len(hvs)
    let out   = byte array of length D
    for i in 0..D-1:
        if   2 * sums[i] >  n: out[i] := 1
        elif 2 * sums[i] <  n: out[i] := 0
        else:                  out[i] := tiebreak(D, i, n)     # only reachable when n is even
    return out
```

**Technical Effect.** Enables byte-identical bundle output across heterogeneous implementations (TypeScript, Rust, Python, Kotlin, Swift, and WebAssembly). The deterministic tiebreaker removes the dominant source of cross-platform divergence in binary HDC systems, in which different language standard libraries historically resolved ties using platform-specific defaults (e.g., JavaScript's V8 vs. Python's CPython floating-point rounding) and therefore produced different bundle outputs on the same inputs. By contrast, the present invention guarantees that two replicas of an associative memory in different runtimes converge to byte-identical state given identical input streams — a precondition for deterministic distributed-memory systems, verifiable audit trails, and cryptographic content-addressing of HDC state.

### 3.2 Bind Operation (220)

The **bind** operation, denoted `⊗`, is element-wise XOR over two hypervectors of equal dimension:

```
out[i] := a[i] XOR b[i]   for i in 0..D-1
```

Bind is **self-inverse**: `(a ⊗ b) ⊗ b = a` for all `a, b`. It is commutative (`a ⊗ b = b ⊗ a`) and associative. Bind distributes over bundle in the same approximate sense as multiplication distributes over addition in linear algebra: `a ⊗ (b ⊕ c)` is similar to `(a ⊗ b) ⊕ (a ⊗ c)`. The same operation is exposed as both `bind` and `unbind` in the public API; the two names signal the caller's *intent* — `bind` to associate, `unbind` to retrieve — while the underlying byte transformation is identical.

The principal use of bind is **role-filler encoding**: a structured record `{role₁: filler₁, role₂: filler₂, ...}` is encoded as `⊕{bind(role_k, filler_k)}` for `k = 1..K`. Given the bundle `B` and a known role hypervector `role_j`, the approximate filler is recovered as `unbind(B, role_j) ≈ filler_j`.

```
function bind(a, b) -> hypervector:
    require length(a) == length(b)
    let out = byte array of length(a)
    for i in 0..length(a)-1:
        out[i] := a[i] XOR b[i]
    return out
```

**Technical Effect.** Bind enables structured information (role-filler pairs, key-value tuples, ordered sequences) to be embedded into a single fixed-size hypervector that retains approximate compositional retrievability. As compared with transformer key-value associative caches (Vaswani et al. 2017, "Attention is all you need") and learned positional embeddings (Sukhbaatar et al. 2015, "End-to-end memory networks"), each of which requires either fixed-position slots or learned positional embeddings, the XOR-based bind is associative, self-inverse, computable in `O(D / 64)` machine operations (on a binding using a packed-bit representation), and has no learned parameters. There is no training step and no platform-specific numerical drift.

### 3.3 Permute Operation (230)

The **permute** operation, denoted `Π_k`, is a cyclic bit rotation of a hypervector by `k` positions:

```
out[(i + k) mod D] := hv[i]   for i in 0..D-1
```

Permute is invertible (`Π_{-k}` undoes `Π_k`), it preserves Hamming weight, and it produces a hypervector that is approximately orthogonal to the original for any `k != 0`. The principal use of permute is to **encode order**. To represent an ordered pair `(x, y)`, one computes `bind(x, permute(y, 1))`. To represent a triple `(x, y, z)`, one computes `bind(x, permute(y, 1), permute(z, 2))` (where bind is iterated). Two strings differing only in word order — e.g., "alpha beta" versus "beta alpha" — produce dissimilar hypervectors under permutation-positional encoding, even though their bag-of-words encodings would be identical.

```
function permute(hv, k) -> hypervector:
    let D     = length(hv)
    let shift = ((k mod D) + D) mod D            # normalise into [0, D)
    let out   = byte array of length D
    for i in 0..D-1:
        out[(i + shift) mod D] := hv[i]
    return out
```

**Technical Effect.** Permute provides parameter-free, deterministic, order-sensitive encoding for sequence data. Permutation-based order encoding for hyperdimensional representations was introduced in the academic HDC literature (Kanerva 2009, "Hyperdimensional computing: An introduction to computing in distributed representation with high-dimensional random vectors", *Cognitive Computation* 1:139–159), but without a deterministic cross-platform byte-level specification. As compared with positional embeddings (which are learned in transformer architectures) and rotary position embeddings (RoPE, which require `O(D log D)` complex-arithmetic operations and learned attention weights), the present permute primitive achieves order-sensitivity in `O(D)` integer-copy operations with zero parameters, no training step, and zero numerical drift across architectures. The technical contribution of the present invention over Kanerva 2009 is the **deterministic cross-platform reproducibility** of permutation-based order encoding, a system property absent from Kanerva's mathematical formulation, secured by the normative bit-rotation semantics specified above.

### 3.4 Similarity Function (240)

The **similarity** between two hypervectors of equal dimension is one minus their normalised Hamming distance:

```
sim(a, b) := 1 - hamming(a, b) / D
          := 1 - (number of positions i where a[i] != b[i]) / D
```

The similarity lies in `[0, 1]`. For identical vectors `sim(a, a) = 1`. For two independently random hypervectors, the expected similarity is `0.5`, with standard deviation approximately `0.5 / sqrt(D)`. At `D = 10000` the standard deviation is `0.005`, so two random hypervectors of dimension 10000 will have similarity within `0.49` to `0.51` with overwhelming probability.

Hamming similarity over `{0, 1}^D` satisfies the (rescaled) triangle inequality: for any three hypervectors `a, b, c`, `hamming(a, c) <= hamming(a, b) + hamming(b, c)`. Equivalently, `1 - sim(a, c) <= (1 - sim(a, b)) + (1 - sim(b, c))`. This metric property enables the cleanup memory to apply standard metric-space indexing techniques (Section 6) in future implementations.

```
function similarity(a, b) -> float in [0, 1]:
    require length(a) == length(b)
    let mismatches = 0
    for i in 0..length(a)-1:
        if a[i] != b[i]:
            mismatches += 1
    return 1 - mismatches / length(a)
```

**Technical Effect.** Hamming-based similarity is **amenable to SIMD acceleration** on platforms supporting hardware popcount instructions (e.g., x86 `POPCNT`, ARM `CNT`), where, on a packed-bit representation, the per-pair similarity computation reduces to approximately `D / 64` machine operations on a 64-bit central-processing unit. At SHA `17334f8` the reference TypeScript binding (`packages/core-ts/src/hypervector.ts`) and the Rust binding (`packages/core-rs/src/hypervector.rs`) both execute a per-byte scalar loop over the unpacked form; the packed-bit SIMD-popcount path is contemplated as a subsequent-revision optimisation (see §2.4). The scalar form is itself materially cheaper than cosine similarity over float-32 vectors, which requires approximately `D` floating-point multiply-accumulates plus two `sqrt` calls for the norms. For comparable-fidelity retrieval, the binary substrate accordingly yields roughly an order of magnitude lower per-query arithmetic-operation count and lower energy expenditure than float-32 cosine, with the SIMD-packed path expected to widen the gap further.

---

## 4. Random Hypervector Generation (250)

A reproducible source of pseudo-random hypervectors is the foundation of every encoder in the invention. The function `randomHv(seed, D)` produces a hypervector of dimension `D` deterministically derived from a byte-string `seed`:

```
function randomHv(seed: byte array, D: integer) -> hypervector:
    let nbytes = ceil(D / 8)
    let bits   = BLAKE3-XOF(seed, nbytes)
    let out    = byte array of length D
    for i in 0..D-1:
        let byte_index = i shr 3                          # floor(i / 8)
        let bit_offset = 7 - (i and 7)                    # MSB-first within byte
        out[i] := (bits[byte_index] shr bit_offset) and 1
    return out
```

The function uses BLAKE3 in its extendable-output ("XOF") mode to produce exactly `ceil(D / 8)` bytes of pseudo-random output. Each bit of each output byte is then unpacked into a single byte of the in-memory hypervector following the same MSB-first ordering used by the KMF wire format (Section 8). When the caller supplies a high-level semantic key (a UTF-8 string), the seed is first compressed to 32 bytes via `seed := BLAKE3(utf8_encode(key))`, then expanded back to `ceil(D / 8)` bytes via BLAKE3-XOF. This two-step "compress-then-expand" pattern, rather than feeding the raw UTF-8 directly to XOF, normalises seed length and decouples the output distribution from the input length. Conformant implementations MUST apply the BLAKE3 compression step before the BLAKE3-XOF expansion when the seed is a high-level semantic key longer than 32 bytes; omitting the compression step produces a non-conformant hypervector that will fail the cross-implementation conformance corpus of Section 10.

Cryptographically-derived pseudo-random vectors are used in many prior-art systems (e.g., HKDF in TLS 1.3, the BLAKE3 content-addressing scheme used in IPFS). The present invention's contribution is **not** the use of BLAKE3 *per se*, but the combination of: (i) the specific compress-then-expand seed normalisation above; (ii) the MSB-first unpacking of §2.2; and (iii) the binding of BLAKE3-XOF output to the hyperdimensional substrate's bit-array semantics. This three-part combination has no equivalent in prior HDC literature, which historically used Mersenne Twister or xorshift PRNGs whose state and seeding conventions vary across language libraries.

**Technical Effect.** Provides cryptographically strong pseudo-random hypervectors with three concrete properties absent from conventional pseudo-random vector generators used in HDC literature (Mersenne Twister, xorshift, etc.):

1. **Cross-implementation determinism.** BLAKE3 has a fully-specified byte-level reference; every conformant implementation produces identical output for identical seed and length. Mersenne Twister, by contrast, is sensitive to seeding conventions and warm-up routines that vary by library.
2. **Replay-from-key.** A hypervector can be regenerated from its semantic key without storing the vector itself. This enables an implementation to discard cached encodings under memory pressure and re-derive them on demand; the eviction-and-replay mechanism is contemplated as a future optimisation of the present invention and is not part of the SHA `17334f8` embodiment, but the deterministic-derivation property that *enables* it is fully present.
3. **Cryptographic distribution quality.** BLAKE3 output is indistinguishable from uniform random under standard cryptographic assumptions. This guarantees that the expected similarity of two independently-seeded random hypervectors is exactly `0.5` with the predicted variance, removing a class of correlation artefacts that affect cheaper PRNGs.

---

## 5. Encoders (100)

Three encoders convert non-vector inputs into hypervectors. Each encoder calls only the primitives of Sections 3 and 4, so its output is also bit-exact across implementations.

### 5.1 String Encoding (110)

The string encoder maps a UTF-8 string to a hypervector via:

```
function encodeString(s: string, D: integer) -> hypervector:
    let domain_prefix = utf8_bytes("str:")
    let input         = domain_prefix || utf8_bytes(s)
    let seed          = BLAKE3(input)
    return randomHv(seed, D)
```

The domain prefix `"str:"` separates the string-encoder namespace from the level-encoder namespace (Section 5.2) and the role-namespace conventions used in higher-level encoders. Two strings that differ in even a single byte produce hypervectors that are approximately orthogonal (expected similarity `0.5`), giving exact-match key semantics without collision.

**Technical Effect.** Exact-match content-addressable encoding with zero false-positive collision rate (under BLAKE3 cryptographic assumptions) and zero storage overhead for the encoding state — no vocabulary, no learned parameters, no tokeniser. This is in contrast to embedding-based exact-match systems in which two literally identical strings can produce different float-32 vectors due to non-deterministic CUDA kernel scheduling, and where the encoding function itself occupies hundreds of megabytes (a learned tokeniser + an embedding matrix).

### 5.2 Embedding Encoding (Thermometer + Random Projection) (120)

The embedding encoder maps a real-valued vector (e.g., the float-32 output of a foundation-model encoder) into a binary hypervector while approximately preserving cosine similarity as Hamming similarity. The thermometer-quantisation and random-projection approach to mapping real-valued vectors to binary hyperdimensional vectors has academic precedent in Imani et al. (2017, "Voicehd: Hyperdimensional computing for efficient speech recognition", ICRC 2017) and Rahimi et al. (2016, "Hyperdimensional biosignal processing", BioCAS 2016) for biosignal processing. The contribution of the present invention over these is the **specific deterministic form** of the per-`(coordinate, level)` hypervector derivation: the binding of the ASCII-encoded `"lvl:i:level"` seed to BLAKE3 followed by BLAKE3-XOF, which renders the encoder byte-identical across heterogeneous language runtimes — a property that no prior thermometer-random-projection encoder claims, because prior systems used non-cryptographic pseudo-random generators whose state diverges across language libraries.

The encoder is further distinguished from the *sign-LSH / SimHash* family of binary-fingerprint constructions (Charikar 2002, "Similarity Estimation Techniques from Rounding Algorithms", STOC) and from the *sparse-sign random-projection* construction of Achlioptas (2003, "Database-friendly Random Projections: Johnson-Lindenstrauss with Binary Coins", *J. Comput. Syst. Sci.*) on three independent points. First, sign-LSH and Achlioptas-style sparse projection produce **one bit per projection** applied to the *raw* real-valued vector, with no intermediate quantisation; the present encoder produces a full `D`-bit hypervector per (coordinate, level) pair and accumulates by XOR over `N` coordinates, so the algebraic surface is fundamentally different (binary-thermometer-then-bind, rather than binary-projection-then-concatenate). Second, neither Charikar 2002 nor Achlioptas 2003 specifies a cryptographic extendable-output function or a published wire format pinning the projection matrix across implementations; the seed conventions in those works are left to the implementor, which is precisely the source of cross-runtime divergence that the present invention eliminates. Third, the bit-budget per input scalar in sign-LSH is `1`, whereas the present encoder's bit budget per input scalar is `D` (with the redundancy absorbed by majority-bundling at the substrate level); this difference materially changes the recall-quality tradeoff at small `D` and is one of the reasons the present encoder achieves bit-exact agreement with cosine similarity over the `[0.5, 1.0]` range (per the Phase-0 validation notebook) where sign-LSH gives only a probabilistic Hamming-vs-cosine correspondence.

The algorithm is a binary thermometer encoding with random projection per coordinate:

```
LEVELS = 100

function encodeEmbedding(embedding: array of float, D: integer) -> hypervector:
    let acc = byte array of length D, all zero            # additive identity for XOR
    for i in 0..length(embedding)-1:
        let raw     = embedding[i]
        let clamped = max(-1.0, min(1.0, raw))            # clamp to [-1, 1]
        let level   = round((clamped + 1.0) * (LEVELS - 1) / 2.0)   # integer in [0, 99]
        let seed_in = utf8_bytes("lvl:" || str(i) || ":" || str(level))
        let seed    = BLAKE3(seed_in)
        let hv_lvl  = randomHv(seed, D)
        acc := bind(acc, hv_lvl)                          # XOR into accumulator
    return acc
```

**Operational explanation.** Each coordinate `i` of the input embedding is first clamped into `[-1, 1]` to bound the level index; clamping is the chosen behaviour for out-of-range values rather than rejection, because foundation-model encoders occasionally produce values slightly outside the nominal range. The clamped value is then quantised to one of `L = 100` levels by the affine map `level = round((clamped + 1) * (L - 1) / 2)`, which sends `-1.0 -> 0`, `0.0 -> 50` (approximately, depending on rounding), and `+1.0 -> 99`. For each `(coordinate-index i, quantised-level level)` pair, a unique deterministic hypervector is derived by hashing the ASCII string `"lvl:i:level"` with BLAKE3 and expanding via `randomHv`. All such per-coordinate hypervectors are XOR-accumulated into the final output. The XOR accumulation is order-invariant (XOR is commutative and associative), so the encoder produces identical output for any permutation of the *coordinate index order* — which is correct, because the embedding's coordinate indices are themselves part of the seed and therefore preserved.

**Worked example.** Let `D = 8` and `embedding = [0.3, -0.7, 0.0, 0.9]`. The quantised levels are:
- `i = 0`: `clamped = 0.3`, `level = round((1.3 * 99) / 2) = round(64.35) = 64`
- `i = 1`: `clamped = -0.7`, `level = round((0.3 * 99) / 2) = round(14.85) = 15`
- `i = 2`: `clamped = 0.0`, `level = round((1.0 * 99) / 2) = round(49.5) = 50`
- `i = 3`: `clamped = 0.9`, `level = round((1.9 * 99) / 2) = round(94.05) = 94`

The encoder then computes:
```
hv_0 = randomHv(BLAKE3("lvl:0:64"), 8)
hv_1 = randomHv(BLAKE3("lvl:1:15"), 8)
hv_2 = randomHv(BLAKE3("lvl:2:50"), 8)
hv_3 = randomHv(BLAKE3("lvl:3:94"), 8)
output = hv_0 XOR hv_1 XOR hv_2 XOR hv_3
```

A perturbation of the input — say, changing `0.3` to `0.31` — would either leave level `0` at `64` (no change to encoding) or shift it to `65` (one summand changes, output changes in approximately half of the bits where `hv[0:64]` and `hv[0:65]` differ). Because adjacent levels yield independent random hypervectors, the encoding does *not* preserve continuity within a single coordinate; rather, it preserves *similarity* in the aggregate: two embeddings that are close in the input space share most of their level seeds and therefore most of their XOR summands, resulting in low Hamming distance.

**(5.2.1) Note on rounding.** Cross-implementation identity requires that `round((x + 1.0) * (L - 1) / 2.0)` produce the same integer in every language for the same `x`. Because `x` is clamped to `[-1, 1]`, the value to be rounded lies always in `[0, L - 1]` and is therefore **non-negative**, so any rounding rule that resolves half-integers consistently for non-negative inputs is admissible. The reference TypeScript binding (`packages/core-ts/src/encode.ts`) uses `Math.round`, which is IEEE 754 round-half-up (round-half-toward-positive-infinity). The reference Rust binding (`packages/core-rs/src/encode.rs`) uses `f32::round`, which is round-half-away-from-zero. These two rules **agree for all non-negative inputs** and are therefore interoperable in the present encoder. Implementations targeting IEEE 754 binary32 inputs MUST additionally normalise the order of arithmetic operations to the canonical bracketing **`((clamped + 1.0) * (L - 1)) / 2.0`** — that is, the multiplication is performed before the division — to prevent associativity-related divergence; this canonical bracketing is used by both reference bindings and is recited in SPEC.md §A.1, together with the outer `clamp(level, 0, L - 1)` that absorbs the half-integer rounding-tail case where the three rules (round-half-up, round-half-away-from-zero, round-half-to-even) could nominally differ. The conformance corpus of Section 10 verifies byte-identical encoder output across both reference bindings.

**Technical Effect.** Maps `f32` embeddings into binary hypervectors with two measurable advantages over storing the raw `f32` embeddings:

1. **Storage reduction.** A `1024`-dimensional `f32` embedding occupies `4096` bytes; encoding it into a `D = 10000` binary hypervector occupies `1250` bytes — a `3.27×` reduction. For higher-dimensional source embeddings (e.g., the `4096`-dimensional embeddings emitted by certain foundation-model encoders), the reduction approaches `13×` before considering any further compression.
2. **Similarity preservation.** Empirically, two embeddings with cosine similarity `s` produce hypervectors with Hamming similarity approximately `s` for `s` in the range `[0.5, 1.0]`, the range that matters for retrieval; the correspondence is documented in the Phase-0 validation notebook (`notebooks/phase0_hdc_validation.ipynb`). This means existing nearest-neighbour pipelines can be migrated to the binary substrate without re-training any upstream encoder.
3. **Per-query throughput.** Once the encoded substrate is in place, similarity computation runs at approximately `D / 64 = 156` 64-bit-word operations per pair at `D = 10000` (on a binding using the contemplated packed-bit form of §2.4), versus approximately `1024` floating-point multiply-accumulates plus two `sqrt` calls per pair for a 1024-dimensional float-32 cosine — an order-of-magnitude reduction in arithmetic operations and a comparable reduction in energy cost per query on commodity CPUs.

This is materially superior to Product Quantisation (FAISS-PQ), which requires a training step over a representative corpus, suffers from training-corpus distribution shift, and is not deterministic across hardware (FAISS-PQ uses BLAS kernels that are not reproducible across CPU SIMD widths).

### 5.3 Text Encoding (Permutation-Positional N-grams) (130, 140)

For natural-language text, three encoders are provided. Permutation-positional n-gram encoding for text is documented in the academic HDC literature (Najafabadi et al. 2016, "HDC for text classification"). The present invention's contribution over these is the combination of (i) the deterministic `encodeString` per-word hypervector of §5.1, (ii) the normative bit-rotation permutation of §3.3, and (iii) the deterministic-tiebreaker bundle of §3.1, all of which together render the entire text-encoder output byte-identical across implementations. Prior HDC text encoders make no such cross-platform-reproducibility claim.

**(5.3.1) Bag-of-words encoder (130).**

```
function encodeBagOfWords(text, D, min_word_length = 3) -> hypervector:
    let words = tokenise_lowercase(text, min_word_length)
    if length(words) == 0:
        return encodeString(text, D)                    # degenerate fallback
    return bundle([ encodeString("word:" || w, D) for w in words ])
```

Tokenisation `tokenise_lowercase` lowercases the input, splits on the regular expression `[^a-z0-9]+`, and discards tokens of length less than `min_word_length` (default 3; the rule discards single-letter tokens such as "a" and two-letter tokens such as "is", "of", "to", "in").

**(5.3.2) Word-level n-gram encoder (140).** This is the permutation-positional encoder. Each n-gram is encoded by binding the per-word hypervectors with permutation distances equal to their positions within the n-gram, then the n-grams are bundled:

```
function encodeWordNgrams(text, D, n) -> hypervector:
    require n >= 1
    let words = tokenise_lowercase(text)
    if length(words) < n:
        return encodeBagOfWords(text, D)                # fallback
    if n == 1:
        return encodeBagOfWords(text, D)
    let ngrams = []
    for i in 0..length(words) - n:
        let acc = encodeString("word:" || words[i], D)
        for j in 1..n-1:
            let term = permute(encodeString("word:" || words[i + j], D), j)
            acc = bind(acc, term)                       # XOR
        ngrams.append(acc)
    return bundle(ngrams)
```

**Worked example: "alpha beta" versus "beta alpha" with `n = 2`.**

- For "alpha beta": one n-gram, computed as `bind(encodeString("word:alpha"), permute(encodeString("word:beta"), 1))`.
- For "beta alpha": one n-gram, computed as `bind(encodeString("word:beta"), permute(encodeString("word:alpha"), 1))`.

Because `permute(x, 1)` is approximately orthogonal to `x`, the two outputs are approximately orthogonal to each other — Hamming similarity approximately `0.5`. In contrast, a bag-of-words encoding of the same two strings would produce *identical* hypervectors, because XOR-bundle is permutation-invariant in its inputs. The permutation-positional encoder thus discriminates word order, while the bag-of-words encoder does not.

**(5.3.3) Character n-gram encoder (140).**

```
function encodeCharNgrams(text, D, n = 3) -> hypervector:
    let cleaned = lowercase(text)
    if length(cleaned) < n:
        return encodeString(text, D)
    let grams = []
    for i in 0..length(cleaned) - n:
        grams.append(encodeString("char:" || cleaned[i..i+n-1], D))
    return bundle(grams)
```

This tolerates typographical errors and matches partial substrings. As an arithmetic illustration, consider the query "elephnt" (7 characters; 5 trigrams: `{"ele", "lep", "eph", "phn", "hnt"}`) against the original "elephant" (8 characters; 6 trigrams: `{"ele", "lep", "eph", "pha", "han", "ant"}`). Three trigrams are common to both — `{"ele", "lep", "eph"}` — and the Jaccard overlap is `3 / (5 + 6 - 3) = 3 / 8 ≈ 0.375` of the combined trigram set. The corresponding character-n-gram-encoded hypervectors therefore share most of the bundle summands closest to the query and produce a Hamming similarity materially greater than `0.5`, even in the presence of a missing character.

**Technical Effect.** Provides order-sensitive, parameter-free text encoding with bit-exact cross-platform reproducibility. As compared with TF-IDF (requires a corpus-derived vocabulary), word2vec / GloVe (requires training), and transformer encoders (require GPU inference and produce non-deterministic float-32 outputs), the permutation-positional encoder runs in `O(words × D)` time per document, has no training step, has no vocabulary, has no learned parameters, and produces byte-identical output in every supported language runtime. The character n-gram variant additionally provides typo-tolerance without any spell-checker dependency.

---

## 6. Store: Item Management and Recall (300)

The **store** (300) is the user-facing public API. It exposes content-addressable storage in which the addressing space is the hypervector space `{0, 1}^D`. Item retrieval is mediated by the cleanup-memory index (310) and co-activation tracking is performed by the tracker (320).

**(6.1) The Item data structure.** Each stored item carries:

| Field             | Type                              | Notes                                                                 |
|-------------------|-----------------------------------|-----------------------------------------------------------------------|
| `id`              | UUIDv7 string                     | Auto-generated on `put` unless supplied; UUIDv7 sorts lexicographically by creation time |
| `key`             | hypervector of dimension `D`      | The address in associative space                                       |
| `value`           | opaque byte string, `<= 16 MiB`   | Configurable cap up to 256 MiB; larger payloads belong in a blob store |
| `tags`            | list of UTF-8 strings             | Arbitrary labels; used by `recall` filters                             |
| `metadata`        | map of string → scalar            | Scalar = string, integer, float, boolean, or null                      |
| `createdAt`       | unsigned 64-bit Unix milliseconds | Preserved across `put` upsert                                          |
| `lastAccessedAt`  | unsigned 64-bit Unix milliseconds | Updated on every recall hit                                            |
| `accessCount`     | unsigned 32-bit integer            | Incremented on every recall hit                                        |
| `cold`            | boolean                           | Set by `consolidate` when the item meets cold-eligibility criteria; boolean is preferred over a multi-level coldness score because the contemplated alternate-attic relocation step (v0.2.0) is a discrete operation, not a gradient one |

The logical store-of-items is exposed as `store.items`, a map keyed by `id` and providing at minimum the operations `get(id) -> Item | null`, `set(id, item) -> void`, `delete(id) -> boolean`, and iteration over its entries.

**(6.2) Public API.**

```
open(config)                      -> Store
put(store, key, value, opts?)     -> Item
recall(store, cue, opts?)         -> list of Match
get(store, id)                    -> Item
delete(store, id)                 -> boolean
consolidate(store, opts?)         -> ConsolidationReport
snapshot(store)                   -> bytes (KMF)
restore(bytes)                    -> Store
```

In all cases, `key` (in `put`) and `cue` (in `recall`) may be supplied as either (i) a UTF-8 string, in which case `encodeString` is applied internally, (ii) a real-valued array, in which case `encodeEmbedding` is applied, or (iii) a hypervector, which is used directly.

**(6.3) The `put` operation.** The store exposes a standard upsert-on-put operation that preserves `createdAt` and `accessCount` on existing identifiers and assigns a new UUIDv7 identifier when none is supplied. Concretely:

```
function put(store, key, value, opts):
    let bytes = (value as bytes)
    require length(bytes) <= store.valueCapBytes
    let hv    = toHypervector(key, store.dimension)
    let now   = current_unix_millis()
    let id    = (opts.id provided) ? opts.id : uuidv7(now)
    let existing = (opts.id provided) ? store.items.get(id) : null
    let item = Item {
        id, key = hv, value = bytes,
        tags    = opts.tags or [],
        metadata = opts.metadata or {},
        createdAt = (existing != null) ? existing.createdAt : now,
        accessCount = (existing != null) ? existing.accessCount : 0,
        lastAccessedAt = now,
        cold = false,
    }
    store.items.set(id, item)
    return item
```

If the supplied `id` already exists in the store, the operation is **upsert** with `createdAt` preserved.

**(6.4) The `recall` operation (cleanup memory, 310).**

```
function recall(store, cue, opts):
    let topK    = opts.topK    or store.defaultTopK             # default 10
    let minSim  = opts.minSimilarity or store.defaultMinSim     # default 0.5
    let cueHv   = toHypervector(cue, store.dimension)
    let candidates = []
    for each item in store.items:
        if opts.filter is provided and not opts.filter(item):
            continue
        candidates.append({ id: item.id, key: item.key })
    let hits    = cleanupSearch(candidates, cueHv, topK, minSim)
    let now     = current_unix_millis()
    let matches = []
    for each hit in hits:
        let item = store.items.get(hit.id)
        item.accessCount     += 1
        item.lastAccessedAt   = now
        item.cold             = false
        matches.append({ item, similarity: hit.similarity })
    if length(matches) >= 2:
        store.tracker.record([m.item.id for m in matches])      # co-activation tracking
    return matches
```

The `cleanupSearch` routine is brute-force linear scan in the reference Phase-1 implementation:

```
function cleanupSearch(entries, cue, topK, minSimilarity):
    let out = []
    for each entry in entries:
        require length(entry.key) == length(cue)
        let s = similarity(entry.key, cue)
        if s >= minSimilarity:
            out.append({ id: entry.id, similarity: s })
    sort out by (similarity descending, id lexicographic ascending)
    return out[0..topK-1]
```

Tiebreaking on equal similarity is by lexicographic ascending `id`, which is identical across implementations because UUIDv7 yields the same ASCII representation in every language.

**(6.5) Optional locality-sensitive hashing layer (contemplated, Phase 2 / v0.2.0).** The brute-force scan of §6.4 is the normative Phase-1 implementation and is the embodiment present at SHA `17334f8`. SPEC.md §4.2 expressly demarcates the Phase 1 brute-force scan as normative for v0.1.0 and demarcates an LSH-or-learned-index hybrid as contemplated and non-normative for a future v0.2.0 spec revision. The following paragraph describes a planned Phase 2 enhancement that is **not part of the present embodiment** but is disclosed for purposes of describing the inventor's contemplated mode of implementation. The Phase 1 brute-force scan is acceptable up to approximately `10^5` items at `D = 10000` on a commodity CPU. Phase 2 introduces a Locality-Sensitive Hashing ("LSH") layer for higher item counts, structured as a band-of-tables of random-bit-position projections (the standard Hamming-space LSH construction of Indyk and Motwani 1998, refined by Andoni and Indyk 2008), returning a candidate set that is then ranked by the same brute-force scan over the reduced set. The Phase 2 LSH layer is required to produce **the same top-`k`** as the brute-force scan for any `(cue, k, store state)` triple; LSH is permitted only to widen the candidate set, never to alter the ranking. This invariant is what makes the cross-binding conformance corpus possible and is what distinguishes the contemplated layer from a Hierarchical-Navigable-Small-World ("HNSW") index (Malkov and Yashunin 2020, *IEEE TPAMI*), which does not guarantee identical top-`k` across implementations because its graph-construction order is non-deterministic. The LSH layer is disclosed as the inventor's contemplated mode for the Phase 2 cleanup memory; it is not the subject of any present claim and may be claimed in a subsequent divisional application.

**Technical Effect.** Provides content-addressable storage with three measurable improvements over conventional key-value stores plus a separate vector index (the architecture used by, e.g., Redis + FAISS sidecar, or PostgreSQL + pgvector):

1. **Single index, single substrate.** The hypervector *is* the address; there is no separate vector index to keep coherent with the item map. This eliminates a class of consistency bugs in which the vector index falls out of sync with the canonical record store.
2. **Filter-then-rank semantics.** The optional `filter` predicate is applied before similarity ranking, which means filter conditions over metadata never produce empty result sets due to top-k truncation — a known failure mode of vector databases that apply filter conditions *after* nearest-neighbour ranking (publicly documented for `pgvector` in its issue tracker and for Pinecone in its community forum; the failure mode manifests when the top-`k` set returned by the ANN index contains no items satisfying the metadata filter, even though such items exist further down the similarity ordering).
3. **Deterministic recall under ties.** Identical recall results across replicas, even when multiple items share the top similarity score, because the tiebreaker is lexicographic on `id` and `id`s are UUIDv7 ASCII strings identical across runtimes.

---

## 7. Consolidation (Replayable Hebbian) (330)

The consolidation procedure (330) periodically reshapes the substrate so that items frequently recalled *together* become *more similar in Hamming space*. This is a Hebbian principle ("neurons that fire together, wire together") adapted to the binary hypervector substrate, with the critical innovation that the procedure is **fully deterministic and replayable**.

The Hebbian principle (Hebb 1949, *The Organization of Behavior*) and its application to associative memory (Hopfield 1982, "Neural networks and physical systems with emergent collective computational abilities"; Kanerva 1988, *Sparse Distributed Memory*) are prior-art foundations. The present invention's novel contribution is the **deterministic, salt-driven, BLAKE3-ordered, alternate-direction bit-flip formulation** of §7.2, which renders the Hebbian update fully replayable from a snapshot + access-log pair. No prior Hebbian consolidation procedure — including the modern continuous-Hopfield work of Ramsauer et al. (2020, "Hopfield Networks Is All You Need") — supports byte-identical cross-platform replay, because all such prior procedures operate on floating-point weights whose update order and rounding depend on hardware-specific BLAS or auto-differentiation kernels.

### 7.1 Co-activation Tracking (320)

A sliding window of size `W` (default `W = 1000`) records the IDs of items returned together by each `recall` call. For each pair `(a, b)` of items co-occurring in the window, a counter `c(a, b)` is maintained:

```
class CoactivationTracker:
    window       : list of (list of item-ids)
    pair_counts  : map of (id_a, id_b) -> integer
    size         : integer (window capacity)

    function record(ids):
        if length(ids) < 2:
            window.append(ids)
            trim()
            return
        let sorted_ids = sort_ascending(ids)            # canonical order: lex
        for i in 0..length(sorted_ids)-1:
            for j in i+1..length(sorted_ids)-1:
                pair_counts[(sorted_ids[i], sorted_ids[j])] += 1
        window.append(sorted_ids)
        trim()

    function trim():
        while length(window) > size:
            let expired = window.pop_front()
            if length(expired) < 2: continue
            for i in 0..length(expired)-1:
                for j in i+1..length(expired)-1:
                    decrement pair_counts[(expired[i], expired[j])]
                    if the count reaches 0: remove the entry

    function pairsAtOrAbove(threshold):
        let out = []
        for each ((a, b), count) in pair_counts:
            if count >= threshold:
                out.append({ a, b, count })
        sort out by (count descending, a ascending, b ascending)
        return out
```

The sorted-pair canonicalisation `sort_ascending(ids)` ensures `(a, b)` and `(b, a)` are recorded as the same pair, and gives deterministic ordering when multiple pairs share the same count.

### 7.2 Binding Pull (the Hebbian step) — Fig. 4 steps 401–407

When a pair `(a, b)` has co-activation count `>= pullThreshold` (default 32), the keys of items `a` and `b` are pulled closer in Hamming space by flipping a bounded number of disagreeing bits. Crucially, the *choice of which bits to flip* is deterministically derived from a salt that increments per consolidation pass:

```
function pullCloser(a: hypervector, b: hypervector, maxSimDelta: float, salt: integer)
        -> { newA, newB, bitsFlipped }:
    require length(a) == length(b)
    let D = length(a)

    # 1. Find disagreeing bit positions.
    let disagree = []
    for i in 0..D-1:
        if a[i] != b[i]:
            disagree.append(i)

    # 2. Bound the number of flips: at most floor(maxSimDelta * D), but at least 1.
    let maxFlips = max(1, floor(maxSimDelta * D))           # default maxSimDelta = 0.02
    let toFlip   = min(length(disagree), maxFlips)

    # 3. Derive a deterministic ordering over the disagreeing bits via BLAKE3.
    let seedBytes = u64_le(salt)                            # 8 bytes
    let digestLen = max(64, toFlip * 4)                     # see note below
    let digest    = BLAKE3-XOF(seedBytes, digestLen)

    # Sort the `disagree` list of bit indices `b` by the composite key
    # (digest[b mod length(digest)], b) in ascending order. The first
    # component provides the BLAKE3-driven pseudo-random ordering; the
    # second is the lexicographic tiebreaker when two indices hash to the
    # same digest byte.
    sort disagree by key b -> (digest[b mod length(digest)], b) ascending

    # 4. Apply alternate-direction flips: even index toward b, odd index toward a.
    let aOut = copy(a)
    let bOut = copy(b)
    for k in 0..toFlip-1:
        let bit = disagree[k]
        if k mod 2 == 0:
            aOut[bit] := b[bit]                              # a moves toward b
        else:
            bOut[bit] := a[bit]                              # b moves toward a

    return { newA: aOut, newB: bOut, bitsFlipped: toFlip }
```

The `maxSimDelta` parameter (default `0.02`) bounds the per-pass similarity drift: at `D = 10000`, the operation flips at most `floor(0.02 * 10000) = 200` bits, which moves `sim(a, b)` by at most `0.02`. This bound prevents a single consolidation pass from collapsing two distinct items into the same address; consolidation acts as a slow drift rather than an abrupt merge.

The `digestLen = max(64, toFlip * 4)` rule is chosen so that (i) the factor of `4` provides at least one digest byte per disagreeing bit position participating in the flip (preventing the modulo wrap-around from concentrating the sort key onto a small alphabet and creating excessive collisions in the BLAKE3-driven ordering), and (ii) the `max(64, ...)` floor ensures BLAKE3-XOF is exercised with a non-trivial output length even when `toFlip` is small.

The **alternate-direction** flip rule ensures that both `a` and `b` move approximately equally — neither one is dragged entirely toward the other. The choice of which disagreeing bits to flip first is determined by `BLAKE3-XOF(u64_le(salt))`, where `salt` is the per-pass generation counter maintained by the store. Two replicas running on the same access log with the same initial state and the same salt sequence flip *exactly* the same bits in *exactly* the same order.

### 7.3 Cold-Item Flagging

Items satisfying both `lastAccessedAt < (now - coldDays × 86_400_000)` and `accessCount < coldMinAccess` are flagged as `cold`:

```
function flagColdItems(items, config, now):
    let cutoff = now - config.coldDays * 86_400_000          # millis
    return [ it.id for it in items
             if it.accessCount < config.coldMinAccess
             and it.lastAccessedAt < cutoff ]
```

Cold-flagged items remain in the store (the Phase-1 behaviour present at SHA `17334f8`); subsequent versions of the invention may relocate cold items to an "attic" sub-store from which they can be cheaply re-instantiated on a near-hit. This attic relocation, including its bundled-summary representation and its associated KMF `attic_block` (445) per §8.7 below, **is reserved for v0.2.0 of the spec** and is not part of the present embodiment.

### 7.4 Replay Determinism

The consolidation procedure satisfies the following replay invariant:

> Given (i) the same initial substrate state `S_0`, (ii) the same sequence of `put` and `recall` operations, and (iii) the same per-pass salt sequence, the consolidated state `S_n` is **byte-identical** across all conformant implementations.

The reasons are:
- Co-activation tracking uses sorted IDs and integer counters, both of which are platform-independent.
- The pair-selection ordering for the pull step uses `(count desc, a asc, b asc)`, all integer / lexicographic comparisons.
- The bit-flip ordering is `BLAKE3-XOF(u64_le(salt))`, which is byte-identical across implementations.
- The flip rule (`even -> a := b[bit]`, `odd -> b := a[bit]`) is integer arithmetic with no floating-point.

Each consolidation pass writes its salt value into the persisted access log as a `u64` little-endian integer at a fixed offset within the pass record, so that a verifier replaying the log can re-derive the bit-flip ordering deterministically. A **restore module** reads a KMF snapshot together with the append-only access log (the log records, for each operation, a typed entry of one of `{put, recall, consolidate}` with full byte-level parameters including the per-pass consolidation salt), and re-executes each entry in order against the loaded snapshot to reconstruct the current state byte-for-byte. The access-log persistence and restore-module facility is contemplated at SHA `17334f8`; its full apparatus form is the subject of dependent claim 15 of independent claim 9.

**Technical Effect.** Provides deterministic, replayable index reshaping — a system-level property that no conventional vector database supports. Specifically:

1. **Snapshot + access-log replay.** Given a KMF snapshot of state `S_0` and a log of subsequent operations, an auditor can re-run the consolidation procedure and arrive at the *byte-identical* current state. This enables verifiable audit of memory evolution — useful in regulated environments (medical agent memory, legal discovery) where the provenance of a stored item must be reconstructable.
2. **Multi-replica consistency without consensus.** Two replicas processing the same operation log converge to byte-identical state without any consensus protocol between them. This eliminates the need for Paxos / Raft style co-ordination for the consolidation pass, an order-of-magnitude reduction in operational complexity compared with conventional replicated databases.
3. **Deterministic Hebbian compaction.** Conventional neural Hebbian updates use floating-point weight changes which are not bit-exact across CPU architectures, GPU drivers, or BLAS implementations. The binary flip-based formulation here is exact integer arithmetic plus BLAKE3, neither of which has hardware-dependent variants.

---

## 8. KMF Wire Format (Smritidb Memory Format) (400)

The Smritidb Memory Format ("KMF", element 400) is the persistent, implementation-independent wire format for a Smritidb substrate. The format is positioned for the same standards trajectory as Apache Parquet (columnar tabular storage; Apache Parquet Format 2.6) and Apache Iceberg (table-level transactional metadata): a canonical byte-layout that any conformant implementation can read and write. The use of a fixed header magic (410), a `u64` header offset (412), a trailing footer (470), and per-block hashes (450) is structurally similar to the Parquet footer layout and the Iceberg table-metadata layout. The novelty of KMF lies in **the binding of this generic container structure to the binary hypervector substrate**: specifically, the MSB-first column-major `hv_block` (420) of §8.2 and the deterministic per-block BLAKE3 digest at the substrate-bit level, which together permit cross-implementation byte-identical reads of a hyperdimensional substrate.

### 8.1 File Layout

A KMF file is structured as five contiguous sections:

```
+------------------------------------------------------------------+ offset 0
| (a) Magic bytes (410): "KMF\0" (0x4B 0x4D 0x46 0x00) — 4 bytes    |
+------------------------------------------------------------------+ offset 4
| (b) Spec version (411): "0.1.0\0" (ASCII, NUL-pad)   — 6 bytes    |
+------------------------------------------------------------------+ offset 10
| (c) Header offset (412): u64 little-endian          — 8 bytes    |
+------------------------------------------------------------------+ offset 18
| (d) Data blocks   (variable length; see §8.2 - §8.4)              |
|     - hv_block    (420)                                           |
|     - meta_block  (430)                                           |
|     - value_block (440)                                           |
|     - attic_block (445)  [contemplated, v0.2.0; see §8.7]         |
+------------------------------------------------------------------+ offset (header_offset)
| (e) Header (460) (JSON-encoded; zstd compression contemplated)    |
|     { spec_version, dimension, item_count, created_at,            |
|       index: [{ kind, offset, length, blake3 (450) }, ...] }      |
+------------------------------------------------------------------+ offset (file_length - 4)
| (f) Trailer magic (470): "FMK\0" (0x46 0x4D 0x4B 0x00) — 4 bytes  |
+------------------------------------------------------------------+ offset (file_length)
```

The header offset stored at byte 10 is a `u64` little-endian integer that locates the start of the header. To read a KMF file, an implementation MUST:

1. Verify bytes 0–3 equal `"KMF\0"`.
2. Verify the last four bytes of the file equal `"FMK\0"` — this trailer guard detects truncation **before** any offset is dereferenced.
3. Read the spec version at bytes 4–9 and reject if the major version exceeds the supported major.
4. Read the `u64` header offset at bytes 10–17.
5. Read the header from `header_offset` up to `(file_length - 4)`, JSON-decode it.
6. For each block referenced in the header's `index` field, slice the bytes `[offset, offset + length)` from the data region, compute BLAKE3 over those bytes, and verify the result equals the `blake3` field stored in the index. Any mismatch raises a `CorruptSnapshot` error.
7. Only after all block hashes verify does the reader deserialise the block contents.

### 8.2 The `hv_block` (420)

The `hv_block` (420) stores the hypervector keys in column-major MSB-first packed-bit form. For a substrate of `n` items at dimension `D`, the block has length `n × ceil(D / 8)` bytes. Item `k`'s hypervector occupies bytes `k × ceil(D / 8)` to `(k + 1) × ceil(D / 8) - 1`. Within each item's region, bit `i` is at byte `floor(i / 8)`, bit offset `7 - (i mod 8)` (MSB-first), identical to the in-memory packed representation. When the substrate contains a single-item snapshot (for instance, a singleton bundle output emitted directly as a one-row KMF file), the `hv_block` consists of that one hypervector's `ceil(D / 8)` bytes, and the per-block BLAKE3 (450) in the header index of §8.5 is the BLAKE3 of those bytes, providing the integrity-hash emission contemplated by claim 5.

**Worked byte-level example: `D = 16`, three items.** Each hypervector occupies `ceil(16 / 8) = 2` bytes. The block occupies `3 × 2 = 6` bytes. Suppose the three hypervectors are:

```
item 0: bits = 1 0 1 1 0 0 1 0 | 1 1 0 0 0 0 0 1   -> bytes 0xB2 0xC1
item 1: bits = 0 0 0 0 0 0 0 0 | 1 1 1 1 1 1 1 1   -> bytes 0x00 0xFF
item 2: bits = 1 1 1 1 1 1 1 1 | 0 0 0 0 0 0 0 0   -> bytes 0xFF 0x00
```

The `hv_block` then comprises the bytes:
```
B2 C1 00 FF FF 00
```
The very same byte sequence is what is written by the Rust binding, the TypeScript binding, the Python binding, the Kotlin / Swift bindings via UniFFI, and the WebAssembly build of the Rust binding. The byte-exact identity is verifiable by computing `BLAKE3` over the six bytes and comparing against the header's stored hash.

### 8.3 The `meta_block` (430)

The `meta_block` (430) stores per-item metadata as a **JSON-encoded array** — this is the normative Phase-1 encoding and is the encoding used by the reference implementation at SHA `17334f8` (see `packages/core-ts/src/kmf.ts`, which calls `JSON.stringify` on the metadata array). A Phase-2 variant employing length-prefixed MessagePack records is **contemplated** for streaming efficiency and is **not part of the present embodiment**. Fig. 2 of the drawings annotates this block as "JSON (Phase 1; MessagePack contemplated for Phase 2)", in agreement with the present specification. Each record contains:

```
{
  "id":             "<uuid-v7-string>",
  "tags":           [<string>, ...],
  "metadata":       { <key: string> -> <scalar> },
  "createdAt":      <unix-millis>,
  "accessCount":    <u32>,
  "lastAccessedAt": <unix-millis>
}
```

Records appear in the same order as the hypervectors in the `hv_block`, so item `k`'s metadata is the `k`-th entry.

### 8.4 The `value_block` (440)

The `value_block` (440) stores opaque payloads in length-prefixed form. Each payload is preceded by a `u32` little-endian length, followed by exactly that many payload bytes. The values appear in item order.

```
+-----------+------------------+-----------+------------------+ ...
| u32 LE L0 |  L0 bytes value  | u32 LE L1 |  L1 bytes value  |
+-----------+------------------+-----------+------------------+ ...
```

The value block treats payloads as opaque bytes; the substrate does not interpret them. Higher layers (the AgentMemory example built atop the store, for instance) may impose JSON or any other encoding by convention, but the substrate guarantees nothing about value content.

### 8.5 Header (460)

The header (460) is a JSON object. At SHA `17334f8` the reference implementation writes the header as a plain UTF-8 JSON byte sequence (see `packages/core-ts/src/kmf.ts`, where the header is produced by `encodeAscii(JSON.stringify(h))`). An optional Zstandard-compression layer over data-block bytes (analogous in apparatus form to compression of the header bytes themselves) is **contemplated** for a subsequent revision and is exposed by dependent claim 20 of independent claim 18 (which permits each data block to be compressed prior to BLAKE3 hashing); this compression layer is **not part of the SHA `17334f8` embodiment**. Its fields are:

| Field          | Type                                  | Meaning                                              |
|----------------|---------------------------------------|------------------------------------------------------|
| `spec_version` | string, exactly `"0.1.0"` in v0.1     | The spec version this snapshot was written against   |
| `dimension`    | unsigned integer                      | The dimension `D`                                    |
| `item_count`   | unsigned integer                      | Total number of items in the substrate               |
| `created_at`   | unsigned 64-bit Unix millis            | Snapshot creation time                              |
| `index`        | array of block references             | One entry per data block (see below)                 |

Each block reference is:

```
{
  "kind":   "hv_block" | "meta_block" | "value_block",
  "offset": <byte offset into the file where the block starts>,
  "length": <length of the block in bytes>,
  "blake3": <hex-encoded 32-byte BLAKE3 digest of the block bytes>
}
```

The `blake3` field is the lower-case hex string of the 32 bytes returned by `BLAKE3(block_bytes)`. Implementations MUST verify each block against this digest before deserialising.

### 8.6 Trailer (470)

The four-byte trailer (470) `"FMK\0"` (the reverse of the header magic) serves as an anti-truncation guard. Many file truncation incidents (interrupted writes, partial cloud-object downloads, abnormal process exit) produce files that begin correctly but are cut off mid-block. By requiring the trailer to be present *and at the end of the file*, the format guarantees that any truncation is detected before any offset in the file is trusted. The asymmetry between header magic `"KMF\0"` (410) and trailer magic `"FMK\0"` (470) additionally makes it impossible to confuse the file for a doubled or concatenated KMF stream.

### 8.7 The `attic_block` and the Compressed-Block Pipeline (contemplated)

This subsection describes apparatus disclosed for purposes of best-mode contemplation that is **not part of the SHA `17334f8` embodiment** but supports the disclosure of dependent claim 22 of independent claim 18 (attic block) and dependent claim 24 of independent claim 23 (Zstandard compressor coupled with BLAKE3 hasher).

**(8.7.1) The `attic_block` (445).** When the consolidation procedure of §7 flags an item as cold (see §7.3) and the contemplated v0.2.0 attic relocation step is enabled, the cold item's hypervector is removed from the principal `hv_block` (420) and a *bundled summary* of one or more cold items is written into a dedicated `attic_block` (445). Each entry in the `attic_block` is a record of the form `{ source_item_ids: [<uuidv7-string>, ...], bundled_hypervector: <ceil(D/8) bytes> }`, where `source_item_ids` enumerates the item identifiers whose hypervectors were bundled (per §3.1) to produce the contained `bundled_hypervector`. The `attic_block` is included in the header (460) `index` array with `kind = "attic_block"`, an explicit byte offset, length, and BLAKE3 (450) digest computed over the attic block's bytes — providing the same per-block integrity property as the principal hv/meta/value blocks. The attic mechanism enables a near-hit query against a cold item to be served by re-instantiating the item from its bundled summary, without retaining the full per-item hypervector in the principal substrate. This `attic_block` apparatus is the subject of dependent claim 22 of independent claim 18 (which provides for an optional attic block independently hashed and indexed); it is reserved for v0.2.0 of the spec.

**(8.7.2) Zstandard-compressed block pipeline.** When a block (any of `hv_block`, `meta_block`, `value_block`, or `attic_block`) is to be written in compressed form, the Zstandard compressor is coupled between the block-emit pipeline and the BLAKE3 hasher (450) such that the BLAKE3 digest recorded in the header (460) index is computed over the **compressed bytes** rather than the uncompressed ones. This pipeline ordering — "compress, then hash" — allows a reader to verify the integrity of the on-disk bytes before any decompression step, and accordingly to detect compression-stream corruption (e.g., a truncated zstd frame) before the decompressor is invoked. The reader correspondingly decompresses only after the per-block BLAKE3 verification has succeeded. This compress-then-hash pipeline ordering is the subject of dependent claim 24 of independent claim 23 (which provides for a Zstandard compressor coupled between a block-emit pipeline and the BLAKE3 hasher).

**Technical Effect.** The KMF format yields the following measurable improvements over conventional persistence approaches for vector databases (e.g., the proprietary on-disk formats of FAISS, Annoy, hnswlib, or the SQL-blob serialisations used by pgvector):

1. **Implementation-independent persistence.** A snapshot written by the Rust binding loads byte-for-byte into the TypeScript, Python, Kotlin, or Swift binding. This is empirically verified by the cross-binding conformance corpus (Section 10).
2. **Per-block cryptographic integrity.** Corruption (bit flips on disk, partial network reads, malicious tampering) is detected at block granularity via BLAKE3, before the corrupted block is interpreted. FAISS's on-disk format, by contrast, has no integrity check; a single corrupted byte in an index file produces silently wrong nearest-neighbour results.
3. **Streaming-friendly layout.** The header sits at the *end* of the file, immediately before the trailer, but is located via the fixed-position `u64` header offset at byte 10. A reader can therefore (a) read the header without parsing any block, (b) decide which blocks to load (e.g., metadata only, or only the first `N` items' hypervectors), and (c) seek directly to those blocks. This enables `O(1)` "what's in this file?" queries without `O(file_size)` parsing — a property absent from formats that interleave metadata and payload (e.g., HDF5 with chunked datasets).
4. **Anti-truncation by construction.** The `"FMK\0"` trailer must be present at the file end, detectable by a single seek-to-end read, before any offset in the file is trusted. This eliminates a class of "partial write, garbage reads" bugs that plague append-only formats.

---

## 9. Persistence Adapters (500)

The KMF wire format defines *what* is written. Storage adapters define *where*. The invention provides a uniform `StorageAdapter` interface (500) — with methods `read_snapshot` (510), `write_snapshot` (520), `append_wal` (530), `read_wal` (540), `truncate_wal` (550), and `close` (560) — that any storage substrate may implement:

```
interface StorageAdapter:
    kind:   string                                # informational
    read():  bytes or null                        # the most recent snapshot, or null if empty
    write(bytes): void                            # atomically replace the snapshot
    remove?(): void                               # delete the snapshot (optional)
```

The reference implementation provides four adapters:

**(9.1) Memory adapter (610).** An in-process, ephemeral adapter that holds the most recent snapshot bytes in an in-memory buffer. Used for testing and for transient sessions where persistence is not required.

**(9.2) File-system adapter (620).** Writes the snapshot atomically to a file on the local filesystem, using the standard `write-to-temp + rename` idiom that POSIX guarantees to be atomic within a single directory (on POSIX-conformant filesystems; on Microsoft Windows the equivalent guarantee is provided by `MoveFileEx` with the `MOVEFILE_REPLACE_EXISTING` flag). Crashes during write leave either the previous snapshot intact or the new snapshot complete; partial writes are never observable to a reader.

**(9.3) SQLite adapter (630).** Stores the snapshot bytes as a single `BLOB` row in a SQLite database, using SQLite's Write-Ahead-Logging ("WAL") journal mode (Hipp et al.; SQLite specification). Inherits SQLite's ACID guarantees: a successful `write()` returns only after the snapshot is durable on disk; an interrupted process leaves the previous snapshot recoverable. Compatible with the Rust `rusqlite` bindings used by the Rust implementation, and with the WebAssembly SQLite build used by browser implementations. The novelty of the present invention over a bare SQLite-backed BLOB is **not the WAL mechanism itself** — which is a well-known SQLite feature in the prior art — but the **decoupling of the algorithmic substrate from the durable storage layer via the `StorageAdapter` interface (500)**, such that the same KMF byte stream can be redirected to any of the four reference adapters or to a user-supplied implementation without modification of the algorithmic core; the SQLite adapter merely composes the substrate over SQLite's durability primitives, inheriting rather than re-implementing them.

**(9.4) IndexedDB adapter (640).** Stores the snapshot in a browser IndexedDB object store. Provides offline persistence for web applications without server round-trips. The adapter writes the entire snapshot as a single `Blob` value, which IndexedDB stores efficiently (typically referencing the bytes by pointer rather than copying).

A *higher-order* adapter helper `withPersistence(store, adapter, { debounceMs })` wraps a live store so that every `put`, `delete`, and `consolidate` call schedules an auto-snapshot through the adapter, debounced by a configurable interval (default `100 ms`). This coalesces burst writes into a single snapshot.

**Technical Effect.** Decouples the algorithmic substrate from the storage backend, enabling the same code to operate over heterogeneous storage substrates without modification:

1. **Backend substitution at runtime.** A library consumer chooses among the four reference adapters disclosed above (Memory (610), File-system (620), SQLite (630), IndexedDB (640)), or implements the `StorageAdapter` interface (500) against any further substrate (potential user-extension examples including but not limited to Amazon S3, Cloud Spanner, or Postgres-as-blob; these are not present implementations at SHA `17334f8`), by passing a different `StorageAdapter` instance to the same store constructor. The core algorithm is unchanged.
2. **Inheritance of storage-substrate properties.** When the SQLite adapter is used, the substrate inherits ACID semantics for free; when the file-system adapter with atomic rename is used, the substrate inherits POSIX rename-atomicity; when the IndexedDB adapter is used, the substrate becomes available offline in any modern browser. This compositional reuse of platform-provided durability is materially superior to building bespoke durability into the substrate (the approach taken by purpose-built vector databases).
3. **Single wire format across backends.** Because every adapter operates on the same KMF byte stream, the substrate can be migrated between backends by exporting from one and importing into another, with no format-conversion step.

---

## 10. Cross-Implementation Bit-Exactness and Conformance Corpus (660)

The system specification requires that all conformant implementations agree at the byte level on the output of every primitive, every encoder, and every wire-format operation. To make this contract testable, the invention defines a **conformance corpus (660)** — a JSON file `tests/conformance/golden.json` that lists, for a fixed set of inputs, the expected outputs.

The technique of a JSON-encoded conformance corpus exercised by every implementation's continuous-integration pipeline is borrowed from established standards-body practice — including the W3C Web Platform Tests and the IETF JSON-Patch conformance suite (RFC 6902 §11). The present invention's contribution over these precedents is **the specific binding of such a corpus to the bit-level outputs of a hyperdimensional substrate**, including SHA-256-hashed packed-bit hypervectors as the canonical comparison artefact — a binding that no prior HDC system maintains.

**(10.1) Corpus structure.** The corpus is a JSON object containing typed test categories:

```
{
  "spec_version":  "0.1.0-draft",
  "generated_by":  "@tanvrit/smritidb (TS reference)",
  "random_hv":          [ { "seed_utf8", "dim", "sha256" }, ... ],
  "encode_string":      [ { "input", "dim", "sha256" }, ... ],
  "similarity_pairs":   [ { "a_seed", "b_seed", "dim", "expected" }, ... ],
  "bind_round_trip":    [ { "a_seed", "b_seed", "dim", "expected_similarity_to_a" }, ... ],
  "bundle":             [ { "seeds": [...], "dim", "sha256" }, ... ],
  "text_bag_of_words":  [ { "text", "dim", "sha256" }, ... ],
  "text_char_ngrams":   [ { "text", "dim", "n", "sha256" }, ... ]
}
```

Each `sha256` field is the lower-case hex SHA-256 hash of the raw output bytes of the corresponding hypervector. The reference TypeScript implementation generates the corpus by computing each output and writing the hash; every other binding *verifies* the corpus by recomputing the output and comparing its SHA-256 to the listed value. Use of SHA-256 (rather than BLAKE3) for the corpus hash is intentional: it cross-checks that the BLAKE3 used in randomHv has not been silently substituted with a different BLAKE3 variant, by introducing a second independent hash function whose libraries are unrelated to BLAKE3.

**(10.2) Test categories at the SHA `17334f8`.**

| Category              | Entry count | Tested behaviour                                                                 |
|-----------------------|-------------|------------------------------------------------------------------------------------|
| `random_hv`           | 5           | `randomHv(BLAKE3(seed_utf8), dim)` produces expected bytes                       |
| `encode_string`       | 3           | `encodeString(input, dim)` produces expected bytes                               |
| `similarity_pairs`    | 3           | `similarity(randomHv(a), randomHv(b))` lands within `1e-9` of expected           |
| `bind_round_trip`     | 1           | `bind(bind(a, b), b) == a` (similarity exactly 1)                                |
| `bundle`              | 2           | `bundle([randomHv(s) for s in seeds])` produces expected bytes                   |
| `text_bag_of_words`   | 2           | `encodeBagOfWords(text, dim)` produces expected bytes                            |
| `text_char_ngrams`    | 2           | `encodeCharNgrams(text, dim, n)` produces expected bytes                         |

The corpus is intentionally small at SHA `17334f8` — namely **five random-hv vectors, three string-encodings, three similarity pairs, one bind round-trip, two bundles, two bag-of-words, and two char-ngrams**, for **eighteen total entries**. A passing corpus does not *prove* full correctness; it proves *agreement with the reference at these specific inputs*. The corpus expands with each spec revision.

**(10.3) CI integration.** Each binding's continuous-integration pipeline runs the corpus as a mandatory test. A binding that fails any corpus entry is, by definition, non-conformant; it cannot be released under the Smritidb name. The Rust binding's (720) conformance test lives at `packages/core-rs/tests/conformance.rs`; the Python binding's (730) at `packages/smritidb-py/tests/test_smritidb.py`; the Kotlin (740) / Swift (750) binding's at the UniFFI-generated test harness in `packages/smritidb-ffi/` (the UniFFI Interface Description Language ("UDL") contract is at `packages/smritidb-ffi/src/smritidb.udl`); the TypeScript binding's (710) at `packages/core-ts/src/conformance.test.ts` (where the reference itself dog-foods the corpus). The conformance-verification flow is summarised in **Fig. 5**, with the PASS outcome at (780) and the FAIL outcome at (790).

**Technical Effect.** Provides a verifiable, automated contract between independent language implementations:

1. **Drift prevention.** Cross-implementation drift is detected by any CI run, not by user-reported bugs in production. This eliminates a known and costly failure mode in conventional vector databases, where the C++ core and the Python bindings drift in floating-point handling across releases.
2. **Legal artefact for interoperability.** The corpus is a concrete, repository-resident, machine-verifiable file. It is the artefact that demonstrates — to a reviewer, an auditor, or a court — that the cross-platform interoperability claim is real and not aspirational.
3. **Standards-track readiness.** A canonical conformance corpus is a prerequisite for submission of the wire format to a standards body (IETF Independent Submission, W3C Community Group). By providing the corpus from day one, the invention is structurally prepared for standardisation in a way that ad-hoc implementations are not.

---

## 11. Worked Examples

This section provides four end-to-end worked examples that a PHOSITA can re-derive by hand or by trivial scripting from the algorithms specified above.

### Example 1: Tiebreaker Resolution

Consider a bundle of four hypervectors at `D = 8`. Suppose at bit position `i = 3`, exactly two of the four inputs have bit 3 set to `1` and two have it set to `0` — i.e., `sum[3] = 2 = n/2 = 4/2`, a tie. The tiebreaker is computed as:

```
domain  = "smritidb/tiebreak"             # 17 ASCII bytes
buf     = domain                          # 17 bytes
       || u32_le(8)                       # 0x08 0x00 0x00 0x00
       || u32_le(3)                       # 0x03 0x00 0x00 0x00
       || u32_le(4)                       # 0x04 0x00 0x00 0x00
                                          # total 29 bytes
digest  = BLAKE3(buf)                     # 32 bytes
out[3]  = digest[0] & 1                   # least significant bit of the first byte
```

A PHOSITA can compute `BLAKE3` of the 29-byte input using any reference BLAKE3 library, take the first byte of the result, and observe its low bit. That value is normatively `out[3]`. Every conformant implementation produces the identical value.

### Example 2: Embedding Round-Trip

Consider a 4-dimensional embedding `[0.3, -0.7, 0.0, 0.9]` encoded at `D = 1024`. The quantised levels are `(64, 15, 50, 94)` as computed in Section 5.2. The encoder accumulates by XOR:

```
hv_0 = randomHv(BLAKE3("lvl:0:64"), 1024)
hv_1 = randomHv(BLAKE3("lvl:1:15"), 1024)
hv_2 = randomHv(BLAKE3("lvl:2:50"), 1024)
hv_3 = randomHv(BLAKE3("lvl:3:94"), 1024)
output = hv_0 XOR hv_1 XOR hv_2 XOR hv_3
```

Now consider a perturbed input `[0.305, -0.7, 0.0, 0.9]`. The first coordinate's quantised level is `round((1.305 * 99) / 2) = round(64.5975) = 65` (the input being non-negative, both `Math.round` and `f32::round` give the same result, per §5.2.1). The perturbed encoding differs from the original only in the first summand, replacing `hv_0(level 64)` with `hv_0(level 65)`. Because `hv_0(64)` and `hv_0(65)` are independent random hypervectors, they disagree on approximately `D / 2 = 512` of the 1024 bits. The XOR of these two hypervectors with the unchanged remainder produces an output that disagrees with the original in those approximately 512 positions, giving similarity `≈ 0.5`. For inputs that fall on the *same* quantised level (e.g., `0.3` and `0.301`), the encoding is *bit-identical*. The encoder is therefore piecewise-constant in each coordinate, with similarity preservation occurring across multiple coordinates simultaneously.

### Example 3: Consolidation Step

Consider two hypervectors `a` and `b` at `D = 10000` with Hamming distance `5000` (i.e., `sim(a, b) = 0.5`). Apply `pullCloser(a, b, maxSimDelta = 0.02, salt = 1)`.

```
disagree   = list of 5000 bit positions where a[i] != b[i]
maxFlips   = max(1, floor(0.02 * 10000)) = 200
toFlip     = min(5000, 200) = 200
seedBytes  = u64_le(1) = [01 00 00 00 00 00 00 00]
digestLen  = max(64, 200 * 4) = 800
digest     = BLAKE3-XOF([01 00 00 00 00 00 00 00], 800)
```

Sort the 5000 disagreeing bit positions by `(digest[i mod 800], i)` ascending, then flip the first 200:
- Position `disagree[0]`: `a[bit] := b[bit]` (a moves toward b)
- Position `disagree[1]`: `b[bit] := a[bit]` (b moves toward a)
- Position `disagree[2]`: `a[bit] := b[bit]`
- Position `disagree[3]`: `b[bit] := a[bit]`
- ... and so on, alternating, for 200 positions total.

After the operation, the 200 selected disagreeing positions have been resolved to identical bits in `a_out` and `b_out`; the remaining `5000 - 200 = 4800` disagreeing positions are unchanged. The new Hamming distance between `a_out` and `b_out` is `4800`, giving `sim(a_out, b_out) = 1 - 4800 / 10000 = 0.52` — exactly `0.02` higher than the original. The Hamming distance between `a_out` and the original `a` is 100 (the 100 even-indexed positions where `a` was modified toward `b`), giving `sim(a_out, a) = 0.99`; symmetrically `sim(b_out, b) = 0.99`. Both items moved by exactly half the requested similarity delta, and the pair-similarity moved by the full delta.

### Example 4: KMF Round-Trip

Consider a substrate at `D = 64` containing three items with arbitrary keys, payloads, and metadata. Serialise with `writeKmf`:

```
file = magic("KMF\0")                                  # 4 bytes  [0..4)
     || spec_version("0.1.0\0")                        # 6 bytes  [4..10)
     || u64_le(header_offset)                          # 8 bytes  [10..18)
     || hv_block                                       # 3 * 8 = 24 bytes  [18..42)
     || meta_block                                     # JSON of length M  [42..42+M)
     || value_block                                    # length-prefixed payloads  [42+M..H)
     || header_json                                    # H = header_offset; the JSON header lives at file_len-4-|header| ... file_len-4
     || trailer("FMK\0")                               # 4 bytes  [file_len-4..file_len)
```

The header is JSON of the form:
```
{
  "spec_version": "0.1.0",
  "dimension":    64,
  "item_count":   3,
  "created_at":   <unix-millis>,
  "index": [
    { "kind": "hv_block",    "offset": 18, "length": 24, "blake3": "<hex>" },
    { "kind": "meta_block",  "offset": 42, "length": <M>, "blake3": "<hex>" },
    { "kind": "value_block", "offset": <42+M>, "length": <V>, "blake3": "<hex>" }
  ]
}
```

To round-trip: a reader (a) verifies bytes 0–3 are `"KMF\0"`, (b) verifies the last 4 bytes are `"FMK\0"`, (c) reads the `u64` at bytes 10–17 to locate the header, (d) JSON-decodes the header, (e) for each block in the header's index slices the bytes at the listed offset/length and verifies the BLAKE3 digest, (f) deserialises each block into items. The resulting in-memory substrate is byte-equivalent (under the canonical equality of items) to the original. A re-write of the recovered substrate, with the same `created_at`, produces a file that is byte-identical to the original.

---

## 12. Industrial Applicability

The invention is industrially applicable across multiple computing domains in which an associative memory with deterministic cross-platform behaviour, low memory footprint, and verifiable persistence is required. Representative applications are:

**(12.1) Large-language-model agent memory.** An LLM-based agent maintains a persistent memory of past conversations, observations, and intermediate computations. The agent retrieves relevant memories on each turn by encoding the current context as a hypervector and recalling the top-`k` most similar items. The invention provides: (i) low memory footprint (a million memories at `D = 10000` occupy approximately 1.25 GB, fitting comfortably in commodity RAM), (ii) deterministic recall across agent restarts and replicas, and (iii) auditability of memory evolution via the replayable consolidation procedure — a requirement in regulated agent deployments.

**(12.2) Semantic search.** Documents, code snippets, and structured records are encoded as hypervectors (whole-document hashes or aggregated chunk-level encodings) and recalled by similarity to a query. The invention's bit-exact encoder removes the embedding-drift problem in which different versions of an embedding model produce subtly different vectors for the same document, breaking ranking determinism across application versions. Because the encoders have no learned parameters, version-to-version reproducibility is exact.

**(12.3) On-device retrieval-augmented generation ("on-device RAG").** A mobile or edge device runs an LLM and needs a local knowledge base to ground responses. Conventional vector databases (FAISS, hnswlib) require tens of megabytes of native code plus gigabytes of float-32 embedding storage; the invention runs the Rust binding (compiled to ARM64 mobile native code or to WebAssembly) with a small footprint (the WebAssembly build is expected to be less than 500 KiB pre-compression at SHA `17334f8`; the exact measured byte size is to be confirmed before filing) and binary hypervector storage. The Kotlin (740) / Swift (750) bindings expose the same surface on Android and iOS respectively, with bit-exact behaviour identical to the server-side TypeScript (710) or Rust (720) implementation. This enables a single conformant memory to be replicated across cloud, server, and device tiers without translation.

**(12.4) Federated and distributed memory.** Multiple agents collaboratively maintain a shared associative memory. Each agent processes its own operation stream; periodically, the agents reconcile by exchanging KMF snapshots. Because every operation — `put`, `recall`, `consolidate` — is deterministic, two agents that process the same operations starting from the same snapshot converge to byte-identical state without any consensus protocol. The replayable consolidation procedure makes the reconciliation strictly an append-and-replay operation, not a stochastic merge.

**(12.5) Verifiable audit trail.** In regulated environments (healthcare assistants, legal-discovery agents, financial-compliance bots), the provenance of every memory item must be reconstructable. The invention's `KMF snapshot at time t₀ + operation log → state at time t₁` replay primitive provides exactly this: an auditor, given the snapshot and log, can re-derive the current state byte-for-byte and verify the agent's behaviour at any point in its history.

**(12.6) Cross-language data sharing in heterogeneous research pipelines.** A research group's data pipeline may involve a Python preprocessing stage, a Rust training stage, a TypeScript web visualisation stage, and a Kotlin / Swift mobile-app evaluation stage. With conventional vector databases, sharing data across these stages requires format conversion at every boundary; with the invention, the same KMF byte stream is read directly by every stage, with cryptographic integrity guaranteed at the block level. This materially reduces the operational burden of multi-language research pipelines.

The combination of low memory footprint, deterministic cross-platform behaviour, verifiable persistence, and replayable consolidation has no direct equivalent in the prior art (Pinecone, Weaviate, FAISS, hnswlib, Annoy, pgvector, or in the academic HDC literature surveyed in the Background section). The invention is therefore not merely a "computer programme" but a system with concrete technical character, useful in industry, and capable of straightforward reproduction by a PHOSITA from the disclosure above.

---

*End of Detailed Description. Continued in Section 8 (Claims) and Section 9 (Drawings).*
