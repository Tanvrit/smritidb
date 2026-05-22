# Round 2 Critique — 08-claims.md

**Reviewed at SHA:** 17334f8 (verified via `git show 17334f8:packages/core-ts/src/hypervector.ts`, `consolidate.ts`, `kmf.ts`, `text.ts`, `encode.ts`, `tests/conformance/golden.json`, `SPEC.md`).

**Brief sanity-check:** Brief instructed me to verify independent-claim numbers (1, 7, 13, 18, 23, 28, 30, 37, 42, 52, 58). Actual file has 11 independent claims at: **1, 7, 13, 18, 25, 30, 37, 42, 46, 52, 58**. Brief numbers 23 and 28 are dependents of claim 18; the true independents are 25 (Hebbian system triplet) and 46 (encoder). Critique uses the verified numbers.

---

## Perspective 1 — IPO Examiner (FER-style)

The following objections are framed as if drafted by an Assistant Controller of Patents under Sections 14 and 15 of the Patents Act, 1970.

### Claim 1 (Tiebreaker method)

1. **§3(k) — computer programme per se / algorithm.** *Claim 1 is objected to under Section 3(k) on the grounds that* the substance of the claim is an arithmetic procedure (vertical bit-sum, comparison against `n/2`, hash-derived selection) performed by a processor, and the recitation of "computer-implemented method" and "by a processor" is, on the test laid down in [Ferid Allani v. Union of India, 2019 SCC OnLine Del 11867], insufficient where the technical effect is not articulated in the body of the claim itself. The recitation of "thereby enabling verifiable cross-hardware reproducibility of an associative memory substrate" in the preamble is merely a statement of intended use and does not import a technical effect into the body of the claim.
2. **§2(1)(ja) — inventive step.** *Further objected under Section 2(1)(ja)* that the use of a cryptographic hash to break a tie in a deterministic procedure is a routine application of cryptographic seeding well-known prior to the priority date (BLAKE3 spec 2020; Kanerva 1988 disclosed majority bundling with an admitted tie ambiguity), and a person skilled in the art would have arrived at the claimed combination without exercise of inventive faculty.
3. **§10(4) — sufficiency.** The claim refers to "bit-index `i` encoded as a 32-bit little-endian unsigned integer" but does not state what value is to be used where `D > 2^32`. The disclosure of `D ∈ {1024, 8192, 10000, 16384}` in dependent claim 3 cures this for those values, but the broad claim spans unsupported dimensions.

### Claim 7 (Tiebreaker system)

1. **§3(k).** *Objected on the grounds that* the limitation "at least one processor and a memory storing instructions" is a generic computer recital known from any general-purpose computer; the substantive matter is the same algorithm as Claim 1. Reframing of an algorithm as a system claim does not, by itself, escape §3(k) (per [Microsoft Technology Licensing LLC v. Asst. Controller of Patents and Designs, 2023 SCC OnLine Mad 6373]).
2. **§2(1)(ja).** *Objected* that claim 7 adds only conventional hardware (SIMD popcount in dep. 8, NVRAM in dep. 9, FFI in dep. 10) to claim 1; the combination is obvious.

### Claim 13 (Tiebreaker CRM)

1. **§3(k).** *Objected* that a "non-transitory computer-readable storage medium storing instructions" claim is the very form of claim that the Patents Office has historically held to be a claim to a computer programme per se where the underlying method is itself algorithmic. The CRM triplet must demonstrate a technical effect *qua* the storage medium (e.g., reduced storage I/O), which is not present in the body of the claim.
2. **§2(1)(ja).** As for claim 1, mutatis mutandis.

### Claim 18 (Replayable Hebbian consolidation)

1. **§3(c) — mere discovery of a scientific principle.** *Objected under Section 3(c)* on the grounds that the claim recites Hebbian learning ("neurons that fire together wire together," Hebb 1949) which is a scientific principle of neuroscience. The claim is the discovery that this principle can be made deterministic by hashing — a mere discovery cannot found a patent.
2. **§3(k) — algorithm.** *Objected* that step (e) is the application of a known cryptographic primitive (BLAKE3 XOF) to seed an ordering — an algorithmic step.
3. **§2(1)(j) — novelty.** *Objected* that Kanerva 1988 (SDM) discloses Hebbian-style counter consolidation in a binary hypervector substrate, and the limitation "maxSimDelta" is a mere choice of numerical parameter (0.02 in dep. 19) which is not by itself novel.
4. **§10(4).** The "access log" is referenced in the preamble ("together with an access log") but no step in the body of the claim defines what the access log contains. Without antecedent basis the scope of "access log" is indeterminate.

### Claim 25 (Hebbian consolidation system)

1. **§3(k).** Same objection as claim 7, mutatis mutandis: generic processor + memory recital does not save an algorithmic core.
2. **§2(1)(ja).** *Objected* that the system claim adds no further technical feature beyond claim 18 except generic storage and the recital of "eviction-driven decrement," which is a well-known sliding-window data-structure technique.

### Claim 30 (KMF wire-format method)

1. **§3(k).** *Objected* that "writing one or more data blocks ... to an output byte sequence" is a presentation-of-information / data-arrangement step. Specification of magic bytes, offsets and hash digests does not impart a technical effect *unless* the claim recites a downstream technical consequence (e.g., reduced storage I/O, faster random access).
2. **§3(d) — mere new use.** *Objected under Section 3(d)* that prepending a magic-byte string, computing a hash of each block, and writing a header are individually-known techniques used in Apache Parquet (footer), HDF5 (magic), MessagePack (typed blocks), and signed-archive formats. The combination, as claimed, is the mere aggregation of known steps.
3. **§2(1)(ja).** *Objected* that Apache Parquet (Parquet specification, Apache Foundation, available pre-2020) discloses a footer-positioned schema, columnar layout, and per-row-group block boundaries; the substitution of BLAKE3 for an XXHash or CRC checksum is a workshop change.

### Claim 37 (KMF wire-format system)

1. **§3(k).** Same generic-hardware objection as claim 7; the system claim recites no SIMD, no FPGA, no I/O DMA — only "at least one processor and a memory."
2. **§2(1)(ja).** Objection as for claim 30; the addition of a "Zstandard compressor coupled between a block-emit pipeline and the said BLAKE3 hasher" (dep. 38) is a known I/O pipeline configuration.

### Claim 42 (KMF wire-format CRM)

