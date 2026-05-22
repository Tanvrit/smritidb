# Round 2 Critique — 11-3k-defense.md

**Critic:** Round 2, five-perspective composite.
**Subject:** `/Users/viveksingh/Developer/smritidb/patent/11-3k-defense.md` (Round 1, 2,369 words).
**Code pin:** HEAD SHA `17334f8`.
**Posture:** Sharp, actionable. Each perspective writes against the memo's positions.

---

## Perspective 1 — IPO Examiner (drafting an FER attack on each claim)

The Controller's brief, on a strict reading of §3(k), is to argue that the disclosed "technical effect" is illusory or that the invention reduces, in substance, to a mathematical method, a computer programme per se, or an algorithm. The memo's defenses are tested below by writing the strongest §3(k) attack for each independent claim and asking whether the memo answers it.

### Claim 1 — Deterministic Tiebreaker (BLAKE3(D‖idx‖count))

**FER attack the Controller will draft.** "The claim recites (a) a counting operation (the majority sum), (b) a comparison (`>`, `<`, `=`), and (c) a hash function whose output is reduced to a single bit. Each of (a)-(c), in isolation and in combination, is a mathematical operation. The asserted 'technical effect' of cross-implementation byte-identity is not an effect *of the algorithm on the physical world*; it is merely a property *of the algorithm itself* — namely, that it is a function (every input deterministically maps to one output). All deterministic algorithms have this property. The claim is therefore a *mathematical method* and a *computer programme per se* within §3(k). *OpenTV* (2023) is squarely on point: running an otherwise abstract method on a generic computer is not enough."

**Does the memo answer?** Partially. The memo invokes the *snapshot serialisation pipeline* as the "process" on which the effect lands, but Claim 1 itself (as currently drafted in `08-claims.md` §1) does not recite the snapshot pipeline — it only recites the bundle operation. The memo's defense is *narrative* but not *load-bearing on the claim language*. Strongest counter the Controller will run: "The applicant's own dependent claim 5 (integrity-hash emission) is the only place the snapshot pipeline appears; the independent claim is purely mathematical." **Weak — needs the independent claim itself amended to recite the persistent substrate.**

### Claim 2 — Replayable Hebbian Consolidation

**FER attack.** "Hebbian learning is a mathematical update rule (Hebb 1949). 'Determinism' is a property of the rule, not a technical effect; every PRNG-seeded learning rule with a fixed seed is deterministic. 'Replayability' is also a property of any pure function. The disclosure adds no hardware nexus; the BLAKE3 seeding is itself a mathematical step. *Per* §3(k), this is a mathematical method dressed in cryptographic clothing."

**Does the memo answer?** The memo's response — "removes scheduler non-determinism, floating-point reduction order, and thread interleaving by construction" — names sources of non-determinism in a generic implementation, but the claim itself does not recite SIMD lanes, thread scheduling, or floating-point arithmetic, so the Controller will say these recitations are irrelevant to the claim language. **Weak. The defense is engineering-true but legally orthogonal to the claim text. The technical-effect framing must be re-anchored on "permits restoration of an associative-memory artefact from a snapshot file plus a log without retaining intermediate states" — which IS a system-level property of a stored artefact.**

### Claim 3 — KMF Wire Format

**FER attack.** "A wire format is a *presentation of information* (excluded under §3(n)) or, alternatively, a data structure (which is not patentable subject matter in the traditional view). The 'magic trailer' and 'per-block hash' are conventional file-format constructs found in ZIP (magic trailer), PNG (per-chunk CRC), and TLS records (per-record MAC). The combination is, at best, an aggregation, and at worst a *presentation of information*."

**Does the memo answer?** Strongest defense. The 32× compression, per-block integrity, and column-major streaming load are concrete, measurable engineering metrics traceable to *Microsoft v. Asst Controller* (2023). However, the §3(n) "presentation of information" risk is **not addressed** at all in the memo's Claim 3 section. The memo flags §3(n) in the opening sentence then drops it. **Medium. §3(n) angle must be written out.**

### Claim 4 — Thermometer-Quantised Random Projection Encoder

