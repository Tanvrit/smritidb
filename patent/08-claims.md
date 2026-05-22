<!-- Round 4 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Fixed: Unity-of-invention paragraph claim references corrected from stale Round-1 numbering (1, 5, 9, 13, 19, 23, 26, 31, 33) to the current Round-2 independent-claim numbering (1, 7, 9, 18, 23, 27, 31, 35); description of each independent aspect re-matched to the surviving claim's actual scope (method+system pair for tiebreaker and KMF wire format; method for Hebbian consolidation; method for thermometer encoder; system for conformance-gating; method for permutation-positional text encoder).
- Fixed: Claim 9(c) antecedent basis — "the binary key hypervector of item `a` ... the binary key hypervector of item `b`" replaced with first-mention "a binary key hypervector of item `a` ... a binary key hypervector of item `b`"; subsequent references in (e), (f) updated to "the said" form.
- Strengthened: Claim 18 — added an explicit specification-version/conformance-corpus-version anchor to the header object (item 3 of the four-element combination identified in `10-prior-art.md` §6 differentiation matrix), distinguishing KMF from Parquet magic footer, HDF5 Fletcher32, Merkle/Git/IPFS content-addressing, and CBOR/FlatBuffers/Cap'n Proto/Bencode/Protobuf serialisation.
- Strengthened: Claim 18 dependent 19 — removed redundant "JSON document" recital that duplicated claim 18(e); replaced with the substantive single-pass-streaming differentiator over Parquet's footer-first read.
- Strengthened: Claim 27 — added an express disclaimer over Charikar 2002 sign-LSH and Rahimi 2016/2017 PRNG-seeded level encoders by anchoring the per-coordinate seed string in BLAKE3-XOF rather than in a sliding correlation or implementation-defined PRNG state.
- Strengthened: Claim 35 — added an express disclaimer over Charikar 2002 SimHash (bag-of-features, order-insensitive) by tying the permutation-positional limitation to the technical effect of word-order discrimination.
- Verified: every multi-dependent range still anchors at a non-multi-dependent claim (Round-2 audit unchanged); claim numbering remains contiguous 1..36 with 8 independents at 1, 7, 9, 18, 23, 27, 31, 35; Indian-English orthography unchanged.
-->

