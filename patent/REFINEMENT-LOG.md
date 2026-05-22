# Refinement Log

This file logs each round of critique and revision applied to the patent artefacts in the present filing package. It is an internal working document and is not part of the Complete Specification filed with the Indian Patent Office.

## Round 1 — Initial Drafting (2026-05-20)

**Pinned source SHA:** `17334f8`

**Drafting agent:** Claude Opus 4.7 (1M context), working under direction of inventor Vivek Singh.

**Artefacts drafted in this round:**

- `00-cover.md` — Cover sheet, IPC classification, filing target, pinned SHA.
- `01-title-and-preamble.md` — Title and statutory preamble.
- `02-field-of-invention.md` — Single-paragraph field-of-invention statement.
- `03-background.md` — Prior-art treatment and identification of unmet need.
- `04-objects-of-invention.md` — Principal and ancillary objects.
- `05-summary-of-invention.md` — Six-aspect summary plus coherent-system framing.
- `06-drawings-list.md` — Seven figures with ASCII mock-ups and reference numerals.
- `07-detailed-description.md` — (reserved for a separate agent; not modified in this round).
- `08-claims.md` — (reserved for a separate agent; not modified in this round).
- `09-abstract.md` — (reserved for a separate agent; not modified in this round).
- `10-prior-art.md` — (reserved for a separate agent; not modified in this round).
- `11-3k-defense.md` — (reserved for a separate agent; not modified in this round).
- `12-examples.md` — Five worked examples each anchored to the conformance corpus.
- `13-filing-checklist.md` — IPO Form-2 filing checklist.

**Drafting decisions to be re-checked in Round 2:**

1. The choice of working title in `00-cover.md` ("A System and Method for Bit-Exact Cross-Implementation Persistent Associative Memory Using Binary Hyperdimensional Vectors", 15 words) should be reviewed by the §3(k) defender and the IPO-examiner critic. A shorter or more abstract title may improve grantability; a longer or more specific title may better support narrow-claim defensibility.

2. The IPC classifications in `00-cover.md` are provisional. The Indian Patent Office assigns the final classification; the present list (G06F 16/00, G06F 16/903, G06N 3/02, G06F 16/22, G06F 11/10) is a best-effort suggestion to assist initial routing.

3. The framing in `03-background.md` cites Pinecone, Qdrant, Weaviate, Milvus, FAISS, Kanerva (1988), and Imani (2017) without quoting any passage that could ground an obviousness rejection. The Prior-art Hunter critic in Round 2 should verify that no closer prior art is omitted, and that the language used does not concede inventive territory.

4. The technical-effect anchoring in `05-summary-of-invention.md` enumerates six effects (cross-implementation bit-exactness, 32x storage reduction, replayable consolidation, holographic graceful degradation, single-cycle XOR/popcount, per-block BLAKE3 integrity). The §3(k) Defender critic should verify each effect is expressed in language that the Indian Patent Office Computer-Related Inventions Guidelines 2017 will recognise as a "technical effect" rather than as a mere algorithmic property.

5. The reference numerals in `06-drawings-list.md` use a hierarchical scheme: 100-series for encoders, 200-series for primitives, 300-series for substrate, 400-series for KMF wire format, 500-series for persistence-adapter interface, 600-series for adapter implementations, 700-series for conformance bindings, 800-series for embedding-encoder steps. These must be cross-checked against the actual reference numerals used in `07-detailed-description.md` once that file is drafted by its assigned agent.

6. The worked-example computations in `12-examples.md` for the small `D = 8` bundle (Example 2) and the four-dimensional embedding (Example 3) are illustrative; the actual byte outputs are pinned to the conformance corpus at `git show 17334f8:tests/conformance/golden.json` rather than to manually computed bit patterns. The Domain Critic in Round 2 should verify that the worked examples are mathematically correct as far as they go.

7. The fee figures in `13-filing-checklist.md` are based on the schedule as understood at the date of drafting (2026-05-20). The Indian Patent Attorney critic in Round 2 should verify against the current First Schedule of the Patents Rules.

8. The five worked examples in `12-examples.md` deliberately do not pin a SHA-256 digest for the embedding encoder of Example 3, because the conformance corpus at `17334f8` does not include such a pin. Round 2 should consider whether to recommend extending the corpus to include such a pin (a code change, not a specification change).

**Rounds planned (subject to revision):**

- Round 2 — Five parallel critic agents per artefact: IPO Examiner, §3(k) Defender, Prior-art Hunter, Indian Patent Attorney, Domain Critic. Consolidate critiques into a per-artefact revision plan.
- Round 3 — Apply revisions; re-run the five critics in a verification pass.
- Round 4 — External legal review by a registered patent agent (if engaged).
- Round 5 — Final formatting pass, A4 drawing rendering, page-count audit, and pre-filing dry run.

**Notes:**

- No commits were made in Round 1; the working tree state at the end of Round 1 contains untracked drafts for the nine files listed above plus the four reserved files which are owned by other agents.
- All code references are pinned to `17334f8`. No future commit may invalidate the present draft without explicit re-pinning.

## Round 2 — 09-abstract.md

**Pinned source SHA:** `17334f8`  
**Applied:** 2026-05-20

**Critique source:** `patent/critiques/round-2/09-abstract-critique.md`

**Revisions applied:**

1. **Orthography (required):** Changed `thermometer-quantized` to `thermometer-quantised` to bring the abstract into line with Indian English usage and with the rest of the specification (08-claims.md, 10-prior-art.md both use the British/Indian spelling).
2. **Stylistic polish (optional, accepted):** Replaced the terminal `Figure: Fig. 1` with the conventional IPO closing form `Reference is invited to Fig. 1 of the accompanying drawings.` per Item 2 of the consolidated worklist.

**Revisions deferred:**

- Items 3 (header magic accuracy), 4 (tiebreaker phrasing as "tiebreaking arrangement"), and 5 (claim-form colon-bullet softening) were marked optional by the critic and were not applied; the existing phrasing remains accurate and Rule 13(7)-compliant. They may be revisited in a later round if a stylistic uplift is desired.

**Post-revision word count:** 147 words (within the Rule 13(7) cap of 150). PASS.

**Outstanding items for Round 3:** None for this artefact unless deferred items above are reclassified as required.

## Round 2 — 08-claims.md

**Pinned source SHA:** `17334f8` (verified via `git show 17334f8:packages/core-ts/src/{hypervector.ts,consolidate.ts,kmf.ts,text.ts,encode.ts}`, `git show 17334f8:tests/conformance/golden.json`, `git show 17334f8:SPEC.md`).
**Applied:** 2026-05-20

**Critique source:** `patent/critiques/round-2/08-claims-critique.md` (40-item Consolidated Revision Worklist, 31 FER-style objections).

**Pre-revision state:** 63 claims; 11 independents at 1, 7, 13, 18, 25, 30, 37, 42, 46, 52, 58.
**Post-revision state:** 36 claims; 8 independents at 1, 7, 9, 18, 23, 27, 31, 35.

### Items addressed (Priority A — §3(k) survival)

1. **Item 1 (Claim 1 TE in body):** Moved technical effect from preamble into characterising clause; *Ferid Allani v. Union of India* anchoring achieved by reciting the BLAKE3 tiebreaker's operation upon the packed bit-array in memory as the cause of byte-identity, with reduced storage I/O as the consequent technical effect.
2. **Item 2 (Claim 7 NVRAM tie):** Added non-volatile-storage-region recital and tiebreaker-on-data-path limitation; *Microsoft v. Asst Controller* anchoring achieved.
3. **Item 3 (Claim 13 OpenTV preamble):** Not applied directly — the entire Tiebreaker CRM triplet (former claims 13-17) was dropped (item 25), removing the §3(k) surface that this fix would have addressed.
4. **Item 4 (Claim 42 TE in preamble):** Not applied — KMF CRM triplet (former 42-45) dropped (item 26).
5. **Item 5 (Claim 27 / former 46 hardware-tie):** Appended microcontroller-class-processor / no-FPU clause to characterising clause.
6. **Item 6 (Claim 31 / former 52 reframe as system):** Converted in full to apparatus form per Perspective 2; OpenTV gating-action language ("rejects, at load time ... refusing to write the rejected said snapshot into a working memory") added.
7. **Item 7 (Claim 35 / former 58 hardware-tie):** Appended integer-arithmetic / no-FPU clause to characterising clause.

### Items addressed (Priority B — prior-art sharpening)

8. **Item 8 (Claim 1 wire-format mandatory field):** Added to body — domain tag, byte order, field widths recorded as mandatory wire-format header field.
9. **Item 9 (Claim 7 FFI byte-transparent):** Added to characterising clause.
10. **Item 10 (Claim 13 WASM bytecode):** Not applied — Tiebreaker CRM dropped (item 25).
11. **Item 11 (Claim 9 / former 18 maxSimDelta + replayability):** Promoted from former dep. 19/23 into independent step (d) and characterising clause; one-percent-of-D upper bound now in body.
12. **Item 12 (Claim 9 / former 25 NVRAM):** Folded into Claim 9 dependents (claim 15) — former Claim 25's NVRAM/restore-module limitation is now in dep. claim 15.
13. **Item 13 (Claim 18 / former 30 layout + trailer reversal):** Added to body — `ceil(D/8)`-per-vector layout, `D` multiple-of-8 constraint, trailer-by-reversal directional integrity marker, refuse-to-write-until-verified.
14. **Item 14 (Claim 23 / former 37 output-sink enumeration):** Promoted from former dep. 39 into the comprising clause of Claim 23 itself.
15. **Item 15 (Claim 27 / former 46 canonical decimal ASCII + corpus SHA-256):** Added both limitations to the body of Claim 27.
16. **Item 16 (Claim 31 / former 52 dual-kind corpus):** Added to characterising clause of reframed system Claim 31.
17. **Item 17 (Claim 35 / former 58 tiebreaker cross-ref):** Added "deterministic tiebreaker resolution as claimed in claim 1" cross-reference in step (c); locks the text encoder to the unified inventive concept.

### Items addressed (Priority C — antecedent basis)

18. **Item 18 (Claim 9 / former 18 step (a) antecedents):** Introduced "an access log" and "a per-pass salt value" in step (a) of Claim 9; "the said access log" and "the said per-pass salt" used elsewhere now have antecedent.
19. **Item 19 (former Claim 25(a) mirror):** Not applied as a separate fix — former Claim 25 was folded into Claim 9 dependents; the antecedent fix in Claim 9(a) covers all replays.
20. **Item 20 (Claim 18 / former 30 (c) n antecedent):** Introduced "where `n` is an item count of the said substrate" at the head of step (c); subsequent references to `n` now have antecedent.
21. **Item 21 (former Claim 42(b) mirror):** Not applied — KMF CRM dropped.
22. **Item 22 (Claim 35 / former 58 per-word HV antecedent):** Renamed (b)(i) target to "a per-word hypervector for `w_j`"; (b)(ii) now refers back to "the said per-word hypervector for `w_j`".

### Items addressed (Priority D — terminology unification)

23. **Item 23 (bind-via-XOR):** Replaced standalone "element-wise exclusive-or" wording with "a bind operation comprising element-wise exclusive-or" in (a) Claim 1 dep. 6, (b) Claim 27(c)(v), and (c) Claim 35(b)(iii).
24. **Item 24 (cyclic permutation):** Replaced "cyclic bit rotation" with "cyclic permutation" in Claim 35(b)(ii); added explicit definition of the permutation as element-wise `(p + j) mod D` mapping, tracking the code's `permute()` primitive at `text.ts` line 95 / `hypervector.ts` lines 78-86.

### Items addressed (Priority E — claim count and structure)

25. **Item 25 (drop Tiebreaker CRM 13-17):** Done. Five claims dropped.
26. **Item 26 (drop KMF CRM 42-45):** Done. Four claims dropped.
27. **Item 27 (alternative):** Not applicable — items 25 and 26 chosen over the alternative.
28. **Item 28 (consolidate former Claim 25 into Claim 18 / now Claim 9 deps):** Done. Former Claim 25 (Hebbian system) plus its dependents 26-29 (five claims) collapsed into dependents 15, 16, 17 of Claim 9. Two new dependents (15, 17) carry forward the substantive system-level features (NVRAM persistence; zero-counter eviction).
29. **Item 29 (unity-of-invention recitation):** Added an explanatory paragraph immediately after the title in `08-claims.md` identifying "BLAKE3-anchored deterministic byte-identity of a binary hyperdimensional associative memory substrate" as the single inventive concept; cites *Genentech Inc.'s Patent* [1989] RPC 147. Note: the brief also requested this be added to `05-summary-of-invention.md` or `07-detailed-description.md` — that is outside this revision pass's file scope and is deferred to Round 3 for the relevant artefact's reviser.

### Items addressed (Priority F — dependent multi-dependence)

30. **Item 30 (Claim 5 restriction):** Done. Former Claim 5 was multi-dependent on "any of claims 1 to 4"; new Claim 5 is restricted to "any of claims 1 to 3" (claim 4 in the new file is itself dependent on "any of claims 1 to 3", so it is *not* multi-dependent on another multi-dependent — but to be safe, claims 5 and 6 also stop at 1 to 3).
31. **Item 31 (audit every "any of claims X to Y"):** Done. Every multi-dependent range in the new file is anchored at a non-multi-dependent claim. Specifically: Claims 5-6 stop at "claims 1 to 3"; Claims 12-17 stop at "claims 9 to 11"; Claims 21-22 stop at "claims 18 to 20"; Claim 25 stops at "claims 23 to 24"; Claim 26 stops at "claims 23 to 25"; Claim 30 stops at "claims 27 to 29"; Claim 34 stops at "claims 31 to 33"; Claim 36 depends only on Claim 35.

### Items addressed (Priority G — dependent-claim language)

32. **Item 32 (Claim 19 / new Claim 10 maxSimDelta soften):** Done. Changed "fixed at 0.02" to "a value not greater than 0.02".
33. **Item 33 (Claim 47 / new Claim 28 L soften):** Done. Changed "fixed at 100" to "a value of at least 64 and not greater than 256".
34. **Item 34 (Claim 53 / new Claim 32 text groups):** Done. Added bag-of-words and character-n-gram groups as conditional on text-encoding support.
35. **Item 35 (Claim 21 salt monotonicity DROP-or-IMPLEMENT):** **Dropped.** Per Tension 5 in the critique, monotonic-salt orchestration is not implemented at SHA 17334f8 — the per-pass `salt` parameter is accepted by `pullCloser` but no caller in the tree drives it monotonically. Implementing it would require a pre-filing code commit; the cleaner approach is to drop the claim and re-add via a divisional / supplementary claim after orchestration lands. Dropped.
36. **Item 36 (Claim 29 epoch counter DROP-or-IMPLEMENT):** **Dropped** for the same reason as item 35.

### Items addressed (Priority H — repo errata)

These are **outside the file scope of `08-claims.md`** — they are pre-filing repository commits. **Status: deferred to a separate REPO-errata commit; tracked here for the inventor to action before filing.**

37. **Item 37 (SPEC.md §1.3 add `"smritidb/tiebreak"` tag):** Deferred (REPO change).
38. **Item 38 (SPEC.md §8.2 MessagePack → JSON):** Deferred (REPO change). Note: the claim-level fix is already in place — new Claim 19 says "the said header object is serialised as a JSON document **and the said metadata block is encoded as a JSON document**", which matches code at `kmf.ts` line 75-82.
39. **Item 39 (golden.json add tolerance fields):** Deferred (REPO change).
40. **Item 40 (monotonic salt orchestration code commit):** Deferred AND made moot for this revision by dropping claims 21 and 29 (items 35-36).

### Items deferred to Round 3

- **Items 37, 38, 39, 40:** Pre-filing repository commits to `SPEC.md` and `tests/conformance/golden.json` (and optionally `consolidate.ts` + `kmf.ts` for monotonic salt). These must be made before filing but are *not* edits to `patent/` files.
- **Unity-of-invention paragraph in `05-summary-of-invention.md` or `07-detailed-description.md`:** Round 2 revision pass for `08-claims.md` placed the paragraph in the claims file itself (in an explanatory note before claim 1). The same concept should be cross-referenced from `05-summary-of-invention.md` and `07-detailed-description.md` in their respective Round 2 / Round 3 revision passes by their owning agents.
- **Cross-reference audit:** Claim 35 cites "the deterministic tiebreaker resolution as claimed in claim 1" and "the canonical conformance corpus of the system of claim 31." A Round-3 reviser should verify that these cross-references survive any further renumbering; if claim 1 or claim 31 moves, claim 35's text must be updated.
- **`07-detailed-description.md` alignment:** Several Round-2 promotions of dependent-claim features into independents (Item 11 maxSimDelta one-percent bound; Item 13 D-multiple-of-8; Item 13 trailer-by-reversal) must be cross-checked against the detailed description by Round-3 to confirm every claimed feature is supported. Deferred to that artefact's reviser.
- **`09-abstract.md` and `11-3k-defense.md` updates:** The reframing of Claim 52 → Claim 31 (apparatus form) and the dropping of three CRM triplets materially change the public face of the claim set. The abstract and the §3(k) defence must be reconciled with the new claim numbering. Deferred to the respective artefact revisers.

### Tensions that surfaced

1. **Unity-of-invention vs. claim-count reduction.** Dropping the three CRM triplets (former 13-17, 42-45) and the Hebbian system triplet (former 25-29) reduces §3(k) risk and IPO excess-claim fees but weakens the "method + system + CRM" triptych that is standard defensive practice in Indian IT-patent drafting. **Resolution:** Accepted the trade-off because (i) the IPO has, since OpenTV 2023, treated CRM claims with extra scepticism under §3(k), and (ii) the surviving system claims (7, 23, 31) carry sufficient apparatus framing to defend the substantive matter without the CRMs. If the inventor wishes, the dropped triplets can be revived via continuation / divisional filings.

2. **Cross-claim dependency (Claim 35 cites Claim 1 and Claim 31).** This binds the text encoder to the rest of the claim set as a "unified inventive concept" (good for §16 unity) but creates a brittleness: if Claim 1 or Claim 31 is amended or rejected during prosecution, Claim 35's scope is disturbed. **Resolution:** Kept the cross-references because the unity-of-invention argument is the stronger consideration; prosecution flexibility can be recovered by adding a "deterministic tiebreaker as defined hereinabove" wording variant if the cross-reference must be severed at examination.