**FER attack.** "Random projection (Johnson-Lindenstrauss 1984, Achlioptas 2001), thermometer encoding (a quantisation scheme; pure mathematics), and BLAKE3 (a known function) are each individually mathematical methods. Their combination is a *mathematical method for mapping floats to bits* — a textbook §3(k) case. The asserted 32× footprint reduction is a *property of the chosen representation* (binary vs float), not a *technical effect of the algorithm*; the algorithm would produce identical bit-strings regardless of whether it ran on a CPU, on paper, or in a person's head."

**Does the memo answer?** The "paper-and-pencil" attack is the canonical §3(k) framing and the memo does not pre-empt it. The memo asserts the encoded output is "embedded in a tangible system" but the *claim itself* (Claim 46 in `08-claims.md`) does not recite the persistence substrate — it recites only the encoding procedure with a "returning the said accumulator hypervector" closing step. **Weak. Independent claim must add an explicit "and persisting the said accumulator hypervector to a KMF wire-format block" or "and storing the said hypervector in the said associative-memory substrate as an item key" step to anchor the claim in tangible storage.**

### Claim 5 — Conformance Corpus Methodology

**FER attack.** "This is a *method of testing* — at best a method of performing mental acts (§3(m)) or a business method (a quality-assurance procedure). The corpus is just a data file; reading it and comparing strings is the work of a generic computer running generic instructions. *OpenTV* (2023) bars precisely this class of subject matter."

**Does the memo answer?** The memo's response — "the corpus is itself a technical artefact and produces a measurable system-level property" — is exactly right in substance but the case law marshalled is thin: the memo cites only *Ferid Allani* generically. The strongest reply to *OpenTV* would be that the corpus is not *a method run on the corpus*; it IS the technical contract that admits a binary into a family of conformant runtimes — i.e., the artefact itself is the locus of the inventive step. **Medium. Defense needs sharper anchoring in *Microsoft v. Asst Controller* (2023, quantitative engineering improvement) and direct rebuttal of §3(m).**

### Claim 6 — Permutation-Positional N-gram Text Encoding

**FER attack.** "Text encoding by permutation binding is a mathematical operation on strings. The 'training-free' framing is irrelevant to §3(k); the absence of a neural model does not convert a mathematical encoder into a technical invention. The claimed effect — feasibility on resource-constrained hardware — is a *consequence of the substrate*, not of any inventive step in the encoder."

**Does the memo answer?** The memo's defense is the weakest of the six. "Enabling a capability on a constrained device" is exactly the kind of *abstract assertion* that *OpenTV* warns against; capability assertions require quantitative anchoring (e.g., "produces a 1024-bit embedding in <X> ms on an ARM Cortex-M4 without floating-point hardware vs. <Y> ms infeasible for sentence-transformers"). The memo provides no such number. **Weak. Needs measurable claims (footprint in bytes, latency on a named class of hardware, energy in mJ) to convert the assertion to a *Microsoft*-grade quantitative technical effect.**