1. **§3(k).** Same CRM-triplet objection as claim 13.
2. **§10(4).** Step (f) recites "on read, verifying ... before reconstructing any substrate item" but no antecedent step in the writing pipeline defines "the substrate item." A single CRM should not conflate writer and reader contracts without antecedent basis for each.

### Claim 46 (Thermometer-quantised encoder)

1. **§3(k).** *Objected* that the method is a sequence of arithmetic operations (clamp, round, concatenate, hash, XOR) — a quintessential algorithm.
2. **§2(1)(ja) — over Imani 2017.** *Objected* that Imani et al. (VoiceHD, ICRC 2017) disclose thermometer encoding from a bounded-range vector to a binary hypervector via per-level random projections; the only differentiator is the cryptographic anchoring of the projection, which is a workshop substitution of one PRNG (an arbitrary seedable RNG) for another (BLAKE3 XOF) and would be obvious to a skilled person seeking cross-implementation reproducibility.
3. **§3(c).** *Objected* that "preserving cosine similarity approximately as Hamming similarity" is a property of the Johnson-Lindenstrauss lemma — a discovery of a mathematical principle — and is not patentable per se.

### Claim 52 (Conformance corpus method)

1. **§3(k).** *Objected* that the claim is a method of testing software, which is itself a computer programme; the artefact ("a JSON document") is a presentation of information, not a technical means.
2. **§3(d).** *Objected* that the cryptographic-validation methodology (NIST FIPS 140-3 / CAVP / CAVS) discloses, prior to the priority date, a corpus of input/expected-output pairs against which an implementation is tested for byte-identity. The use of the same methodology against a "vector substrate" is a mere new use of a known process within the meaning of Section 3(d).
3. **§2(1)(j).** *Objected* that NIST CAVP test vectors for AES, SHA-256 and HMAC anticipate the abstract concept of a corpus of `(input, expected_digest)` pairs verifying byte-identity across implementations.
4. **§3(k) — strongest objection.** This claim is the most vulnerable: it claims a methodology, packaged as a JSON document, executed in a CI pipeline. There is no apparatus, no signal transformation, no technical effect *that resides in the corpus itself*.

### Claim 58 (Permutation-positional text encoder)

1. **§3(k).** *Objected* that the method is a deterministic text-processing pipeline (normalise, split, hash, rotate, XOR, bundle) which is, in substance, an algorithm.
2. **§2(1)(ja) — over Plate 1995 + Imani text-encoding.** *Objected* that Plate 1995 (Holographic Reduced Representations) discloses positional encoding by permutation, and Imani's text-encoding works disclose n-gram HDC. The combination is obvious; the substitution of binary substrate for real-valued substrate is a workshop change.
3. **§10(4).** The claim recites "applying a cyclic bit rotation of the corresponding word hypervector by exactly `j` bit positions" but the specification (at SPEC §3 and code at `text.ts` lines 79-92) uses `permute(hv, j)` which is element-wise shift in unpacked form, not bit-rotation on packed bits. The claim wording does not match the disclosure.

---

## Perspective 2 — §3(k) Defender

The IPO has, since *Ferid Allani v. Union of India*, [(2019) 261 DLT 305 (Del)] and reinforced by *Microsoft Technology Licensing LLC v. Asst. Controller of Patents*, [2023:MHC:6373] and *OpenTV Inc. v. Controller of Patents*, [2023:DHC:3305], required that a software-implemented invention demonstrate a "technical effect" or "technical contribution" beyond mere computation. The body of each independent claim — not merely its preamble — must articulate that effect.

### Audit of preambles

| Claim | Preamble TE language | Verdict |
|---|---|---|
| 1 | "thereby enabling verifiable cross-hardware reproducibility of an associative memory substrate" | Pre-text effect only. Body recites pure arithmetic. **VULNERABLE.** |
| 7 | "thereby reducing storage input/output by permitting a single canonical snapshot to be shared between implementations" | Body recites computation steps only. **VULNERABLE.** |
| 13 | "thereby providing a technical means of verifying associative-memory substrate equivalence" | The phrase is in the characterising clause, but the body steps are algorithmic. **MEDIUM RISK.** |
| 18 | "thereby providing a holographic-fault-tolerant memory whose state can be reconstructed without retaining intermediate substrate copies" | Strong technical effect (storage reduction). Body still algorithmic. **MEDIUM RISK.** |
| 25 | "thereby providing a verifiable reconstruction path that reduces persistent storage requirements" | As for claim 18. **MEDIUM RISK.** |
| 30 | "thereby providing BLAKE3-backed data integrity and reduced storage input/output" | Strongest TE in claim set: explicit corruption detection + I/O reduction. **LOW RISK.** |
| 37 | "thereby providing reduced storage input/output through column-major partial loading" | Strong TE. **LOW RISK.** |
| 42 | (no TE in preamble; only "for serialising and deserialising") | **HIGH RISK — must add TE.** |
| 46 | "thereby providing a deterministic, training-free, lower-compute-load embedding pipeline that requires no neural-network inference" | Strong TE. **LOW RISK.** |
| 52 | "thereby providing a reproducibility guarantee that reduces cross-binding debugging effort" | Effect is on developer effort, not on the machine. **HIGH RISK — §3(k) bait.** |
| 58 | "thereby providing a lower-compute-load and improved-memory-footprint alternative to learned text embeddings" | Strong TE. **LOW RISK.** |

### Specific reframings

**Claim 1 — add to characterising clause (body, not preamble):**

> Old: `characterised in that the use of a cryptographic hash digest seeded by the tuple (D, i, n) together with a fixed domain-separation tag renders the tiebreaker free of any implementation-defined ordering...`
>
> New: `characterised in that the said BLAKE3-derived tiebreaker bit, by operation upon the packed bit-array stored in the said memory, causes the said packed output hypervector to be byte-identical across heterogeneous processor architectures, thereby reducing storage input/output of the said associative memory substrate by permitting a single persisted snapshot to be loaded and verified bit-exactly by every conformant implementation without re-execution of the bundle operation.`

Save: *Ferid Allani* — the technical effect (reduced I/O, verifiable cross-architecture equivalence) is now in the body of the claim and operates on a stored data structure.

**Claim 7 — replace generic "processor and memory" recital:**

> Add after the system-comprising recital: `wherein the said memory comprises a non-volatile storage region holding the said packed bit-array between executions, and the said processor is operable to write to and read from the said non-volatile storage region, the said deterministic tiebreaker module being executed on the data path between the said memory and the said non-volatile storage region.`

