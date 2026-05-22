# Round 2 Critique — 09-abstract.md

## Per-perspective notes (concise)

### 1. IPO Examiner
- **Classifiability:** Good. Mentions associative memory store, binary hypervectors, wire format, Hebbian consolidation, random projection — examiner can index under G06F 16/00 (data retrieval), G06N 3/00 (neural / associative), G06F 11/10 (error detection).
- **Claim language leak:** The phrasing "comprising" and "characterised by" mirror claim form. IPO is tolerant of this in abstracts, but the colon-bullet enumeration ("by means of: a deterministic tiebreaker …; a packed-bit wire format …; …") resembles claim-clause structure. Acceptable but borderline.
- **Rule 13(7) (≤ 150 words):** Body is **140 words** — within the limit. PASS.
- **Word count verified:** Yes, 140 words (excluding the `# Abstract` heading).

### 2. §3(k) Defender
- Avoids the words "algorithm" and "computer programme". GOOD.
- Leads with "computer-implemented system and method for an associative memory store" — system framing, not method-step framing. GOOD.
- Technical effects are listed explicitly ("bit-exact cross-platform interoperability", "thirty-two-fold reduction in memory footprint", "verifiable fault tolerance through corruption detection"). GOOD.
- One soft spot: "deterministic tiebreaker for bundle operations" reads slightly algorithmic. Mitigated by being one of four features in a larger system clause.

### 3. Prior-art Hunter
- Claim "Conventional vector databases suffer from silent numerical drift across language runtimes" — defensible (FAISS, pgvector, Pinecone do not publish bit-exact conformance; 10-prior-art.md substantiates this).
- "the disclosed system addresses these technical problems" — appropriately hedged ("addresses", not "solves").
- "approximately thirty-two-fold reduction" — hedged with "approximately". GOOD.
- No over-claim of "novel", "unprecedented", "first-ever" language. Low §2(1)(j) attack surface.

### 4. Indian Patent Attorney
- **Indian English orthography:** Mostly compliant ("characterised", "organised" — wait, "organised" is correct). However **"thermometer-quantized"** uses American "-ized". Should be **"thermometer-quantised"** (08-claims.md and 10-prior-art.md both use "quantised").
- **Single paragraph:** Yes. PASS.
- **Figure reference at end:** Yes — "Figure: Fig. 1". PASS. (Style nit: IPO convention is just "Fig. 1" or "Reference is invited to Fig. 1"; "Figure: Fig. 1" is slightly redundant but acceptable.)
- **No reference numerals in body:** PASS.
- **Voice formal:** PASS.

### 5. Domain Critic (SHA 17334f8)
- **"binary hypervectors"** — accurate (`packages/core-rs/src/lib.rs`, `SPEC.md`).
- **"deterministic tiebreaker … cryptographic hash over data, index and count"** — accurate; BLAKE3 over `(D, i, n)` triplet, verified in claim 1 and `crates/`.
- **"packed-bit wire format … per-block hash integrity and a magic trailer"** — accurate; KMF format in `packages/core-rs/src/kmf.rs` has both per-block BLAKE3 hashes AND a `TRAILER = [0x46, 0x4d, 0x4b, 0x00]` (and a leading MAGIC). Note: the abstract says "magic trailer" but the format actually has BOTH a magic header AND a magic trailer. The abstract is not wrong, but understates the design (header magic is the primary identifier; trailer is the truncation check). Acceptable simplification.
- **"replayable Hebbian consolidation procedure with bounded similarity drift"** — accurate per 07-detailed-description.md §6.
- **"thermometer-quantized random projection encoder for bounded floating-point inputs"** — accurate per `SPEC.md` §A.1 and 07-detailed-description.md §5.2.
- **"bit-exact cross-platform interoperability between native, managed and browser runtimes"** — accurate. `tests/conformance/golden.json` is the bit-exact contract, verified across TS/Rust/Python/Kotlin/Swift. Browser = wasm build (commit b8be10d). PASS.
- **"approximately thirty-two-fold reduction in memory footprint relative to single-precision vectors"** — arithmetically accurate (32 bits per f32 → 1 bit per dimension = 32×). Hedged with "approximately" to cover the small per-block hash overhead in KMF. GOOD.
- **"verifiable fault tolerance through corruption detection"** — accurate; per-block BLAKE3 in KMF detects single-bit and block-level corruption.

## Consolidated Revision Worklist

1. **Orthography fix (REQUIRED):** Change `thermometer-quantized` → `thermometer-quantised` to match Indian English usage and the rest of the specification (08-claims.md, 10-prior-art.md both use "-quantised").

2. **Figure reference (OPTIONAL):** Change `Figure: Fig. 1` → `Reference is invited to Fig. 1 of the accompanying drawings.` — matches conventional IPO abstract closing form. If brevity is prioritised, leave as is; no rule violation.

3. **Header magic accuracy (OPTIONAL, low priority):** The phrase `per-block hash integrity and a magic trailer` could read `per-block hash integrity and magic header-trailer framing` to reflect that BOTH ends carry magic bytes. Adds two words (would make body 142, still under 150). Skip if word budget is tight; existing wording is not inaccurate.

4. **Tiebreaker phrasing (OPTIONAL, §3(k) hedge):** Consider `a deterministic tiebreaking arrangement for bundle operations` instead of `a deterministic tiebreaker for bundle operations` — slightly more apparatus-flavoured, fractionally less algorithmic. Skip if word budget is tight.

5. **Claim-form colon-bullet (OPTIONAL):** The current `by means of: a … ; a … ; a … ; and a …` reads claim-like. If softening is desired, replace with prose: `by means of a deterministic tiebreaker …, a packed-bit wire format …, a replayable Hebbian consolidation procedure …, and a thermometer-quantised random projection encoder …`. Saves the colon and the semicolons; same word count.

**Minimum required change:** Item 1 only (orthography). Items 2–5 are stylistic polish; the abstract is otherwise compliant with Rule 13(7), §3(k) framing, and the codebase at SHA 17334f8.