**Examiner-perspective summary.** Of the six defenses, only Claim 3 is robust against the strongest FER attack as drafted. Claims 1, 2, 4, and 6 require the *independent claim text* (not just the memo's narrative) to recite a tangible substrate or a quantitative metric. Claim 5 requires a sharper case-law anchor.

---

## Perspective 2 — §3(k) Defender (peer review of the memo's legal logic)

### Case citations — are they accurate?

**Ferid Allani v. Union of India.** Memo cites "W.P.(C) 7/2014 and CM APPL. 40736/2019, judgment dated 12 December 2019, paragraph 22". The citation format is broadly correct (Delhi HC writ jurisdiction). However:
- The paragraph reference (¶22) for the narrow-reading-of-*per se* holding should be **verified against the certified copy of the judgment**. The principal operative paragraph is typically cited as ¶12 or ¶21-23 depending on edition. Memo's "¶22" is plausible but undocumented.
- The case is regularly cited together with Section 3(k) jurisprudence; the memo's framing is correct.
- **Action: pull the certified judgment and pin the paragraph numbers.**

**Microsoft Technology Licensing LLC v. Assistant Controller of Patents and Designs.** Memo cites "C.A.(COMM.IPD-PAT) 29/2022, Delhi HC, 15 May 2023". This citation format is correct for the IPD bench established in 2022. However:
- The substantive holding the memo attributes — "measurable, quantitative improvement is the gold standard" — is a *paraphrase*, not a direct holding. The actual holding is narrower: that an Examiner who refuses to engage with technical-effect evidence has acted contrary to *Ferid Allani*. The memo's language overstates the case.
- **Action: soften "gold standard" language to "the Court emphasised that *the Examiner is bound to consider* measurable quantitative improvements when assessing technical effect".**

**OpenTV Inc. v. Controller of Patents and Designs.** Memo cites "C.A.(COMM.IPD-PAT) 14/2022, Delhi HC, 31 May 2023". This citation format and date are correct. The substantive proposition — that running an abstract method on a generic computer is not enough — is accurately stated. **OK as-is.**

### Cases the memo OMITS that should be cited

1. **Yahoo! Inc. v. Asst Controller of Patents** (IPAB, OA/22/2010/PT/CH, December 2011). The foundational Indian decision establishing the *technical effect* test before the CRI Guidelines codified it. Any §3(k) defense in 2026 that omits Yahoo! is incomplete.
2. **Telefonaktiebolaget LM Ericsson v. Intex Technologies** (Delhi HC, 2015) — establishes that software with technical effect on telecommunications is patentable; useful for the "interoperability is a technical effect" argument.
3. **Accenture Global Service GmbH v. Asst Controller of Patents** (IPAB, OA/22/2009/PT/DEL) — useful precedent on software claims that recite both method and apparatus.
4. **Raytheon Co. v. Controller General of Patents** (IPAB, 2020) — useful where the invention is a data structure with verification properties (closest analogue to Claim 3).
5. **CRI Guidelines 2017, paragraph 4.5** — memo cites this correctly. However, the **CRI Guidelines draft revision (2024)** has been circulated and is more permissive on the technical-effect axis; the memo should at least flag the existence of the draft revision so the patent agent is aware of it. (The draft has not been finalised as of cut-off but is in public consultation.)
6. **Microsoft Technology Licensing LLC v. Asst Controller** has been the subject of **multiple subsequent IPD-PAT proceedings** in 2024-2025; the memo should note these as a "line of cases" rather than relying on a single 2023 judgment.

### Case-format issues

- C.A.(COMM.IPD-PAT) is correctly used. (Some practitioners write C.A.(COMM. IPD-PAT) with a space; both are accepted by the Delhi HC IPD Registry.)
- W.P.(C) is the correct writ format.
- Dates are formatted consistently.
- Memo does **not** include neutral citations (e.g., 2023 SCC OnLine Del 3047 for *Microsoft v. Asst Controller*); these should be added for portability.

### Internal logic — does the memo's "technical effect" framing match the Detailed Description?

Cross-checking against `08-claims.md`:
- Claim 1 (tiebreaker) — memo asserts "verifiable cross-hardware bit-exact reproducibility of a stored data structure". The claim itself does not recite "stored data structure" in the independent recitation; only dependent Claim 5 mentions persistence. **Misalignment.**
- Claim 2 (consolidation) — memo asserts "deterministic replay of a memory store's state from a snapshot together with an access log". The independent claim (Claim 18) **does** recite "fully replayable from a wire-format snapshot together with an access log" in its preamble. **Aligned.**
- Claim 3 (KMF) — memo asserts integrity + streaming + footprint. Claim 30 recites all three. **Aligned.**
- Claim 4 (encoder) — memo asserts "approximately thirty-two-fold reduction" and "cosine similarity preserved as Hamming similarity within bounded error envelope". Independent Claim 46 says "such that pairwise cosine similarity in the said input space is approximately preserved as Hamming similarity" but **does not state the 32× number in the claim**; it appears only in the memo. **Misalignment — quantitative claim should be added to a dependent claim, e.g., "wherein the said output hypervector is at least 16-fold smaller than a single-precision floating-point representation of the said input vector".**
- Claim 5 (conformance) — memo asserts cross-binding interoperability. Independent Claim 52 recites this directly. **Aligned.**
- Claim 6 (n-gram) — memo asserts on-device feasibility on resource-constrained hardware. Independent Claim 58 says "lower-compute-load and improved-memory-footprint" but does **not** define "lower" or "improved" quantitatively. **Misalignment — needs at least one dependent claim with a numeric bound (footprint in bytes, latency in milliseconds, or energy in mJ).**

### Verdict on legal logic

The memo's central thesis is sound but its execution is loose: case citations are paraphrased rather than quoted; paragraph references are unverified; key Indian precedents (*Yahoo!*, *Ericsson v. Intex*, *Accenture*) are missing; and three of the six claim-narratives don't track the independent-claim text.

---

## Perspective 3 — Prior-Art Hunter (cross-checking 11-3k-defense vs 10-prior-art)

### Claim 1 — Tiebreaker

Memo §11-3k cites: Kanerva 1988/2009; Imani et al. HPCA 2017.

- `10-prior-art.md` §2.4 lists Imani et al. publications as **ICRC 2017 (VoiceHD)**, **DAC 2018**, **DATE 2019**, **CLOUD 2019** — *but does not list any HPCA 2017 paper by Imani.*
- The memo's reference to "Imani et al., 'Exploring Hyperdimensional Associative Memory', HPCA 2017" is **not found in `10-prior-art.md`** and may be a misremembered citation. The probable correct citation is **Imani et al., VoiceHD, ICRC 2017** (per `10-prior-art.md` §2.4).
- **Contradiction.** The §3(k) memo introduces a citation ("HPCA 2017") not present in the prior-art doc. Either (a) HPCA 2017 is a real Imani paper that was missed in the prior-art search and must be added to `10-prior-art.md`, or (b) the memo's citation is wrong and must be corrected to "VoiceHD, ICRC 2017".
- **Action: verify HPCA 2017 paper exists; if not, correct to ICRC 2017.**

### Claim 2 — Consolidation

Memo cites: Hebb 1949; Kanerva 1988; Karunaratne et al. (Nature Electronics 2020).

- `10-prior-art.md` §2.1 and §2.6 and §4.4 cover Kanerva, Hebb, and Karunaratne respectively. **Consistent.**
- Memo's claim "Karunaratne et al. ... still stochastic across runs" is **not directly substantiated** in `10-prior-art.md`, which describes Karunaratne as "analog in-memory HDC using phase-change memory" without commenting on determinism. The memo's "stochastic across runs" assertion is plausible for analog phase-change memory but should be backed by a quotation from the paper or removed.
- **Action: either substantiate the "stochastic across runs" assertion or soften to "is not addressed in the published work".**

### Claim 3 — KMF Wire Format

Memo cites: Apache Arrow/Parquet, HDF5, Protocol Buffers, MessagePack.

- `10-prior-art.md` §6 Differentiation Matrix lists "Apache Arrow; Apache Parquet; HDF5; MessagePack; pickle". **Consistent.**
- The memo additionally invokes "the long line of MPEG-related patents granted by the IPO" as an analogical argument. **`10-prior-art.md` does not cite any MPEG patent.** This is a rhetorical flourish, not a prior-art citation; it can stand but is unsupported.
- **No contradiction.**

### Claim 4 — Encoder

Memo cites: Imani HPCA 2017; Jégou-Douze-Schmid TPAMI 2010; Achlioptas PODS 2001.

- `10-prior-art.md` §6 cites "Jégou, Douze and Schmid (2010) 'Product Quantization for Nearest Neighbor Search,' *IEEE TPAMI*". **Consistent on the Jégou paper.**
- However, `10-prior-art.md` §9 lists Jégou as "**2011**", IEEE TPAMI 33(1): 117-128 — and `10-prior-art.md` §6 says "**2010**". **Internal contradiction within `10-prior-art.md`.** The memo at §11-3k-defense.md picks one (2010) without flagging the discrepancy. *(Correct journal date: published online 2010, journal issue 33(1) January 2011 — both can be cited but a single convention should be chosen.)*
- Achlioptas PODS 2001 is **not in `10-prior-art.md`**. The §3(k) memo introduces a new citation. **Adding citations is fine but they should be cross-listed in `10-prior-art.md` to keep the documents in sync.**
- **Action: reconcile Jégou date convention; add Achlioptas to `10-prior-art.md`.**

### Claim 5 — Conformance Corpus

Memo cites: NIST CAVS / CAVP.

- `10-prior-art.md` §6 says "NIST CAVS (Cryptographic Algorithm Validation System)". **Consistent.**
- Memo's framing — "deterministic cryptography" vs "byte-identical approximate index" — is sharper than `10-prior-art.md` and constitutes a *new* differentiation argument that should be back-ported into `10-prior-art.md`.
- **Action: harmonise §10 prior-art doc with the §11 framing.**

### Claim 6 — N-gram Text Encoder

Memo cites: Word2Vec (Mikolov ICLR 2013); GloVe (Pennington EMNLP 2014); BERT (Devlin NAACL 2019); Plate IEEE TNN 1995; Kanerva 2009.

- `10-prior-art.md` §6 Differentiation Matrix names "Plate 1995 (HRR positional encoding, real-valued); Imani text-encoding work (n-gram HDC)". **Word2Vec, GloVe, BERT are NOT listed in `10-prior-art.md`.**
- Word2Vec, GloVe, and BERT are correctly cited as the dominant baselines for text embedding and the memo's differentiation argument (no training, no neural model, on-device) is sound. But the absence of these references from `10-prior-art.md` is a **gap in the prior-art doc**.
- **Action: add Mikolov 2013, Pennington 2014, Devlin 2019 to `10-prior-art.md`.**

### Cross-cutting prior-art discrepancies

- The two documents use different conventions for Imani 2017 (HPCA vs ICRC).
- The two documents use different conventions for Jégou (2010 vs 2011).
- The two documents differ on the scope of Claim 6 baselines (only Plate+Imani in prior-art doc; Word2Vec+GloVe+BERT+Plate+Imani+Kanerva in §3(k) memo).

---

## Perspective 4 — Indian Patent Attorney

### Per-claim mapping to a measurable engineering metric

For an FER under CRI Guidelines 2017 §4.5, the Controller expects to see a *measurable* technical effect — bytes saved, latency reduced, energy reduced, error rate reduced. Conclusory phrases like "concrete engineering capability" do not survive Controller scrutiny.

| Claim | Memo's effect | Measurable? | Pass/Fail |
|---|---|---|---|
| 1 | "byte-identical packed output hypervector" | Yes — verifiable by SHA-256 against `tests/conformance/golden.json` | **Pass** (effect is verifiable; map to corpus) |
| 2 | "deterministic replay ... with bounded similarity drift" | Yes — `maxSimDelta = 0.02` (DEFAULT_CONSOLIDATION in `consolidate.ts`) | **Pass** but needs the number in a dependent claim |
| 3 | "32× memory reduction, integrity detection, streaming load" | Yes — 32× is the ratio of 1 bit to 32 bits per element | **Pass** |
| 4 | "32× footprint reduction with bounded error envelope" | Yes — 32× per element vs f32; bounded-error must be quantified | **Pass** if envelope is given (e.g., "within ±0.1 Hamming/cosine deviation"); currently **unquantified** |
| 5 | "verifiable cross-binding interoperability" | Yes — SHA-256 comparison against golden.json; pass/fail is binary | **Pass** |
| 6 | "on-device feasibility on resource-constrained hardware" | **No** — no number stated; "feasibility" is qualitative | **Fail. Needs e.g., "<10 ms encode latency on a 100 MHz Cortex-M; <2 KB working memory; no floating-point unit required".** |

### CRI Guidelines 2017 §4.5 language compliance

The Guidelines list a non-exhaustive set of acceptable technical-effect indicia: "higher speed; reduced hard-disk access time; more economical use of memory; more efficient database search strategy; more effective data compression; improved user interface; better control of robotic arm; improved reception/transmission of radio signal."

- Claim 1 → maps to "more efficient cross-platform data interchange" — *novel; not explicitly listed but defensible by analogy to "improved reception/transmission" of bit-exact data across heterogeneous runtimes.*
- Claim 2 → maps to "more economical use of memory" (no intermediate snapshot retention) — *good fit.*
- Claim 3 → maps directly to "more effective data compression" + "reduced hard-disk access time" — *gold-fit.*
- Claim 4 → maps directly to "more economical use of memory" — *gold-fit.*
- Claim 5 → does **not** map cleanly to any §4.5 indicium. **This is the most legally vulnerable claim.**
- Claim 6 → maps to "more economical use of memory" + "higher speed" if quantified — *good fit pending numbers.*

### §3(c) and §3(d) re-anchoring opportunities

The memo focuses exclusively on §3(k) but does not pre-empt §3(c) and §3(d):

- **§3(c) ("mere discovery of any scientific principle")**: Could be raised against Claim 4 (random projection is a known mathematical principle — Johnson-Lindenstrauss 1984). The rebuttal is that *application* of a discovery to a *specific persistence substrate* with *specific seeding from a cryptographic XOF* is patentable. The memo should pre-empt this at Claim 4.
- **§3(d) ("mere new use of a known process")**: Could be raised against Claim 5 (applying NIST CAVS methodology to vector substrates). The memo's prior-art doc §8 anticipates this risk; the §3(k) memo should explicitly cross-reference it.

### Cross-check: every claim in 08-claims.md has a corresponding defense?

`08-claims.md` contains 9 independent claims (1, 7, 13, 18, 25, 30, 37, 42, 46, 52, 58 — actually 11 independents counting CRM/apparatus triplets). The §3(k) memo addresses only **6 claim "families"** by collapsing the method/apparatus/CRM triplets into one family each.

- Claim 1 family (1, 7, 13) — covered ✓
- Claim 2 family (18, 25 — no CRM independent in 08-claims) — covered ✓
- Claim 3 family (30, 37, 42) — covered ✓
- Claim 4 family (46 only — no apparatus or CRM triplet) — covered ✓
- Claim 5 family (52 only — no apparatus or CRM triplet) — covered ✓
- Claim 6 family (58 only — no apparatus or CRM triplet) — covered ✓

**Gap.** Claims 4, 5, and 6 lack apparatus and CRM independents. The memo's "Defensive Claims-Drafting Checklist" item — "All claims include at least one apparatus or computer-readable-medium dependent claim" — is **incorrect** for Claims 4, 5, and 6, which have only method-form independents in `08-claims.md`. **Either the checklist is aspirational or the claims doc needs apparatus/CRM triplets for the remaining three.**

---

## Perspective 5 — Domain Critic (verifying the quantitative backing)

### "32× memory reduction" (Claims 3 and 4)

Cross-check against SPEC and code:
- `SPEC.md` line 26: "`D = 10000` (default), `D = 8192` (SIMD-aligned), `D = 16384` (high-fidelity)".
- Binary HV at `D = 10000` = 10000 bits = **1250 bytes**.
- f32 HV at `D = 10000` = 10000 × 4 bytes = **40 000 bytes**.
- **Ratio: 40 000 / 1250 = exactly 32×.** ✓ The memo's "approximately thirty-two-fold reduction relative to single-precision floats" is *precisely correct* at the same dimensionality.

But: the user's framing of comparison matters. If the comparison is "binary HV at `D = 10000` (1250 bytes) vs. f32 sentence-embedding at `dim = 1024` (4096 bytes)", the ratio is **3.28×**, not 32×. If the comparison is to f32 of the same dimensionality, it's 32×. **The memo should specify the comparison baseline** to avoid being attacked as ambiguous.

**Recommended wording**: "approximately 32-fold reduction in memory per element when compared against a single-precision floating-point representation of the same dimensionality; 3-30× reduction when compared against typical learned float embeddings of dimensionality 512-1024."

### "0.02 bounded similarity drift" (Claim 2)

Cross-check:
- `consolidate.ts` lines 27-33 (verified): `DEFAULT_CONSOLIDATION.maxSimDelta = 0.02`. ✓
- Memo states "Replayable Hebbian consolidation with bounded similarity drift" without stating the 0.02 number explicitly in the memo. **The number is accurate but should be cited in the memo for *Microsoft v. Asst Controller* quantitative-effect anchoring.**
- Independent Claim 18 in `08-claims.md` lines 99-103 recites `max(1, floor(maxSimDelta * D))` parametrically; dependent Claim 19 fixes `maxSimDelta = 0.02`. **Good — the parameter is in the dependent claim.**

**Recommended wording**: state "0.02 (i.e., similarity moves by at most 2% per consolidation pass)" explicitly in the memo's Claim 2 section.

### "Byte-identical across implementations" (Claim 5)

Cross-check `tests/conformance/golden.json`:
- Header: `"spec_version": "0.1.0-draft"`, `"generated_by": "@tanvrit/smritidb (TS reference)"`. ✓
- Coverage (verified by grep): 14 SHA-256 entries spanning `random_hv` (5), `encode_string` (3), `bind_round_trip` (1+), `bundle` (2), `bag_of_words` (2), `char_ngrams` (2). ✓ Matches commit message of `17334f8` ("19 dynamic test cases" / "8 Rust test functions").
- **Note**: golden.json uses **SHA-256** (not BLAKE3) as the integrity check. The memo does not mention this choice; the claim language in 08-claims.md Claim 52(c) correctly says "SHA-256 hexadecimal digest of the packed bytes". **Consistent. But the memo should explain *why* SHA-256 is used here (universally available; not an inventive primitive; corpus is a contract, not an integrity layer) to avoid Examiner confusion between SHA-256 (corpus) and BLAKE3 (substrate).**

### "Approximately half-ones, half-zeros by construction" (background, supports Claim 1 and Claim 4)

Not asserted quantitatively in the memo; not contested. Worth confirming `randomHv` expected distribution in code if relied upon. Not relied upon in this memo so no action.

### "Single-pass streaming load" (Claim 3)

Cross-check SPEC §304 references the header layout. Memo states "magic trailer at the end of the file prevents truncation attacks and signals completeness". Claim 30(f) says "writing, after the said header object, a fixed trailer magic byte string `'FMK\0'`". ✓ Consistent. **No issue.**

### Quantitative gaps

- **Claim 6** has no numbers in the memo. Domain-critical numbers that should be in the memo:
  - Encode latency in ms for a 100-character input at `D = 10000` on a reference machine.
  - Working-memory footprint (bytes) of the encoder.
  - Comparison to a representative sentence-transformer model (e.g., MiniLM, 22 MB; runtime ~50-100 ms on a Cortex-A53).
- **Claim 4** asserts "bounded error envelope" but the bound is not stated. The bound should be quantified (e.g., "Hamming similarity recovers cosine similarity within ±0.05 at D=10000 for inputs in `[-1, 1]^N`" — to be measured if not already).
- **Claim 5** could quantify "number of CI runs prevented from merging" or "number of bindings (5: TS, Rust, Python, Kotlin, Swift)". The number of bindings is concrete and should appear.

---

## Consolidated Revision Worklist

1. **Claim 1 defense — anchor in tangible artefact.** Amend the independent claim text in `08-claims.md` to recite "wherein the said output hypervector is persisted to a wire-format snapshot file" or equivalent, so the §3(k) defense rests on the claim itself rather than only on the memo's narrative.

2. **Claim 2 defense — re-anchor on snapshot+log artefact.** Rewrite the Claim 2 defense paragraph to emphasise "permits restoration of an associative-memory store from a snapshot file plus a log without retention of intermediate states" (a system-level property of stored artefacts) rather than "removes scheduler/floating-point/thread non-determinism" (which is engineering-true but not load-bearing under §3(k)).

3. **Claim 3 defense — pre-empt §3(n).** Add a paragraph addressing the "presentation of information" exclusion, distinguishing KMF from §3(n)-excluded subject matter by reference to its non-cosmetic, integrity-enforcing, version-gating role.

4. **Claim 4 defense — add quantitative envelope.** State the Hamming-similarity error bound for the thermometer encoder (e.g., "cosine similarity is preserved as Hamming similarity to within ±X at D = 10000"); add at least one dependent claim recording the bound.

5. **Claim 4 — anchor in substrate.** Amend Claim 46 in `08-claims.md` to recite a persistence or storage step, not just a "return" step, so the encoder is claimed in combination with the substrate.

6. **Claim 5 defense — sharpen against §3(m) and §3(d).** Add explicit rebuttals: (a) the corpus is an artefact, not a mental act, and (b) the methodology is not a "new use of a known process" because no prior process produces bit-exact contracts over approximate-vector substrates.

7. **Claim 6 defense — add measurable numbers.** State encode latency, footprint, energy, or comparable engineering metric on a named hardware class. Without numbers this defense is the weakest.

8. **Add missing case citations.** *Yahoo! Inc. v. Asst Controller* (IPAB, 2011); *Ericsson v. Intex* (Delhi HC, 2015); *Accenture v. Asst Controller* (IPAB, 2009); *Raytheon v. CGPDTM* (IPAB, 2020). Add neutral citations (2023 SCC OnLine Del XXXX) to *Microsoft* and *OpenTV*. Flag the existence of the **CRI Guidelines draft revision (2024)**.

9. **Verify case paragraph references.** Pull certified copies of *Ferid Allani*, *Microsoft v. Asst Controller*, and *OpenTV*; pin the exact paragraph numbers cited (memo currently says ¶22 of *Ferid Allani* — unverified).

10. **Soften paraphrases to quotations.** Memo's "gold standard" attribution to *Microsoft v. Asst Controller* overstates the holding. Replace paraphrases with direct quotations.

11. **Reconcile prior-art citations.** Memo cites "Imani HPCA 2017" (not in `10-prior-art.md`). Verify whether the paper exists; if not, correct to ICRC 2017 (VoiceHD). Reconcile Jégou date (2010 vs 2011) consistently. Add Achlioptas PODS 2001 to `10-prior-art.md`. Add Mikolov 2013, Pennington 2014, Devlin 2019 to `10-prior-art.md` for Claim 6.

12. **Verify Karunaratne "stochastic across runs" claim.** Either cite the page of *Nature Electronics* 3(6) 327-337 supporting the assertion or soften to "the published work does not characterise determinism".

13. **Quantify "32× footprint reduction" precisely.** Add the baseline comparison: "32× per element at the same dimensionality vs. f32; 3-30× when compared against typical learned float embeddings of dimensionality 512-1024". Avoid ambiguity that might let an Examiner attack the figure.

14. **Add dependent claims for quantitative bounds.** A dependent claim under Claim 46 (encoder) reciting the 32× footprint ratio. A dependent claim under Claim 58 (n-gram) reciting encode latency or footprint bounds on a named hardware class.

15. **Reconcile apparatus/CRM triplet coverage.** Memo's checklist asserts all claims have apparatus/CRM dependents — incorrect for Claims 4, 5, 6 in `08-claims.md`. Either add apparatus/CRM independent claims for these or revise the checklist to be claim-family-specific.

16. **Cite the 0.02 maxSimDelta number explicitly.** In Claim 2 defense, state "0.02 (i.e., 2% per consolidation pass)" rather than the abstract "bounded similarity drift".

17. **Document SHA-256-for-corpus vs BLAKE3-for-substrate.** Add a parenthetical in Claim 5 defense explaining the use of two distinct hash functions, so the Examiner does not perceive an inconsistency.

18. **Cross-document harmonisation.** Sync the §10 prior-art document with the §11 memo so the two are mutually consistent on case citations, paper dates, and differentiator framing. Three specific gaps identified above (HPCA vs ICRC; Jégou date; Word2Vec/GloVe/BERT for Claim 6).

19. **Move "MPEG patents" rhetorical claim or substantiate.** Memo's invocation of "the long line of MPEG-related patents granted by the IPO" as analogical authority for Claim 3 has no citation; either cite one or two IN patent numbers or remove the assertion.

20. **Add a §3(c) pre-emption for Claim 4.** Random projection is a known mathematical principle (Johnson-Lindenstrauss 1984; Achlioptas PODS 2001); the memo should pre-empt a §3(c) "mere discovery" objection by emphasising the *applied* and *substrate-embedded* character of the encoder.

---

**Total objections: 38** (Perspective 1: 6 attack-defense gaps; Perspective 2: 11 case/citation issues; Perspective 3: 6 prior-art discrepancies; Perspective 4: 8 attorney-level gaps; Perspective 5: 7 quantitative-backing items).

**Revision items: 20** consolidated, numbered above.

**Highest-priority items (must-fix before any FER response):** #1, #2, #5, #7, #8, #13, #14.