Save: *Microsoft v. Asst Controller* held that a system claim escapes §3(k) where the claim recites a "specific hardware configuration that produces a tangible technical effect"; tying the tiebreaker to the NVRAM data path supplies that tangibility.

**Claim 13 — add explicit hardware tie-in:**

> Replace "instructions which, when executed by one or more processors" with `instructions which, when loaded into a memory of a computing apparatus and executed by one or more processors of the said apparatus, configure the said apparatus to`.

Save: *OpenTV* held that CRM claims survive §3(k) where the claim configures *the apparatus* (not merely "performs the method"). The replacement language is the canonical EPO/IPO formulation.

**Claim 18 — already has strong TE; tighten:**

> Add to characterising clause: `the said byte-identical post-consolidation substrate being persistable to a non-volatile storage device and re-instantiable on a different computing apparatus by re-execution of the said method against the said snapshot and access log, thereby dispensing with persistent storage of intermediate substrate states and reducing the storage footprint of the said associative memory.`

**Claim 25 — same fix as claim 18 mutatis mutandis.**

**Claim 30 — strongest claim; minor tightening:**

> The body already recites BLAKE3 per-block hashing and trailer verification. Add a single hardware-tied clause: `the said reader being operable to read the said byte sequence from a non-volatile storage device into a memory of a computing apparatus, and to refuse to write any substrate state into a working memory of the said apparatus until the said per-block BLAKE3 hash digests have been verified.`

**Claim 37 — same fix.**

**Claim 42 — add explicit TE in preamble AND body:**

> Preamble currently ends `"for serialising and deserialising a binary hyperdimensional associative memory substrate, the said method comprising:"` — append: `"in a manner that provides corruption detection prior to substrate state reconstruction and that reduces storage input/output by permitting a single canonical wire-format file to be shared between heterogeneous implementations,"` before "the said method comprising."

**Claim 46 — add hardware-tie sentence to characterising clause:**

> Append: `the said method being executed entirely in integer arithmetic and bitwise operations on the said processor, without invocation of any floating-point matrix-multiplication unit or neural-network inference accelerator, such that the said binary hypervector is producible on a microcontroller-class processor lacking floating-point hardware.`

This converts "no neural network" from a negative pre-text to a hardware-tied technical effect.

**Claim 52 — REFRAME AS SYSTEM CLAIM (per Round 1 Tension 6):**

Current claim 52 is method-form and is the highest §3(k) risk in the set. Recommended fix: convert to a system claim in apparatus form.

> Replacement preamble: `52. A computer system for verifying cross-implementation byte-identity of a binary hyperdimensional associative memory substrate, the system comprising at least one processor, a non-volatile storage device storing a canonical conformance corpus file, and a memory storing instructions which, when executed by the said processor, cause the system to ...`
>
> Body: convert steps (a)-(d) to "maintain in the said non-volatile storage device", "execute, against the said memory, each said test group", "compute, in the said processor", "report".
>
> Characterising clause: `characterised in that the said system rejects, at load time of any substrate snapshot produced by a foreign implementation, any said snapshot whose computed output, when subjected to each test group, fails to match the said expected digest; thereby providing a load-time technical guard against the admission of a non-conformant substrate to the said associative memory.`

Save: *OpenTV v. Controller of Patents* expressly recognises that a software-implemented validation method can escape §3(k) where the apparatus refuses admission of non-conformant data — i.e., the technical effect is the *gating action* on the machine state.

**Claim 58 — minor; add hardware-tie sentence:**

> Append to characterising clause: `the said method being computable entirely by integer arithmetic and bitwise operations, such that the said binary hypervector is producible on a processor lacking any floating-point unit, and is storable in a non-volatile storage device as an associative-memory key without further encoding.`

---

## Perspective 3 — Prior-art Hunter

For each independent claim, the closest single reference, the closest combination, and the *minimum additional limitation* needed to survive obviousness.

### Claim 1 — Tiebreaker method

