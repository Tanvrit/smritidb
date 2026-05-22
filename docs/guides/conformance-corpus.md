# Conformance Corpus

The conformance corpus is the bit-exact contract every Smritidb
implementation must satisfy. It lives at:

- `tests/conformance/golden.json` — a small JSON document with named
  inputs and the expected SHA-256 of each output.
- `tests/conformance/kmf_fixture.bin` — a binary KMF snapshot that
  every binding's KMF reader must parse and re-emit byte-for-byte.

The corpus is the source of truth for "two implementations agree". The
TypeScript reference (`packages/core-ts`) emits the corpus; every other
binding consumes it as part of its test suite. If the corpus passes,
the binding is wire-compatible with every other binding that also
passes.

## What the corpus pins

| Entry kind            | Invariant                                                                                                       |
|-----------------------|-----------------------------------------------------------------------------------------------------------------|
| `random_hv`           | `sha256(randomHv(utf8(seed), dim))` matches the listed `sha256`.                                                |
| `encode_string`       | `sha256(encodeString(input, dim))` matches the listed `sha256`.                                                 |
| `similarity_pairs`    | `|similarity(randomHv(a_seed), randomHv(b_seed)) - expected| < 1e-9`.                                            |
| `bind_round_trip`     | `bind(bind(a, b), b) == a` exactly; similarity = 1.0.                                                            |
| `bundle`              | `sha256(bundle([randomHv(s) for s in seeds]))` matches the listed `sha256`.                                     |
| `text_bag_of_words`   | `sha256(encodeBagOfWords(input, opts, dim))` matches the listed `sha256`.                                       |
| `text_char_ngrams`    | `sha256(encodeCharNgrams(input, opts, dim))` matches the listed `sha256`.                                       |
| KMF fixture           | `readKmf(kmf_fixture.bin)` parses, and `writeKmf(snapshot)` re-emits the same bytes.                            |

Hashes are SHA-256 over the **raw byte representation** of the
hypervector — one byte per bit, value 0 or 1. The choice of one byte
per bit (rather than packed bits) is intentional: it makes the corpus
invariant under the language's representation choice.

## Running the corpus per binding

| Binding   | Command                                                                       |
|-----------|-------------------------------------------------------------------------------|
| TypeScript| `cd packages/core-ts && pnpm test`                                            |
| Rust core | `cd packages/core-rs && cargo test --test conformance`                        |
| Python    | `cd packages/smritidb-py && pytest tests/test_conformance.py`                 |
| Kotlin/JVM| `cd packages/smritidb-kmp && gradle :jvmTest`                                 |

Every binding's CI invocation includes the conformance test by default;
a regression there is a hard fail.

## The KMF fixture

The KMF binary fixture (`kmf_fixture.bin`) is a snapshot of a small
hand-crafted store with two items, both 1024-dim, with deterministic
hypervectors. It exercises:

- The KMF header.
- The block index.
- One item with a tagged value.
- One item with a UTF-8 value containing multi-byte characters.

Round-trip: every binding's `readKmf` must produce a snapshot that, when
fed back through `writeKmf`, reproduces the original bytes exactly. This
catches both deserializer bugs and re-serializer drift (for example, a
binding that orders JSON keys differently from the TypeScript
reference's insertion-order semantics).

The companion file `kmf_fixture.json` (when present) holds a
human-readable view of the same snapshot for debugging.

## Regenerating the corpus

The TypeScript reference is the producer:

```sh
cd packages/core-ts && pnpm build
node tests/conformance/emit_kmf_fixture.mjs > ../../tests/conformance/kmf_fixture.bin
# golden.json is emitted from the same script via:
node -e "/* see emit script */" > ../../tests/conformance/golden.json
```

After regenerating, **every other binding's conformance test will fail**
until it re-runs against the new corpus. That is the intended workflow:
the corpus is updated only when a deliberate change to the wire format
or to a hypervector primitive is being made.

## Adding a new binding

The minimum bar for a new binding to be considered "smritidb-conformant":

1. Implement `randomHv`, `encodeString`, `bind`, `unbind`, `bundle`,
   `similarity` against the SPEC §1 primitives.
2. Write a conformance test in the binding's native test framework that:
   - Loads `golden.json`.
   - For each entry kind, runs the corresponding primitive and
     SHA-256-hashes the output (or compares the float for similarity).
   - Asserts the hash matches the listed `sha256`.
3. Wire the test into CI.

Optional but strongly recommended: also implement `readKmf` /
`writeKmf` and round-trip `kmf_fixture.bin`. A binding that can't read
KMF can't participate in the cross-binding interop story.

For the design rationale behind the bit-exact-across-languages
invariant, see [`MANIFESTO.md`](../MANIFESTO.md) and
[`docs/guides/cross-language-interop.md`](cross-language-interop.md).