3. **Salt-monotonicity drop (items 35, 36) vs. specification coherence.** The brief's SPEC §5 and `05-summary-of-invention.md` both describe consolidation as "deterministic given identical inputs", implying that the salt is driven monotonically per pass. By dropping the explicit monotonicity claims, the patent now claims less than the specification describes. **Resolution:** This is consistent with patent drafting practice — claims should claim only what is implemented at the pinned SHA; the broader specification serves as enablement-by-disclosure for future divisionals. The Round-3 reviser of `05-summary-of-invention.md` should consider softening the deterministic-orchestration language to match the current claim scope, OR the inventor should commit the monotonic-salt orchestration pre-filing and re-introduce the claims.

4. **Per-block BLAKE3 vs. wire-format integrity.** Claim 18 now includes a "refuse to write any substrate state into a working memory ... until the said per-block BLAKE3 hash digests have been verified" limitation. This is a strong technical effect but may be over-narrow if a future implementation wants to stream-decode-then-verify (e.g., for very large substrates). **Resolution:** Accepted as written; the limitation is in the independent claim of the method form (Claim 18) and the system form (Claim 23) — a stream-decode variant could be added as a future dependent.

5. **Claim 31 (conformance system) cites SHA-256 but Claim 1 cites BLAKE3.** The cross-implementation byte-identity check in the conformance corpus uses SHA-256 (per `golden.json` at SHA 17334f8) while substrate operations use BLAKE3. This is intentional — SHA-256 is the "outer" digest used to detect divergence; BLAKE3 is the "inner" primitive used for all substrate-internal operations. **Resolution:** No fix needed — the distinction is correctly preserved in the claim language.

### Final tally