- **Closest single:** Kanerva 1988 §3.2 (majority bundling, ties admitted as implementation-defined).
- **Closest combination:** Kanerva 1988 + BLAKE3 spec (O'Connor 2020).
- **Risk:** A skilled examiner will hold that "use a cryptographic hash to break a tie deterministically" is an obvious workshop move once cross-binding identity is desired.
- **Sharpening — add to body:** `wherein the said BLAKE3 hash digest is computed over a byte string of exactly (length-of-tag + 12) bytes consisting of the ASCII domain tag "smritidb/tiebreak", followed by the dimension D, the bit-index i, and the multiplicity n each as 32-bit little-endian unsigned integers in that order and without padding, and wherein the said domain tag, the said byte order, and the said field widths are recorded as a mandatory field of the wire-format header of any persisted snapshot of the said associative memory substrate.`
  - This binds the claim to a *specific byte layout* that is recorded in the *wire format*, which Kanerva 1988 cannot anticipate.

### Claim 7 — Tiebreaker system

- **Closest single:** Kanerva 1988 + any general-purpose computer system claim.
- **Sharpening:** Tie to FFI / WASM execution environment. Add to characterising clause: `wherein the said tiebreaker module is exposed via a foreign-function-interface boundary that is byte-transparent, such that an output produced by a native-code invocation thereof is byte-identical to an output produced by a WebAssembly-sandboxed invocation thereof on the same input multiset.`

### Claim 13 — Tiebreaker CRM

- **Closest single:** As claim 1.
- **Sharpening:** Add WebAssembly bytecode limitation into the independent claim itself (currently in dep. 15). Move dep. 15 into the independent claim as a non-optional limitation, with `wherein the said instructions comprise a portable bytecode executable across heterogeneous processor architectures without re-compilation`.

### Claim 18 — Replayable Hebbian consolidation

- **Closest single:** Kanerva 1988 §6 (SDM counter consolidation).
- **Closest combination:** Kanerva 1988 + Hebb 1949 + BLAKE3.
- **Risk:** Mid-level. Hebb teaches the principle; Kanerva teaches the substrate; BLAKE3 supplies the determinism. The novelty must rest on the *bounded similarity drift* + *replayability from log*.
- **Sharpening — add to body (already partially present in claim 19, must be in independent):** `wherein the said `maxSimDelta` is bounded such that floor(maxSimDelta * D) is at most one percent of D, and wherein the said consolidation log is sufficient, together with the prior substrate snapshot, to reconstruct the post-consolidation substrate state byte-for-byte without retention of any intermediate substrate copy`.

### Claim 25 — Hebbian system

- **Closest:** as claim 18.
- **Sharpening:** Move "non-volatile storage device" from dep. 27 into the independent.

### Claim 30 — KMF wire format

- **Closest single:** Apache Parquet specification (Parquet footer + per-row-group hash).
- **Closest combination:** Parquet + IPFS content-addressing + Arrow column-major.
- **Risk:** HIGH. Format claims are routinely attacked on obviousness. The differentiator must be the *combination* — column-major bit packing aligned to `D mod 8 = 0`, magic trailer (NOT footer-record like Parquet), per-block BLAKE3, header-after-data.
- **Sharpening — add to body:** `wherein the said hypervector block is laid out with each hypervector occupying exactly ceil(D/8) bytes with no per-vector padding and with most-significant-bit-first packing within each byte, the said dimension D being constrained to be a multiple of 8, such that a partial reader may compute the byte offset of the k-th hypervector within the said block as k * ceil(D/8) without consulting any per-vector index; and wherein the said trailer magic string differs from the said header magic string by reversal of its first three bytes, providing a directional integrity marker distinguishable from accidental file concatenation.`
  - "Trailer differs from header by reversal" (`KMF\0` vs `FMK\0` — confirmed in code) is a strong distinguishing limitation that Parquet does not teach.

### Claim 37 — KMF system

- As claim 30. Add: "the said system being operable to produce a byte-identical wire-format file when emitting to a local file system, an IndexedDB object store, a SQLite database row, or an object-storage bucket" (currently in dep. 39 — promote).

### Claim 42 — KMF CRM

- As claim 30.

### Claim 46 — Thermometer encoder

- **Closest single:** Imani et al. 2017 (VoiceHD, ICRC 2017).
- **Risk:** HIGHEST in the claim set. Imani teaches thermometer level encoding to binary hypervectors. The differentiator must be the cryptographic seeding AND the cross-implementation byte-identity AND the conformance verification.
- **Sharpening — add to body:** `wherein the said per-coordinate seed byte string includes both the index i and the level lvl_i in decimal ASCII form, separated by a single ASCII colon, the said decimal ASCII form being the canonical decimal representation without leading zeros or sign characters; and wherein an output produced by a first implementation of the said method on a first processor architecture is byte-identical to an output produced by a second implementation of the said method on a second, different processor architecture, the said byte-identity being verifiable by computing the SHA-256 digest of the packed bytes of the said accumulator hypervector and comparing the said digest, byte-for-byte, against a value recorded in a canonical conformance corpus shipped with each said implementation.`
  - The "canonical decimal ASCII form" limitation is specific enough to bypass Imani's PRNG-seeded encoder (Imani never specifies a stringified seed format).

### Claim 52 — Conformance corpus

- **Closest single:** NIST CAVP / FIPS 140-3 test vectors.
- **Risk:** HIGH. The methodology is well-known in cryptographic validation.
- **Sharpening:** Differentiation must rest on the *approximate* nature of the vector substrate AND the per-test-group structure that pairs deterministic operations (hash) with approximate operations (similarity, with tolerance). Add to body: `wherein the said canonical conformance corpus comprises both (i) bit-exact test groups whose expected output is a SHA-256 digest of packed bytes, and (ii) approximate test groups whose expected output is a numeric scalar paired with a stated absolute tolerance, the combination of both kinds in a single corpus being adapted to verify an approximate-vector data structure for which only certain primitive operations are required to be bit-exact while other operations are required only to be within tolerance.` This dual-kind structure is not taught by NIST CAVP, which is purely bit-exact.

### Claim 58 — Permutation-positional text encoder

- **Closest single:** Plate 1995 (HRR positional encoding) + Imani text-encoding works.
- **Risk:** MEDIUM-HIGH.
- **Sharpening:** Already includes (i) binary substrate, (ii) BLAKE3-anchored per-word HV, (iii) positional rotation. Strengthen by adding the *tiebreaker* dependency in body: `wherein the said bundling step (c) employs the deterministic tiebreaker defined by claim 1, such that the output of the said method is byte-identical across heterogeneous implementations verified by the conformance corpus of claim 52.` This locks the text encoder to the rest of the claim set as a unified inventive concept (also helps §16 unity-of-invention; see Perspective 4).

---

## Perspective 4 — Indian Patent Attorney

### Single-sentence form

The IPO convention is that each claim is a single grammatical sentence. **Audit:**

- Claims 1, 7, 13, 18, 25, 30, 37, 42, 46, 52, 58: all are single sentences with semicolon-separated steps and a closing "characterised in that" clause. **PASS.**

### Antecedent basis

- **Claim 18(a):** "a bounded sliding window of identifier batches recorded for successive recall operations against the said substrate" — "the said substrate" has no antecedent. Preamble says "a binary hyperdimensional associative memory substrate" — antecedent should be "the said associative memory substrate". **FIX:** make the preamble's "a binary hyperdimensional associative memory substrate" the antecedent and use exactly the same phrase throughout the body.
- **Claim 18 preamble:** mentions "an access log" once ("from a wire-format snapshot together with an access log") but no step in (a)-(f) defines the access log contents. **FIX:** insert in step (a) `; and maintaining an access log recording, for each recall operation, the identifier batch presented` — this provides antecedent basis for "the access log" used elsewhere.
- **Claim 18(e):** "an 8-byte little-endian unsigned integer" salt — but no antecedent for "a salt." Preamble does not introduce the salt. **FIX:** in step (a) introduce "a per-pass salt value" so that step (e) can refer back as "the said salt."
- **Claim 25(c):** "a per-pass salt" — first appearance; should be introduced earlier or replaced with "an integer salt value associated with the said consolidation pass."
- **Claim 30(c)(ii):** "a metadata block comprising `n` rows of item metadata" — "n" appears without antecedent (claim 30 never introduces n in (a) or (b)). **FIX:** modify (c) to begin `wherein the said hypervector block (i) comprises n packed binary hypervectors, where n is the said item count, ...`.
- **Claim 30(e):** "an item count" — first use; provides own antecedent. **PASS** (introduces by "an").
- **Claim 42:** mirror of claim 30; same n antecedent fix needed in step (b).
- **Claim 46(c)(v):** "the said accumulator hypervector" — antecedent established in (b). **PASS.**
- **Claim 52(a):** "a canonical conformance corpus encoded as a JSON document" — should be "**the** canonical conformance corpus" in (b), (c), (d) ... most are correct but (d) says "the said comparison" — antecedent is in (c). **PASS.**
- **Claim 58(b)(ii):** "the corresponding word hypervector" — antecedent is "a binary hypervector of dimension D" in (b)(i), which is grammatically not a hypervector "of `w_j`." **FIX:** rename (b)(i) target to "a per-word hypervector for w_j" so (b)(ii) can refer back as "the said per-word hypervector."

### "Characterised in that" placement

All independent claims place "characterised in that" after the open-comprising step list. **PASS overall.** However:

- **Claim 1's** characterising clause is 119 words long and contains a "such that" subordinate clause and a "thereby" final clause — this is acceptable but stylistically heavy; consider splitting into two characterising sentences via "; and further characterised in that". **MINOR.**
- **Claim 30's** characterising clause has three coordinated verifications joined by "and" — readable. **PASS.**

### Dependent-claim references and multi-dependence

- Indian practice (per the Manual of Patent Office Practice and Procedure, 2019) permits multi-dependent claims **provided they do not depend on other multi-dependent claims**.
- **Claim 3** depends on "claim 1 or claim 2." **PASS.**
- **Claim 4** depends on "any of claims 1 to 3." **PASS** (depends only on independent + a single-dependent + an OR-dependent — not on another multi-dependent).
- **Claim 5** depends on "any of claims 1 to 4." Claim 4 is itself a multi-dependent — this risks objection under the rule against chained multi-dependence. **FIX:** restrict to "any of claims 1 to 3."
- **Claim 6, 22, 23, 24, 28, 29, 33, 34, 35, 36, 40, 41, 44, 45, 48, 49, 50, 51, 54, 55, 56, 57, 60, 61, 62, 63** all use "any of claims X to Y" patterns. Audit each for chained multi-dependence. **REVIEW EVERY ONE** but the consistent fix is: each "any of claims X to Y" range must not include any other multi-dependent claim.

### Unity of invention (§16)

The eleven independent claims fall under five "themes":

| Theme | Claims | Common inventive concept |
|---|---|---|
| Tiebreaker | 1, 7, 13 | BLAKE3-anchored deterministic bit selection |
| Consolidation | 18, 25 | Replayable BLAKE3-seeded bit-flip ordering |
| Wire format | 30, 37, 42 | Per-block BLAKE3 + magic trailer + column-major bit packing |
| Encoder | 46, 58 | BLAKE3-anchored deterministic feature-to-HV mapping |
| Conformance | 52 | Cross-implementation byte-identity guarantee |

The **single inventive concept** that unites all five themes is: *deterministic, cross-implementation byte-identity of a binary hyperdimensional associative memory substrate, achieved by anchoring every operation to a single cryptographic hash function (BLAKE3)*. This is articulable as a unity-of-invention argument.

**Risk:** the IPO may take the view that themes 1-5 are five separate inventive concepts, in which case a divisional filing (under §16) would be required. **RECOMMENDATION:** retain unity by adding a "common general inventive concept" preamble to the specification (in §5 Summary or §07 Detailed Description) that explicitly identifies the BLAKE3-anchoring as the common concept. Cite *Genentech Inc.'s Patent* [1989] RPC 147 (UK, but persuasive in India) on the test for unity.

If the IPO raises a §16 objection, the fallback is to file as one parent + four divisionals — costs roughly 4x; therefore make the unity argument now.

### Comprising vs consisting of

- All independent claims use "comprising." **PASS** (open form, correct for India).
- **Claim 3:** "from the group consisting of" — correct closed Markush form for a list of enumerated alternatives. **PASS.**
- **Claim 11, 39, 48:** same. **PASS.**

### Numbering continuity

- Claims are numbered 1-63 continuously. **PASS.**
- However, **Claim 30(c)** uses sub-enumeration (i), (ii), (iii) without a parallel pattern in claims 18 or 46. **STYLE NIT, not objectionable.**

### Total claim count

- 63 claims is **excessive** for India. Per the First Schedule of the Patents Rules 2003 (as amended 2024), the official fee structure charges per-claim above 10. 63 claims = 53 excess claims = ~INR 88,000 in excess-claim fees for a small entity.
- **RECOMMENDATION:** retract the system+CRM triplets for at least two of the weaker themes (52, 46) — keep method-only for those. This brings the count below 55, with one less independent (saves ~₹16,000) and avoids the strongest §3(k) attacks on the methodology claim 52.

---

## Perspective 5 — Domain Critic (code-alignment fixes)

### Tension 1 — Total claim count (63 vs ~36 brief target)

**Verdict:** Retract two triplets. Retain (1, 7, 13) and (30, 37, 42) as the strongest. Drop (25) as a separate independent — fold its features into dep. claims of (18). Drop (37, 42) → no, retain (30, 37, 42) for wire-format because the wire format is a system that genuinely sits in different runtimes. Drop the CRM triplet (13, 42) if claim count must come down. **CONCRETE PROPOSAL: drop claim 25 and its dependents 26-29 (save 5 claims); drop the CRM triplet for the encoder claim 46 (save 0, none exists). Target: 58 claims.**

Alternative more aggressive: drop claims 13-17 (tiebreaker CRM) and 42-45 (KMF CRM). Saves 10 claims, brings to 53. CRM triplets are most §3(k)-vulnerable.

### Tension 2 — SPEC §1.3 lacks `"smritidb/tiebreak"` prefix

Verified: `git show 17334f8:SPEC.md` line 44: `> For each bit position with an exact tie, the result bit is H(D || index || count)[0] where H is BLAKE3 and || is byte concatenation.` — NO domain prefix.

Verified: `git show 17334f8:packages/core-ts/src/hypervector.ts` lines 13 and 86-95: `const TIEBREAKER_DOMAIN = "smritidb/tiebreak"; ... seed[i] = TIEBREAKER_DOMAIN.charCodeAt(i); ... view.setUint32(0, D, true); view.setUint32(4, index, true); view.setUint32(8, count, true);`

Code prepends the tag; SPEC does not mention it. Claim 1 step (d) and claim 2 both recite the tag.

**FIX:** repo errata commit to SPEC.md §1.3 BEFORE filing, adding the tag. Patent claim wording follows code (correctly), but if SPEC and code disagree, an examiner can attack §10(4) sufficiency. **Action item: SPEC errata commit required pre-filing.**

### Tension 3 — SPEC §8.2 says MessagePack but code uses JSON

Verified: SPEC.md line 316: `| meta_block | n rows of { id, tags, metadata, createdAt, accessCount, lastAccessedAt } in MessagePack. |`

Verified: `git show 17334f8:packages/core-ts/src/kmf.ts` line 75-82: `const metaJson = encodeAscii(JSON.stringify(snapshot.items.map(...)))` — uses JSON, not MessagePack.

Claim 30(c)(ii) says "a metadata block comprising n rows of item metadata" — encoding-agnostic. **PASS for the independent.** Dep. claim 31 explicitly says "JSON document" for the header — matches code.

**FIX:** SPEC errata commit changing "MessagePack" to "JSON" in §8.2. **OR** add to claim 31 (or a new dep) the statement "wherein the said metadata block is encoded as a JSON document." Both: do the SPEC errata AND add the dep claim, so the claim is supported by both code and SPEC.

### Tension 4 — `pullCloser` salt mechanism

Verified: `git show 17334f8:packages/core-ts/src/consolidate.ts` lines 117-127: `pullCloser(a, b, maxSimDelta, salt)`; salt is encoded as 8-byte little-endian via `setBigUint64(0, BigInt(salt), true)`; digest length is `Math.max(64, toFlip * 4)`; ordering is by `digest[bit % digest.length]`.

Claim 18(e) and 22 recite the salt mechanism — **matches code exactly.**

**Verdict:** intent confirmed. Keep claim 18(e) and 22 as written. **No fix needed.**

### Tension 5 — Salt monotonicity (claims 21, 29) not implemented in code

Verified: `git show 17334f8:packages/core-ts/src/consolidate.ts` — the `pullCloser` function takes `salt` as a parameter but the *caller* is responsible for monotonicity. Searching for monotonic salt assignment in the codebase at SHA 17334f8: there is no caller in the tree at this SHA that drives `pullCloser` with a monotonic salt; the consolidation orchestration (which would assign `salt = 0, 1, 2, ...`) is **deferred.** SPEC §5 says "deterministic given identical inputs"; orchestration is in v0.2.0.

Claim 21: "salt is incremented monotonically across consolidation passes" — **not in code.**
Claim 29: "per-pass salt is derived from an opaque consolidation-epoch counter persisted alongside the said substrate" — **not in code.**

**FIX (two options):**
- **(a) Remove claims 21 and 29.** Cleaner. Reduces claim count.
- **(b) Implement monotonic salt orchestration in code at a pre-filing commit (claim 21) and persist an epoch counter in the snapshot header (claim 29).** This is a small change: add a `salt` counter to `ConsolidationConfig` state, increment per pass, store in KMF header. Roughly 30 lines of code.

**RECOMMENDATION: option (b)** — the orchestration is described in the brief and SPEC; aligning code is correct intent. Make the pre-filing commit. If time-bound, fall back to (a).

### Tension 6 — Claim 52 borderline §3(k)

Already addressed in Perspective 2. **Fix: convert to system/apparatus claim per Perspective 2 reframing.**

### Tension 7 — Antecedent basis tightness (claim 18(e), 30)

Already addressed in Perspective 4. **Fixes:**
- Claim 18: add "an access log" introduction in step (a), add "a per-pass salt value" introduction in step (a).
- Claim 30: introduce "n is the said item count" in step (c).

### Tension 8 — Text encoder claim 58(b)(iii) uses "element-wise exclusive-or" but theme says "bind"

Verified: `git show 17334f8:packages/core-ts/src/text.ts` line 95: `acc = xor(acc, term);` — uses XOR. Function `xor` is defined at line 113: `for (let i = 0; i < a.length; i++) out[i] = a[i]! ^ b[i]!;` — element-wise XOR on unpacked bytes.

Verified: `git show 17334f8:packages/core-ts/src/hypervector.ts` line 38-44: `bind(a, b)` is also `a[i] ^ b[i]` — the *same operation* with different name.

Claim 58(b)(iii): "combining the said unrotated first hypervector with the n - 1 rotated subsequent hypervectors by element-wise exclusive-or to produce the said n-gram hypervector" — this is correct for the code (`xor` is used), but is **terminologically inconsistent** with claim 46(c)(v) which uses "element-wise exclusive-or" to bind level hypervectors and claim 1 dep. 6 which says "element-wise exclusive-or of pairs of hypervectors."

**FIX:** unify terminology. Replace every occurrence of "element-wise exclusive-or" with "binding operation defined as element-wise exclusive-or" or simply "bind operation." This keeps the substantive matter unchanged but ties the claim language to the SPEC §3 "bind" primitive. **Concrete change:**

- Claim 1 dep. 6: replace `"by element-wise exclusive-or of pairs of hypervectors"` with `"by a bind operation comprising element-wise exclusive-or of pairs of hypervectors"`.
- Claim 46(c)(v): replace `"by element-wise exclusive-or with the said per-coordinate hypervector"` with `"by a bind operation comprising element-wise exclusive-or with the said per-coordinate hypervector"`.
- Claim 58(b)(iii): replace `"by element-wise exclusive-or"` with `"by a bind operation comprising element-wise exclusive-or"`.

### Tension 9 — golden.json entry counts vs claim wording

Verified: `git show 17334f8:tests/conformance/golden.json` test-group keys are:
1. `random_hv` (5 entries) ✓
2. `encode_string` (3 entries) ✓ — but claim 53 says "string-encoding group" which is correct
3. `similarity_pairs` (3 entries) ✓
4. `bind_round_trip` (1 entry) ✓
5. `bundle` (2 entries) ✓
6. `text_bag_of_words` (2 entries) — claim 53 says "**a bag-of-words group** or **a bag-of-words group**" — must check
7. `text_char_ngrams` (2 entries) — claim 53 does **not** mention this group

Claim 53 says: `"the said test groups comprise at minimum a random-hypervector group, a string-encoding group, a similarity-pairs group, a bind-round-trip group, and a bundle group"`.

Code has all five of these PLUS `text_bag_of_words` and `text_char_ngrams`. Claim 53 says "at minimum" so the extra groups are allowed. **PASS** — but consider widening claim 53 to mention text groups, OR keep "at minimum" wording so the claim covers future additions.

**RECOMMENDATION:** Add to claim 53: `"and, where text encoding is supported by the implementation, a bag-of-words text-encoding group and a character-n-gram text-encoding group."` This locks the disclosed test groups into the claim.

Also: claim 52(a) says corpus comprises "(i) a canonical input including a textual seed, a numeric dimension, and where applicable an operation type, and (ii) an expected output." But the code's `similarity_pairs` group has `expected: 0.5185546875` (a scalar) and `bind_round_trip` has `expected_similarity_to_a: 1` (a scalar). Claim 52(a)(ii) does say "or a numeric expected scalar with a stated tolerance" — covers scalar form. **But: `expected_similarity_to_a: 1` is not paired with a tolerance in the corpus.** Either:
- **(a)** Add `tolerance` fields to `golden.json` (repo errata).
- **(b)** Soften claim wording: `"...or a numeric expected scalar paired with an implicit or stated tolerance, the said implicit tolerance being floating-point representation-equivalent."`.

**RECOMMENDATION: option (a)** — small repo edit to `golden.json` adding `tolerance: 0.0` to bit-exact scalar entries and `tolerance: 0.01` to similarity entries.

### Additional code/claim verifications

- **Claim 11 dependency on dimension D ∈ {1024, 8192, 10000, 16384}** — code at `SPEC.md` §1.1 says `D` is configurable but the default is 10000; no enumeration constraint in the code. **OK** as a dependent claim narrowing.
- **Claim 46(c)(ii)** says seed string is `"lvl:" + decimal-i + ":" + decimal-level`. Code (`encode.ts` line 18): `const seedString = \`${LEVEL_DOMAIN}${i}:${level}\`;` → literally `lvl:0:50` etc. **MATCHES.** No ASCII-form constraint in code but the JS template literal produces decimal ASCII by definition. Cross-language constraint must be explicit in claim → already in claim 46(c)(ii). **PASS.**
- **Claim 58(b)(ii)** says cyclic bit rotation by exactly `j` bit positions. Code uses `permute(hv, j)` where `permute` does element-wise shift (`out[(i + shift) % D] = hv[i]`) — **NOT a bit rotation on packed bits**, but a permutation on unpacked single-bit-per-byte form. Since each byte holds one bit, the result is *mathematically equivalent* to a bit rotation, but the claim language "cyclic bit rotation" implies an operation on packed bits. **FIX:** replace "cyclic bit rotation" with "cyclic permutation" in claim 58(b)(ii). The claim should track the SPEC primitive (`permute`) not the on-disk packed-bit semantics.
- **Claim 30(d)** says "encoding the said digest as a hexadecimal string" — code (`kmf.ts` line 95): `blake3: toHex(blake3(b.bytes))`. **MATCHES.**

---

## Consolidated Revision Worklist

The following is the prioritised, claim-numbered, action-by-action list for the Round 2 revision pass. Each item references a specific claim and a specific edit. Items marked **REPO** are pre-filing repository changes (SPEC errata, code commits, corpus edits) and must precede the patent filing.

### Priority A — Survival under §3(k) (highest examiner risk)

1. **Claim 1** — Move the technical effect from the preamble into the characterising clause. Add: `"by operation upon the packed bit-array stored in the said memory, causes the said packed output hypervector to be byte-identical across heterogeneous processor architectures, thereby reducing storage input/output ... by permitting a single persisted snapshot to be loaded and verified bit-exactly by every conformant implementation without re-execution of the bundle operation."`
2. **Claim 7** — Add hardware-tie clause: `"wherein the said memory comprises a non-volatile storage region holding the said packed bit-array between executions, and the said deterministic tiebreaker module is executed on the data path between the said memory and the said non-volatile storage region."`
3. **Claim 13** — Replace preamble `"instructions which, when executed by one or more processors"` with `"instructions which, when loaded into a memory of a computing apparatus and executed by one or more processors of the said apparatus, configure the said apparatus to"`. (Per OpenTV 2023.)
4. **Claim 42** — Insert into preamble before "the said method comprising": `"in a manner that provides corruption detection prior to substrate state reconstruction and that reduces storage input/output by permitting a single canonical wire-format file to be shared between heterogeneous implementations,"`.
5. **Claim 46** — Append to characterising clause: `"the said method being executed entirely in integer arithmetic and bitwise operations on the said processor, without invocation of any floating-point matrix-multiplication unit or neural-network inference accelerator, such that the said binary hypervector is producible on a microcontroller-class processor lacking floating-point hardware."`
6. **Claim 52** — Convert from method-form to system-form per Perspective 2 reframing (full preamble + body rewrite as a `"computer system for verifying ... comprising at least one processor, a non-volatile storage device storing a canonical conformance corpus file, and a memory storing instructions"`). Add at the end of the characterising clause: `"the said system rejects, at load time of any substrate snapshot produced by a foreign implementation, any said snapshot whose computed output ... fails to match the said expected digest"`.
7. **Claim 58** — Append to characterising clause: `"the said method being computable entirely by integer arithmetic and bitwise operations, such that the said binary hypervector is producible on a processor lacking any floating-point unit."`

### Priority B — Prior-art sharpening (inventive-step survival)

8. **Claim 1** — Add to body: `"wherein the said domain tag, byte order, and field widths are recorded as a mandatory field of the wire-format header of any persisted snapshot of the said associative memory substrate."`
9. **Claim 7** — Add: `"wherein the said tiebreaker module is exposed via a foreign-function-interface boundary that is byte-transparent, such that an output produced by a native-code invocation thereof is byte-identical to an output produced by a WebAssembly-sandboxed invocation thereof."`
10. **Claim 13** — Promote dep. 15 (WebAssembly bytecode) into the independent: `"wherein the said instructions comprise a portable bytecode executable across heterogeneous processor architectures without re-compilation."`
11. **Claim 18** — Move maxSimDelta upper-bound and replayability statement from dep. 19 and dep. 23 into the independent body: `"wherein floor(maxSimDelta * D) is at most one percent of D, and wherein the said consolidation log is sufficient, together with the prior substrate snapshot, to reconstruct the post-consolidation substrate state byte-for-byte without retention of any intermediate substrate copy."`
12. **Claim 25** — Promote "non-volatile storage device" from dep. 27 into the independent body.
13. **Claim 30** — Add to body: `"wherein the said hypervector block is laid out with each hypervector occupying exactly ceil(D/8) bytes with no per-vector padding, the said dimension D being constrained to be a multiple of 8; and wherein the said trailer magic string differs from the said header magic string by reversal of its first three bytes, providing a directional integrity marker."`
14. **Claim 37** — Promote dep. 39 ("output sink selected from local file system, IndexedDB, SQLite, object storage") into the independent body.
15. **Claim 46** — Add to body: `"wherein the said per-coordinate seed byte string includes both the index i and the level lvl_i in canonical decimal ASCII form without leading zeros or sign characters, separated by a single ASCII colon; and wherein an output produced by a first implementation of the said method on a first processor architecture is byte-identical to an output produced by a second implementation of the said method on a second, different processor architecture, the said byte-identity being verifiable by computing the SHA-256 digest of the packed bytes ... against a value recorded in a canonical conformance corpus."`
16. **Claim 52** — Add to body: `"wherein the said canonical conformance corpus comprises both (i) bit-exact test groups whose expected output is a SHA-256 digest, and (ii) approximate test groups whose expected output is a numeric scalar paired with a stated absolute tolerance."`
17. **Claim 58** — Add to body: `"wherein the said bundling step (c) employs the deterministic tiebreaker as claimed in claim 1, such that the output of the said method is byte-identical across heterogeneous implementations."`

### Priority C — Antecedent basis (drafting hygiene)

18. **Claim 18(a)** — Insert at end of step (a): `"; and maintaining an access log recording, for each said recall operation, the identifier batch presented and a per-pass salt value associated with the said operation"`. This establishes antecedent basis for "the access log" (preamble) and "the said salt" (step e).
19. **Claim 25(a)** — Mirror fix: introduce "an access log" and "a per-pass salt value" in step (a).
20. **Claim 30(c)** — Modify opening of (c): `"writing one or more data blocks, the said data blocks comprising n entries where n is the said item count, each said data block being of a kind selected from the group consisting of..."` to provide antecedent for n in the body.
21. **Claim 42(b)** — Mirror fix for n.
22. **Claim 58(b)(ii)** — Replace `"the corresponding word hypervector"` with `"the said per-word hypervector"` and amend (b)(i) to introduce as `"a per-word hypervector for w_j"`.

### Priority D — Terminology unification

23. **Claims 1 (dep. 6), 46(c)(v), 58(b)(iii)** — Replace every occurrence of `"element-wise exclusive-or"` (as a standalone primitive) with `"a bind operation comprising element-wise exclusive-or"` to tie the claim language to the SPEC §3 "bind" primitive.
24. **Claim 58(b)(ii)** — Replace `"cyclic bit rotation"` with `"cyclic permutation"` to track the code's `permute()` primitive rather than the packed-bit semantics.

### Priority E — Claim count and structure

25. **Drop claims 13-17 (Tiebreaker CRM triplet)** — most §3(k)-vulnerable; the method claim 1 and the system claim 7 cover the substantive matter. Saves 5 claims.
26. **Drop claims 42-45 (KMF CRM triplet)** — same rationale. Saves 4 claims.
27. **Alternative if CRMs are retained:** apply Priority A fixes 3 and 4 to anchor them on apparatus.
28. **Consolidate claim 25 into claim 18 dependents** — drop 25-29 as separate independent triplet; fold into dependent claims of 18. Saves another 5 claims.
29. **Add unity-of-invention recitation to the specification (NOT a claim change but a §10(4) cross-reference)** — in `05-summary-of-invention.md` or `07-detailed-description.md`, add an explicit paragraph identifying "BLAKE3-anchored deterministic byte-identity of a binary hyperdimensional associative memory substrate" as the **single inventive concept** uniting all themes. Cite *Genentech*. (Per Perspective 4 §16 analysis.)

### Priority F — Dependent-claim multi-dependence fix

30. **Claim 5** — Restrict from `"any of claims 1 to 4"` to `"any of claims 1 to 3"` (claim 4 is multi-dependent; avoid chained multi-dependence).
31. **Audit every "any of claims X to Y" in claims 6, 22, 23, 24, 28, 29, 33, 34, 35, 36, 40, 41, 44, 45, 48, 49, 50, 51, 54, 55, 56, 57, 60, 61, 62, 63** — replace any chained multi-dependence with a single-claim or two-claim "or" reference.

### Priority G — Dependent-claim language tightening

32. **Claim 19** — `"fixed at 0.02"` is a hard limit; soften to `"a value not greater than 0.02"` to allow safe-harbour configuration.
33. **Claim 47** — `"L is fixed at 100"` — same softening to `"a value of at least 64 and not greater than 256"` (matches SPEC's stated defaults envelope).
34. **Claim 53** — Add: `"and, where text encoding is supported by the implementation, a bag-of-words text-encoding group and a character-n-gram text-encoding group."` (tracks corpus contents.)
35. **Claim 21** — DECISION: either (a) DROP, or (b) commit monotonic-salt orchestration to code before filing. **Recommend (b).**
36. **Claim 29** — Same decision as claim 21.

### Priority H — Repo errata (pre-filing commits)

37. **REPO: SPEC.md §1.3** — add `"smritidb/tiebreak"` ASCII tag prefix to the tiebreaker recipe. Two-line edit.
38. **REPO: SPEC.md §8.2** — change `"in MessagePack"` to `"in JSON or MessagePack as specified by the implementation; the reference implementation uses JSON"`. One-line edit.
39. **REPO: tests/conformance/golden.json** — add `tolerance` fields to scalar entries in `similarity_pairs` and `bind_round_trip` groups. Five-line edit.
40. **REPO (optional, per claim 21/29 decision):** commit monotonic salt orchestration to `consolidate.ts` plus epoch-counter persistence in `kmf.ts` header. ~30 lines.

---

## Tally

- **Objections raised (Perspective 1, IPO FER-style):** **31 distinct objections** across 11 independent claims (Claim 1: 3; Claim 7: 2; Claim 13: 2; Claim 18: 4; Claim 25: 2; Claim 30: 3; Claim 37: 2; Claim 42: 2; Claim 46: 3; Claim 52: 4; Claim 58: 3; cross-cutting: 1).
- **Revisions in the Consolidated Worklist:** **40 numbered action items** (Priority A: 7; Priority B: 10; Priority C: 5; Priority D: 2; Priority E: 5; Priority F: 2; Priority G: 5; Priority H: 4).

**Strongest immediate threats (file killers if not addressed):** items 6 (Claim 52 §3(k)), 11 (Claim 18 prior-art), 13 (Claim 30 trailer-reversal limitation), 37-38 (SPEC errata commits).
