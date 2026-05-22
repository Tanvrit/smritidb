<!-- Round 3 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Example 3 (b): formula corrected from `l_i = floor((v_i + 1) / 2 * L)` to `l_i = clamp(round((v_i + 1) * (L - 1) / 2), 0, L - 1)` to match `core-ts/src/encode.ts` and `core-rs/src/encode.rs` at SHA 17334f8 and the corresponding Round-2 corrections in 06-drawings-list.md Fig. 6 step 803 and 07-detailed-description.md §5.2.1; numeric example values updated accordingly.
- Example 3 (c): per-coordinate seed re-expressed as `"lvl:" || decimal_ascii(i) || ":" || decimal_ascii(l_i)` per Claim 27(c)(ii) canonical decimal ASCII form.
- Example 3: phantom "Dimension-binding by permutation" step (was step 805 in Round 1) deleted to match Round-2 Fig. 6 correction; encoder is XOR-accumulate only, with no tiebreaker invocation.
- Example 5 (e), (h), and §5.3 (b), (d): meta_block metadata-row encoding changed from "MessagePack" to "JSON (Phase 1; MessagePack contemplated for Phase 2)"; header zstd-compression marked as Phase 2 contemplated.
-->

# Worked Examples (Embodiments)

The following worked examples are provided in support of the detailed description set out in §07 of the present specification, and in support of the claims set out in §08. Each example specifies its setup parameters, its step-by-step computation, its expected output, and a verification command anchored to the pinned source SHA `17334f8`. The said verification commands are reproducible by any third party in possession of the source repository at the said SHA.

Reference numerals follow the convention established in `06-drawings-list.md`: 100-series for major blocks, 200-series for primitive operations, 300-series for in-memory substrate, 400-series for KMF wire-format fields, 500-series for persistence-adapter interface methods, and 800-series for encoder steps.

---

## Example 1 — Deterministic Random Hypervector from a UTF-8 Seed

### 1.1 Setup

The random-hypervector primitive (250) of Fig. 1 is invoked with the following parameters:

- Input seed (UTF-8 byte string): `"hello"`.
- Substrate dimension `D = 1024`.
- Expected output: a packed bit-array of length `ceil(1024 / 8) = 128` bytes, with most-significant-bit-first ordering within each byte.

### 1.2 Step-by-step computation

(a) The UTF-8 bytes `[0x68, 0x65, 0x6c, 0x6c, 0x6f]` are supplied as the seed.

(b) A BLAKE3 extendable-output expansion is performed: `H := BLAKE3_XOF(seed, output_len = 128)`. The said expansion is defined deterministically by the BLAKE3 specification and produces the same 128-byte sequence on every conformant implementation.

(c) The 128 output bytes are interpreted as the packed bit representation of the binary hypervector of dimension 1024, with MSB-first bit ordering within each byte.

### 1.3 Expected output

The SHA-256 digest of the 128-byte packed output is, exactly:

```
debc098fcb2315102c8eccba2120dd02d85ea495f2b482ebb582e6fd63e4fdc3
```

This digest is the bit-exact contract for every conformant implementation of `randomHV(seed = "hello", D = 1024)`.

### 1.4 Verification

```sh
git show 17334f8:tests/conformance/golden.json | jq '.random_hv[2]'
```

returns the entry:

```json
{
  "seed_utf8": "hello",
  "dim": 1024,
  "sha256": "debc098fcb2315102c8eccba2120dd02d85ea495f2b482ebb582e6fd63e4fdc3"
}
```

A conformant implementation may be verified against this entry by computing `randomHV("hello", 1024)` and reporting `SHA-256` of the resulting 128 bytes; the said digest must equal the digest given above. This is the procedure used by the conformance test suite at `git show 17334f8:packages/core-ts/src/conformance.test.ts` (TypeScript) and `git show 17334f8:packages/core-rs/tests/conformance.rs` (Rust), each of which passes against the said golden corpus.

The corpus further includes entries for `D = 64`, `D = 256`, a different seed (`"world"`), and a Unicode seed (`"é unicode 你好"`), each of which exercises the same primitive at a distinct parameter combination.

---

## Example 2 — Bundle of Three Hypervectors with Tiebreaker Resolution at D = 8

### 2.1 Setup