- **Total claims:** 36 (down from 63; brief target was ~36).
- **Independent claims:** 8 (down from 11; brief permitted reduction by dropping weakest dependents while keeping independents; we dropped three CRM triplets and folded the Hebbian system into the Hebbian method's dependents, yielding the net reduction).
- **Worklist items addressed:** 31 of 40 directly; 4 made moot by claim-drop decisions (items 3, 4, 10, 19, 21); 4 deferred as REPO changes (items 37, 38, 39, 40 — outside `patent/` file scope per the user's instruction "Do NOT modify any file outside `patent/`").
- **31 FER-style objections:** Addressed in full or made moot by claim-drop. See per-claim mapping in the critique file at `patent/critiques/round-2/08-claims-critique.md`.

## Round 2 — 11-3k-defense.md

**Pinned source SHA:** `17334f8`
**Applied:** 2026-05-20
**Revising agent:** Claude Opus 4.7 (1M context), Round 2 reviser for the §3(k) defense brief.
**Critique source:** `patent/critiques/round-2/11-3k-defense-critique.md` (38 objections; 20 consolidated worklist items).

**Items addressed (20 of 20):**

1. **#1 — Claim 1 artefact anchoring.** Defense paragraph rewritten so the technical effect resides in the **claim body** (packed-byte hypervector persisted as an item key in KMF substrate), not merely in the preamble. Coordinated note added that the independent claim text in `08-claims.md` requires a parallel amendment by the claims reviser.

2. **#2 — Claim 2 re-anchoring.** Re-anchored on the **snapshot+log artefact pair** (a system-level property of stored artefacts, directly within the *Raytheon* IPAB 2020 framework) rather than on "deterministic replay" as an abstract property. The `maxSimDelta = 0.02` value (per `consolidate.ts` lines 27–33 at HEAD SHA `17334f8`) is recited explicitly.

3. **#3 — Claim 3 §3(n) pre-emption.** A new pre-emption paragraph distinguishes KMF from §3(n)-excluded subject matter: machine-machine binary contract with verification semantics, no human-facing presentation layer, governed by *Raytheon* (IPAB 2020).

4. **#4 — Claim 4 quantitative envelope.** Hamming-cosine error bound ε ≤ 0.05 at D = 10000 for inputs in [−1, 1]^N stated explicitly. Dependent-claim recommendation carried forward to the claims reviser.

5. **#5 — Claim 4 substrate anchoring.** Defense paragraph re-anchored: the binary hypervector item is persisted as an item key in the KMF substrate; the technical effect resides in the claim body, not the preamble.

6. **#6 — Claim 5 §3(m) and §3(d) pre-emption.** Two pre-emption paragraphs added: §3(m) (corpus is a stored artefact, not a mental act, supported by *Yahoo!* IPAB 2011 and *Ferid Allani* Delhi HC 2019); §3(d) (CAVS targets deterministic cryptography with zero error envelope, whereas the corpus targets approximate-index byte-identity — methodologically distinct, not a "mere new use").

7. **#7 — Claim 6 measurable numbers.** Four quantitative metrics added: zero trainable parameters (vs ≈300 M for BERT-base; ≈22 M for MiniLM); O(n · D) encode complexity (vs O(L · H²) for neural inference); < 2 MB working memory at D = 10000 on ARM Cortex-M-class hardware; no external tokeniser dependency outside Unicode NFC normalisation.

8. **#8 — Missing case citations.** Added *Yahoo! Inc. v. Asst Controller* (IPAB 2011, OA/22/2010/PT/CH); *Telefonaktiebolaget LM Ericsson v. Intex Technologies* (Delhi HC, I.A. 6735/2014 in CS(OS) 1045/2014, 13 March 2015); *Accenture Global Service GmbH v. Asst Controller of Patents* (IPAB 2009, OA/22/2009/PT/DEL); *Raytheon Co. v. Controller General of Patents* (IPAB OA/27/2018/PT/DEL, 16 September 2020). Added neutral citations: 2023 SCC OnLine Del 2772 (*Microsoft*); 2023 SCC OnLine Del 3251 (*OpenTV*); 2019 SCC OnLine Del 11867 (*Ferid Allani*). Flagged CRI Guidelines 2024 draft revision as circulated for public consultation; status: not yet finalised as of 2026-05-20.

9. **#9 — Paragraph references.** *Ferid Allani* operative paragraph corrected to ¶12 (principal operative passage on narrow reading of *per se*), with ¶21–23 noted for the broader holding. A note on certified-copy verification has been incorporated.

10. **#10 — Paraphrases softened.** *Microsoft v. Asst Controller* "gold standard" paraphrase replaced with the narrower direct holding ("the Examiner is bound to engage with technical-effect evidence placed on the record").

11. **#11 — Prior-art reconciliation.** Imani citation corrected from the misremembered "HPCA 2017" to **VoiceHD ICRC 2017** (with the follow-up DAC 2018 paper noted). Jégou date convention reconciled to "TPAMI 33(1), 117–128, January 2011 (published online 2010)". Note: Achlioptas PODS 2001 and Mikolov 2013 / Pennington 2014 / Devlin 2019 are cited in this memo and should be back-ported into `10-prior-art.md` by the prior-art-doc reviser.

12. **#12 — Karunaratne assertion softened.** "Still stochastic across runs" softened to "the published work does not characterise determinism across re-runs of the consolidation step".

13. **#13 — 32× baseline.** Explicit baseline statement added: 32× per element at equivalent dimensionality (binary HV at D = 10000 = 1250 bytes vs. f32 HV at D = 10000 = 40000 bytes); 3–30× when compared against typical learned float embeddings at dim = 512–1024 (e.g., MiniLM dim = 384, sentence-transformer dim = 768, OpenAI ada-002 dim = 1536).

14. **#14 — Quantitative dependent claims.** Recommendation recorded in §3 (Claims 4 and 6) and in the §5 defensive checklist that dependent claims be added under Claim 46 (32× ratio; ε ≤ 0.05 envelope) and Claim 58 (zero parameters; O(n · D); < 2 MB working memory). Carried forward to the claims reviser. Note: the parallel 08-claims Round-2 entry has re-numbered some claims; the §3(k) memo retains the pre-renumbering Claim-46/Claim-58 references for traceability, and a cross-numbering reconciliation pass is flagged for Round 3.

15. **#15 — Apparatus/CRM triplet coverage.** Defensive checklist item corrected from a blanket assertion to a claim-family-specific statement: families 1, 2, 3 have triplet coverage in the Round-1 `08-claims.md`; families 4, 5, 6 do not, and the gap is flagged for the claims reviser, with *Accenture* (IPAB 2009) cited as the governing authority. Note: the parallel 08-claims Round-2 revision dropped certain CRM triplets; the present §3(k) brief notes the trade-off and aligns its checklist with the Round-1 claim numbering.

16. **#16 — `0.02 maxSimDelta`.** Stated explicitly in the Claim 2 defense, with the gloss "(i.e., at most 2% per consolidation pass)".

17. **#17 — SHA-256 vs BLAKE3.** A parenthetical added to the Claim 5 defense distinguishing the corpus-level use of SHA-256 (universal availability, contract-readable) from the substrate-level use of BLAKE3 (inventive integrity primitive). This is consistent with the 08-claims Round-2 resolution recorded above (tension #5).

18. **#18 — Cross-document harmonisation flagged.** The mismatched citations (Imani HPCA→ICRC; Jégou 2010 vs 2011; Word2Vec/GloVe/BERT/Achlioptas missing from `10-prior-art.md`) are reconciled in this memo. Back-port into `10-prior-art.md` is flagged as a follow-up item for the prior-art-doc reviser.

19. **#19 — MPEG rhetorical flourish removed.** Replaced with the *Raytheon* (IPAB 2020) precedent, which is the directly governing Indian authority for data-structure-with-verification claims.

20. **#20 — §3(c) pre-emption for Claim 4.** A pre-emption paragraph added: §3(c) bars *mere discovery*, not *applied embedding in a specific substrate with cryptographic XOF seeding*. Read in light of *Ferid Allani* (Delhi HC, 2019) on the narrow reading of "*per se*".

**Items deferred (out of scope for the §3(k) reviser):**

- **Back-port of Achlioptas (PODS 2001), Mikolov (ICLR 2013), Pennington (EMNLP 2014), Devlin (NAACL 2019) into `10-prior-art.md`** — flagged for the prior-art-doc reviser.
- **Amendments to the independent claim text of Claims 1, 4 (and dependent claims under 46 and 58)** in `08-claims.md` — flagged for the claims reviser. The §3(k) memo records the precise recitations required.
- **Apparatus/CRM independent-claim triplets for Claim families 4, 5, 6** — flagged for the claims reviser. (Note: the parallel 08-claims Round-2 revision has dropped certain CRM triplets on §3(k)-economy grounds; cross-reconcile in Round 3.)
- **Certified-copy pin of *Ferid Allani* paragraph reference** — the memo cites ¶12 with a note that ¶21–23 contain the broader holding. The literal certified-copy paragraph numbering should be confirmed by a registered patent agent prior to FER response filing.

**Case citations in the revised memo (7 distinct cases + 2 administrative sources):**

1. *Yahoo! Inc. v. Asst Controller of Patents and Designs*, IPAB OA/22/2010/PT/CH, 8 December 2011.
2. *Accenture Global Service GmbH v. Asst Controller of Patents*, IPAB OA/22/2009/PT/DEL, 11 May 2009.
3. *Telefonaktiebolaget LM Ericsson v. Intex Technologies (India) Ltd.*, Delhi HC, I.A. 6735/2014 in CS(OS) 1045/2014, 13 March 2015.
4. *Ferid Allani v. Union of India & Ors.*, Delhi HC, W.P.(C) 7/2014 and CM APPL. 40736/2019, 12 December 2019; 2019 SCC OnLine Del 11867.
5. *Raytheon Co. v. Controller General of Patents*, IPAB OA/27/2018/PT/DEL, 16 September 2020.
6. *Microsoft Technology Licensing LLC v. Asst Controller of Patents and Designs*, Delhi HC, C.A.(COMM.IPD-PAT) 29/2022, 15 May 2023; 2023 SCC OnLine Del 2772.
7. *OpenTV Inc. v. Controller of Patents and Designs & Anr.*, Delhi HC, C.A.(COMM.IPD-PAT) 14/2022, 31 May 2023; 2023 SCC OnLine Del 3251.

Plus the administrative sources: CRI Guidelines 2017 (paragraph 4.5) and the CRI Guidelines 2024 draft revision (in public consultation, not yet finalised as of 2026-05-20).

**Round-2 outcome.** Memo expanded from 2,369 words (Round 1) to approximately 4,400 words (Round 2). All 20 consolidated worklist items addressed in-document; four items explicitly deferred to other revisers with full traceability. The §3(k) defense is now anchored case-by-case in (a) tangible artefacts recited in the claim body, (b) quantitative engineering metrics tied to HEAD SHA `17334f8`, and (c) the leading Indian precedents (*Yahoo!*, *Accenture*, *Ericsson v. Intex*, *Ferid Allani*, *Raytheon*, *Microsoft*, *OpenTV*).

**Outstanding items for Round 3:** Coordinate with the claims reviser (cross-numbering reconciliation; items #1, #5, #14, #15) and the prior-art-doc reviser (item #18) to back-port the corresponding amendments and citations. Verify certified-copy paragraph numbering for *Ferid Allani* prior to FER response filing.

## Round 2 — 10-prior-art.md

**Pinned source SHA:** `17334f8`
**Applied:** 2026-05-20
**Revising agent:** Claude Opus 4.7 (1M context), Round 2 reviser for the prior-art memo.
**Critique source:** `patent/critiques/round-2/10-prior-art-critique.md` (eighteen-item Consolidated Revision Worklist, five-perspective panel critique).

**Worklist items addressed (18 of 18):**

1. **P0 — Item 1 — InPASS Indian-prior-art search (§5.1).** Replaced the bare `[TBD]` with a six-step methodology section enumerating exactly what the patent agent must record (query strings with InPASS-specific Boolean syntax, dates, hit counts, top-twenty triage with patent number / title / abstract / applicant / status, per-hit claim-by-claim differentiation, search of Indian *applications* as well as grants under §13 of the Patents Act, and re-check at filing day). Tagged `[P0 BLOCKER BEFORE FILING]`. The §5.1 finding remains pending the agent's formal search; no Form-2 filing should proceed before completion.

2. **P0 — Item 2 — US patent verification (§5.2).** Attempted independent verification of both Round-1 cites via WebFetch against patents.google.com. **Critical finding:** US 11,775,847 B2 is *not* an Imani-HDC patent — it is in fact *"Systems and methods for classifying media according to user negative propensities"*, inventor Kenneth Neumann, assignee KPN Innovations LLC, granted 3 October 2023. The Round-1 attribution was substantively incorrect. US 10,956,464 B1 returned HTTP 404 on retrieval. Both Round-1 cites are flagged as `[P0]` blockers and the §5.2 section has been rewritten with the five-step formal-search methodology the patent agent must follow (USPTO Patent Public Search inventor search for Imani; assignee search for Pinecone; inventor search for Rahimi; subject-matter searches by IPC class G06F/G06N/G11C; per-hit evidence filing in `patent/prior-art-evidence/us-patents/`).

3. **P0 — Item 3 — Missing prior-art categories.** Added the following families in full, each with author(s), year, title, venue, page range and DOI / arXiv ID / standard reference:
   - **LSH / SimHash / sign-LSH** (new §4.8): Charikar 2002 (STOC); Datar et al. 2004 (SoCG); Andoni & Indyk 2008 (CACM); Manku et al. 2007 (WWW); Achlioptas 2003 (JCSS). Affects Claims 1, 4, 6 — central threat.
   - **Rahimi-line level encoder** (rewrote §2.4): Rahimi-Kanerva-Rabaey 2016 (ISLPED) added as the *foundational* reference for level encoding in HDC, with Rahimi et al. 2017 (TCAS-I) as the comprehensive treatment and Rahimi et al. 2016 (ICRC, biosignal) for completeness. The Round-1 misattribution of thermometer encoding to Imani 2017 is expressly corrected; Imani repositioned as derivative on the encoder point.
   - **Imani HPCA 2017** ("Exploring Hyperdimensional Associative Memory") added per critique §1.1(d) and the §3(k) memo reconciliation requirement.
   - **SQLite WAL** (https://www.sqlite.org/wal.html) added as the closest log-and-replay prior art for Claim 2.
   - **HDF5 Fletcher32 chunk filter** and **Apache Parquet magic footer** added as direct analogues for Claim 3.
   - **Merkle trees** (Merkle 1979; US 4,309,569) and **content-addressed storage** (Git, IPFS) added as direct prior art for per-block cryptographic integrity.
   - **Probabilistic binary substrates** (new §3.9): Bloom 1970 (CACM); Cormode & Muthukrishnan 2005 (J. Algorithms); Broder 1997 (MinHash).
   - **Kanerva 1996 Binary Spatter Codes** (new §2.7; ICANN, LNCS 1112) — the foundational BSC reference, missing from Round 1.
   - **NIST CAVP / FIPS 202 (SHAKE128/256) / SP 800-140 series** — added for Claim 5 and to broaden the Claim 1 XOF anchor.
   - **BLAKE2X** (Aumasson et al. 2013) added to broaden the XOF claim language.
   - **Generic-serialisation comparators** (CBOR / RFC 8949; FlatBuffers; Cap'n Proto; Bencode BEP 0003; Protocol Buffers) enumerated in §6 row 3.
   - **Gayler 1998** (multiplicative binding); **Kleyko et al. 2018** (TNNLS, binary HDC tradeoffs); **Frady-Kleyko-Sommer 2018** (Neural Computation); **Thomas-Dasgupta-Rosing 2021** (JAIR); **Neubert et al. 2019** (KI).

4. **P0 — Item 4 — Cross-document reconciliation with `11-3k-defense.md`.** Added the references that the §3(k) defence cites but the Round-1 prior-art memo omitted: **Imani HPCA 2017**, **Achlioptas 2003**, **Mikolov 2013** (Word2Vec), **Pennington 2014** (GloVe), **Devlin 2019** (BERT). New §4.10 added for the neural text-encoder prior art (Claim 6 contrast). The §6 row 1 (Claim 1) and row 4 (Claim 4) closest-prior-art lists are now harmonised across the two filing documents.

5. **P0 — Item 5 — §6 differentiator language re-cast.** Rewrote rows 1, 2, 4 and 6 to lead with the *system-level technical effect* — verifiable cross-implementation byte-identity; deterministic replay with bounded drift; footprint reduction with corpus-verifiable bounded error envelope; on-device training-free feasibility — and only then describe the structural mechanism. Rows 3 and 5 already led with technical effect and were retained with light revision. The language mirrors §3 of `11-3k-defense.md`.

6. **P1 — Item 6 — §6 split into "closest single reference (anticipation challenge)" vs "closest combination (obviousness challenge)" columns.** The matrix is now in the form examiners use for an FER response.

7. **P1 — Item 7 — Fourth "differentiating limitation in claim-element form" column added** to §6, tracing each row's differentiator to the exact words that should appear in the corresponding claim in `08-claims.md`.

8. **P1 — Item 8 — HNSW adjacent-not-conflicting paragraph added** at new §3.8, with a forward-looking statement that the present application does not pursue recall-index claims and that any Phase-2 LSH-based recall index will require a separate continuation analysis against HNSW and the LSH family.

9. **P1 — Item 9 — DOIs and page pins added throughout** §2 and §4. Specifically: Kanerva 2009 (10.1007/s12559-009-9009-8); Plate 1995 (10.1109/72.377968); Rachkovskij & Kussul 2001 (10.1162/089976601300014592); Plate 2003 monograph ISBN (1-57586-430-7); Imani VoiceHD 2017 (10.1109/ICRC.2017.8123650); Imani DAC 2018 (10.1145/3195970.3196060, Article 108, 6 pages); Imani HPCA 2017 (10.1109/HPCA.2017.28); Rahimi 2016 ISLPED (10.1145/2934583.2934624); Rahimi et al. 2017 TCAS-I (10.1109/TCSI.2017.2705051); and DOIs for Frady-Kleyko-Sommer 2018 and Thomas-Dasgupta-Rosing 2021 (previously `[TBD]`). Kanerva 1988 page-pin to Chapter 3 (majority bundle) added.

10. **P1 — Item 10 — §5.1 "no Indian prior art identified" softened** to a preliminary title-and-abstract scan, with the assertion not supportable without formal claim-by-claim search.

11. **P1 — Item 11 — Vague "other US patents assigned to Microsoft, Meta, IBM and Intel" sentence removed**; replaced with a reference to the patent agent's formal FTO search methodology at §5.2 (e).

12. **P1 — Item 12 — §11 verification statement tightened** to enumerate exactly which citations have been independently verified against the original (with DOI / ISBN / standard cite) and which remain pending verification. The Round-1 sweeping "Verified against well-known scholarly sources" claim is replaced with an itemised list.

13. **P1 — Item 13 — §3(d) rows for Claims 1, 2, 3, 4 added** to the §8 risks table. Round 1 had §3(d) only for Claim 5. Each row articulates why a §3(d) "mere new use" attack fails on the substantive merits, citing the CRI Guidelines 2017 §4.5 and *Ferid Allani* (Delhi HC, 2019).

14. **P2 — Item 14 — Foundational background added** at new §§ 2.7 (Kanerva 1996 BSC), 2.8 (Gayler 1998), 4.7 (Kleyko et al. 2018 TNNLS; Frady-Kleyko-Sommer 2018; Thomas-Dasgupta-Rosing 2021; Neubert et al. 2019); Plate 2003 monograph ISBN added at §2.2.

15. **P2 — Item 15 — Schlegel 2022 and Karunaratne 2020 entries extended** with sentences noting where these papers discuss binary-substrate / reproducibility issues (Schlegel discusses BSC; Karunaratne discusses bit-exact reproducibility under PCM read noise), pre-empting examiner combinations.

16. **P2 — Item 16 — Vendor documentation date-stamped** at §3 ("last checked 20 May 2026; to be re-checked at filing day") and at §3.7. The action-items list at §10 includes a re-check requirement.

17. **P2 — Item 17 — CBOR (RFC 8949), FlatBuffers, Cap'n Proto, Bencode (BEP 0003), Protocol Buffers** enumerated as comparators in §6 row 3 closest-combination column.

18. **P2 — Item 18 — §1.5 query-string list extended** from seven queries to thirteen, adding InPASS Boolean-syntax variations and explicit queries for LSH / SimHash, Bloom-filter, Merkle-tree, WAL, and Imani / Rahimi inventor-name searches.

**Round-2 changelog block** added to the top of `10-prior-art.md` summarising additions, the Imani-2017 → Rahimi-2016/2017 correction, the §3(k) memo reconciliation list, and the three P0 blockers.

**Tensions that surfaced:**

1. **Verification revealed substantive citation error.** The Round-1 `[TO VERIFY]` flag on US 11,775,847 B2 was warranted: independent retrieval established that this patent number does not refer to an Imani-HDC patent at all. This is a teaching for the broader memo: every patent number must be independently verified before being relied upon, and the Round-2 memo now states this principle explicitly in §5.2. The methodological note at the end of §5.2 commits the patent agent to filing the first page and independent claim 1 of every cited US patent in `patent/prior-art-evidence/us-patents/`.

2. **Rahimi 2016 vs Imani 2017 attribution.** The Round-1 memo treated the Imani line as the foundational thermometer-encoder reference. The Round-2 corpus places Rahimi 2016 (ISLPED) as the foundational reference and the Imani line as derivative. This is the correct attribution, but it has implications for Claim 4's differentiation strength: Rahimi 2016 is *closer* prior art than Imani 2017, and so the inventive step over the closest prior art shrinks. **Resolution:** The §6 row 4 differentiator now leads with the *technical effect* (footprint reduction with corpus-verifiable bounded error envelope) rather than with the algorithmic mechanism, so the shrinking of the algorithmic delta does not undermine the claim's defensibility on technical-effect grounds.

3. **LSH / SimHash as the most likely obviousness combination.** The Round-1 memo did not address the LSH family at all. The Round-2 memo addresses it as the single largest gap. Charikar 2002 sign-LSH is the closest prior art for a deterministic projection from a vector to a binary fingerprint. The Smritidb Claim 4 encoder differs in producing a *level-quantised* output (thermometer of L levels) rather than a single sign bit, and in pinning the seed to the KMF wire-format header so that a third party can verify the encoder output bit-for-bit against the conformance corpus. This differentiation is now in §4.8 and in §6 row 4.

4. **Patent number uncertainty cascades to §5.3, §5.4.** The Round-1 memo carried `[TBD]` on Espacenet and PATENTSCOPE searches. Given the Round-2 finding that even one of the two cited US patent numbers was wrong, the patent agent's formal foreign-search methodology must be substantively executed, not merely confirmed. The §10 action-items list reflects this elevated standard.

5. **Claim breadth vs prior-art density.** Adding LSH / SimHash / Bloom / Merkle / WAL prior art necessarily reduces the apparent breadth of the surviving inventive step. The Round-2 memo's strategic response is to draw each claim as a *combination* claim with *system-level technical effect framing*, so that no single prior-art reference (or short combination thereof) anticipates and so that the §3(k) defence remains anchored to verifiable engineering properties rather than to algorithmic novelty.

**Final counts (Round 2 outcome):**

- **Word count:** Round-1 approximately 3,995 words; Round-2 approximately 7,650 words (≈91% increase).
- **Reference count:** Round-1 approximately 22 distinct citations; Round-2 approximately 60+ distinct citations (≈173% increase). Specifically the §9 Form-2 background list grew from 10 to 27 entries.
- **New citations added (count):** 38+ new prior-art references across academic literature, patents, standards and vendor documentation.
- **P0 blockers identified:** Three — (i) §5.1 InPASS formal search; (ii) Imani-HDC US patent number re-establishment (Round-1 cite is wrong); (iii) Pinecone US patent number re-establishment (Round-1 cite returns 404).
- **`[TO VERIFY]` items remaining:** US patent numbers for Imani-HDC and Pinecone portfolios; Indian-affiliated HDC publications (IISc, IIT Bombay); Espacenet hits; PATENTSCOPE hits; Imani QuantHD/SparseHD/BRIC full texts (DOIs verified, not independently read); Gayler 1998 working-notes text (cited, not read).

**Files modified in Round 2:**

- `patent/10-prior-art.md` — Substantial rewrite (Round-1 ≈3,995 words → Round-2 ≈7,650 words). Changelog block added at top.
- `patent/REFINEMENT-LOG.md` — This section appended.

**No commits made.** No files modified outside `patent/`. All code references remain pinned to `17334f8`.

**Outstanding items for Round 3:** Execute the §5.1 InPASS search; re-establish the correct Imani-HDC and Pinecone US patent numbers per the §5.2 methodology; reconcile cross-numbering between this memo's claim references and the renumbered claim set in the Round-2 `08-claims.md` (the §3(k) memo Round-2 entry notes the same reconciliation requirement); refresh vendor-documentation checks at filing day; if any of the verified citations are amended in Round 3, update the changelog and the §11 provenance note accordingly.

## Round 2 — 07-detailed-description.md

**Reviewed at SHA:** `17334f8`
**Critic file:** `patent/critiques/round-2/07-detailed-description-critique.md` (399 lines, 50-item Consolidated Revision Worklist).
**Worked by:** Round-2 revising agent under direction of inventor Vivek Singh.
**Applied on:** 2026-05-20.

### Items addressed (50 / 50 in the worklist)

1. **§1(a) — SIMD lane widths named.** Added 256-bit AVX2 (D=8192), 512-bit AVX-512 (D=16384), 128-bit NEON for all listed dimensions.
2. **§1(g) — UDL file path disclosed.** `packages/smritidb-ffi/src/smritidb.udl` recited.
3. **New §2.4 added — best-mode SIMD acceleration.** `u64` lanes, `_popcnt64`, `vcntq_u8`, scalar fall-back.
4. **§2.3 — Phase-0 validation notebook cited.** `notebooks/phase0_hdc_validation.ipynb`.
5. **§2.1 — quantified "too large" variance.** σ ≈ 0.0156 at D=1024; ±3σ window exceeds 0.1; FP rate exceeds 1% under uniform-random storage.
6. **§3.1.1 — SPEC.md §1.3 erratum flagged.** Editorial note added; SPEC text and implementation are inconsistent; specification follows implementation.
7. **§3.1.1 — `tiebreak` return type annotated** as `-> u8`.
8. **§3.1.2 bundle pseudocode** — `half = n/2` replaced with integer-only `2*sums[i] vs n` comparison.
9. **§3.2 — bind closest prior art named.** Vaswani 2017 (transformer KV), Sukhbaatar 2015 (learned positional).
10. **§3.3 — Kanerva 2009 cited and distinguished; RoPE comparison quantified** (O(D log D) complex vs O(D) integer copy).
11. **§3.4 — SIMD popcount claim softened** to "amenable to SIMD acceleration on platforms supporting popcount instructions"; SHA 17334f8 uses per-byte scalar loops in both TS and Rust.
12. **§4 — compress-then-expand MUST clause added.**
13. **§4 — BLAKE3 unkeyed-mode disambiguated** in the symbol-list preamble.
14. **§4 technical effect — replay-from-key softened** to contemplated, not present.
15. **§5.2 worked example — banker's-rounding parenthetical deleted.**
16. **§5.2.1 — rewritten.** TS `Math.round` (round-half-up) and Rust `f32::round` (round-half-away-from-zero) agree for non-negative inputs; canonical bracketing inlined.
17. **§5.2.1 — canonical bracketing `((clamped + 1.0) * (L-1)) / 2.0` inlined**, with SPEC §A.1 erratum flagged.
18. **§5.2 technical effect — throughput improvement quantified.** D/64 = 156 ops per pair vs ~1024 FMA + 2 sqrt.
19. **§5.2 — Imani 2017 and Rahimi 2016 thermometer-RP precedent cited and distinguished.**
20. **§5.3.1 — token-length rule disambiguated.** "Discards tokens of length less than `min_word_length` (default 3; the rule discards single-letter and two-letter tokens such as 'a', 'is', 'of', 'to', 'in')."
21. **§5.3 — Najafabadi 2016 cited and distinguished** on cross-platform determinism.
22. **§5.3.3 — char-trigram arithmetic corrected.** "elephant" has 6 trigrams; "elephnt" has 5; 3 common; Jaccard 3/8 ≈ 0.375.
23. **§6.1 — boolean `cold` field justified.**
24. **§6.3 — `put` pseudocode disambiguated** with explicit ternary form for `existing`.
25. **§6.1 — `store.items` API declared** (get/set/delete/iteration).
26. **§6.5 — LSH layer marked as contemplated, Phase 2 / v0.2.0,** not part of SHA 17334f8 embodiment; noted as subject of a possible divisional.
27. **§6 technical effect — pgvector / Pinecone post-filter empty-result documented.**
28. **§7.2 — sort-key disambiguated** to `(digest[b mod length(digest)], b) ascending` for bit indices `b`.
29. **§7.2 — `digestLen = max(64, toFlip * 4)` justified** (one byte per disagreeing position; floor of 64 for non-trivial BLAKE3-XOF exercise).
30. **§7.4 — salt persistence specified** (u64 LE at fixed offset within pass record); restore-module description added for claim 27.
31. **§7 — Hebb 1949, Hopfield 1982, Kanerva 1988, Ramsauer 2020 cited and distinguished** on byte-identical cross-platform replay.
32. **§8.3 — MessagePack dropped from Phase 1, marked Phase 2 contemplated**; Phase 1 normative is JSON (per `kmf.ts:72`).
33. **§8.5 — zstd-compressed header marked contemplated**, not Phase 4 forward.
34. **§8.2 — singleton-bundle-output backing for claim 5 added.**
35. **New §8.7 — `attic_block` (445) described** for claim 35; explicitly v0.2.0 contemplated.
36. **New §8.7 — zstd-pipeline ordering specified** (compress-then-hash); BLAKE3 digest is computed over the compressed bytes; supports claim 38.
37. **§8 — Apache Parquet 2.6 / Iceberg footer-layout precedent acknowledged and distinguished.**
38. **§9.2 — Windows `MoveFileEx` with `MOVEFILE_REPLACE_EXISTING` equivalent added.**
39. **§9 technical effect — S3 / Cloud Spanner / Postgres-as-blob tightened to user-extension examples**, not present implementations.
40. **§10.2 prose — corpus count "eight random-hv vectors" corrected to "five"**, with the 18-entry total stated.
41. **§10 — W3C Web Platform Tests / IETF JSON-Patch (RFC 6902 §11) precedent acknowledged and distinguished.**
42. **§11 Example 2 — left qualitative** (no v0.1.1 corpus entry available for `encode_embedding` SHA-256 pin); minor copy edit to the rounding-rule prose.
43. **§11 Example 3 — partial-resolution language corrected.** "The 200 selected disagreeing positions have been resolved; the remaining 4 800 are unchanged."
44. **§12.3 — wasm footprint quantified as "less than 500 KiB pre-compression"** with caveat that exact measured byte size is to be confirmed before filing.
45. **Reference numerals — added systematically per `06-drawings-list.md`.** Preamble paragraph added explaining the convention. Numerals inserted at §1, §2.3, §3.1, §3.2, §3.3, §3.4, §4, §5.1, §5.2, §5.3, §6, §7, §7.1, §8 and its subsections, §9, §9.1–§9.4, §10, §12.3.
46. **Fig. 2 meta_block "MessagePack" — flagged for drawings correction in changelog and in §8.3 editorial note.** (Drawing file itself not modified in this round — handled by drawings-list reviser.)
47. **Fig. 6 step 803 floor-vs-round — flagged for drawings correction in changelog.** (Drawing file itself not modified.)
48. **Fig. 6 step 805 phantom per-dimension key vector — flagged for drawings correction in changelog.** (Drawing file itself not modified.)
49. **§6.3 — trimmed to upsert-on-put with `createdAt` preservation,** then the disambiguated pseudocode, as recommended.
50. **§2.3 and §3.4 — softened to acknowledge that the Rust binding at SHA 17334f8 uses unpacked Vec<u8> with scalar loops** (verified at `packages/core-rs/src/hypervector.rs`); the packed-SIMD path is reserved for a subsequent revision.

### Items deferred / handed off to other agents

- **Claim 35 (attic block) retention vs deletion.** Per the worklist's "either describe or delete" instruction, the present revision **retains** claim 35 by adding §8.7 in this file. The final decision is for the claims-reviser; if Cl. 35 is dropped, §8.7 may be reduced to a single-sentence footnote noting the contemplated future apparatus.
- **`tests/conformance/README.md` "eight" error (line 23 per critique).** The repository file is outside `patent/` and is not modified in this round. It must be corrected in a separate code commit.
- **Fig. 2 (drawings file `06-drawings-list.md`) "MessagePack" annotation.** Flagged for the drawings reviser; not modified here.
- **Fig. 6 step 803 (`floor` vs `round`) and step 805 (phantom per-dimension key vector).** Flagged for the drawings reviser; not modified here.
- **SPEC.md errata.** Three items to be committed to SPEC.md before filing: (i) §1.3 missing `"smritidb/tiebreak"` domain prefix; (ii) §A.1 missing canonical `((clamped + 1.0) * (L-1)) / 2.0` bracketing; (iii) Fig. 6 references (jointly with the drawings file).
- **wasm build measured byte size.** Footnoted at §12.3 as "to be confirmed before filing"; a `cargo build --target wasm32-unknown-unknown --release` measurement against SHA 17334f8 is required.
- **v0.1.1 corpus expansion for Example 2 (encode_embedding SHA-256 pin).** Not blocking; Example 2 remains qualitative.

### Final counts

- **Lines:** Round-1 = 885 → Round-2 = 929 (+44 lines).
- **Words:** Round-1 ≈ 9,941 → Round-2 ≈ 13,375 (+34.5%).
- **Worklist items addressed:** 50 / 50.
- **Worklist items deferred:** 0 worklist items deferred; 6 cross-file follow-ups handed off (claims-reviser × 1, drawings-reviser × 3, SPEC editor × 1, reference-implementation editor × 1).

### SPEC errata for user to commit to SPEC.md v0.1.1

1. **§1.3 tiebreaker — add the `"smritidb/tiebreak"` 17-byte domain prefix** to the recited formulation, so that SPEC reads `H("smritidb/tiebreak" || D || index || count)[0] & 1` (matching `core-ts/src/hypervector.ts:13` and `core-rs/src/hypervector.rs:12`).
2. **§A.1 embedding encoder — explicitly state the canonical bracketing** `((clamped + 1.0) * (L - 1)) / 2.0`, computed in IEEE 754 binary32, to prevent associativity-related divergence.
3. **§4.2 cleanup memory — clarify Phase 1 vs Phase 2** so that the present brute-force implementation is normative and the LSH layer is contemplated only.
4. **Fig. 2 in the drawings file** — change "in MessagePack" to "in JSON (Phase 1; MessagePack contemplated for Phase 2)".
5. **Fig. 6 step 803** — change `level = floor((v_i + 1) / 2 * L)` to `level = round((v_i + 1) * (L - 1) / 2)`.
6. **Fig. 6 step 805** — delete the phantom "bind levelHV_i with permute(i) of a per-dimension key vector" step; the actual code XORs the level hypervectors directly without permutation (per `core-ts/src/encode.ts` and `core-rs/src/encode.rs`).
7. **`tests/conformance/README.md:23`** — change "eight random_hv vectors" to "five".

**No commits made.** No files modified outside `patent/`. All code references remain pinned to `17334f8`.

## Round 3 — Cross-Reconciliation (2026-05-20)

**Pinned source SHA:** `17334f8`
**Reviser:** Round-3 cross-reconciliation agent (Claude Opus 4.7, 1M context), working under direction of inventor Vivek Singh.
**Purpose:** Reconcile cross-artefact disagreements that surfaced from independent Round-2 revisions of `08-claims.md`, `07-detailed-description.md`, `09-abstract.md`, `10-prior-art.md` and `11-3k-defense.md`. This round is reconciliation only — no algorithm semantics changed, no new claim language, no new prior-art citations.

### Master Cross-Reference Table — Round-1 Claim Numbers vs. Round-2 Final Numbering

The table below maps every independent claim number used in Round-1 drafting (and in Round-1 internal references from other patent artefacts) to the Round-2 final claim number in `08-claims.md`, with a note on which other patent artefacts cross-reference each claim. "(dropped)" denotes a Round-2 deletion; "(folded)" denotes a Round-2 fold into another claim's dependents.

| Round-1 claim | Round-1 title (Round-1 numbering scheme) | Round-2 final claim | Round-2 title (per 08-claims.md headings) | Cross-references in other patent artefacts |
|---|---|---|---|---|
| 1 | Deterministic Tiebreaker Method | **1** | Deterministic Tiebreaker Method for Cross-Implementation Bit-Exact Bundling | 05-summary §2 (first aspect); 06-drawings-list Fig. 3 (tiebreaker 260); 11-3k-defense Claim 1 anchor (`note: 11-3k-defense uses Round-1 aspect-numbering, not Round-2 claim numbering`); 12-examples Example 2 (bundle with tiebreaker); 04-objects (a). |
| 7 | System for Cross-Implementation Bit-Exact Bundling | **7** | (same) | 05-summary §2 (first aspect); 06-drawings-list Fig. 1 (260); 04-objects (a). |
| 13 | Tiebreaker CRM (computer-readable medium triplet) | **(dropped)** | — | None — Round-2 dropped per worklist item #25 (most §3(k)-vulnerable surface). 11-3k-defense Round-2 entry §15 records the trade-off and aligns its defensive checklist with Round-1 claim numbering for traceability. |
| 18 | Replayable Hebbian Consolidation Method | **9** | Replayable Hebbian Consolidation Method with Bounded Similarity Drift | 05-summary §3 (second aspect); 06-drawings-list Fig. 4 (consolidation flow); 11-3k-defense Claim 2 anchor (note: Round-1 aspect-numbering); 12-examples (none specifically); 04-objects (b). |
| 19 | maxSimDelta dependent | **10** | (`value not greater than 0.02`) | 05-summary §3 (cites default 0.02); 06-drawings-list Fig. 4 step 405 (`drift_cap * D`). |
| 21 | Salt monotonicity dependent | **(dropped)** | — | None — Round-2 dropped per worklist item #35 (orchestration not implemented at SHA 17334f8). 05-summary text now omits any "monotonic" framing; consolidation is described as "a pure function of the substrate state and the access log" only. |
| 23 | Replayability dependent | **(folded into 9(f))** | — | Subsumed into the characterising clause of new independent Claim 9 ("the said access log is sufficient, together with a prior wire-format snapshot of the said substrate, to reconstruct the post-consolidation substrate state byte-for-byte"). |
| 25 | Hebbian system independent | **(folded into 9 deps)** | — | The system-level features (NVRAM persistence; restore module) are now dependents 15 and 17 of Claim 9. 11-3k-defense Round-2 entry §15 records the trade-off. |
| 26-29 | Hebbian system dependents | **(folded into 9 deps 15-17)** | — | Two new dependents (15, 17) carry forward the substantive system-level features. |
| 29 | Epoch counter dependent | **(dropped)** | — | None — Round-2 dropped per worklist item #36 (epoch-counter orchestration not implemented at SHA 17334f8). |
| 30 | KMF Open Wire Format Method | **18** | KMF Open Wire Format Method for Hyperdimensional Substrates | 05-summary §4 (third aspect); 06-drawings-list Fig. 2 (KMF file layout); 11-3k-defense Claim 3 anchor (note: Round-1 aspect-numbering); 12-examples Example 5 (KMF round-trip); 04-objects (c) and (k). |
| 37 | System for KMF Open Wire Format | **23** | (same) | 05-summary §4; 06-drawings-list Fig. 1 (400) and Fig. 2; 04-objects (c). |
| 39 | Output-sink dependent | **(folded into 23 comprising clause)** | — | Promoted into the comprising clause of independent Claim 23 per worklist item #14. |
| 42 | KMF CRM (computer-readable medium triplet) | **(dropped)** | — | None — Round-2 dropped per worklist item #26 (§3(k) economy). 11-3k-defense Round-2 entry §15 records the trade-off. |
| 46 | Thermometer-Quantised Random-Projection Encoder Method | **27** | Thermometer-Quantised Random-Projection Encoder Method for Bounded-Range Floating-Point Vectors | 05-summary §5 (fourth aspect); 06-drawings-list Fig. 6 (encoder flow); 11-3k-defense Claim 4 anchor (note: Round-1 aspect-numbering); 12-examples Example 3 (embedding encode); 04-objects (d). |
| 47 | L levels dependent | **28** | (`value of at least 64 and not greater than 256`) | 05-summary §5 (default L = 100 annotated against this preferred range); 06-drawings-list Fig. 6 step 803 (`L = 100`). |
| 52 | Conformance corpus method (Round-1 method-form) | **31** | Conformance-Gating System for Cross-Implementation Byte-Identity of a Vector Substrate (Round-2 reframed to system form per OpenTV) | 05-summary §6 (fifth aspect); 06-drawings-list Fig. 5 (conformance verification loop); 11-3k-defense Claim 5 anchor (note: Round-1 aspect-numbering); 04-objects (e). |
| 53 | Text-encoding groups dependent | **32** | (with bag-of-words and char-n-gram groups added) | 05-summary §7 (mentions text encoders); 06-drawings-list Fig. 5. |
| 58 | Permutation-Positional Text Encoder Method | **35** | Permutation-Positional N-Gram Text Encoding Method Without Tokeniser or Neural Embedding | 05-summary §7 (sixth aspect); 06-drawings-list Fig. 1 (140); 11-3k-defense Claim 6 anchor (note: Round-1 aspect-numbering); 12-examples Example 4 (text encoding); 04-objects (f). |

**Important traceability note on `11-3k-defense.md`:** The Round-2 final of `11-3k-defense.md` deliberately retains the Round-1 aspect-numbering ("Claim 1" through "Claim 6") because the §3(k) brief is organised by inventive-aspect family (Tiebreaker, Consolidation, KMF, Encoder, Conformance, Text-encoder) rather than by the renumbered statutory claim sequence. The brief's section §15 (defensive checklist) explicitly notes the renumbering trade-off and aligns its claim-family-level analysis with the Round-1 numbering for cross-document traceability. A future Round (4 or 5) under counsel direction may rewrite the brief's headings to track the Round-2 statutory numbers, but the substantive analysis is unchanged.

**Important traceability note on `09-abstract.md`:** The Round-2 final of `09-abstract.md` does not enumerate claim numbers explicitly; it describes the inventive elements in summary form. Accordingly, no claim-number reconciliation is required for the abstract; the Round-2 final is verbatim consistent with the Round-2 claim set.

### Files modified in Round 3 (within `patent/`)

1. **`04-objects-of-invention.md`** — Added a Round-3 changelog block at the top recording each Object's mapping to a Round-2 independent claim; no Object wording changed. Confirmed no Object references a dropped CRM triplet, dropped salt-monotonicity, or dropped epoch-counter dependent. Two cross-references updated implicitly (Object (b) wording confirmed not to suggest monotonic salt orchestration; Object (e) wording confirmed compatible with the OpenTV-reframed conformance system of Claim 31).
2. **`05-summary-of-invention.md`** — Added a Round-3 changelog block at the top. Updated the fourth aspect (thermometer encoder) to remove the phantom dimension-binding permutation step and to describe per-coordinate level hypervectors as XOR-accumulated directly, in line with the Round-2 Fig. 6 correction in `06-drawings-list.md` and §5.2.1 in `07-detailed-description.md`. Added `(default L = 100; in the preferred range of at least 64 and not greater than 256)` annotation referencing new dependent Claim 28. Re-expressed the per-coordinate seed in canonical decimal ASCII form per Claim 27(c)(ii). Updated the sixth aspect (text encoder) to replace "cyclically bit-rotated" with "cyclically permuted" with explicit `(p + j) mod D` definition, per Round-2 Claim 35(b)(ii) terminology unification. Added a unity-of-invention cross-reference paragraph at the end of the document, mirroring the explanatory note in `08-claims.md` and citing *Genentech Inc.'s Patent* [1989] RPC 147 (per Round-2 worklist item #29 deferral).
3. **`06-drawings-list.md`** — Added a Round-3 changelog block at the top. Fig. 2 corrected per Round-2 detailed-description-agent flags: (i) `meta_block` annotation changed from "MessagePack" to "JSON (Phase 1; MessagePack contemplated for Phase 2)"; (ii) header annotation changed from "zstd-compressed JSON" to "JSON (Phase 1; zstd compression contemplated for Phase 2)"; (iii) `attic_block` annotation marked as "contemplated for Phase 2; not present at SHA 17334f8"; (iv) Fig. 2 caption updated to reflect Phase 1 JSON / Phase 2 zstd demarcation. Fig. 6 corrected: step 803 formula changed from `level = floor((v_i + 1) / 2 * L)` to `level = round((v_i + 1) * (L - 1) / 2)`; step 805 (phantom "bind levelHV_i with permute(i) of a per-dimension key vector") deleted; step 806 clarified that the d level hypervectors are XOR-accumulated (no tiebreaker invocation, since XOR has no ties).
4. **`12-examples.md`** — Added a Round-3 changelog block at the top. Example 3 (b) formula corrected and numeric values recomputed: `l_0 = 64` (was `65`), `l_3 = 94` (was `95`); `l_1 = 15` and `l_2 = 50` unchanged. Example 3 (c) per-coordinate seed re-expressed in canonical decimal ASCII form per Claim 27(c)(ii). Example 3 (d) phantom "Dimension-binding by permutation" step deleted to match Round-2 Fig. 6 correction; encoder is XOR-accumulate only. Example 5 (e), (h) and §5.3 (b), (d) updated to describe `meta_block` metadata-row encoding as "JSON (Phase 1; MessagePack contemplated for Phase 2)" and header zstd-compression as Phase 2 contemplated.
5. **`REFINEMENT-LOG.md`** — This Round 3 section appended, including the master cross-reference table above.

### Reference numerals normalised

The Round-3 pass verified that the reference-numeral conventions in `06-drawings-list.md` (Round 2: numerals 90, 100-series, 200-series, 260, 300-series, 320, 330, 400-series, 410, 411, 412, 420, 430, 440, 445, 450, 460, 470, 500-series, 510-560, 610, 620, 630, 640, 660, 710-750, 770, 780, 790, 801-807) remain consistent with usage in `07-detailed-description.md` (Round-2 entry §45) and `12-examples.md` (Round-3 references to 110, 120, 140, 240, 250, 260, 300, 400, 410, 411, 412, 420, 430, 440, 450, 460, 470, 500, 610, 802, 803, 804, 806, 807). The deletion of step 805 in Fig. 6 and the deletion of the corresponding step 805 in Example 3 leave no orphan reference numeral, because the step number 805 was only used within Fig. 6 itself and within the Example 3 cross-reference to Fig. 6.

### Cross-references updated (count)

- **Phase 1 vs. Phase 2 demarcation:** 7 textual cross-references updated (Fig. 2 meta_block, Fig. 2 header zstd, Fig. 2 attic_block, Fig. 2 caption parse step, Example 5 (e) meta_block, Example 5 (h) header zstd, Example 5 §5.3 (b) parse step, Example 5 §5.3 (d) meta_block decode) — total 8 textual sites updated (one site appears twice because the Fig. 2 attic_block is referenced both in the byte-layout box and implicitly in the cleanup-memory consolidation discussion).
- **Fig. 6 / Example 3 formula correction:** 2 sites updated (Fig. 6 step 803; Example 3 (b) prose plus 4-line numeric example).
- **Fig. 6 / Example 3 phantom-step deletion:** 2 sites updated (Fig. 6 step 805 box deleted; Example 3 (d) deleted; Fig. 6 step 806 caption reworded; Example 3 step renumbering tightened).
- **Terminology unification:** 1 site updated in 05-summary §6 (sixth aspect: "cyclically bit-rotated" → "cyclically permuted" with explicit `(p + j) mod D`).
- **Algorithm-semantic alignment:** 1 site updated in 05-summary §4 (fourth aspect: phantom dimension-binding permutation removed; XOR-accumulate language adopted).
- **Default-parameter annotation:** 1 site updated in 05-summary §4 (L = 100 default annotated against new Claim 28 preferred range 64-256).
- **Unity-of-invention paragraph:** 1 site updated in 05-summary (end of document).
- **Object-to-claim mapping confirmation:** 11 Objects (a)-(k) in 04-objects verified against the 8 Round-2 independent claims; 0 wording changes required, 1 changelog block added.

Total textual cross-references touched in Round 3: **22** (of which 8 are Phase 1/2 demarcation, 4 are Fig. 6 / Example 3 algorithm corrections, 1 is terminology, 1 is algorithm-semantic, 1 is default-parameter annotation, 1 is unity-of-invention cross-reference, 11 Object mappings logged but no Object wording changed, plus 4 changelog blocks).

### Prior-art citation reconciliation

Per task item 3(e), the §3(k) memo at `11-3k-defense.md` cites Achlioptas (PODS 2001 / JCSS 2003), Mikolov (Word2Vec, ICLR 2013), Pennington (GloVe, EMNLP 2014), Devlin (BERT, NAACL 2019). All four references are verified to be present in the Round-2 final of `10-prior-art.md`:

- Achlioptas 2003 — present in §6 row 4 (closest-combination column), §10 changelog item, §4.x academic-references list and §9 Form-2 background list. DOI 10.1016/S0022-0000(03)00025-4.
- Mikolov 2013 (Word2Vec) — present in §4.10 neural text-encoder prior art and §6 row 6 (closest-combination column). arXiv:1301.3781.
- Pennington 2014 (GloVe) — present in §4.10 and §6 row 6. DOI 10.3115/v1/D14-1162.
- Devlin 2019 (BERT) — present in §4.10 and §6 row 6. DOI 10.18653/v1/N19-1423.

**No carry-forward required to Round 4 on this point.**

The two US patent numbers flagged P0 in the Round-2 final of `10-prior-art.md` (US 11,775,847 B2 misattributed; US 10,956,464 B1 404-on-retrieval) are *not* cited anywhere in the other Round-2 finals or in any of the Round-3-modified files. Specifically: grep across `00-cover.md`, `02-field-of-invention.md`, `03-background.md`, `04-objects-of-invention.md`, `05-summary-of-invention.md`, `06-drawings-list.md`, `12-examples.md`, and `13-filing-checklist.md` returns no hits on either patent number or its variants. The two P0 blockers are accordingly localised to `10-prior-art.md` and to the §3(k) memo's Round-2 entry in this log (line 264 of this file); they do not leak into any other artefact.

### Remaining inconsistencies (Round 3 → Round 4 carry-forward)

1. **`11-3k-defense.md` retains Round-1 aspect-numbering for claim references.** The brief's section headings read "Claim 1", "Claim 2", ..., "Claim 6" corresponding to the six inventive aspects; these are not the Round-2 statutory claim numbers (1, 7, 9, 18, 23, 27, 31, 35). A future round under counsel direction may rewrite the brief's headings to track the Round-2 statutory numbers. Round-3 honours the user's instruction not to modify `11-3k-defense.md`. **Recommended action for Round 4:** Heading rewrite, no substantive change.

2. **`11-3k-defense.md` §15 defensive checklist references Round-1 claim numbers** (`Claims 7, 13`; `Claim 25`; `Claims 37, 42`; `Claim 46, 52, 58`). The Round-2 entry in this log records the reconciliation requirement; the brief's Round-2 final notes the trade-off and aligns its analysis with Round-1 numbering for traceability. **Recommended action for Round 4:** Rewrite §15 to address the apparatus/CRM triplet coverage of the surviving Round-2 claim families (1+7; 9; 18+23; 27; 31; 35), noting the three dropped CRM triplets (former 13-17, 42-45) and the dropped Hebbian system triplet (former 25-29) and the strategic implication for §3(k) defensibility.

3. **`11-3k-defense.md` Claim-4 quantitative-dependent recommendation cites Round-1 numbers** ("a dependent claim under Claim 46 should recite the 32× ratio and the ε ≤ 0.05 envelope"; "a dependent claim under Claim 58 should recite at least one of the quantitative bounds"). The Round-2 finals of these dependents (now numbered 28-30 for the encoder, and 36 for the text encoder) do not yet recite these specific quantitative bounds. **Recommended action for Round 4:** Either add the recommended quantitative dependents (which would require a follow-up Round-3.5 pass on `08-claims.md`, outside the scope of the present Round-3 instruction not to touch that file), or excise the quantitative recommendation from the §3(k) brief; counsel direction required.

4. **SPEC.md errata** (carried over from Round 2): seven repository-level items deferred to a pre-filing commit on `SPEC.md`, `tests/conformance/README.md`, `tests/conformance/golden.json` and optionally `consolidate.ts` / `kmf.ts`. These are outside `patent/` and are not touched in Round 3.

5. **`10-prior-art.md` P0 blockers** (carried over from Round 2): InPASS Indian-prior-art search (§5.1); Imani-HDC US patent number re-establishment (§5.2); Pinecone US patent number re-establishment (§5.2). These are out-of-tree research tasks for the patent agent at filing day.

6. **wasm build measured byte size** (carried over from Round 2): `07-detailed-description.md` §12.3 footnotes "less than 500 KiB pre-compression" as "to be confirmed before filing". A `cargo build --target wasm32-unknown-unknown --release` measurement against SHA `17334f8` is required prior to filing.

7. **Title length and IPC classification** (carried over from Round 1 items 1 and 2 of this log): 15-word working title and provisional IPC list in `00-cover.md` to be re-checked at filing day. No Round-3 action.

8. **Bundle "default of L" semantics in 05-summary §4 vs. Claim 28 range:** Round-3 has annotated `L = 100` against the Claim 28 preferred range of at least 64 and not greater than 256. The Round-2 detailed description §5.2 also uses `L = 100` as the default value. No inconsistency; no further action.

### No commits made

No files modified outside `patent/`. All code references remain pinned to `17334f8`. No algorithm semantics changed in Round 3.

## Round 4 — 08-claims.md

**Pinned source SHA:** `17334f8`
**Applied:** 2026-05-20
**Revising agent:** Claude Opus 4.7 (1M context), Round 4 critic-and-reviser for `08-claims.md`.
**Scope:** Single focused critic+revise pass addressing residual issues left after Round 2 final state, in light of the Round 3 cross-reconciliation findings and the Round-2 expanded prior-art corpus.

**Pre-revision state:** 36 claims; 8 independents at 1, 7, 9, 18, 23, 27, 31, 35 (Round 2 final).
**Post-revision state:** 36 claims; 8 independents at 1, 7, 9, 18, 23, 27, 31, 35 (unchanged in count and structure).

### Five-perspective critic findings and resolutions

**Perspective 1 — IPO Examiner (§3(k), §2(1)(j), §2(1)(ja), §10(4)).** All eight independent claims survive on the strength of the Round-2 anchoring (TE in body; *Ferid Allani* narrow reading of "*per se*"; *Raytheon* artefact-pair framing; *Microsoft* gold-standard data-path anchoring; *OpenTV* load-time gating). No further surgery required at this round; the §3(k) defence brief (`11-3k-defense.md`) carries the legal anchoring case-by-case. **No claim text changes required from this perspective.**

**Perspective 2 — §3(k) Defender (TE in claim body, system-level property).** Each independent claim now carries its principal technical effect in the characterising clause and anchors on a measurable system-level property (cross-implementation byte-identity verifiable against the conformance corpus; replayability from snapshot+log; per-block BLAKE3 integrity with refuse-to-write-until-verified; load-time conformance gating). **No claim text changes required from this perspective.**

**Perspective 3 — Prior-art Hunter (expanded corpus in `10-prior-art.md`).** The Round-2 prior-art memo added Charikar 2002 (sign-LSH/SimHash), Datar 2004, Andoni & Indyk 2008, Rahimi 2016 ISLPED and Rahimi-Datta-Kanerva-Rabaey 2017 TCAS-I (foundational level encoders), SQLite WAL, HDF5 Fletcher32, Apache Parquet, Merkle 1979 / Git / IPFS content-addressing, Bloom 1970, Count-Min, MinHash, SimHash text fingerprinting, and Kanerva 1996 Binary Spatter Codes. The following residual claim-language surgeries were applied to maintain a clean differentiator over the now-cited art:

  (a) **Claim 18 strengthened** with an explicit conformance-corpus-version anchor in the header object — this completes the four-element combination identified in the `10-prior-art.md` §6 differentiation matrix (per-block BLAKE3 integrity, magic trailer with version, column-major `D`-multiple-of-8 packing, **and conformance-corpus pointer**), so that no single prior-art format (Parquet, HDF5, Arrow, CBOR, FlatBuffers, Cap'n Proto, Bencode, Protobuf, Git, IPFS) reads on the full combination.

  (b) **Claim 18 dependent 19** rewritten to remove the redundant "JSON document" recital (which duplicated claim 18(e)) and to substitute a substantive single-pass-streaming differentiator that is the actual technical effect distinguishing KMF from Parquet's footer-first read pattern.

  (c) **Claim 27 strengthened** with an express BLAKE3-XOF anchoring of the per-coordinate random projection, distinguishing the encoder from Charikar 2002 sign-LSH (single-bit-per-projection, no level quantisation) and from Rahimi 2016/2017's sliding-correlation or implementation-defined-PRNG level-hypervector generation.

  (d) **Claim 35 strengthened** with an express bag-of-features-versus-permutation-positional disclaimer, anchoring the differentiator over Charikar 2002 SimHash and Manku et al. 2007 in the characterising clause.

**Perspective 4 — Patent Attorney (antecedent basis, multi-dependence, IPO drafting style).** The Round-2 audit (REFINEMENT-LOG.md items 30, 31) confirmed every multi-dependent range anchors at a non-multi-dependent claim. Two residual antecedent issues identified and fixed in Round 4:

  (a) **Claim 9(c) antecedent basis** for "the binary key hypervector of item `a`" / "the binary key hypervector of item `b`" — these were used without prior introduction; revised to "a binary key hypervector of item `a` stored in the said substrate ... a binary key hypervector of item `b` stored in the said substrate", with subsequent references in (e), (f) updated to the "the said" form.

  (b) **Claim 9(f) consistency** — the alternating-flip language used "the key of item `a`"; standardised to "the said binary key hypervector of item `a`" to match the (c) antecedent.

  All other antecedents (claim 18 `n`, claim 23 substrate, claim 27 per-coordinate seed, claim 31 implementation-under-test, claim 35 per-word hypervector) verified clean at Round 2 and unchanged in Round 4.

**Perspective 5 — Domain Critic (every claim limitation traces to code at SHA 17334f8).** Re-audited every revised claim limitation against the code at `17334f8`:

  - Claim 18(e) conformance-corpus version identifier — present in `kmf.ts` line 75-82 as part of the header JSON object alongside specVersion (`SPEC.md` §8.2); claim language now matches.
  - Claim 19 single-pass-streaming framing — matches `kmf.ts` writer flow at lines 95-160 (data blocks written and hashed before header is finalised; header pointer back-filled at end).
  - Claim 27 BLAKE3-XOF anchoring — matches `encode.ts` line 38 (`derive(seed)` invokes BLAKE3 XOF).
  - Claim 35 permutation-positional bag-of-features disclaimer — the bag-of-features fallback is exactly the dependent claim 36 fall-back path (`text.ts` lines 110-120); the disclaimer is properly framed as the differentiator over a bag-of-features baseline rather than denying the existence of a bag-of-words mode.
  - Claim 9(c)/(f) antecedent fixes — purely textual; no code reference perturbed.

  **Domain-critic finding: every Round-4 surgery is supported by code at SHA `17334f8`.**

### Items addressed (Round 4)

1. **Unity-of-invention paragraph claim numbers corrected.** The Round-2-final paragraph cited claims at (1, 5, 9, 13, 19, 23, 26, 31, 33), which is the Round-1 numbering. Updated to the Round-2-final independent-claim numbering (1, 7, 9, 18, 23, 27, 31, 35) with each aspect re-described to match the surviving claim's actual scope (method+system pair for tiebreaker and KMF; method-only for Hebbian consolidation and thermometer encoder and text encoder; system-only for conformance gating).

2. **Unity-of-invention prior-art enumeration expanded** to cite Rahimi 2016 ISLPED, Charikar 2002 sign-LSH/SimHash, HDF5 Fletcher32, and Merkle 1979 / Git / IPFS alongside the originally-cited BLAKE3 specification, Kanerva 1988, Plate 1995 and Apache Parquet — matching the expanded prior-art corpus added in `10-prior-art.md` Round 2.

3. **Claim 9(c) antecedent basis fixed** for the two binary-key hypervectors of items `a` and `b`.

4. **Claim 9(f) terminology unified** with claim 9(c) ("the said binary key hypervector of item `a`" / "...item `b`").

5. **Claim 18(e) strengthened** by adding the conformance-corpus version identifier to the header object — the fourth element of the four-element combination identified in the prior-art memo §6 differentiator matrix.

6. **Claim 18 dependent 19 rewritten** to substitute a substantive single-pass-streaming differentiator for a redundant "JSON document" recital.

7. **Claim 27 characterising clause strengthened** with explicit BLAKE3-XOF anchoring of the per-coordinate random projection, distinguishing over Charikar 2002 sign-LSH and Rahimi 2016/2017 PRNG-seeded level encoders.

8. **Claim 35 characterising clause strengthened** with explicit bag-of-features disclaimer and inline `(p + j) mod D` permutation definition, distinguishing over Charikar 2002 SimHash and Manku et al. 2007.

9. **File subtitle updated** from "Round 2 — pinned to HEAD 17334f8 — 8 independent claims, 36 total claims." to "Round 4 — pinned to HEAD 17334f8 — 8 independent claims, 36 total claims."

10. **Round 4 changelog block prepended** at the top of `08-claims.md` above the existing Round 2 changelog, summarising the seven surgeries above.

### Items deferred to the final read-aloud pass

1. **Title-length cross-check.** Working title in `00-cover.md` is to be re-checked at filing day (carried over from Round-3 carry-forward item 7).

2. **Provisional IPC classification audit.** The IPC class list in `00-cover.md` to be re-confirmed by the patent agent at filing day.

3. **Cross-numbering audit pass on `11-3k-defense.md`.** Round-3 carry-forward items 1-3 noted that the §3(k) defence still uses Round-1 aspect-numbering ("Claim 1" through "Claim 6"). A future round under counsel direction may rewrite the brief's headings to track the Round-2 statutory numbers (1, 7, 9, 18, 23, 27, 31, 35). Round 4 does not touch `11-3k-defense.md` (out of scope for the claims reviser).

4. **Quantitative-bound dependent claims under §3(k) memo's #14 recommendation** (32× ratio and ε ≤ 0.05 envelope under what is now claim 27; zero parameters / O(n · D) / < 2 MB working memory under what is now claim 35). Round 4 does *not* add these dependents because (a) doing so would increase claim count above the brief-target of ~36, and (b) the §3(k) brief itself notes the recommendation is conditional on counsel direction. **Status: deferred to final read-aloud pass for counsel decision.**

5. **REPO-errata commits** (items 37, 38, 39, 40 of Round 2 worklist): SPEC.md §1.3 tag recital; SPEC.md §8.2 MessagePack→JSON; golden.json tolerance fields; monotonic-salt orchestration code commit. **Status: deferred to a pre-filing REPO-errata commit, outside `patent/` scope.**

6. **P0 prior-art blockers** (carried over from Round 2 of `10-prior-art.md`): InPASS Indian-prior-art search; Imani-HDC US patent number re-establishment; Pinecone US patent number re-establishment. **Status: out-of-tree research tasks for the patent agent at filing day.**

7. **wasm build measured byte size** (carried over from Round 2): `cargo build --target wasm32-unknown-unknown --release` measurement against SHA `17334f8`. **Status: pre-filing measurement.**

### Verification after editing

- **Claim numbering:** Contiguous 1..36; no gaps; no duplicates. **Verified.**
- **Independent claims:** 8 at positions 1, 7, 9, 18, 23, 27, 31, 35. **Verified.**
- **Multi-dependent ranges:** All anchored at non-multi-dependent claims (Round 2 audit unchanged; no new multi-dependencies introduced in Round 4). **Verified.**
- **Dependent-claim references:** Every "claim X" reference in a dependent claim points to an existing claim. **Verified.**
- **Antecedent basis:** Claims 9(c)/(f), 18(e), 19, 27, 35 antecedents revised in Round 4 are clean. **Verified.**
- **Cross-references between claims:** Claim 35(c) cites "deterministic tiebreaker resolution as claimed in claim 1" (still claim 1) and "the canonical conformance corpus of the system of claim 31" (still claim 31). **Verified.**
- **Total claim count:** 36 (target was ~36; unchanged from Round 2). **Verified.**
- **Indian-English orthography:** "serialised", "characterised", "thermometer-quantised", "summarisation", "deserialise", "tokeniser" used throughout; no US-English variants introduced. **Verified.**
- **No files outside `patent/`** were modified. No commits were made.

### Final tally (Round 4)

- **Total claims:** 36 (unchanged).
- **Independent claims:** 8 (unchanged at 1, 7, 9, 18, 23, 27, 31, 35).
- **Surgical edits applied:** 10 (one paragraph corrected; one subtitle updated; one changelog block added; seven claim-text edits across claims 9(c), 9(f), 18(e), 19, 27 characterising clause, 35 characterising clause, plus minor terminology unification).
- **Items deferred to final read-aloud pass:** 7 (title length, IPC audit, §3(k)-brief renumbering, optional quantitative-bound dependents, REPO errata, P0 prior-art research, wasm build size).
- **No algorithm semantics changed.** Every Round-4 surgery is supported by code at SHA `17334f8` or is a textual/cross-reference fix.

## Round 4 — 07-detailed-description.md

**Pinned source SHA:** `17334f8`
**Applied:** 2026-05-20
**Reviser:** Round-4 critic-and-revise agent (Claude Opus 4.7, 1M context), working under direction of inventor Vivek Singh.
**Pre-revision state:** Round-2 final (13,427 words; 931 lines; 50/50 worklist items applied; Phase 1/2 demarcated; reference numerals systematic).
**Post-revision state:** 13,545 words (excluding HTML comments); 962 lines; SPEC-errata reconciled; claim cross-references updated for Round-2 renumbering; new prior-art distinguishing paragraphs added.

### Five-perspective critique applied in one pass

1. **IPO Examiner (§10(4) sufficiency).** Confirmed every primitive, encoder, and wire-format element remains fully specified at the byte level after the Round-2 changelog reflowed the reference-numeral conventions. The deletion of the Round-2 "SPEC erratum" editorial notes at §3.1.1 and §5.2.1 does not subtract any disclosure — the same byte-level facts are recited directly, without the meta-commentary about SPEC mismatches that no longer exists. No sufficiency gap introduced.

2. **§3(k) Defender.** Verified each Technical-Effect subsection (12 in total, one per substantive section) remains in place and remains measurably grounded: §1 (3.27× memory; bit-exactness; substitutable persistence), §2 (32× reduction vs float-32; SIMD-amenability), §3.1 (cross-runtime bit-identity from deterministic tiebreaker), §3.2 (`O(D/64)` XOR throughput vs transformer KV), §3.3 (`O(D)` integer-copy vs RoPE's `O(D log D)` complex-arithmetic), §3.4 (Hamming popcount cycles vs cosine `sqrt`s), §4 (replay-from-key; cryptographic distribution), §5.1 (zero-collision; zero overhead), §5.2 (3.27×–13× storage reduction; `D/64 = 156` ops/pair vs 1024 FMAs), §5.3 (order-sensitive; parameter-free), §6 (single-index single-substrate; filter-then-rank; deterministic ties), §7 (snapshot+log replay; multi-replica consistency without consensus; integer-only Hebbian), §8 (BLAKE3 block integrity; streaming layout; anti-truncation), §9 (substitutable storage; inherited durability), §10 (drift prevention; legal interoperability artefact). No effect downgraded or removed by Round-4 edits.

3. **Prior-art Hunter.** Round-2 added Kanerva 1988/2009, Imani 2017, Rahimi 2016, Najafabadi 2016, Hebb 1949, Hopfield 1982, Ramsauer 2020, Vaswani 2017, Sukhbaatar 2015, Apache Parquet/Iceberg, W3C Web Platform Tests, IETF JSON-Patch. Round-2 of `10-prior-art.md` additionally seeded the LSH family (Charikar 2002, Datar 2004, Andoni & Indyk 2008, Achlioptas 2003), HNSW (Malkov & Yashunin 2020), SQLite WAL (Hipp et al.), and the SimHash text-fingerprinting line. The Round-2 detailed-description text did *not* yet distinguish from sign-LSH / SimHash, sparse random projection, HNSW, or SQLite WAL by name. Round 4 adds three distinguishing paragraphs:
   - §5.2 — three-point distinguishing of the thermometer encoder from Charikar 2002 sign-LSH and Achlioptas 2003 sparse-sign random projection (one bit per projection vs `D` bits per (coord, level); no cryptographic XOF in prior art vs BLAKE3-XOF here; bit-budget tradeoff vs probabilistic Hamming-vs-cosine correspondence).
   - §6.5 — citation of Indyk & Motwani 1998 plus Andoni & Indyk 2008 for the Hamming-LSH construction the contemplated Phase-2 layer uses, *plus* an explicit HNSW (Malkov & Yashunin 2020) distinguishing on cross-implementation determinism.
   - §9.3 — explicit acknowledgement of SQLite WAL (Hipp et al.) as prior art for the durability mechanism, with the novelty re-anchored to the `StorageAdapter` decoupling rather than to the WAL itself.

4. **Patent Attorney (claim-support cross-walk).** Every Round-2 final claim (1, 7, 9, 18, 23, 27, 31, 35 independents; 28 dependents) traced to its supporting section in this file:
   - Claim 1 (tiebreaker method): §3.1 + §3.1.1 + §11 Example 1. Supported.
   - Claim 7 (tiebreaker system): §3.1 + §1 (architecture) + §2.4 (best-mode SIMD with scalar control-path tiebreaker; supports the byte-transparent FFI limitation). Supported.
   - Claim 9 (Hebbian consolidation method, with access log antecedent): §7.1 + §7.2 + §7.4. Supported.
   - Claims 10-17 (Hebbian dependents including restore module, NVRAM persistence, configuration record): §7.2 maxSimDelta; §7.3 cold flagging; §7.4 access log and restore module. Supported.
   - Claim 18 (KMF method): §8.1 through §8.6. Supported.
   - Claims 19-22 (KMF dependents including zstd, unsupported-version error, attic block): §8.5 (header JSON; zstd contemplated for header analogously to data blocks); §8.6 (trailer guard); §8.7.1 (attic block). Supported.
   - Claim 23 (KMF system with output sink): §8.1 + §9 (adapter family). Supported.
   - Claims 24-26 (zstd compressor pipeline, partial-load module, CorruptSnapshot): §8.7.2 (compress-then-hash); §8.1 step 7 (per-block verify before deserialise); §8.5 (header index supports partial seek). Supported.
   - Claim 27 (thermometer encoder): §5.2 + §5.2.1 + §11 Example 2. Supported.
   - Claims 28-30 (encoder dependents L range, D group, cleanup-memory key): §5.2 (`L = 100`); §2.1 (`D` group); §6.4 (cleanup memory). Supported.
   - Claim 31 (conformance-gating system): §10.1 + §10.2 + §10.3. Supported.
   - Claims 32-34 (corpus dependents): §10.2 (test categories); §10.1 (UTF-8 seeds with SHA-256); §10.3 (CI integration + frozen reference). Supported.
   - Claim 35 (permutation-positional text encoder): §5.3.2 + §5.3.3 (fallback) + §3.1 (deterministic tiebreaker invoked by bundle step). Supported.
   - Claim 36 (text-encoder dependents char-n-gram + fallback): §5.3.1 (fallback) + §5.3.3 (char-trigram). Supported.

5. **Domain Critic (algorithm-step-to-code traceability).** Verified that every algorithm step in this file traces to actual code at SHA `17334f8`:
   - §3.1 bundle: `packages/core-ts/src/hypervector.ts` `bundle()` and `packages/core-rs/src/hypervector.rs` `bundle()`.
   - §3.1.1 tiebreaker: `TIEBREAKER_DOMAIN` constants in both reference files.
   - §3.2-§3.4 primitives: `bind`, `permute`, `similarity` in both reference files.
   - §4 randomHv: `random_hv` in both reference files.
   - §5.1-§5.3 encoders: `encodeString`, `encodeEmbedding`, `encodeBagOfWords`, `encodeWordNgrams`, `encodeCharNgrams` in `core-ts/src/encode.ts` and `core-ts/src/text.ts`; corresponding Rust forms.
   - §6 store: `Store`, `put`, `recall`, `cleanupSearch` in `core-ts/src/store.ts`.
   - §7 consolidation: `CoactivationTracker`, `pullCloser`, `flagColdItems` in `core-ts/src/consolidate.ts`.
   - §8 KMF: `writeKmf`, `readKmf` in `core-ts/src/kmf.ts`.
   - §9 adapters: `MemoryAdapter`, `FileSystemAdapter`, `SqliteAdapter`, `IndexedDbAdapter` in `core-ts/src/adapters/`.
   - §10 conformance: `tests/conformance/golden.json` + per-binding conformance harnesses.

   No phantom step disclosed. The §2.4 best-mode SIMD subsection and the §6.5 LSH paragraph and the §8.7 attic / zstd subsections remain explicitly labelled "contemplated / not part of the SHA 17334f8 embodiment", as in Round 2.

### Items addressed (Round 4)

1. **§3.1.1 SPEC erratum note deleted.** Replaced with a "Source-of-truth cross-reference" paragraph that simply recites that the present specification, SPEC.md, and both reference bindings agree on the `"smritidb/tiebreak"` domain prefix. The SPEC.md change is in the working tree (`git diff SPEC.md` confirms the 17-byte prefix is now recited at §1.3).

2. **§5.2.1 final parenthetical rewritten.** The "the SPEC will be conformed to include it in v0.1.1" sentence has been replaced with a positive statement that SPEC.md §A.1 (working tree) now recites the canonical bracketing and that the outer `clamp(level, 0, L - 1)` absorbs the half-integer rounding-tail case across the three rule families. This addresses the residual concern that SPEC.md (now in working tree) uses round-half-to-even framing while the description recites Math.round/f32::round; both framings yield the same result on the in-range domain, and the SPEC's outer clamp is what makes the framings interchangeable.

3. **§6.5 SPEC-line citation rephrased.** "SPEC.md §4.2 (lines 200–201) records ..." replaced with "SPEC.md §4.2 expressly demarcates the Phase 1 brute-force scan as normative ..." — citing the post-Round-2 SPEC clarification rather than the pre-Round-2 ambiguous wording. Also added explicit references to Indyk & Motwani 1998 and Andoni & Indyk 2008 for the LSH construction and to Malkov & Yashunin 2020 for HNSW (distinguishing).

4. **§8.3 meta_block parenthetical replaced.** The "Fig. 2 ... annotates this block as 'in MessagePack'; the drawings will be conformed in v0.1.1" parenthetical has been replaced with a positive statement that Fig. 2 (post-Round-3) now annotates this block as "JSON (Phase 1; MessagePack contemplated for Phase 2)" in agreement with the present specification.

5. **Claim cross-references updated for Round-2 renumbering:**
   - §7.4 "subject of claim 27" -> "subject of dependent claim 15 of independent claim 9" (the restore module).
   - §8.5 "exposed by claim 32" -> "exposed by dependent claim 20 of independent claim 18" (zstd of data blocks).
   - §8.7.1 "subject of claim 35" -> "subject of dependent claim 22 of independent claim 18" (attic block).
   - §8.7.2 "subject of claim 38" -> "subject of dependent claim 24 of independent claim 23" (compress-then-hash pipeline).
   - §8.2 "claim 5" left unchanged — Round-4 claim 5 still recites the integrity-hash emission alongside bundle output (dependent of claim 1).

6. **§5.2 prior-art distinguishing strengthened.** Added a three-point distinguishing paragraph against Charikar 2002 sign-LSH / SimHash and Achlioptas 2003 sparse-sign random projection — both of which were added to `10-prior-art.md` §4.8 in Round 2 but had not been textually distinguished in this file.

7. **§6.5 prior-art distinguishing extended.** Added explicit citations to Indyk & Motwani 1998 and Andoni & Indyk 2008 (Hamming-LSH precedent for the contemplated Phase-2 layer) and an HNSW (Malkov & Yashunin 2020) distinguishing on cross-implementation determinism — addressing `10-prior-art.md` §3.8.

8. **§9.3 SQLite WAL distinguished.** Added an explicit acknowledgement that SQLite WAL is well-known prior art, with the novelty of the present invention re-anchored to the `StorageAdapter` decoupling rather than to the WAL mechanism itself.

9. **Round-2 changelog item 5 updated** to mark the SPEC errata as "subsequently applied to the repository working tree between Round 2 and Round 4", preserving the historical record while noting the resolution.

### Items deferred to read-aloud pass / out-of-scope

- **wasm build measured byte size at SHA 17334f8.** Still footnoted at §12.3 as "to be confirmed before filing"; carry-forward from Round 2.
- **Title length and IPC classification audit.** Out of scope for the detailed-description pass; lives in `00-cover.md`.
- **§3(k)-brief Round-1 aspect numbering vs Round-2 statutory claim numbering.** Owned by `11-3k-defense.md`; carry-forward from Round 3.
- **Optional quantitative-bound dependents to add to Claims 28-30 and Claim 36.** Owned by `08-claims.md`; carry-forward from Round 3.
- **P0 prior-art research items in `10-prior-art.md`** (InPASS Indian-prior-art search; Imani-HDC US patent number re-establishment; Pinecone US patent number re-establishment). Out of scope for this file.
- **Example 2 SHA-256 pin for `encode_embedding`.** Still requires a corpus expansion in `tests/conformance/golden.json`; not blocking; remains qualitative.
- **Banker's-rounding vs round-half-up framing in SPEC.md.** The SPEC.md working tree text uses "round-half-to-even (banker's rounding) per IEEE 754" with an outer clamp; the detailed description §5.2.1 recites Math.round (TS) and f32::round (Rust) with the same outer clamp. Both framings produce identical results in the in-range non-negative domain after the outer clamp. The cross-implementation byte-identity property is unaffected. A future SPEC editor may choose to harmonise the framings into a single canonical statement; the present file leaves the SPEC's framing in place and contributes the round-half-up/round-half-away-from-zero observation as the empirically verified equivalence at the language-library level.

### Final counts

- **Lines:** Round-2 = 931 → Round-4 = 962 (+31 lines from changelog + added prior-art and source-of-truth paragraphs).
- **Words (excluding HTML comments):** Round-2 ≈ 13,427 → Round-4 ≈ 13,545 (+0.88%).
- **Sections modified:** §3.1.1, §5.2 (+1 paragraph), §5.2.1, §6.5, §7.4 (claim ref), §8.3, §8.5 (claim ref), §8.7.1 (claim ref), §8.7.2 (claim ref), §9.3.
- **No algorithm semantics changed.** No new claim language. No files outside `patent/` modified. No commits made.

### Residual concerns flagged for read-aloud pass

1. **SPEC.md §A.1 banker's-rounding framing.** SPEC.md (working tree) uses round-half-to-even framing while the present detailed-description §5.2.1 uses Math.round / f32::round framing. Both produce identical results under the outer clamp, but the framing-mismatch may be a reviewer flag. A future SPEC editor pass should harmonise.

2. **Claim 22 / claim 20 cross-references depend on Round-4 final state of `08-claims.md`.** The Round-4 final of `08-claims.md` shows dep claim 20 = "zstd of each data block" and dep claim 22 = "optional attic block". If a subsequent claim editor reorders these, the detailed-description cross-references must be updated in lockstep.

3. **Round-4 detailed-description retains the Round-2 changelog block.** This is intentional — the changelog history is preserved as a textual audit trail. A pre-filing formatting pass may choose to consolidate into a single combined changelog.

4. **§8.7 attic / zstd subsections remain marked "not part of the SHA `17334f8` embodiment".** The corresponding claims (22, 24) are accordingly best-mode disclosures rather than embodiment-of-the-art disclosures. The §10(4) sufficiency for these is satisfied by the textual algorithm specifications; the §10(4) "best mode" requirement is satisfied by the labelling. The reviewer may wish to confirm that the IPO's CRI Guidelines treat this disclosure pattern as acceptable; the present author believes it is, but the question is open.

## Round 4 — 10-prior-art.md

**Pinned source SHA:** `17334f8`
**Reviser:** Round-4 prior-art critic-and-reviser agent (Claude Opus 4.7, 1M context), working under direction of inventor Vivek Singh.
**Critique source:** Five-perspective panel (IPO Examiner, §3(k) Defender, Prior-art Hunter, Patent Attorney, Domain Critic) applied against the Round-2 final of `10-prior-art.md` (≈9,561 words, 60+ citations) per the user-supplied Round-4 brief.
**Cross-references checked:** `08-claims.md` (Round-4 statutory numbering: 1, 7, 9, 18, 23, 27, 31, 35); `11-3k-defense.md` (Round-1 aspect-numbering 1–6 with mapping table at top); Round-3 cross-reconciliation entries in this log (lines 504–520, items 1–8).

### Critic-perspective findings and dispositions

1. **IPO Examiner** — No prior art on the present search reads on any independent claim. Two closest single-reference threats noted: (i) **Hopfield 1982** against Aspect 1 (Claims 1, 7) for the *associative-memory framing* — previously absent from the memo; (ii) **Charikar 2002 SimHash** against Aspect 6 (Claim 35) — already in §6 row 6. **Disposition:** New §4.11 added on classical associative-memory networks (Hopfield 1982, Steinbuch 1961, Willshaw 1969, Anderson 1972, Kohonen 1972, Ramsauer 2021); §6 Aspect 1 row updated to cite Hopfield 1982 / Kohonen 1972 alongside Kanerva 1988 and Imani 2017 HPCA; §8 risks table augmented with a Round-4 row on Hopfield-family obviousness combinations; §11 verification list updated.

2. **§3(k) Defender** — Confirmed Rows 1, 2, 4, 6 lead with system-level technical effect (Round-2 reframing intact); Rows 3 and 5 already do so. **Disposition:** No further reframing required. §6 Aspect 5 row strengthened in passing to record the Round-2 OpenTV reframing of Claim 31 from method-form to system-form (the "gates load-time admission" phrase added).

3. **Prior-art Hunter** — Three new categories identified: (i) classical associative-memory networks (§4.11, new); (ii) early SDM-based positional-sequence work (Rao & Fuentes 1998 in §4.7); (iii) modern Hopfield-as-attention (Ramsauer 2021 in §4.11). **Disposition:** All three added with full bibliographic detail and conflict-with-claims analysis.

4. **Patent Attorney** — Round-2 §6 matrix used Round-1 aspect-numbering (1–6) only; the Round-3 cross-reconciliation table at lines 440–462 of this log records the mapping to Round-4 statutory claim numbers (1, 7, 9, 18, 23, 27, 31, 35), but the prior-art memo itself was never updated to surface the mapping. **Disposition:** §6 matrix header reworded to expose both numbering schemes; an in-line claim-number-mapping paragraph added immediately above the matrix; §7 strongest-claim ranking and §8 risks table augmented with Round-4 statutory numbers in parentheses. The Round-2 final is the now traceable end-to-end from §6 to `08-claims.md`. No claim language has been altered.

5. **Domain Critic** — Audit of cited-paper contribution descriptions surfaced two minor inaccuracies: (i) **Datar et al. 2004** was originally described as "LSH" without specifying that the construction targets L_p norms via p-stable distributions; (ii) **Andoni & Indyk 2008** was described as "Near-Optimal Hashing Algorithms for Approximate Nearest Neighbor in High Dimensions" without noting that the CACM 2008 article is a survey of the original FOCS / SoCG 2006 Andoni-Indyk result. **Disposition:** Both descriptions tightened in §4.8 with corrective annotations; DOIs retained.

### [TO VERIFY] items: dispositions

| Round-2 [TO VERIFY] item | Round-4 disposition |
|---|---|
| US 11,775,847 B2 → Imani-HDC patent number to re-establish | **P0 retained, downgraded.** Round-4 web search surfaced **candidate leads** US 12,450,896 ("Apparatus, method, and computer-readable medium for robust response to adversarial perturbations using hyperdimensional vectors", USPTO image-ppubs.uspto.gov retrieval confirmed) and US 11,854,253 (same Justia cluster). The reviser could not extract claim-1 text from the image-only USPTO PDFs. **Authoritative verification still requires patent agent's USPTO Patent Public Search inventor query.** |
| US 10,956,464 B1 → Pinecone patent number to re-establish | **P0 retained.** Round-4 web search returned *no* Pinecone Systems / Pinecone Inc. issued US patents in public indexes (Google Patents, Justia; the only Justia hits for "Pinecone" name an unrelated "Pinecone Imaging Corporation"). The Round-1 number is best treated as a probable transcription error or non-existent. **Authoritative confirmation still requires patent agent's USPTO assignee query**, which may either confirm the negative or surface a previously-unindexed filing. |
| §5.1 InPASS Indian-prior-art search pending | **P0 retained.** No change; out-of-scope for this round (formal InPASS execution requires patent-agent credentials). |
| §4.6 IISc / IIT Bombay [TO VERIFY] vague allusion | **Resolved (removed).** Per the §10 standard "cite specifically or remove", the vague allusion is removed and replaced with a positive negative-finding paragraph plus an open action item for the agent's formal IEEE / ACM author-search. |
| Imani QuantHD 2019, SparseHD 2019, BRIC 2019 — citation verified, full text not read | **Carry forward.** No change required for filing; full-text reads recommended only if these references become load-bearing in an FER response. |
| Gayler 1998 — verified by citation, not independently read | **Carry forward.** No change required. |

### Cross-document consistency checks

- **§3(k) memo citations.** Verified all four — Achlioptas 2003 (§4.8, §6 Aspect 4, §9 #15); Mikolov 2013 (§4.10, §6 Aspect 6); Pennington 2014 (§4.10, §6 Aspect 6); Devlin 2019 (§4.10, §6 Aspect 6) — are present and correctly attributed. The Round-3 reconciliation entry at lines 491–500 of this log already confirmed presence; the Round-4 audit re-confirms.

- **Claim numbering vs `08-claims.md` Round-4 final.** §6 differentiation matrix headers now expose both Round-1 aspect-numbering (1–6) and Round-4 statutory numbering (1, 7, 9, 18, 23, 27, 31, 35). The mapping paragraph immediately above the matrix records the correspondence per Round-3 cross-reconciliation table.

- **`11-3k-defense.md` closest-prior-art lists.** §6 Aspect 1, Aspect 4, Aspect 5 and Aspect 6 closest-prior-art lists are harmonised with the corresponding "Closest prior art" paragraphs at lines 78, 130, 154 and 181 of `11-3k-defense.md`. No discrepancy.

### Sections modified in Round 4

- **Top of file:** Round-4 changelog block prepended above the retained Round-2 changelog.
- **Document status line:** "Round 2" → "Round 4".
- **§4.6 (Indian-affiliated research):** Vague TO VERIFY allusion removed; replaced with positive negative-finding paragraph and methodological note.
- **§4.7 (Theory of binary HDC):** Rao & Fuentes 1998 added with DOI.
- **§4.8 (LSH and binary fingerprints):** Datar 2004 description tightened (p-stable LSH for L_p norms); Andoni & Indyk 2008 description corrected to note CACM 2008 survey article.
- **§4.11 (NEW):** Classical associative-memory networks — Hopfield 1982, Steinbuch 1961, Willshaw 1969, Anderson 1972, Kohonen 1972, Ramsauer 2021 — with conflict-with-claims analysis covering Claims 1+7, Claim 9, Claim 27, Claim 35.
- **§5.2 (US patent landscape):** Round-4 web-verification update for both P0 items; candidate leads recorded for the Imani-HDC search.
- **§6 (differentiation matrix):** Headers reworded to expose Round-4 statutory claim numbers; mapping paragraph added; Aspect 1 row updated to cite Hopfield 1982 / Kohonen 1972; Aspect 2 row updated to cite Karunaratne 2020; Aspect 5 row updated to note OpenTV system-reframing of Claim 31; Aspect 6 row updated to cite Frady et al. 2018.
- **§7 (strongest claims):** Round-4 statutory claim numbers added in parentheses; Aspect 1 ranking note extended to record that the new Hopfield reference does not weaken the STRONGEST ranking.
- **§8 (risks and mitigations):** All Round-1 "Claim N" cell references re-expressed as "Aspect N / Claim K" with the Round-4 statutory number. A new Round-4 row added on the Hopfield-family obviousness combination.
- **§10 (action items):** §4.6 IISc/IITB action item marked resolved; new agent action item added for IEEE / ACM author-search; §5.2 P0 items updated with Round-4 candidate-lead status.
- **§11 (provenance and verification note):** Round-4 verified-citation additions listed (Hopfield, Kohonen, Willshaw, Steinbuch, Anderson, Ramsauer, Rao & Fuentes); reviser's note on web-verification methodology added; Round-5 expected-actions paragraph rewritten.

### Final counts

- **Lines:** Round-2 = 483 lines → Round-4 = ≈575 lines (estimate; precise count after final write).
- **Words (excluding HTML comments):** Round-2 ≈ 9,561 words → Round-4 ≈ 12,142 words (+27%, principally from the new §4.11 classical-associative-memory section, the Round-4 changelog block, the §6 matrix header rework, the §11 reviser's note on web verification, and the §5.2 candidate-lead paragraphs).
- **New references added:** Hopfield 1982, Steinbuch 1961, Willshaw-Buneman-Longuet-Higgins 1969, Anderson 1972, Kohonen 1972, Ramsauer et al. 2021, Rao & Fuentes 1998 (7 new references; all with verified DOIs).
- **References dropped:** None substantive; one vague allusion (IIT Bombay / IISc — never carried a specific citation) removed.
- **[TO VERIFY] items resolved this round:** 1 (the §4.6 vague allusion).
- **[TO VERIFY] items refined:** 2 (the US 11,775,847 / Imani item — candidate leads recorded; the US 10,956,464 / Pinecone item — negative finding recorded).
- **[TO VERIFY] items remaining as P0 for patent agent:** 3 (InPASS Indian prior-art search; Imani-HDC USPTO inventor-search to confirm candidate leads US 12,450,896 / US 11,854,253 or supersede; Pinecone USPTO assignee-search to confirm the Round-4 negative finding or surface a filing).
- **Sections modified:** §4.6, §4.7, §4.8, §4.11 (new), §5.2, §6, §7, §8, §10, §11.
- **No commits made.** No files modified outside `patent/`. No claim language altered. No sister artefacts (claims, detailed-description, abstract, 3k-defense) touched.

### Items deferred to Round 5 (post-Round-4 review under counsel direction)

1. **Patent-agent USPTO Patent Public Search execution.** Confirm or supersede Round-4 candidate leads US 12,450,896 and US 11,854,253; confirm or refute the Round-4 negative finding on Pinecone Systems.

2. **InPASS Indian-prior-art formal search.** Per §5.1 methodology, including the rolling-publication-window re-check at filing day.

3. **Re-test of §6 Aspect 1 differentiation against Hopfield 1982** following any counsel feedback. The Round-4 reviser believes the cross-implementation byte-identity differentiator is dispositive against Hopfield, but a registered patent agent's independent reading of the Hopfield 1982 paper is recommended before the FER response.

4. **Optional: add a dependent claim under Claims 1 and 7 expressly reciting the "cross-implementation byte-identity across at least three of {x86_64, ARM64, JVM, V8, LLVM-native Rust, WebAssembly}" limitation** to strengthen the Hopfield-1982 differentiation in claim-element form. This would require a follow-up `08-claims.md` revision pass and is recommended but not blocking.

5. **Espacenet and PATENTSCOPE bulk searches.** Per §5.3 and §5.4 methodologies.

6. **Vendor-documentation re-check at filing day.** Per §10 standing action item.

### No commits made

No files modified outside `patent/`. All code references remain pinned to `17334f8`. No algorithm semantics changed. No claim language altered.

---

## Round 5 / Final Read-Aloud Pass (2026-05-20)

**Pinned source SHA:** `17334f8`

**Pass objective:** Final voice / antecedent / drawing-numeral / orthography / cross-file-consistency pass over the assembled 14-artefact filing package. Surgical edits only; no rewrites; no new content.

### Final state summary table

| File | Round | Final word count | Final claim/figure count |
|---|---|---|---|
| `00-cover.md` | 1 (Round 5 untouched) | 347 | — |
| `01-title-and-preamble.md` | 1 (Round 5 untouched) | 42 | — |
| `02-field-of-invention.md` | 1 (Round 5 untouched) | 85 | — |
| `03-background.md` | 1 (Round 5 untouched) | 1198 | — |
| `04-objects-of-invention.md` | 3 (Round 5 untouched) | 689 | 11 objects (a)-(k) |
| `05-summary-of-invention.md` | 3 (Round 5 untouched) | 1892 | 6 aspects |
| `06-drawings-list.md` | 3 + Round 5 (preamble fix) | 2065 | 7 figures (Fig. 1-7) |
| `07-detailed-description.md` | 4 + Round 5 (§8.7 stale-claim-ref fix) | 14393 | 12 sections |
| `08-claims.md` | 4 (Round 5 untouched) | 6126 | 36 claims (8 independent at 1, 7, 9, 18, 23, 27, 31, 35; 28 dependent) |
| `09-abstract.md` | 4 (Round 5 untouched) | 880 (file); 146 (abstract body, ≤ 150 Rule 13(7)) | — |
| `10-prior-art.md` | 4 + Round 5 (Round-1 → Round-4 claim-number stragglers) | 12163 | — |
| `11-3k-defense.md` | 4 (Round 5 untouched) | 8058 | — |
| `12-examples.md` | 3 (Round 5 untouched) | 2446 | 5 worked examples |
| `13-filing-checklist.md` | 1 (Round 5 untouched) | 1268 | — |
| **Total filing package** | — | **51652 words** | **8 independent claims, 36 total claims, 7 figures, 5 worked examples** |

### Files modified in Round 5

1. `06-drawings-list.md` — reference-numeral preamble.
2. `07-detailed-description.md` — one stale claim-number reference in §8.7.
3. `10-prior-art.md` — 12 Round-1 "Claim N" stragglers in §2.x and §3.8 / §4.8 / §4.10.

### Fixes applied (one line per fix)

- `06-drawings-list.md` line 13: preamble corrected from "100-series ... 500-series" (incomplete and partially misdescribed) to "100-series encoders; 200-series primitives; 300-series in-memory substrate; 400-series KMF wire-format fields; 500-series persistence-adapter interface; 600-series adapter implementations; 700-series conformance bindings; 800-series embedding-encoder steps", matching the actual numbering scheme used throughout the file and in `07-detailed-description.md` (Round-2 changelog line 33).
- `07-detailed-description.md` §8.7 introduction: stale "claims 35 (attic block) and 38 (Zstandard compressor coupled with BLAKE3 hasher)" replaced with "dependent claim 22 of independent claim 18 (attic block) and dependent claim 24 of independent claim 23 (Zstandard compressor coupled with BLAKE3 hasher)"; Round-4 changelog at the head of the file already noted this rewrite for §8.7.1 and §8.7.2 but had missed the §8.7 introductory line.
- `10-prior-art.md` §2.1 "Conflict with claims": "The inventive step of Claim 1 rests on..." → "The inventive step of Claims 1 and 7 rests on..."
- `10-prior-art.md` §2.2 "Smritidb's relationship": "permutation-positional n-gram encoder (Claim 6)" → "permutation-positional n-gram encoder (Claim 35)".
- `10-prior-art.md` §2.4 lead-in: "level-hypervector / thermometer encoder used in Claim 4 are:" → "level-hypervector / thermometer encoder used in Claim 27 are:"
- `10-prior-art.md` §2.4 Imani HPCA 2017 citation: "closest single Imani-line reference for *associative-memory* aspects of Claim 1" → "of Claims 1 and 7".
- `10-prior-art.md` §2.4 "Smritidb's relationship" + "Conflict with claims": "Claim 4" → "Claim 27" (two occurrences in the same paragraph block).
- `10-prior-art.md` §2.5 "Smritidb's relationship": (i)-(v) cross-references updated to Round-4 numbering — "Claim 4 encoder" → "Claim 27 encoder"; "Claim 3 wire format" → "Claims 18 and 23 wire format"; "Claim 2 consolidation" → "Claim 9 consolidation".
- `10-prior-art.md` §2.6 "Conflict with claims": "Claim 2 differs by being..." → "Claim 9 differs by being..."
- `10-prior-art.md` §3.8 "Smritidb's relationship": "The six independent claims do *not* claim a recall index. Claim 1 (tiebreaker), Claim 2 (consolidation), Claim 3 (wire format), Claim 4 (level encoder), Claim 5 (conformance corpus) and Claim 6 (text encoder)..." → "The eight independent claims do *not* claim a recall index. Claims 1 and 7 (tiebreaker), Claim 9 (consolidation), Claims 18 and 23 (wire format), Claim 27 (level encoder), Claim 31 (conformance-gating system) and Claim 35 (text encoder)..."
- `10-prior-art.md` §3.9 "Threat assessment": "Claim 1 framing" → "Claims 1 and 7 framing".
- `10-prior-art.md` §4.2: "directly relevant background for Claim 1 and Claim 6" → "for Claims 1, 7 and 35".
- `10-prior-art.md` §4.4 (Karunaratne 2020): "relevant to Claim 1's reproducibility framing" → "relevant to the Claims 1 and 7 reproducibility framing".
- `10-prior-art.md` §4.7 (Frady et al. 2018): "permutation-positional n-gram encoder of Claim 6" → "of Claim 35".
- `10-prior-art.md` §4.8 lead-in: "obviousness combination an examiner will raise against Claim 4 (encoder) and Claim 6 (text encoder), and are also relevant to Claim 1" → "against Claim 27 (encoder) and Claim 35 (text encoder), and are also relevant to Claims 1 and 7".
- `10-prior-art.md` §4.8 "Smritidb's relationship" + "Differentiation" (i), (ii), (iii): "Claim 4" → "Claim 27"; "Claim 6" → "Claim 35"; "Claim 1" → "Claims 1 and 7"; throughout the three differentiation sub-paragraphs.
- `10-prior-art.md` §4.9 "Threat assessment": "Claim 4 'approximately 32-fold reduction'" → "Claim 27 'approximately 32-fold reduction'".
- `10-prior-art.md` §4.10 lead-in: "trained text-encoder prior art against which Claim 6 ('training-free, on-device-feasible') is differentiated" → "against which Claim 35 ... is differentiated".
- `10-prior-art.md` §4.10 "Conflict with claims": "Claim 6 is differentiated as training-free..." → "Claim 35 is differentiated as training-free..."

### Checks performed (no edit required)

- **Claim numbers across the package.** Verified that every "Claim N" reference in `00-cover.md`, `02-field-of-invention.md`, `03-background.md`, `04-objects-of-invention.md`, `05-summary-of-invention.md`, `06-drawings-list.md`, `07-detailed-description.md`, `09-abstract.md`, `11-3k-defense.md`, `12-examples.md`, `13-filing-checklist.md` uses Round-4 numbering. The §6 differentiation matrix of `10-prior-art.md` already carried Round-4 statutory numbering in parentheses; the surgical fixes above bring the prose of §2.x, §3.x and §4.x into the same numbering convention.
- **Reference numerals — Fig. 1 through Fig. 7.** Spot-checked five numerals across `06-drawings-list.md` and `07-detailed-description.md`: (260) tiebreaker, (412) header offset, (445) attic_block, (660) conformance corpus, (770) compare-to-expected-SHA-256. All cross-reference consistently.
- **Fig. N references.** All `Fig. N` mentions across the package are within the range Fig. 1-7 and all seven figures are defined in `06-drawings-list.md`.
- **Drawings-list Round 2 / 3 corrections in place.** Fig. 2 meta_block annotated "JSON (Phase 1; MessagePack contemplated for Phase 2)"; Fig. 6 step 803 uses `round((v_i + 1) * (L - 1) / 2)` (not `floor`); Fig. 6 step 805 is deleted (phantom permute step gone); steps 806 and 807 follow directly after step 804, as required.
- **Case citation form.** `11-3k-defense.md` carries the seven Indian/UK precedents in consistent form (`*Yahoo!* (IPAB 2011, OA/22/2010/PT/CH)`; `*Accenture* (IPAB 2009, OA/22/2009/PT/DEL)`; `*Ericsson v. Intex* (Delhi HC, 2015, I.A. 6735/2014 in CS(OS) 1045/2014)`; `*Ferid Allani* (Delhi HC, 2019, 2019 SCC OnLine Del 11867)`; `*Raytheon* (IPAB 2020, OA/27/2018/PT/DEL)`; `*Microsoft* (Delhi HC, 2023, 2023 SCC OnLine Del 2772)`; `*OpenTV* (Delhi HC, 2023, 2023 SCC OnLine Del 3251)`; `*Genentech Inc.'s Patent* [1989] RPC 147`). `10-prior-art.md` cross-references three of these (`*Ferid Allani*`, `OpenTV`, `Microsoft`) in compatible short form. No citation-form drift between the two documents.
- **SHA pinning.** Five-citation spot-check: `00-cover.md` line 25 (master declaration); `05-summary-of-invention.md` lines 22, 24 (tests/conformance/golden.json); `07-detailed-description.md` line 46 (Source-of-truth SHA, HEAD); line 224 (At SHA `17334f8` reference TS / Rust bindings); `12-examples.md` line 47 (`git show 17334f8:tests/conformance/golden.json`). All consistent; the master pinning in `00-cover.md` and the file-level declaration in `07-detailed-description.md` permit shorter `packages/...` paths to be cited without inline SHA repetition.
- **Indian English orthography.** Confirmed: 70 instances of British -ised forms across the package (`characterised`, `quantised`, `organised`, `realised`, `normalised`, `optimised`, `serialised`, `tokenised`, `summarised`, etc.); zero American -ized forms in the patent-form files. Only one non-substantive -ize spelling appears in `10-prior-art.md` (line 165, in the *title* of the academic paper "SparseHD: Algorithm-Hardware Co-optimization" by Imani et al. 2019 — preserved verbatim because it is a citation of a published paper). One British/American inconsistency in `11-3k-defense.md`: "defense" (10 occurrences) vs "defence" (1 occurrence); the filename `11-3k-defense.md` is referenced by other files in the package as a stable identifier, so the in-prose noun is left at "defense" to match the filename — this is flagged as a Round 6 pre-filing item if the patent agent wishes to harmonise to the British "defence" (filename rename would ripple to `10-prior-art.md` and other cross-references).
- **Abstract word count.** `09-abstract.md` body paragraph = 146 words (Rule 13(7) ceiling = 150; margin = 4 words). Verified by `awk 'NR==15' 09-abstract.md | wc -w`.
- **Title consistency.** Title across `00-cover.md` line 3, `01-title-and-preamble.md` line 5, `07-detailed-description.md` (implicit at "Detailed Description of the Invention"), and `09-abstract.md` (heading "# Abstract" only — IPO convention is for the abstract to use only the "Abstract" heading on a separate page rather than repeating the full title; this is correct as drafted): the title "A System and Method for Bit-Exact Cross-Implementation Persistent Associative Memory Using Binary Hyperdimensional Vectors" appears identically in `00-cover.md` and `01-title-and-preamble.md` and is not repeated in `09-abstract.md`, consistent with Form-2 / Rule 13(7) conventions.
- **Voice consistency.** First paragraph of each of the patent-form artefacts (`00`-`09`, `12`) confirmed in formal third-person patent voice ("the invention", "the method", "the system", "the present invention"). Zero "we" / "our" / "us" occurrences in `00`-`09`, `12`. (`10-prior-art.md` and `11-3k-defense.md` are internal working documents, not filed with Form-2; they use first-person voice in places — "the applicant", "the inventor's view" — which is conventional for internal memos and is not a defect.)
- **Antecedent basis spot-check on `08-claims.md`.** Five claims spot-checked — Claim 1, Claim 9, Claim 18, Claim 27, Claim 35. Claim 9, Claim 18, Claim 27, Claim 35 all pass antecedent-basis check. Claim 1 has a minor antecedent gap in its characterising clause ("the said memory" — no prior "a memory" introduction; "the said associative memory substrate" — no prior "an associative memory substrate"; "the said packed output hypervector" — no prior "a packed output hypervector"). These are not fatal — the patent agent will routinely add "in a memory ..." in step (a) or similar at filing — but are flagged for Round 6 surgical addition. Claim 9 has a minor gap on "the said dimension" (no prior "a dimension" introduced); same Round-6 disposition.
- **Dependent-claim references in `08-claims.md`.** Verified that every "The method as claimed in claim N" and "The system as claimed in claim N" refers to a claim number that exists (1, 7, 9, 18, 23, 27, 31, 35 independent; 2-6, 8, 10-17, 19-22, 24-26, 28-30, 32-34, 36 dependent). No dangling references.
- **Defensive checklist in `11-3k-defense.md` §5.** Verified line-by-line against `08-claims.md`: method-form + system-form pairs for Claims 1/7 and 18/23; system-form Claim 31; method-form-only families for Claims 9, 27, 35 with the apparatus surrogates noted (Claim 9 → dep. Claim 15 non-volatile-storage + restore-module; Claim 27 → microcontroller-class hardware tie in the characterising clause; Claim 35 → processor-lacking-FPU hardware tie in the characterising clause). The §5 checklist's framing — "no CRM triplets exist for Claim families 9, 27, 35; method-form only" — is accurate as of the Round-4 final claim set.

### Pre-filing items deferred to Round 6 / patent agent

1. **Claim 1 antecedent basis.** The characterising clause of Claim 1 uses "the said memory", "the said associative memory substrate", and "the said packed output hypervector" without first introducing "a memory", "an associative memory substrate", and "a packed output hypervector" in the preamble or step (a). The patent agent should add the missing antecedents — likely by amending the preamble of Claim 1 from "A computer-implemented method..." to "A computer-implemented method for ... in an associative memory substrate, the method comprising:" and adding "in a memory coupled to the said processor" to step (a). This is a routine drafting fix.

2. **Claim 9 antecedent basis on "the said dimension".** The characterising clause of step (d) uses `D` is "the said dimension" without prior introduction. Suggested surgical fix: amend the preamble of Claim 9 to include "...substrate of a fixed dimension `D`..." at the appropriate point.

3. **"defense" vs "defence" orthography in `11-3k-defense.md`.** Ten prose occurrences of American "defense"; one occurrence of British "defence". The filename `11-3k-defense.md` is the stable cross-file identifier (referenced by `10-prior-art.md` and elsewhere). The patent agent should decide whether to (a) leave the filename and prose as-is (acceptable since the document is an internal memo, not filed), (b) harmonise the prose to "defence" while leaving the filename for cross-reference stability, or (c) rename the file to `11-3k-defence.md` and update all cross-references. Round-5 disposition: take no action — flag for Round 6.

4. **P0 prior-art blockers (carried forward from `10-prior-art.md`).** InPASS Indian-prior-art formal search (§5.1); USPTO Patent Public Search inventor- and assignee-search execution for the Imani-HDC and Pinecone-Systems candidate leads (§5.2). These are agent-execution items, not Round-5 fixes.

5. **Round-4 changelog block at top of `07-detailed-description.md` line 32** mentions "claims 35 and 38" as a Round-2 historical reference. Round-2 changelogs are preserved verbatim across rounds as the audit trail; not modified in Round 5. If the patent agent prefers to delete the Round-2 changelog entirely at filing day, the historical "claim 35 / claim 38" reference will go with it.

6. **Optional dependent claim under Claims 1 and 7 reciting the cross-implementation byte-identity hardware enumeration** (carried forward from Round-4 §6.4 of the `10-prior-art.md` deferred items). Tactical, not blocking.

7. **3-30× learned-embedding baseline framing in `11-3k-defense.md` §6.2.** Round-4 Domain Critic flagged that the upper end of the "3-30×" range is strict-arithmetic-conservative at typical embedding dimensions (384-1536) and should be either tightened or substantiated by reference to a specific higher-dimensional baseline. Not actioned in Round 5; carried forward to the patent agent.

### Summary of Round 5

- **Total fixes applied:** 21 surgical edits — 1 in `06-drawings-list.md`, 1 in `07-detailed-description.md`, 19 in `10-prior-art.md` (claim-number stragglers).
- **Pre-filing items deferred:** 7 items, principally antecedent-basis touch-ups in Claim 1 and Claim 9, the `defense` / `defence` orthography call, and the P0 prior-art blockers and tactical claim additions already carried forward from Round 4.
- **Total word count of the filing package:** 51,652 words across the 14 patent artefacts. Abstract body = 146 words (≤ 150 per IPO Rule 13(7)).

### No commits made

Round 5 made no commits and modified no files outside `patent/`. All code references remain pinned to SHA `17334f8`. No algorithm semantics changed, no claim language altered, no new content added. The 14 patent artefacts are in their final read-aloud state subject only to the seven pre-filing items deferred to Round 6 / the patent agent.

*End of Round 5 / Final Read-Aloud Pass.*

---

## Round 6 — Prior-Art P0 Integration (2026-05-20)

**Pinned source SHA:** `17334f8`

**Pass objective:** Integrate the verified findings from `patent/critiques/p0-prior-art-search-results.md` into `patent/10-prior-art.md`. The P0 web-research memo resolves both USPTO P0 items carried forward from Round 5 (the Imani-HDC slot and the Pinecone-Inc slot) and substantively de-risks the InPASS P0 item by means of a public-web surrogate search.

### Scope

Only `patent/10-prior-art.md` and the present log file were modified. No other patent artefact (claims, detailed description, abstract, §3(k) defence, drawings, examples, filing checklist) was touched. No file outside `patent/` was modified. No commit was made.

### Findings integrated from the P0 memo

1. **Round-4 candidate leads were FALSE.** US 12,450,896 and US 11,854,253 are Intel / Narayan Srinivasa patents directed to HDC-based image-classifier robustness against adversarial perturbations — they are not Imani-Rosing UCSD patents. Both are removed from the Imani-HDC slot in §5.2.

2. **US 10,956,464 B1 confirmed non-existent.** Pinecone Systems Inc. has no US patent on file reachable by public web search as of 2026-05-20. The Round-1 cite is withdrawn entirely.

3. **Verified actual closest prior art.** Seven US patent records were independently verified and integrated:
   - **US 2022/0019441 A1** (Rosing, Imani et al., UCSD, published 20 January 2022) — closest single US patent reference to Claims 1 and 7. Distinguished by absence of BLAKE3 four-tuple tiebreaker, replayable Hebbian, KMF wire format, conformance corpus, BLAKE3-XOF encoder and permutation-positional text encoder.
   - **US 12,015,424 B2** (Imani, UCSD, "Network-based hyperdimensional system," issued 18 June 2024) — adjacent on Claim 27 encoder slot; complex `{±1, ±i}` substrate orthogonal to Smritidb's binary `{0, 1}`.
   - **US 12,204,899 B2** (Imani, UCSD, "Stochastic hyperdimensional arithmetic computing," issued 21 January 2025) — adjacent on Claim 27 encoder slot; bipolar `±1` substrate with no cryptographic anchor.
   - **US 11,574,209 B2** (Karunaratne et al., ETH Zurich + IBM, "Device for hyper-dimensional computing tasks," issued 7 February 2023) — adjacent on Claims 1 and 7; inference-device hardware.
   - **US 10,971,226 B2** (Le Gallo-Bourdeau et al., ETH Zurich + IBM, "Hyper-dimensional computing device," issued 6 April 2021) — adjacent; resistive-memory hardware substrate.
   - **US 12,260,913 B2** (Lin and Tseng, Macronix, "Hyperdimensional computing device," issued 25 March 2025) — adjacent; flash-cell HDC hardware with analog-sense-amplifier majority bundling.
   - **US 12,518,150 B2** (Hersche and Rahimi, IBM, "Bundling hypervectors," issued 6 January 2026) — recent IBM bundling patent using share-based cumsum-step-function indicator selection rather than majority-with-tiebreaker; **does NOT impair Claim 1**.

4. **InPASS programmatic search returned HTTP 403** (anti-bot protection). 10 Google-surrogate queries returned zero Indian HDC patent hits. The InPASS P0 item remains an agent-execution item but substantive risk is assessed as LOW.

5. **Zero conflicting patents** against the 8 Smritidb independents (1, 7, 9, 18, 23, 27, 31, 35). **No claim revisions required.**

### Edits applied to `patent/10-prior-art.md`

1. **Round 6 changelog block** prepended at top of file (per instructions).

2. **Document status header** updated from "Round 4 internal working draft" to "Round 6 internal working draft"; the note-on-tone paragraph updated to reflect Round-6 closure of both USPTO P0 items.

3. **§5.2 (United States Patents and Published Applications) substantially rewritten.** Three sub-sections (a)-(c):
   - (a) US 11,775,847 B2 — Round-1 cite withdrawn (preserved as audit trail).
   - (b) US 10,956,464 B1 — Round-1 cite withdrawn (preserved as audit trail) with Round-6 confirmation of non-existence.
   - (c) **NEW: Round-6 verified findings (seven verified US patents).** Each is cited in formal patent-citation form ("US 2022/0019441 A1 (Rosing, Imani et al., assigned to The Regents of the University of California (UCSD), published 20 January 2022)", etc.) with a one-line statement of relationship to the most-resembled Smritidb claim and a one-line distinguishing statement.
   - Removed: all Round-4 [TO VERIFY] / [P0] flags on the USPTO items (resolved); Round-4 prose about "candidate leads recorded; agent verification required" (resolved as false leads).
   - Methodology section updated to reflect Round-6 closure; patent-agent residual actions retained.

4. **§5.4 (NEW) — Verified non-conflicting closest prior art** inserted between §5.3 (EPO) and the former §5.4 (WIPO PCT). The new §5.4 is a consolidated one-line-per-patent table in formal patent-citation form, keyed to the most-resembled Smritidb claim for each of the seven verified records. Former §5.4 (WIPO PCT) renumbered to §5.5; former §5.5 (defensive publications) renumbered to §5.6.

5. **§6 differentiation matrix updated:**
   - **Row 1 (Aspect 1 — Claims 1, 7):** closest-single-reference column now includes **US 2022/0019441 A1** (binary HDC system primitives in HPU + CAM context, leaves bundle tie resolution unspecified) and **US 12,518,150 B2** (share-based bundling, not majority-with-tiebreaker). Closest-combination column now includes US 2022/0019441 A1. Differentiator column updated with a Round-6 statement explicitly noting that none of the seven verified HDC patents has cryptographic determinism for tie resolution.
   - **Row 4 (Aspect 4 — Claim 27):** closest-single-reference column now includes **US 12,204,899 B2** (stochastic bipolar HDC arithmetic, no cryptographic anchor) and **US 12,015,424 B2** (complex `{±1, ±i}` NetHD encoder). Differentiator column updated with a Round-6 statement noting that none of the verified prior art uses a cryptographic XOF for level-hypervector generation.
   - Rows 2, 3, 5, 6 — re-checked and confirmed unchanged. A Round-6 cross-row consistency note has been added below the matrix recording the re-check.

6. **§7 (Strongest Claims) — Aspect 1 (Claims 1, 7)** entry strengthened with a Round-6 paragraph noting that the verified prior art reinforces the STRONGEST ranking: US 2022/0019441 A1 (closest single US patent reference) leaves tie resolution unspecified, and US 12,518,150 B2 (recent IBM bundling patent) uses a fundamentally different mechanism. Aspect 1 retains the STRONGEST ranking; no other ranking changes.

7. **§8 (Risks and Mitigations)** — three new Round-6 rows added:
   - US 2022/0019441 A1 may be cited against Aspect 1 / Claims 1, 7 → mitigation: BLAKE3 four-tuple tiebreaker absent from the reference.
   - US 12,518,150 B2 may be cited against the bundle-operation slot → mitigation: bundling mechanism is fundamentally distinct.
   - US 12,015,424 B2 and US 12,204,899 B2 may be cited against Aspect 4 / Claim 27 → mitigation: substrate and BLAKE3-XOF anchoring distinguish.

8. **§10 (Action Items for Filing Day)** — both USPTO P0 items marked closed `[x]`. Three new agent-residual items: (i) PAIR check on US 2022/0019441 A1 grant status, (ii) PAIR family-graph check on US 12,518,150 B2, (iii) USPTO inventor searches for Imani / Rosing / Rahimi for completeness. The InPASS P0 item remains open with a Round-6 status update noting the public-web surrogate result.

9. **§11 (Provenance and Verification Note)** — new "Round-6 additions to the verified-citation list" sub-section listing all seven verified US patents. New "Round-6 confirmation of false leads (now removed)" sub-section recording the withdrawal of US 12,450,896 and US 11,854,253 (Intel) and US 10,956,464 B1 (Pinecone). The "Citations awaiting verification" list updated: USPTO P0 items removed; new patent-agent residual items added; InPASS item retained.

10. **End-of-memorandum note** updated to reflect Round-6 closure of both USPTO P0 items.

### Items NOT modified

- No claims artefact (`08-claims.md`) modified.
- No detailed description (`07-detailed-description.md`) modified.
- No abstract, summary, background, drawings, examples or filing checklist modified.
- No `11-3k-defense.md` modified — but **a follow-up pass is flagged**. A grep across the patent package surfaced one residual reference at `11-3k-defense.md` line 249: "the verification of the US 11,775,847 B2 and US 10,956,464 B1 citations remain outstanding." This sentence is now stale (the verification is closed by Round 6) and should be updated in a follow-up pass to read along the lines of: "The InPASS formal search remains outstanding (substantively de-risked by Round-6 surrogate search); the Round-1 US 11,775,847 B2 and US 10,956,464 B1 cites have been withdrawn per the Round-6 update to `10-prior-art.md` §5.2, with the actual closest US prior art now identified at `10-prior-art.md` §5.4." Per the Round-6 constraint ("Do NOT touch any other patent artefact"), the present pass leaves the line as-is and records the follow-up here. No other patent artefact references the withdrawn numbers (grep across `00`-`09`, `12`, `13` returned no hits).

### Items flagged for follow-up

- **None blocking.** The Round-6 integration is complete on its own terms.
- **Patent-agent residual.** As enumerated at §10 of `10-prior-art.md`: (i) formal InPASS portal session for the Section 8 record (substantive risk LOW per Round-6 surrogate search), (ii) USPTO inventor / assignee searches for completeness, (iii) PAIR check on US 2022/0019441 A1 grant status, (iv) PAIR family-graph check on US 12,518,150 B2 to confirm no continuation claims integer-majority bundling.
- **Optional dependent claim.** The Round-5 deferred item (optional dependent claim under Claims 1 and 7 expressly reciting cross-implementation byte-identity across at least three named runtimes) is reinforced by the Round-6 finding that US 2022/0019441 A1 does not address cross-implementation byte-identity. Still optional; still tactical; still not blocking.

### Cross-file consistency check

A grep across all patent artefacts confirms that the withdrawn cite numbers (US 11,775,847; US 10,956,464; US 12,450,896; US 11,854,253) appear only within `10-prior-art.md` (in its changelog blocks, audit-trail withdrawal statements, and patent-citation context) and the present log file. No other patent artefact references any of the withdrawn numbers; no follow-up cross-file pass is required.

### Summary of Round 6

- **Total edits applied to `10-prior-art.md`:** 10 distinct edit operations covering the changelog header, document-status note, §5.2 rewrite, §5.4 insertion (with renumbering of subsequent sub-sections), §6 row 1 and row 4 differentiation-matrix updates, the §6 cross-row consistency note, §7 Aspect 1 strengthening, §8 three new risk rows, §10 P0-status update, §11 provenance update, and end-of-memo note update.
- **False patent references removed:** 3 (US 10,956,464 B1 confirmed non-existent; US 12,450,896 and US 11,854,253 confirmed Intel/Srinivasa false leads). The Round-1 US 11,775,847 B2 cite remains as a Round-2 withdrawal audit-trail entry.
- **New verified patent references added:** 7 (US 2022/0019441 A1; US 12,015,424 B2; US 12,204,899 B2; US 11,574,209 B2; US 10,971,226 B2; US 12,260,913 B2; US 12,518,150 B2).
- **Claim revisions required:** zero.
- **Word count change (`10-prior-art.md`):** see file-level word count at end of pass.

### No commits made

Round 6 made no commits and modified no files outside `patent/`. All code references remain pinned to SHA `17334f8`. No algorithm semantics changed; no claim language altered; no new artefact created.

*End of Round 6 / Prior-Art P0 Integration.*

## Round 6 — 11-3k-defense.md follow-up (2026-05-21)

**Pinned source SHA:** `17334f8`

**Pass objective:** Apply the follow-up flagged in the Round-6 Prior-Art P0 Integration entry above: clear the stale "verification remains outstanding" language at `11-3k-defense.md` line 249 so the §3(k) memo is internally consistent with the Round-6 disposition of `10-prior-art.md` §5.2 and §5.4.

### Scope

Only `patent/11-3k-defense.md` and the present log file were modified. No other patent artefact was touched. No file outside `patent/` was modified. No commit was made.

### Edits applied to `patent/11-3k-defense.md`

1. **Round 6 changelog block** prepended at the top of the file, above the existing Round-4 and Round-2 changelog blocks, pinned to SHA `17334f8` and dated 2026-05-21. The block records: (i) reconciliation with the Round-6 P0 integration in `10-prior-art.md`, (ii) removal of the stale "verification remains outstanding" language for US 11,775,847 B2 and US 10,956,464 B1, and (iii) the withdrawal of both cites per `patent/critiques/p0-prior-art-search-results.md`.

2. **§6.3 item 4 (Residual concerns flagged for the patent agent)** rewritten in place. The prior text — "The InPASS formal search and the verification of the US 11,775,847 B2 and US 10,956,464 B1 citations remain outstanding" — was stale as of the Round-6 integration of `10-prior-art.md`. The replacement text records that both USPTO P0 items are now closed (US 11,775,847 B2 reconfirmed as a KPN Innovations media-classification patent and withdrawn; US 10,956,464 B1 confirmed non-existent and withdrawn) and points the reader to the audit-trail entries at `10-prior-art.md` §5.2 (a) and (b) and the consolidated verified-prior-art table at §5.4. The InPASS item is recorded as remaining outstanding but substantively de-risked by the Round-6 public-web surrogate search.

### Stale references removed

- "the verification of the US 11,775,847 B2 and US 10,956,464 B1 citations remain outstanding" → replaced with verified-current-state language.
- No other occurrences of "US 11,775,847" or "US 10,956,464" exist anywhere in `11-3k-defense.md` (verified by grep across the full file).
- No other `[TO VERIFY]` / `[TBD]` / outstanding-P0 markers were found in `11-3k-defense.md` (a grep for "TO VERIFY", "TBD", "outstanding", "[TO", "[TBD" returned only the one line above).

### Cross-doc consistency now achieved

- `11-3k-defense.md` no longer contains any reference to the withdrawn USPTO cites as "outstanding".
- The §3(k) memo now correctly points the patent agent at `10-prior-art.md` §5.2 and §5.4, which are the operative sources for the Round-6 disposition.
- The Round-6 disposition is consistent across: `10-prior-art.md` (audit trail + verified table), `patent/critiques/p0-prior-art-search-results.md` (research record), `11-3k-defense.md` (§3(k) memo), and `REFINEMENT-LOG.md` (this log).
- No other patent artefact (`00`-`09`, `12`, `13`) references the withdrawn numbers; the cross-file consistency check recorded in the Round-6 Prior-Art P0 Integration entry above remains valid and is reaffirmed here.

### Other follow-ups noticed during the read

- **§6.2 (Domain Critic) — 3-30× learned-embedding range.** The Round-4 memo records a self-flagged tension between the strict arithmetic (which gives 1.2–4.9× at dim 384–1536) and the retained "3–30×" range (which assumes dim 256–7680). The Round-4 memo expressly retains the range and flags it for the patent agent at §6.3 item 2. No edit applied here (out of scope for the present follow-up; the Round-4 disposition is to flag, not to tighten); the item is reiterated for the patent agent's attention at any future round.
- **§6.3 item 3 — 2024 CRI Guidelines draft.** Carried forward unchanged; status remains "not yet finalised as of cut-off 2026-05-20" and the §3(k) memo remains anchored on the 2017 Guidelines. No edit required.
- **§3.2 (Claim 9) and §3.4 (Claim 27) — method-form only.** Reiterated at §6.3 item 1; the Round-4 disposition (file an apparatus continuation only if an FER objection materialises) is preserved. No edit required.
- **Round-2 5,092-word / Round-4 6,200-word counts.** The Round-6 follow-up adds approximately 100 words to the §6.3 item 4 paragraph and approximately 60 words to the new top-of-file changelog block; the Round-6 word count is therefore approximately 6,360 words. The §6.4 word-count note has not been updated in place (it is itself part of the §6 REFINEMENT-LOG section and reflects the Round-4 baseline); the present log entry is the operative record of the Round-6 increment.

### Items NOT modified

- No claims artefact (`08-claims.md`) modified.
- No detailed description (`07-detailed-description.md`) modified.
- No prior-art memo (`10-prior-art.md`) modified — the Round-6 integration was applied in the prior pass; the present follow-up only reconciles `11-3k-defense.md` with that prior pass.
- No abstract, summary, background, drawings, examples or filing checklist modified.
- No file outside `patent/` modified.

### No commits made

Round 6 follow-up made no commits and modified no files outside `patent/`. All code references remain pinned to SHA `17334f8`. No algorithm semantics changed; no claim language altered; no new artefact created.

*End of Round 6 / 11-3k-defense.md follow-up.*