<!-- Round 2 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Adjusted: Claim 1 (TE moved into body, Ferid Allani anchoring; wire-format mandatory-field limitation added; bind terminology unified in dep. 6).
- Adjusted: Claim 7 (NVRAM data-path tie; FFI byte-transparent boundary limitation added).
- Adjusted: Claim 13 (OpenTV-style "configure the said apparatus" preamble; WebAssembly bytecode limitation promoted to independent body).
- Adjusted: Claim 18 (antecedent basis for "access log" and "per-pass salt" introduced in step (a); maxSimDelta upper bound and replayability from dep. 19/23 promoted into body).
- Adjusted: Claim 25 (mirror antecedent fix; non-volatile storage promoted from dep. 27 into independent body).
- Adjusted: Claim 30 (per-vector layout, D-multiple-of-8 constraint, and trailer-reversal directional integrity marker added to body; n antecedent fix in (c)).
- Adjusted: Claim 37 (output-sink enumeration promoted from dep. 39 into independent body).
- Adjusted: Claim 42 (TE added to preamble; n antecedent fix in (b)).
- Adjusted: Claim 46 (canonical decimal ASCII seed form + SHA-256-against-corpus byte-identity limitation added to body; hardware-tie clause appended; bind terminology unified in (c)(v)).
- Restructured: Claim 52 — converted from method-form to system-form per Perspective 2 reframing (OpenTV gating action); dual-kind corpus structure (bit-exact + approximate-with-tolerance) added to body.
- Adjusted: Claim 53 (text-encoding groups added).
- Adjusted: Claim 58 (cyclic bit rotation -> cyclic permutation per code's permute(); per-word hypervector antecedent fix in (b)(i)/(b)(ii); bind-via-XOR terminology unified in (b)(iii); tiebreaker-from-claim-1 cross-reference added; hardware-tie clause appended).
- Dropped: Claims 21 and 29 (salt-monotonicity / consolidation-epoch counter not implemented at SHA 17334f8).
- Dropped: Claims 25-29 entire triplet (Hebbian system claim 25 and its dependents 26-29) — folded most expressive features into dependents of claim 18; reduces §3(k) surface and IPO excess-claim fees.
- Dropped: Claims 13-17 entire triplet (Tiebreaker CRM) — most §3(k)-vulnerable; method (1) and system (7) cover the substantive matter.
- Dropped: Claims 42-45 entire triplet (KMF CRM) — same rationale.
- Renumbered: Remaining claims renumbered contiguously 1..N after the above deletions.
- Antecedent: Claim 5 narrowed from "any of claims 1 to 4" to "any of claims 1 to 3" to avoid chained multi-dependence on claim 4 (which is itself multi-dependent).
- Unity-of-invention: New explanatory paragraph added immediately after the title, identifying "BLAKE3-anchored deterministic byte-identity of a binary hyperdimensional associative memory substrate" as the single inventive concept uniting the eight independent claims (per Genentech [1989] RPC 147).
- Note: SPEC errata items (37-39 of worklist) are repo changes deferred to a pre-filing commit; tracked in REFINEMENT-LOG.md Round 2.
-->

# Claims

*Round 4 — pinned to HEAD 17334f8 — 8 independent claims, 36 total claims.*

---

## Explanatory Note on Unity of Invention and Described Embodiments

The claims that follow are unified by a single inventive concept, namely the **BLAKE3-anchored deterministic byte-identity of a binary hyperdimensional associative memory substrate**, by reason of which every primitive operation upon the said substrate — random hypervector generation, bundling, binding, permutation, thermometer encoding, text encoding, Hebbian consolidation, and wire-format serialisation — is reduced to a pure function of its inputs whose output is byte-for-byte identical across every conformant implementation, regardless of the processor architecture, programming language runtime, or memory layout in which the said implementation is executed. The independent claims are directed to six embodiments of this single concept, namely: (i) the deterministic majority-tiebreaker that anchors bundling, in method form (claim 1) and in system form (claim 7); (ii) the replayable Hebbian consolidation, bounded by a fixed similarity-drift parameter, that anchors associative reshape (claim 9); (iii) the open wire format with per-block BLAKE3 integrity that anchors persistence, in method form (claim 18) and in system form (claim 23); (iv) the thermometer-quantised cryptographically anchored projection encoder for bounded-range floating-point input (claim 27); (v) the load-time conformance-gating apparatus for cross-implementation byte-identity (claim 31); and (vi) the permutation-positional text encoder built upon and cross-referencing the deterministic tiebreaker of claim 1 and the conformance corpus of claim 31 (claim 35). This concept is novel over the prior art at the priority date (BLAKE3 specification, O'Connor 2020; Kanerva 1988 sparse distributed memory; Rahimi, Kanerva and Rabaey 2016 ISLPED level encoder; Plate 1995 holographic reduced representations; Charikar 2002 sign-LSH/SimHash; Apache Parquet format; HDF5 Fletcher32; Merkle 1979 / Git / IPFS content-addressed storage) and is patentable as a unified invention under Section 16 of the Patents Act, 1970, the test for which is taken from *Genentech Inc.'s Patent* [1989] RPC 147 (UK, persuasive in India).

---

## Independent Claim 1 — Deterministic Tiebreaker Method for Cross-Implementation Bit-Exact Bundling

1. A computer-implemented method for producing a bit-identical binary hyperdimensional bundle hypervector across a plurality of heterogeneous language implementations executing on different processor architectures, the method comprising:

   (a) receiving, by a processor, a multiset of `n` binary hypervectors each of fixed dimension `D`, wherein each said hypervector consists of bits in `{0, 1}` and is represented as a packed bit-array of `ceil(D/8)` bytes with most-significant-bit-first ordering within each byte;

   (b) computing, for each bit position `i` in the range `[0, D)`, an integer sum `s_i` equal to the count of input hypervectors having bit value 1 at position `i`;

   (c) setting an output bit at position `i` to 1 if `s_i > n/2` and to 0 if `s_i < n/2`; and

   (d) for every bit position `i` at which `s_i` is exactly equal to `n/2`, deterministically resolving the tie by computing a BLAKE3 cryptographic hash digest over a byte string formed by concatenating a fixed domain-separation tag, the dimension `D` encoded as a 32-bit little-endian unsigned integer, the bit-index `i` encoded as a 32-bit little-endian unsigned integer, and the multiset multiplicity `n` encoded as a 32-bit little-endian unsigned integer, and setting the said output bit to the least-significant bit of the first byte of the said BLAKE3 digest;

   characterised in that the said BLAKE3-derived tiebreaker bit, by operation upon the packed bit-array stored in the said memory, causes the said packed output hypervector to be byte-identical across heterogeneous processor architectures, thereby reducing storage input/output of the said associative memory substrate by permitting a single persisted snapshot to be loaded and verified bit-exactly by every conformant implementation without re-execution of the bundle operation; and wherein the said domain-separation tag, the said byte order, and the said field widths are recorded as a mandatory field of the wire-format header of any persisted snapshot of the said associative memory substrate, such that a foreign implementation reading the said snapshot is required to compute the said BLAKE3 digest by the identical byte layout in order to be admitted as conformant.

## Dependent Claims of Claim 1

2. The method as claimed in claim 1, wherein the said fixed domain-separation tag is the ASCII byte string `"smritidb/tiebreak"`, prepended to the said tuple before BLAKE3 hashing, so as to prevent collisions with BLAKE3 invocations made for other purposes by the same implementation.

3. The method as claimed in claim 1 or claim 2, wherein the said dimension `D` is a stored-substrate-wide constant selected from the group consisting of 1024, 8192, 10000, and 16384, and is fixed for the lifetime of the said associative memory substrate.

4. The method as claimed in any of claims 1 to 3, wherein each input hypervector of the said multiset is itself derived by expanding a 32-byte seed through a BLAKE3 extendable-output function to produce `ceil(D/8)` bytes which are then unpacked most-significant-bit-first into a binary hypervector of dimension `D`.

5. The method as claimed in any of claims 1 to 3, further comprising emitting, alongside the said output hypervector, a BLAKE3 integrity hash computed over the packed bytes of the said output hypervector, the said integrity hash being persisted in a header index of a wire-format file to permit downstream verification of bundle output without re-execution.

6. The method as claimed in any of claims 1 to 3, wherein the said multiset of binary hypervectors represents a superposition of a plurality of role-filler bindings generated by a bind operation comprising element-wise exclusive-or of pairs of hypervectors, and the output hypervector produced by the said method is stored as an item key in a content-addressable cleanup memory.

---

## Independent Claim 7 — System for Cross-Implementation Bit-Exact Bundling

7. A computer system for maintaining a binary hyperdimensional associative memory substrate that is reproducible bit-exactly across heterogeneous processor architectures, thereby reducing storage input/output by permitting a single canonical snapshot to be shared between implementations without re-encoding, the system comprising at least one processor and a memory storing instructions which, when executed by the said processor, cause the system to:

   (a) maintain in the said memory a packed bit-array representation of a multiset of `n` binary hypervectors of fixed dimension `D`;

   (b) compute a vertical sum across the said multiset for each of `D` bit positions;

   (c) write to an output packed bit-array a majority bit at each position at which the said sum is strictly greater than or strictly less than `n/2`; and

   (d) for each bit position at which the said sum equals `n/2`, invoke a deterministic tiebreaker module that returns the least-significant bit of the first byte of a BLAKE3 hash digest computed over the concatenation of a fixed ASCII domain-separation tag, the dimension `D` as a 32-bit little-endian unsigned integer, the said bit position as a 32-bit little-endian unsigned integer, and `n` as a 32-bit little-endian unsigned integer;

   characterised in that the said memory comprises a non-volatile storage region holding the said packed bit-array between executions, and the said processor is operable to write to and read from the said non-volatile storage region, the said deterministic tiebreaker module being executed on the data path between the said memory and the said non-volatile storage region, such that the said output packed bit-array is byte-identical across every implementation of the said system regardless of processor word size, endianness in higher-level operations, or programming language runtime; and further characterised in that the said tiebreaker module is exposed via a foreign-function-interface boundary that is byte-transparent, such that an output produced by a native-code invocation thereof is byte-identical to an output produced by a WebAssembly-sandboxed invocation thereof on the same input multiset.

## Dependent Claims of Claim 7

8. The system as claimed in claim 7, wherein the said at least one processor comprises a single-instruction-multiple-data vector unit, and wherein the said vertical sum is computed by a population-count operation over packed-bit lanes followed by horizontal accumulation, while the said deterministic tiebreaker module operates over a scalar control path so that the said output remains identical to a scalar reference implementation.

---

## Independent Claim 9 — Replayable Hebbian Consolidation Method with Bounded Similarity Drift

9. A computer-implemented method for incrementally reshaping a binary hyperdimensional associative memory substrate in response to access activity in a manner that is fully replayable from a wire-format snapshot together with an access log, thereby providing a holographic-fault-tolerant memory whose state can be reconstructed without retaining intermediate substrate copies, the method comprising:

   (a) maintaining, in a memory coupled to a processor, the said binary hyperdimensional associative memory substrate, a bounded sliding window of identifier batches recorded for successive recall operations against the said associative memory substrate, an access log recording, for each said recall operation, the identifier batch presented and a per-pass salt value associated with the said operation, and a co-activation tracker;

   (b) for each pair of identifiers `(a, b)` that co-occurs within any batch of the said sliding window, incrementing a pairwise co-activation counter `c(a, b)` of the said co-activation tracker, and decrementing the said counter when the corresponding batch is evicted from the said sliding window;

   (c) when the said co-activation counter `c(a, b)` reaches or exceeds a pull threshold, identifying a disagreement set comprising every bit position at which a binary key hypervector of item `a` stored in the said substrate differs from a binary key hypervector of item `b` stored in the said substrate;

   (d) bounding the number of bits to flip in the said disagreement set to a value computed as `max(1, floor(maxSimDelta * D))`, where `maxSimDelta` is a bounded-similarity-drift parameter and `D` is the said dimension, and wherein `floor(maxSimDelta * D)` is at most one percent of `D`;

   (e) deriving a deterministic ordering over the said disagreement set by computing a BLAKE3 hash digest over the said per-pass salt encoded as an 8-byte little-endian unsigned integer, and ranking each said disagreeing bit position by a byte of the said digest indexed by the said bit position modulo the digest length, with ties broken by ascending bit position; and

   (f) flipping the said bounded number of bits in alternating direction, such that even-indexed flips set bits of the said binary key hypervector of item `a` to match the corresponding bit of the said binary key hypervector of item `b`, and odd-indexed flips set bits of the said binary key hypervector of item `b` to match the corresponding bit of the said binary key hypervector of item `a`, thereby moving the two said hypervectors equally and by no more than the said `maxSimDelta` in pairwise similarity per consolidation pass;

   characterised in that each step of the said method is a pure function of the prior substrate state, the said access log, and the said per-pass salt, and the said access log is sufficient, together with a prior wire-format snapshot of the said substrate, to reconstruct the post-consolidation substrate state byte-for-byte by re-execution of the said method against the said snapshot and the said access log, without retention of any intermediate substrate copy; the said byte-identical post-consolidation substrate being persistable to a non-volatile storage device and re-instantiable on a different computing apparatus by re-execution of the said method against the said snapshot and access log, thereby dispensing with persistent storage of intermediate substrate states and reducing the storage footprint of the said associative memory.

## Dependent Claims of Claim 9

10. The method as claimed in claim 9, wherein the said bounded-similarity-drift parameter `maxSimDelta` is a value not greater than 0.02, the said pull threshold is fixed at 32, and the said sliding window has a default size of 1000 recall batches.

11. The method as claimed in claim 9 or claim 10, further comprising flagging as cold any item whose last-accessed timestamp is older than a configured cold-age threshold and whose access count is below a configured minimum-access threshold, and persisting the said flag as part of the said substrate to permit subsequent bundled summarisation of cold items.

12. The method as claimed in any of claims 9 to 11, wherein the said deterministic ordering is computed by sorting the said disagreement set by ascending value of `digest[bit_index mod digest_length]`, with ties broken by ascending bit position, and wherein the digest is produced by a BLAKE3 extendable-output function whose output length is at least `max(64, 4 * toFlip)` bytes.

13. The method as claimed in any of claims 9 to 11, further comprising recording, for each consolidation pass, the said per-pass salt, the identifier pair acted upon, and the count of bits flipped, into the said access log, so that a verifier can replay the said method against an earlier said snapshot and produce a byte-identical substrate.

14. The method as claimed in any of claims 9 to 11, wherein the said sliding window is maintained as an append-and-trim queue of identifier batches, and pairwise co-activation counters are decremented when their last contributing batch is evicted, so that the memory footprint of the said co-activation tracker is bounded by the said window size.

15. The method as claimed in any of claims 9 to 11, further comprising a non-volatile storage device storing the said snapshot together with the said access log, and a restore module configured to replay the said access log against the said snapshot to reconstruct an equivalent substrate state on a second computing apparatus.

16. The method as claimed in any of claims 9 to 11, wherein the said memory further stores a typed configuration record exposing the said `maxSimDelta`, the said pull threshold, the said sliding-window size, a cold-age threshold, and a cold-minimum-access threshold as separately tunable parameters.

17. The method as claimed in any of claims 9 to 11, wherein the said pairwise co-activation counters are evicted from the said memory when their value reaches zero, so that the working-set memory footprint of the said co-activation tracker scales with the active co-activation graph rather than the cardinality of the said substrate.

---

## Independent Claim 18 — KMF Open Wire Format Method for Hyperdimensional Substrates

18. A computer-implemented method for serialising and deserialising a binary hyperdimensional associative memory substrate to and from a single byte-addressed wire-format container that is streaming-friendly and verifiable, thereby providing BLAKE3-backed data integrity and reduced storage input/output for an implementation-independent associative memory substrate, the method comprising:

   (a) writing, at the start of an output byte sequence, a fixed magic byte string `"KMF\0"` and a six-byte ASCII specification-version field;

   (b) reserving an eight-byte little-endian header-offset field immediately following the said specification-version field;

   (c) writing one or more data blocks, the said data blocks comprising `n` entries where `n` is an item count of the said substrate, each said data block being of a kind selected from the group consisting of (i) a hypervector block comprising `n` packed binary hypervectors each of `ceil(D/8)` bytes laid out in column-major order with most-significant-bit-first packing within each byte, (ii) a metadata block comprising `n` rows of item metadata, and (iii) a length-prefixed value block comprising `n` opaque payloads each preceded by a 32-bit little-endian unsigned length;

   (d) computing a BLAKE3 hash digest over the said bytes of each said data block, encoding the said digest as a hexadecimal string, and recording the kind, the byte offset, the byte length, and the said hexadecimal hash for each said data block in an index;

   (e) writing, after all the said data blocks, a header object containing the said specification-version, a conformance-corpus version identifier referencing the canonical conformance corpus against which the said substrate has been verified, the said dimension `D`, the said item count `n`, a created-at timestamp, and the said index;

   (f) writing, after the said header object, a fixed trailer magic byte string `"FMK\0"`; and

   (g) populating the previously reserved said header-offset field with the byte offset of the said header object as a 64-bit little-endian unsigned integer;

   characterised in that a reader of the said byte sequence verifies the said trailer magic before trusting the said header-offset, verifies each said data block against its recorded BLAKE3 hash before deserialising the said data block, and rejects any said byte sequence whose said specification-version field is unsupported, so that corruption, truncation, or version drift is detected before substrate state is reconstructed; and further characterised in that (i) the said hypervector block is laid out with each said packed binary hypervector occupying exactly `ceil(D/8)` bytes with no per-vector padding, the said dimension `D` being constrained to be a multiple of 8, such that a partial reader is operable to compute the byte offset of the `k`-th hypervector within the said block as `k * ceil(D/8)` bytes without consulting any per-vector index; and (ii) the said trailer magic byte string `"FMK\0"` differs from the said header magic byte string `"KMF\0"` by reversal of its first three bytes, providing a directional integrity marker distinguishable from accidental file concatenation; and (iii) the said reader is operable to read the said byte sequence from a non-volatile storage device into a memory of a computing apparatus, and to refuse to write any substrate state into a working memory of the said apparatus until the said per-block BLAKE3 hash digests have been verified.

## Dependent Claims of Claim 18

19. The method as claimed in claim 18, wherein the said header object is serialised as a JSON document and the said metadata block is encoded as a JSON document, the said header object being positioned after the said data blocks rather than before them, so as to permit the said data blocks to be streamed and per-block BLAKE3-hashed in a single forward pass without prior knowledge of the said data-block byte lengths and without requiring a seek-to-front rewrite of the said header.

20. The method as claimed in claim 18 or claim 19, wherein the said wire-format container is configurable to compress each said data block with the Zstandard algorithm at a selectable compression level prior to the said BLAKE3 hash being computed, and wherein the said header object records, per block, the compression algorithm and parameters used.

21. The method as claimed in any of claims 18 to 20, wherein the said reader, upon encountering a specification-version field of a different major version than that supported by the said reader, refuses to deserialise the said byte sequence and surfaces a typed `UnsupportedSpecVersion` error condition.

22. The method as claimed in any of claims 18 to 20, further comprising emitting an optional attic block recording one or more bundled summaries of cold items together with their source-item identifiers, the said attic block being independently hashed and indexed in the said index.

---

## Independent Claim 23 — System for KMF Open Wire Format

23. A computer system for persisting a binary hyperdimensional associative memory substrate to a verifiable open wire format, thereby providing reduced storage input/output through column-major partial loading and BLAKE3-backed integrity verification, the system comprising at least one processor, a memory storing instructions, and a non-volatile storage device coupled to the said processor and selected from the group consisting of a local file system, an IndexedDB object store within a web-browser, a SQLite database row, and an object-storage bucket; the said instructions, when executed by the said processor, causing the system to:

   (a) emit a fixed magic byte string and a specification-version field to an output sink hosted by the said non-volatile storage device, followed by a reserved eight-byte header-offset field;

   (b) emit one or more data blocks each comprising a contiguous sequence of bytes of a kind selected from a hypervector block, a metadata block, a length-prefixed value block, and an attic block;

   (c) compute a BLAKE3 hash digest over the bytes of each said data block as the said bytes are emitted;

   (d) emit, after the said data blocks, a header object recording the said specification-version, a dimension `D`, an item count, a created-at timestamp, and an index of `(kind, offset, length, BLAKE3 hash)` tuples;

   (e) emit a fixed trailer magic byte string after the said header object; and

   (f) backfill the said reserved header-offset field with the byte offset of the said header object;

   characterised in that the said header object is written after the said data blocks so as to permit single-pass streaming, the said system is operable to produce a byte-identical wire-format byte sequence regardless of which of the said non-volatile storage devices is selected as the output sink, and a reader module of the said system verifies the said trailer magic, the said specification-version, and the said per-block BLAKE3 hash digests before reconstructing any substrate state into a working memory of the said apparatus.

## Dependent Claims of Claim 23

24. The system as claimed in claim 23, further comprising a Zstandard compressor coupled between a block-emit pipeline and the said BLAKE3 hasher, the said compressor being configurable per block kind, and the said header object recording per-block compression parameters.

25. The system as claimed in claim 23 or claim 24, further comprising a partial-load module configured to seek directly to a hypervector block of the said wire format and demand-load a contiguous slice thereof without requiring deserialisation of the said metadata block or the said value block.

26. The system as claimed in any of claims 23 to 25, wherein the said reader module, upon detecting a mismatch between a computed BLAKE3 hash digest and a digest recorded in the said header object, surfaces a typed `CorruptSnapshot` error condition and aborts reconstruction prior to returning any substrate item to a caller.

---

## Independent Claim 27 — Thermometer-Quantised Random-Projection Encoder Method for Bounded-Range Floating-Point Vectors

27. A computer-implemented method for mapping an input bounded-range floating-point vector of length `N` to a binary hypervector of fixed dimension `D` such that pairwise cosine similarity in the said input space is approximately preserved as Hamming similarity in the said output space, thereby providing a deterministic, training-free, lower-compute-load embedding pipeline that requires no neural-network inference, the method comprising:

   (a) receiving an input vector `v` of `N` floating-point values, each clamped to the range `[-1, 1]`;

   (b) initialising an accumulator hypervector of dimension `D` to all zeros;

   (c) for each index `i` in `[0, N)`:

       (i) computing a thermometer-quantised level `lvl_i` as a function of the said clamped value `v[i]` and a fixed levels constant `L`, specifically `lvl_i = clamp(round((v[i] + 1) * (L - 1) / 2), 0, L - 1)`;

       (ii) constructing a per-coordinate seed byte string by concatenating a fixed domain-separation tag `"lvl:"`, the decimal ASCII representation of `i` in canonical decimal ASCII form without leading zeros or sign characters, the ASCII byte `':'`, and the decimal ASCII representation of the said level `lvl_i` in the same canonical decimal ASCII form;

       (iii) computing a BLAKE3 hash digest of the said seed byte string;

       (iv) expanding the said digest by a BLAKE3 extendable-output function to `ceil(D/8)` bytes and unpacking the said bytes most-significant-bit-first into a per-coordinate binary hypervector of dimension `D`; and

       (v) updating the said accumulator hypervector by a bind operation comprising element-wise exclusive-or with the said per-coordinate hypervector; and

   (d) returning the said accumulator hypervector as the said binary hypervector;

   characterised in that the said method binds a per-coordinate random projection that depends jointly on the coordinate index and the thermometer-quantised level, the said projection being derived from a BLAKE3 extendable-output function applied to the said canonical decimal ASCII seed byte string rather than from a sliding correlation between two random seed hypervectors or from an implementation-defined pseudo-random number generator, so that two said input vectors which are close in cosine similarity produce per-coordinate hypervectors that agree on the same set of bits, and consequently the said exclusive-or accumulators agree on a number of bits proportional to the said cosine similarity, without requiring any trained model, gradient descent, or floating-point matrix multiplication in the said output path; and further characterised in that an output produced by a first implementation of the said method on a first processor architecture is byte-identical to an output produced by a second implementation of the said method on a second, different processor architecture, the said byte-identity being verifiable by computing the SHA-256 digest of the packed bytes of the said accumulator hypervector and comparing the said digest, byte-for-byte, against a value recorded in a canonical conformance corpus shipped with each said implementation; and yet further characterised in that the said method is executed entirely in integer arithmetic and bitwise operations on the said processor, without invocation of any floating-point matrix-multiplication unit or neural-network inference accelerator, such that the said binary hypervector is producible on a microcontroller-class processor lacking floating-point hardware.

## Dependent Claims of Claim 27

28. The method as claimed in claim 27, wherein the said levels constant `L` is a value of at least 64 and not greater than 256, and the said fixed domain-separation tag is the ASCII byte string `"lvl:"`.

29. The method as claimed in claim 27 or claim 28, wherein the said dimension `D` is selected from the group consisting of 1024, 4096, 8192, 10000, and 16384.

30. The method as claimed in any of claims 27 to 29, further comprising storing the said binary hypervector as a key in a content-addressable cleanup memory and retrieving, in response to a noisy cue hypervector, the top-`k` nearest stored hypervectors by Hamming similarity.

---

## Independent Claim 31 — Conformance-Gating System for Cross-Implementation Byte-Identity of a Vector Substrate

31. A computer system for verifying cross-implementation byte-identity of a binary hyperdimensional associative memory substrate, thereby providing a load-time technical guard against the admission of a non-conformant substrate to the said memory, the system comprising at least one processor, a non-volatile storage device storing a canonical conformance corpus file encoded as a JSON document, and a memory storing instructions which, when executed by the said processor, cause the system to:

   (a) maintain, in the said non-volatile storage device, the said canonical conformance corpus comprising a plurality of test groups, each said test group specifying (i) a canonical input including a textual seed, a numeric dimension, and where applicable an operation type, and (ii) an expected output expressed as a SHA-256 hexadecimal digest computed over the packed bytes of the canonical output hypervector or a numeric expected scalar with a stated tolerance;

   (b) execute, against the said memory, each said test group by deriving an actual output hypervector or scalar from the said canonical input using substrate primitives of an implementation under test;

   (c) compute, in the said processor, the SHA-256 hexadecimal digest of the packed bytes of the said actual output hypervector and compare the said digest, byte-for-byte, against the said expected digest in the said corpus; and

   (d) report the said implementation as conformant only if the said comparison succeeds for every said test group of bit-exact kind and the said actual scalar lies within the said stated tolerance of the said expected scalar for every said test group of approximate kind;

   characterised in that the said system rejects, at load time of any substrate snapshot produced by a foreign implementation, any said snapshot whose computed output, when subjected to each said test group, fails to match the said expected digest or to lie within the said stated tolerance, the said system thereby refusing to write the rejected said snapshot into a working memory of the said apparatus and providing a load-time technical guard against admission of a non-conformant substrate to the said associative memory; and further characterised in that the said canonical conformance corpus comprises both (i) bit-exact test groups whose expected output is a SHA-256 digest of packed bytes, and (ii) approximate test groups whose expected output is a numeric scalar paired with a stated absolute tolerance, the combination of both kinds in a single corpus being adapted to verify an approximate-vector data structure for which only certain primitive operations are required to be bit-exact while other operations are required only to be within tolerance.

## Dependent Claims of Claim 31

32. The system as claimed in claim 31, wherein the said test groups comprise at minimum a random-hypervector group, a string-encoding group, a similarity-pairs group, a bind-round-trip group, and a bundle group, each said group being keyed by the same canonical inputs in every said implementation; and, where text encoding is supported by the implementation, a bag-of-words text-encoding group and a character-n-gram text-encoding group.

33. The system as claimed in claim 31 or claim 32, wherein the said canonical conformance corpus further specifies, in addition to the said SHA-256 digest, the said dimension and the said seed encoding as UTF-8, so that no implementation is required to make an undocumented choice that could perturb the said digest.

34. The system as claimed in any of claims 31 to 33, wherein the said canonical conformance corpus is generated by a designated reference implementation and is reviewed and frozen prior to release, and wherein any subsequent change to the said corpus requires a corresponding bump of a specification-version field.

---

## Independent Claim 35 — Permutation-Positional N-Gram Text Encoding Method Without Tokeniser or Neural Embedding

35. A computer-implemented method for mapping a textual input to a binary hypervector of fixed dimension `D` in a manner that preserves word order and is deterministic, training-free, and free of any neural-network inference, thereby providing a lower-compute-load and improved-memory-footprint alternative to learned text embeddings for use as an associative-memory key, the method comprising:

   (a) normalising the said textual input by, where configured, lowercasing all characters and replacing every non-alphanumeric character with a whitespace, then splitting the said normalised text on whitespace into a sequence of words, and discarding any word shorter than a configured minimum word length;

   (b) for each window of `n` consecutive words `(w_0, w_1, ..., w_{n-1})` over the said sequence of words, computing an n-gram hypervector by:

       (i) deterministically encoding each said word `w_j` to a per-word hypervector for `w_j` of dimension `D` by computing a BLAKE3 hash digest of the byte string formed by concatenating a fixed string-domain tag `"word:"` with the UTF-8 bytes of `w_j` and expanding the said digest by a BLAKE3 extendable-output function to `ceil(D/8)` bytes unpacked most-significant-bit-first;

       (ii) leaving the said per-word hypervector for the first word `w_0` of the window unrotated, and for each subsequent word `w_j` with `j` in `[1, n)`, applying a cyclic permutation of the said per-word hypervector for `w_j` by exactly `j` positions, the said cyclic permutation being defined as the element-wise mapping that places the element at position `p` of an input vector at position `(p + j) mod D` of the output vector; and

       (iii) combining the said unrotated first per-word hypervector with the `n - 1` permuted subsequent per-word hypervectors by a bind operation comprising element-wise exclusive-or to produce the said n-gram hypervector; and

   (c) bundling the said n-gram hypervectors across all the said windows by element-wise majority with deterministic tiebreaker resolution as claimed in claim 1, and returning the said bundle as the said binary hypervector encoding the said textual input;

   characterised in that the use of a position-dependent cyclic permutation, defined as the element-wise mapping that places the element at position `p` of an input vector at position `(p + j) mod D` of the output vector for a window-position offset `j`, ensures that two textual inputs differing only in word order produce different n-gram hypervectors and consequently different bundle outputs, thereby distinguishing the said method from a bag-of-features binary fingerprint construction in which window position is not encoded; and the said method does so without any tokeniser vocabulary, any trained word embedding, or any floating-point arithmetic on the hot path; and further characterised in that the output of the said method is byte-identical across heterogeneous implementations verified by the canonical conformance corpus of the system of claim 31; the said method being computable entirely by integer arithmetic and bitwise operations, such that the said binary hypervector is producible on a processor lacking any floating-point unit, and is storable in a non-volatile storage device as an associative-memory key without further encoding.

## Dependent Claims of Claim 35

36. The method as claimed in claim 35, wherein the said configured minimum word length is 3 and the said window size `n` is 2 or 3; and further wherein, when the said sequence of words has length less than `n`, the method falls back to encoding the said normalised text as a bag-of-words bundle of per-word hypervectors without permutation; and further comprising a character-n-gram variant in which the said sequence of words is replaced by a sequence of overlapping character `n`-grams drawn from the said normalised text, and the said per-shingle hypervector is derived from the BLAKE3 hash digest of a fixed character-domain tag `"char:"` concatenated with the said shingle, thereby providing tolerance to single-character typographical errors.

---