For ease of exposition this example uses a deliberately small dimension `D = 8`. The actual production substrate uses `D = 10000` by default; the algorithm is identical, only the bit-array length differs.

Three input binary hypervectors:

```
v1 = 1 0 1 1 0 0 1 0
v2 = 0 0 1 1 1 0 0 1
v3 = 1 1 0 1 0 1 1 0
```

### 2.2 Per-bit majority sums

```
position i :   0  1  2  3  4  5  6  7
sum s_i    :   2  1  2  3  1  1  2  1
```

With multiplicity `n = 3`, the majority rule is: output bit `= 1` if `s_i > 1.5`, output bit `= 0` if `s_i < 1.5`. Since `n = 3` is odd, no tie can occur for `n = 3`, and the output is, without invocation of the tiebreaker (260):

```
out = 1 0 1 1 0 0 1 0
```

### 2.3 Tied bundle: four hypervectors, demonstrating the tiebreaker

Now consider a bundle of multiplicity `n = 4`:

```
v1 = 1 0 1 1 0 0 1 0
v2 = 0 0 1 1 1 0 0 1
v3 = 1 1 0 1 0 1 1 0
v4 = 0 1 0 0 1 1 0 1
sum:  2 2 2 3 2 2 2 2
```

Every position except position 3 has `s_i = 2`, which equals `n / 2`; these positions are ties. The tiebreaker (260) is invoked for positions `i ∈ {0, 1, 2, 4, 5, 6, 7}`. For each said position `i`, the tiebreaker computes:

```
input = "smritidb/tiebreak" || D.to_le_bytes() || i.to_le_bytes() || n.to_le_bytes()
      = "smritidb/tiebreak" || [0x08, 0x00, 0x00, 0x00]
                            || [i_byte, 0, 0, 0]
                            || [0x04, 0x00, 0x00, 0x00]
H = BLAKE3(input)
output bit at position i = H[0] & 1
```

Position 3 is not a tie (`s_3 = 3 > 2`), so the output bit at position 3 is `1` directly from majority. Positions 0-2 and 4-7 obtain their output bit from `BLAKE3` as described.

### 2.4 Verification

The conformance corpus at `git show 17334f8:tests/conformance/golden.json` includes the entries `bundle[0]` (three seeds `"x0", "x1", "x2"` at `D = 1024`) and `bundle[1]` (six seeds at `D = 1024`), with SHA-256 digests:

```
9720b13f8c6da366cedc524cc8105bfc19a4fa6dd86353cb1ab2fd14ccad05fd     (n=3)
237062134b527eaf780e0c1a1a883ac95e039ec0edbf34dece934f37f21bf35c     (n=6)
```

Both the `n = 3` and the `n = 6` cases involve even-valued tie positions which are resolved by the tiebreaker (260) and which any conformant implementation must reproduce byte-for-byte.

---

## Example 3 — Embedding Encode and Similarity Round-Trip

### 3.1 Setup

The thermometer-quantised random-projection encoder (120) of Fig. 6 is invoked with the following parameters:

- Input embedding `v = [0.30, -0.70, 0.00, 0.90]`, an `f32` vector of length `d = 4`.
- Quantisation levels `L = 100`.
- Substrate dimension `D = 1024`.

### 3.2 Step-by-step computation

(a) **Clamp** (step 802): each component is clamped to the range `[-1, 1]`; in this example all components are already in range, so the clamped vector equals the input vector.

(b) **Quantise** (step 803): for each component `v_i`, the integer level is computed as `l_i = clamp(round((v_i + 1) * (L - 1) / 2), 0, L - 1)`:

```
v_0 =  0.30  ->  l_0 = round(1.30 * 99 / 2) = round(64.35) = 64
v_1 = -0.70  ->  l_1 = round(0.30 * 99 / 2) = round(14.85) = 15
v_2 =  0.00  ->  l_2 = round(1.00 * 99 / 2) = round(49.50) = 50
v_3 =  0.90  ->  l_3 = round(1.90 * 99 / 2) = round(94.05) = 94
```

(c) **Per-(coordinate, level) hypervector derivation** (step 804): for each `i`, a level hypervector `levelHV_i` is derived as `randomHV(BLAKE3("lvl:" || decimal_ascii(i) || ":" || decimal_ascii(l_i)))`. The said derivation is fully specified by the BLAKE3 input and the `randomHV` algorithm of Example 1, and is therefore deterministic across implementations.

(d) **XOR-accumulate** (step 806): the `d` per-coordinate level hypervectors are XOR-accumulated into a single accumulator hypervector. The XOR operation has no ties, so the tiebreaker (260) of Fig. 3 is not invoked in this encoder.

(e) **Output** (step 807): a binary hypervector of dimension 1024.

### 3.3 Expected property: similarity preservation

Two input vectors with cosine similarity `s_cos` are expected to produce hypervectors with Hamming similarity `s_ham ≈ s_cos`, within a bounded error that decreases monotonically in `L` and in `D`. The said property may be empirically verified by computing the encoder output for a pair of input vectors and comparing the Hamming similarity of the outputs to the cosine similarity of the inputs.

### 3.4 Verification

The conformance corpus does not, in the present preferred embodiment, pin a specific embedding encoder digest; it pins, instead, the underlying `randomHV` and `bundle` primitives upon which the embedding encoder is constructed. The encoder is, accordingly, deterministic by construction. The encoder implementation may be inspected at `git show 17334f8:packages/core-ts/src/index.ts` (for the embedding-path branch of the `encode` function), and the underlying primitives are exercised by `git show 17334f8:packages/core-ts/src/conformance.test.ts`.

---

## Example 4 — Order-Sensitive Text Encoding: "alpha beta" versus "beta alpha"

### 4.1 Setup

The word-n-gram text encoder (140) is invoked with the following parameters:

- Input text A: `"alpha beta"`.
- Input text B: `"beta alpha"`.
- `n = 2` (bigrams).
- Substrate dimension `D = 1024`.

### 4.2 Step-by-step computation

(a) **Normalisation and tokenisation**: each input is tokenised into a list of normalised tokens. For text A the tokens are `["alpha", "beta"]`; for text B the tokens are `["beta", "alpha"]`.

(b) **Per-token hypervector derivation**: for each token `t`, a token hypervector is computed as `randomHV(BLAKE3("word:" || t))`.

(c) **Permutation by intra-window position**: within each bigram window, the token at position `0` of the window is left unrotated and the token at position `1` is cyclically rotated by 1 bit. The two rotated hypervectors are XOR-bound into a bigram hypervector.

For text A:

```
bigram_A = tokenHV("alpha") XOR permute_1(tokenHV("beta"))
```

For text B:

```
bigram_B = tokenHV("beta") XOR permute_1(tokenHV("alpha"))
```

Because the XOR operation is commutative but the permutation step `permute_1` is not the identity, `bigram_A ≠ bigram_B` in general; the two bigrams differ in approximately `D / 2` positions, since `permute_1(tokenHV("beta"))` and `tokenHV("beta")` are uncorrelated and likewise for `tokenHV("alpha")`.

(d) **Bundle**: for a single bigram per text, the bundle is a no-op; the bigram itself is the output. For longer inputs the bundle of all bigrams is the output, with ties resolved by the tiebreaker (260).

### 4.3 Expected output

The Hamming similarity `sim(encode("alpha beta"), encode("beta alpha"))` is, in the said preferred embodiment, noticeably less than `1` — typically in the range `0.50` to `0.55` for a single-bigram input — demonstrating that the encoder is order-sensitive, in contradistinction to a bag-of-words encoder which would map the two inputs to the same hypervector.

### 4.4 Verification

The text-encoder implementations are at `git show 17334f8:packages/core-ts/src/text.ts` and `git show 17334f8:packages/core-rs/src/text.rs`. The conformance corpus at `git show 17334f8:tests/conformance/golden.json` includes the entries `text_bag_of_words[*]` and `text_char_ngrams[*]` (with SHA-256 digests, for example `6a4408963fa846431ac464cceaed5c6a445dc71ec0a9e3249445aa2d5a7553eb` for the bag-of-words encoding of `"the cat sat on the mat"` at `D = 1024`) which lock the encoder output to be byte-identical across implementations.

---

## Example 5 — KMF Round-Trip: Three Items, Snapshot, Parse, Verify

### 5.1 Setup

A substrate is created with `D = 1024` and configured with the memory persistence adapter (610). Three items are inserted using the user-facing `put` API:

```
put(store, key = "alice",   value = b"engineer")
put(store, key = "bob",     value = b"researcher")
put(store, key = "charlie", value = b"writer")
```

Each call invokes the string encoder (110), which derives the key hypervector as `randomHV(BLAKE3("str:" || key_utf8))`. The substrate (300) records the three items with their UUIDv7 identifiers, their key hypervectors, their value payloads, their tags (none in this example), their metadata (none), their `createdAt` timestamps, their `accessCount` (zero), and their `lastAccessedAt` (equal to `createdAt`).

### 5.2 Snapshot

The `snapshot()` API is invoked, which causes the KMF serialiser (400) of Fig. 1 to emit the wire-format byte stream of Fig. 2:

(a) The magic header (410) `"KMF\x00"` is written.
(b) The spec-version string (411) `"0.1.0"` is written.
(c) A placeholder header-offset (412) of eight bytes is reserved.
(d) An `hv_block` (420) is written containing the packed bits of the three key hypervectors, requiring `3 * 128 = 384` bytes.
(e) A `meta_block` (430) is written containing the JSON-encoded metadata rows (Phase 1; MessagePack encoding contemplated for Phase 2).
(f) A `value_block` (440) is written containing the three value payloads, each length-prefixed.
(g) The BLAKE3 digest (450) of each block is computed.
(h) The header (460) is constructed as a JSON document indexing each block with its offset, length, and BLAKE3 digest; the said JSON is written as plain JSON in Phase 1 (zstd compression of the said JSON contemplated for Phase 2).
(i) The header offset is back-patched into field (412) at byte 10.
(j) The trailer magic (470) `"FMK\x00"` is written.

### 5.3 Parse and verify

The `restore()` API is invoked on the produced byte stream:

(a) The trailer magic at `EOF - 4` is checked: it must equal `"FMK\x00"`, failing which the input is rejected with `CorruptSnapshot`.

(b) The header offset (412) at byte 10 is read; the reader seeks to that offset and parses the JSON header (Phase 1; in Phase 2 the reader additionally decompresses the said header from zstd before parsing).

(c) For each block listed in the header's `index` array, the reader reads the said block's bytes at the indexed offset, computes a BLAKE3 digest over the said bytes, and compares the computed digest to the digest recorded in the header. Any mismatch raises `CorruptSnapshot`.

(d) Once all blocks are verified, the `hv_block` is unpacked into three hypervectors of `D = 1024`, the `meta_block` is decoded from JSON (Phase 1; from MessagePack in Phase 2), and the `value_block` is split by its length prefixes.

(e) A new in-memory substrate is constructed and the three items are inserted with their original UUIDv7 identifiers and their original `createdAt` timestamps.

### 5.4 Round-trip identity

A subsequent `recall("alice", topK = 1)` on the restored substrate must return the item previously inserted under the key `"alice"`, with `similarity = 1.0` (within the tolerance prescribed by the `similarity` primitive (240)). The said round-trip is exercised by the SQLite-adapter integration test at `git show 17334f8:packages/core-ts/src/adapters/sqlite.test.ts`, which performs a `put` / `snapshot` / `restore` / `recall` cycle through a real persistence adapter and verifies that the recall result matches the original input byte-for-byte.

### 5.5 Verification

```sh
git show 17334f8:packages/core-ts/src/adapters/sqlite.test.ts
git show 17334f8:packages/core-ts/src/adapters/sqlite.ts
```

The said files together demonstrate the end-to-end round-trip across the persistence-adapter interface (500) and the KMF serialiser (400).

---

## Summary of the Examples

The five worked examples taken together demonstrate (i) the deterministic and cross-implementation-reproducible character of the random-hypervector primitive (Example 1), (ii) the operation of the tiebreaker (260) in the bundle primitive (Example 2), (iii) the encoding of bounded floating-point inputs through the thermometer-quantised random-projection encoder (Example 3), (iv) the order-sensitive character of the permutation-positional text encoder (Example 4), and (v) the end-to-end persistence, verification, and round-trip of a substrate through the KMF wire format and a persistence adapter (Example 5). The said examples are illustrative and non-limiting; the scope of the claims is as set out in §08 of the present specification.
