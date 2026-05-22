# Round 2 Critique — 10-prior-art.md

**Subject:** Round 1 draft of `/Users/viveksingh/Developer/smritidb/patent/10-prior-art.md` (3,995 words, dated 20 May 2026, prepared on HEAD `17334f8`).
**Critic:** Round 2 panel applying five perspectives in parallel.
**Bottom line:** The memo is well organised and tonally appropriate, but it has **substantial blind spots** in adjacent prior-art families (probabilistic data structures; LSH/SimHash; binary container formats other than Arrow/HDF5; metadata block formats such as CBOR/MessagePack-Schema), **several misattributions or imprecise citations** that an experienced examiner will exploit, **a P0 blocker on the Indian landscape (§5.1 still `[TBD]`)**, and **at least one differentiator (Claim 1) that reads as algorithm-vs-algorithm rather than as a system-property contrast** which weakens the §3(k) defence. The memo also leans too heavily on `[TO VERIFY]` and `[TBD]` placeholders for citations that, if invoked at FER stage, must already have been read.

---

## Perspective 1 — IPO Examiner (adversarial prior-art researcher)

### 1.1 Characterisation of cited prior art

(a) **Kanerva 1988 (§2.1).** The memo says SDM teaches "no tiebreaker policy". This is too strong a statement. Kanerva 1988 explicitly addresses bundle-majority operations and a *random* tiebreak is the canonical practice in the SDM literature. An adversarial examiner will respond: *"a person skilled in the art knows that a random tiebreak exists; substituting a deterministic random source (a cryptographic XOF) for an undefined random source is an obvious engineering choice."* The memo should pre-empt this by (i) acknowledging that random tiebreaks are taught in Kanerva 1988 §§ on majority operations, and (ii) drawing the inventive step around the *byte-identity guarantee* arising from the *specific tuple* `(D || idx || count)` fed to the XOF and the *storage of the corpus seed in the KMF header* (a system-level technical effect). As drafted, Claim 1's differentiator looks like "we use BLAKE3 instead of a PRNG" — which is the weakest possible framing.

(b) **Plate 1995 (§2.2).** The memo says HRR is real-valued and Smritidb is binary. Correct. But Plate's later work (Plate 2003 monograph chapter 4) and the Kanerva-Kleyko-Rachkovskij follow-ups discuss *binary variants* of HRR including the so-called "Binary Spatter Code" (Kanerva 1996, *Binary Spatter Codes*, ICANN 1996) and the FHRR variant (Plate 1994). The memo never cites Kanerva 1996 (BSC). An examiner will say: *"the inventor claims an XOR-binding binary substrate; Kanerva 1996 BSC teaches exactly this."* This must be added to §2 and confirmed not to anticipate.

(c) **Rachkovskij and Kussul 2001 (§2.3).** Correct characterisation, but again incomplete: there is a larger Rachkovskij body of work (Rachkovskij 2001, *Representation and Processing of Structures with Binary Sparse Distributed Codes*, IEEE TKDE 13(2): 261-276) that addresses binary distributed representations more broadly. Worth including in §4.

(d) **Imani 2017-2019 (§2.4).** This is the most threatening academic prior art and the memo treats it too casually. The memo distinguishes on three grounds (PRNG vs XOF, byte-identity across bindings, KMF header storage). All three are *implementation choices*, not algorithmic novelties. An examiner familiar with the broader Imani corpus will point to:
- Imani et al. (2019), "QuantHD: A Quantization Framework for Hyperdimensional Computing," *IEEE TCAD* 39(10): 2268-2278, which explicitly discusses *deterministic* level-hypervector generation from a seed.
- Imani et al. (2019), "BRIC: Locality-based Encoding for Energy-Efficient Brain-Inspired Hyperdimensional Computing," *DAC 2019*, which discusses encoder reproducibility.
- The HPCA 2017 Imani et al. paper, "Exploring Hyperdimensional Associative Memory," which the §3(k) defence memo cites (correctly) but the prior-art memo does *not* cite. This is a glaring inconsistency between two filing documents that must be reconciled.

(e) **BLAKE3 (§2.5).** Correctly characterised, but the memo should note that BLAKE2X (Aumasson et al.) and SHAKE128/256 (NIST FIPS 202) are both XOFs with the same essential property. The claim language ("a cryptographic hash function with extendable output") is correctly broadened — but the memo should *cite SHAKE128 / FIPS 202* as the canonical XOF prior art, so the examiner cannot complain that the broad claim is unsupported.

### 1.2 Missing prior-art categories (P0)

The following families are entirely absent from the memo and threaten one or more claims:

(1) **Bloom filters (Bloom 1970, *Communications of the ACM* 13(7): 422-426) and Count-Min sketches (Cormode & Muthukrishnan 2005, *Journal of Algorithms* 55(1): 58-75) and MinHash (Broder 1997, *Compression and Complexity of Sequences*).**
- Threat: *probabilistic binary substrates with hash-derived bit positions* is a 1970-era idea. The Claim 1 framing ("a cryptographic-hash-derived bit determines whether a tiebroken position becomes 1 or 0") will be characterised by an examiner as a *trivial variant of a Bloom-filter set-membership construction*, where the cryptographic hash is the well-known choice when adversarial inputs are anticipated. Mitigation: distinguish on *substrate semantics* (bundle is a noisy mean of constituent vectors, not a set-membership oracle) and on *system property* (cross-implementation byte-identity, not single-implementation false-positive rates).
- Threat to: Claim 1, Claim 3 (per-block content integrity using BLAKE — Bloom-like indexing into a block array could be alleged).

(2) **LSH — Charikar (2002), "Similarity Estimation Techniques from Rounding Algorithms," *STOC 2002*: 380-388; Datar, Indyk, Immorlica, Mirrokni (2004), "Locality-Sensitive Hashing Scheme Based on p-Stable Distributions," *SoCG 2004*.**
- Threat: *random-projection-to-binary* is the original SimHash / sign-LSH construction (Charikar 2002). The Claim 4 thermometer-quantised random projection encoder is *a variant of sign-LSH with level quantisation*. Examiner argument: "the inventive step over Imani thermometer encoding + Charikar sign-LSH is to seed the random projection deterministically by BLAKE3 — but Andoni & Indyk 2008 (*CACM* 51(1): 117-122) already discusses seeded random projection as a reproducibility measure." This is *the* obviousness combination most likely to be raised. The memo's §4 makes no mention of LSH. **This is the single largest gap.**
- Threat to: Claim 4 (centrally), Claim 6 (Plate + LSH combination), Claim 1 (sign-LSH as a binary-substrate baseline).

(3) **SimHash — Charikar 2002 (same paper as above); Manku, Jain, Das Sarma (2007), "Detecting Near-Duplicates for Web Crawling," *WWW 2007*.**
- Threat: SimHash is a *deterministic* projection from text to a binary fingerprint, used in web-search de-duplication. Claim 6 (permutation-positional binary n-gram encoder) is differentiated from SimHash in that SimHash is unordered bag-of-features, whereas the Smritidb encoder is permutation-positional. But the *general idea of a deterministic, training-free binary text fingerprint* is exactly what SimHash is. Failure to cite SimHash in the §6 matrix is a glaring omission.
- Threat to: Claim 6 (centrally).

(4) **Vector quantization patents — Jégou, Douze, Schmid (2011, already cited in §9 as ref 9 — but only referenced, not analysed); IVF (Sivic & Zisserman 2003, "Video Google," *ICCV 2003*); OPQ (Ge, He, Ke, Sun 2013, "Optimized Product Quantization," *IEEE TPAMI* 36(4): 744-755).**
- Threat: PQ/IVF/OPQ define the prior-art landscape for *footprint reduction by quantisation*. The Claim 4 "approximately 32-fold reduction" technical effect is *the same metric* PQ/OPQ optimise. Mitigation: PQ uses learned codebooks (training-dependent), Smritidb uses thermometer + random projection (training-free and deterministic). Memo should add this distinction explicitly. There may also be US patents assigned to INRIA / Facebook in the PQ family that should be FTO-checked.
- Threat to: Claim 4.

(5) **Apache Arrow / Parquet / Iceberg — Apache Arrow project (https://arrow.apache.org/; 2016-); Parquet specification v2.4 (Apache Parquet project; 2013-); Iceberg table format (Netflix, 2017-).**
- Threat: The memo cites Arrow and Parquet but does *not* analyse them claim-by-claim. Parquet *does* have a magic footer (`PAR1`) and a footer-first read pattern; the memo's §6 statement that "Parquet has a footer but not a magic trailer in the present sense" is a hairline distinction that an examiner will not accept without a much sharper definition of what "in the present sense" means. Arrow IPC has per-message length prefixes and supports streaming; HDF5 has per-chunk filters including checksums (the Fletcher32 filter, see HDF5 Group documentation). **Several of the Claim 3 "differentiating" features have close analogues in Arrow/Parquet/HDF5 that the memo glosses.** The right mitigation is to draw Claim 3 as a *combination claim* with *all four* of: (i) D mod 64 = 0 column-major bit packing, (ii) per-block BLAKE3 (cryptographic, not CRC), (iii) magic trailer with corpus-version field, (iv) self-describing conformance-seed pointer — and to show that no single prior-art format has all four.
- Threat to: Claim 3 (centrally), Claim 5 (by way of the corpus-as-data-structure framing).

(6) **HDF5 (The HDF Group, https://www.hdfgroup.org/; ISO/IEC 21320 reference) and FlatBuffers (Google, 2014) and Cap'n Proto (Varda, 2013).**
- Threat: FlatBuffers and Cap'n Proto offer zero-copy random-access binary serialisation, which is conceptually adjacent to the KMF streaming-load goal. HDF5's chunking, filtering and Fletcher32-checksum architecture is the closest *general-purpose* analogue to Claim 3. The memo cites HDF5 once (§1.3) and never returns to it. The memo must do a §6-level differentiation against HDF5 explicitly.
- Threat to: Claim 3.

(7) **JSON Schema, MessagePack, CBOR (RFC 8949).**
- Threat: For the KMF header / metadata block, CBOR (RFC 8949) is the natural prior-art comparator for a binary tagged metadata format. MessagePack is cited in §6 but never differentiated. The memo should explicitly say: "the KMF header is *not* claimed as a generic metadata format; it is claimed as a *substrate-specific* header that stores the corpus-seed pointer and the conformance-corpus version." Without this, an examiner can attack on grounds that "a tagged binary header is well-known".
- Threat to: Claim 3 (header structure), Claim 5 (corpus reference).

(8) **SQLite WAL (https://www.sqlite.org/wal.html; Hipp et al., 2010).**
- Threat: The "persistence adapter" infrastructure (referenced in the commit message at HEAD as "Phase 4 + 3.5 + 5: persistence adapters, KMP wrapper, examples" and "SQLite adapter") implicates SQLite's WAL design as prior art for *write-ahead-logged, replayable* state changes. This is directly relevant to Claim 2 (replayable consolidation). The memo cites no WAL / journaling literature. If Claim 2 is read by an examiner as "a write-ahead log applied to HDC consolidation", the inventive step shrinks to "BLAKE3-seeded bit-selection inside the consolidation step", which is a much narrower claim than the memo suggests.
- Threat to: Claim 2 (centrally).

(9) **Bencode (BEP 0003 / BitTorrent, Cohen 2001) and Protocol Buffers (Google, 2008).**
- Threat: Generic serialisation prior art. Should be mentioned in the §6 KMF differentiation, even if only to dismiss.
- Threat to: Claim 3.

(10) **Merkle trees (Merkle 1979, US 4,309,569) and content-addressed storage (Git, IPFS).**
- Threat: Per-block cryptographic integrity (Claim 3) is *exactly* what Git's object store and IPFS's blocks do. BLAKE3 itself uses a Merkle tree. The examiner will say: "applying Merkle-tree integrity to a binary file format is the well-known IPFS / Git pattern." The memo must distinguish on grounds of substrate (HDC bit-packed columns, not arbitrary blobs) and on the *integration with the conformance-corpus version field* (a unique combination).
- Threat to: Claim 3 (centrally).

(11) **HNSW (Malkov & Yashunin 2018) — cited in §9 ref 8 but not analysed in §3 or §6.**
- Phase-1 brute-force recall and Phase-2 LSH planning are noted in the codebase. The memo should either (a) include a forward-looking statement that Smritidb's recall layer is *not* claimed (only the substrate is claimed) or (b) differentiate explicitly. Currently the memo is silent and leaves an attack surface.
- Threat to: any future recall-related dependent claim.

(12) **NIST CAVS / CAVP, FIPS 140-3.** The memo cites CAVS as the closest analogue for Claim 5 but does not give a precise reference. CAVP is published at https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program; the test-vector format and the contractual verification model are documented in FIPS 140-3 IG and in SP 800-140. The §6 row for Claim 5 needs a precise CAVS/CAVP citation.
- Threat to: Claim 5.

### 1.3 Verdict on Perspective 1

The memo's coverage of HDC and vector-database prior art is solid; its coverage of *adjacent* prior-art families (LSH/SimHash/Bloom/Merkle/HDF5/SQLite WAL) is **inadequate** and will be exploited at FER. At least categories (2), (3), (5), (8) and (10) are P0.

---

## Perspective 2 — §3(k) Defender

The §3(k) defence brief (`11-3k-defense.md`) organises every claim around a *system-level technical effect* (byte-identity, bounded drift, fault detection, footprint reduction, on-device feasibility). The prior-art memo's §6 differentiators must align with that framing or the two documents will be in tension.

**Findings:**

(a) **Claim 1 differentiator (§6 row 1):** As drafted, the differentiator reads "A cryptographic-hash-derived tie-resolution bit, computed deterministically over `(data, bit_index, count)`, that renders bundle output byte-identical across any conforming implementation." This is mostly system-property framing (good) but the operative description ("a cryptographic-hash-derived tie-resolution bit") still reads as an *algorithmic substitution* (BLAKE3 instead of PRNG). **Recommendation:** rewrite the differentiator to lead with the system property — "the technical effect of cross-implementation byte-identity of stored bundle outputs, achieved by ..." — and only then describe the mechanism. Mirror the language in `11-3k-defense.md` §3 Claim 1.

(b) **Claim 2 differentiator (§6 row 2):** Reads as a tripartite algorithmic property list (deterministic / bounded / replayable). All three are individually algorithmic. The §3(k) defence frames Claim 2 around "deterministic replay of a memory store's state from a snapshot together with an access log, with a provable upper bound on similarity drift" — i.e., a system-level audit-trail property. The prior-art memo should adopt the same framing.

(c) **Claim 3 differentiator (§6 row 3):** Mostly system-effect framing (fault detection, streaming load). Adequate.

(d) **Claim 4 differentiator (§6 row 4):** Reads as an algorithm-vs-algorithm comparison ("BLAKE3-seeded random projection per level"). The §3(k) defence emphasises the *technical effect of footprint reduction with bounded error envelope verifiable against a corpus*. The prior-art memo's differentiator should be rewritten to match.

(e) **Claim 5 differentiator (§6 row 5):** Already corpus-as-artefact framing. Adequate.

(f) **Claim 6 differentiator (§6 row 6):** The "training-free, on-device-feasible operation" subpart is good system-property framing. The "permutation-positional encoding with a permutation derived deterministically by BLAKE3-XOF" subpart still reads algorithmically. Reframe around *the on-device-feasibility technical effect* as the §3(k) defence does.

**Cross-cutting recommendation:** Every row of the §6 matrix should begin with the *technical effect* (one sentence) and only then describe the structural difference. As drafted, four of six rows lead with the structural difference, which is exactly the framing the §3(k) examiner can attack.

---

## Perspective 3 — Prior-art Hunter (peer-review of citations)

### 3.1 Citation completeness audit

| Reference | Status | Issue |
|---|---|---|
| Kanerva 1988 (§2.1) | OK | ISBN given. No page range cited; if relied on for a specific teaching (majority bundling, p. ~), a page-pin is needed. |
| Kanerva 2009 (§2.1) | OK | Volume/issue/pagination given. DOI missing: 10.1007/s12559-009-9009-8. |
| Plate 1995 (§2.2) | OK | DOI missing: 10.1109/72.377968. |
| Plate 2003 monograph (§2.2) | INCOMPLETE | Publisher and year only. ISBN: 1-57586-430-7. |
| Rachkovskij & Kussul 2001 (§2.3) | OK | DOI missing: 10.1162/089976601300014592. |
| Imani VoiceHD 2017 (§2.4) | INCOMPLETE | DOI missing: 10.1109/ICRC.2017.8123650. No page pin. |
| Imani DAC 2018 (§2.4) | `[TO VERIFY pagination]` flagged | Correctly flagged. The DAC 2018 proceedings cite is Article 108, 6 pages. DOI: 10.1145/3195970.3196060. |
| Imani HDCluster DATE 2019 (§2.4) | INCOMPLETE | DOI missing: 10.23919/DATE.2019.8714922. |
| Imani CLOUD 2019 (§2.4) | INCOMPLETE | DOI missing. |
| BLAKE3 (§2.5) | OK | URL given. The canonical "specification" is the v1.0 PDF at https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf. |
| Hebb 1949 (§2.6) | OK | No DOI needed for a 1949 monograph. |
| Ge & Parhi 2020 (§4.1) | OK | DOI: 10.1109/MCAS.2020.2978558. |
| Schlegel et al. 2022 (§4.2) | OK | DOI: 10.1007/s10462-021-10110-3. |
| Kleyko et al. 2022 (§4.3) | OK | DOI: 10.1109/JPROC.2022.3209104. |
| Karunaratne et al. 2020 (§4.4) | OK | DOI: 10.1038/s41928-020-0410-3. |
| Hersche et al. 2023 (§4.5) | OK | DOI: 10.1038/s42256-023-00630-8. |
| Frady-Kleyko-Sommer 2018 (§4.7) | `[TBD]` | Correctly flagged. *Neural Computation* 30(6): 1449-1513. DOI: 10.1162/neco_a_01084. |
| Thomas-Dasgupta-Rosing 2021 (§4.7) | `[TBD]` | Correctly flagged. *JAIR* 72: 215-249. DOI: 10.1613/jair.1.12664. |
| US 11,775,847 B2 (§5.2) | `[TO VERIFY]` | **CRITICAL — see 3.3 below.** |
| US 10,956,464 B1 (§5.2) | `[TO VERIFY]` | **CRITICAL — see 3.3 below.** |
| Malkov & Yashunin 2018 (§9 ref 8) | OK | DOI: 10.1109/TPAMI.2018.2889473. |
| Jégou et al. 2011 (§9 ref 9) | OK | DOI: 10.1109/TPAMI.2010.57. |

### 3.2 Unflagged citations that should be flagged

(a) **IISc Bangalore and IIT Bombay HDC publications (§4.6).** Correctly flagged. Action: either cite specific papers or remove the paragraph. *Vague allusions to unverified prior art weaken the memo.*

(b) **"Other US patents assigned to Microsoft, Meta, IBM and Intel" (§5.2 final bullet).** This is a vague statement that should *either* enumerate specific patents (with numbers) *or* be removed. As drafted, it suggests the inventor is aware of relevant US patents but has not searched them — a worst-of-both-worlds posture.

(c) **"A free-text InPASS search ... returned no granted Indian patents that read on any of the six claims" (§5.1).** This statement is asserted but is contradicted three lines later by "A full claim-by-claim reading of the top fifty hits remains to be done." If the top fifty have not been read, the no-hit assertion is premature. **Recommendation:** soften to "preliminary title-and-abstract scan of approximately N top InPASS hits suggests no obvious anticipation; formal claim-by-claim reading deferred to the patent agent."

(d) **"Verified against well-known scholarly sources" (§11).** This sentence is too sweeping. It implies all citations in the memo have been read and verified against the original. Given the `[TO VERIFY]` flags throughout, this is inconsistent. The §11 statement should be limited to the citations that *have* in fact been verified, and the others enumerated.

### 3.3 Critical [TO VERIFY] items

**US 11,775,847 B2 (Imani et al., Regents of the University of California).** The memo asserts this patent exists and that it covers HDC classifier inference. The reviser **must** fetch the patent (USPTO Patent Public Search at https://ppubs.uspto.gov/) and read claims 1, 10 and any other independent claims, and verify (i) the patent number, (ii) the date of grant, (iii) the assignee, (iv) the claim scope. **If the patent number is wrong, the entire §5.2 paragraph is unreliable.** Equally importantly: there are *many* Imani-named HDC patents; the reviser should run an inventor search at https://ppubs.uspto.gov/pubwebapp/external.html?db=USPAT&q=IN/"Imani,+Mohsen" and triage *all* Imani patents, not just one.

**US 10,956,464 B1 (Pinecone Systems).** Same procedure — verify number, claim scope, and confirm it is an HNSW-variant index. Pinecone has filed several patents; the reviser should run an assignee search and triage all of them.

**Recommendation:** A subsection 5.2.X should be added giving the methodology for the inventor / assignee searches, the date the search was run, and the count of hits returned. Without that audit trail, the §5.2 conclusions are not defensible at FER.

### 3.4 Misrepresentation check

(a) **§2.4 (iii)**: "Imani's level hypervectors are not part of any standardised persistence format." Strictly correct, but Imani's published code typically uses NumPy `.npy` files for storage; the statement should be "no Imani publication discloses a wire format specifying byte-identity across implementations." Sharper, less open to rebuttal.

(b) **§3.7 ("Common gap across all six")**: "None of the six ships a published bit-exact conformance corpus." Correct as of present knowledge — but the reviser should *check the documentation pages of each of the six vendors* and record the date of the check. Anything could have changed.

(c) **§6 row 5 (Conformance corpus)**: "no analogue in vector-database art identified." This is correct, but Pinecone-style integration test suites and the FAISS `bench_all_ivf` benchmarks exist; the memo should distinguish on grounds that those are *benchmarks* not *byte-identity corpora*.

---

## Perspective 4 — Indian Patent Attorney

### 4.1 P0 blocker — Indian landscape (§5.1)

**The §5.1 finding is currently `[TBD — formal search by patent agent before filing.]`** This is the single most important section of the memo for an IPO filing. Without a properly documented InPASS search (queries, dates, screen-captures of result lists, claim-by-claim triage of top hits), the Form-3 statement to the Controller cannot be substantiated and any FER objection citing Indian prior art will be unanswerable.

**Recommendation:**
- The InPASS search must be run *before* Form-2 is filed, not in response to an FER.
- The memo must record: (i) the exact query strings (the §1.5 list is good but must be extended to include InPASS Boolean syntax variations); (ii) the date of each search; (iii) the count of hits; (iv) a list of the top 20 hits with patent number, title, abstract, applicant, status; (v) for each of those 20, a one-line claim-by-claim differentiation.
- The memo should also search the Indian *applications* (not just grants), because pending applications can be cited as prior art under §13 of the Patents Act.
- The memo should record the date the search was run and explicitly acknowledge the "rolling-publication" nature of InPASS (new publications appear weekly).

### 4.2 Claim-by-claim differentiation matrix (§6) — gaps

(a) **Closest prior art column.** The matrix gives one or two references per row. For an FER response, the patent agent will want *the single closest reference per claim* (the "X" reference in EPO terminology). The matrix as drafted blurs the closest reference with combined references. **Recommendation:** split the column into "Closest single reference (anticipation challenge)" and "Closest combination (obviousness challenge)".

(b) **Row 1 (Claim 1).** The "Kanerva 1988" cite is correct but the *passage* in Kanerva 1988 dealing with tie resolution (around Chapter 3, the bundle / majority operation) should be page-pinned. Also: the row should add **Imani HPCA 2017** ("Exploring Hyperdimensional Associative Memory") which the §3(k) defence cites as the closest Imani art for Claim 1. The two filing documents must be consistent.

(c) **Row 2 (Claim 2).** Missing: **SQLite WAL** as the closest prior art for "replayable from a log". Without addressing WAL, the inventive step shrinks.

(d) **Row 3 (Claim 3).** Missing: **HDF5 (Fletcher32 filter, chunked layout)** and **Git/IPFS (Merkle-tree content addressing)** as the closest prior art for per-block integrity. The current cite list is Arrow / Parquet / HDF5 / MessagePack / pickle — but the analysis stops at the list.

(e) **Row 4 (Claim 4).** Missing: **Charikar 2002 (sign-LSH / SimHash)** and **Achlioptas 2003 ("Database-friendly Random Projections")** which the §3(k) defence cites. The two filing documents must be consistent. Achlioptas in particular is the canonical reference for *deterministic* random projection via sparse sign matrices.

(f) **Row 5 (Claim 5).** Adequate, but a precise CAVS/CAVP cite is needed (see §1.2 (12) above).

(g) **Row 6 (Claim 6).** Missing: **SimHash (Charikar 2002)** and **MinHash (Broder 1997)** as the closest deterministic-binary-fingerprint prior art for text. Also missing: classical character-n-gram bag-of-features which the §3(k) defence cites correctly. Two-document inconsistency.

(h) **General.** No row identifies the "differentiating limitation" as a discrete, copyable claim element. **Recommendation:** add a fourth column "Differentiating limitation (claim-element form)" giving exactly the words that should appear in the corresponding claim, so the patent agent can directly trace the §6 analysis to the §8 claim language.

### 4.3 §3(d) considerations

The memo addresses §3(k) extensively but is silent on **§3(d)** ("the mere discovery of a new form of a known substance which does not result in the enhancement of the known efficacy") for the format and consolidation claims. While §3(d) is primarily a pharmaceutical doctrine, the IPO has applied a generalised "new use of known method" reading in some software cases. The risk row in §8 mentions §3(d) for Claim 5 only. **Recommendation:** add §3(d) risk analysis for Claims 1, 2, 3, 4 expressly (BLAKE3 + HDC = "new use of BLAKE3"? IPO will not buy the §3(d) attack but the response should be on file).

---

## Perspective 5 — Domain Critic

### 5.1 Imani 2017 attribution

The memo cites "Imani 2017" for thermometer encoding in §2.4 and the §6 row 4 differentiator. **Verification:** thermometer encoding for HDC bounded floats is published in Imani et al. (2017), "VoiceHD" — correct attribution for the *level-hypervector* encoder. The deeper algorithmic discussion of thermometer / level hypervectors is in:
- Imani, M., Hwang, J., Rosing, T., Rahimi, A. and Rabaey, J. M. (2018). "Low-Power Sparse Hyperdimensional Encoder for Language Recognition." *IEEE Design and Test* 34(6): 94-101. — closer to the encoder used.
- Rahimi, A., Kanerva, P. and Rabaey, J. M. (2016). "A robust and energy-efficient classifier using brain-inspired hyperdimensional computing." *ISLPED 2016*: 64-69. — the *original* level-hypervector / continuous-value encoding for HDC, predating Imani.

**Finding:** Rahimi 2016 (and Rahimi, Datta, Kanerva, Rabaey 2016, "High-Dimensional Computing as a Nanoscalable Paradigm," *IEEE TCAS-I* 64(9): 2508-2521) is the more accurate prior-art anchor for level / thermometer encoding than Imani 2017. The Rahimi line of work is *closer prior art* than Imani's, and the memo cites *neither* Rahimi paper. **This is a substantive misattribution that an examiner will spot.**

**Recommendation:** Add Rahimi 2016 (ISLPED) and Rahimi-Datta-Kanerva-Rabaey 2017 (TCAS-I) to §2.4 as the foundational level-encoder references, and adjust the §6 row 4 to cite Rahimi as the closest reference (with Imani 2017 as the most-prominent follow-on).

### 5.2 HDC paper attribution accuracy

(a) **Kanerva 2009 (§2.1).** Correctly characterised as the introduction-to-HDC review.

(b) **Plate 1995 (§2.2).** Correctly characterised as HRR.

(c) **Rachkovskij & Kussul 2001 (§2.3).** Correctly characterised (binding by thinning).

(d) **Schlegel et al. 2022 (§4.2).** Correctly characterised as a VSA comparison; *also* discusses binary substrates including BSC, which is directly relevant to Claim 1 / Claim 6. Worth a sentence noting that.

(e) **Karunaratne et al. 2020 (§4.4).** Correctly characterised. However, the *Nature Electronics* paper also discusses *bit-exact reproducibility* in the context of PCM read noise — relevant to Claim 1's framing. Worth a sentence.

(f) **Hersche et al. 2023 (§4.5).** Correctly characterised. Tangential to the claims.

(g) **Ge & Parhi 2020 (§4.1).** Correctly characterised as a survey.

(h) **Kleyko et al. 2022 (§4.3).** Correctly characterised as a Proceedings-of-the-IEEE position paper. It does discuss persistence and reproducibility at a high level — worth a forward-looking citation.

### 5.3 HNSW (Malkov & Yashunin 2018)

HNSW is listed in §9 as ref 8 but is *not analysed* anywhere in §2–§6. Given that:
- the Smritidb codebase uses brute-force recall in Phase 1, and
- the design notes contemplate LSH (Phase 2) for approximate recall,

HNSW is the closest *graph-based ANN* prior art, and the memo should explicitly state that Smritidb **does not currently claim a recall index** and so HNSW is non-conflicting. If the Phase-2 LSH layer is to be claimed in a continuation application, that should be flagged in §10.

**Recommendation:** add a one-paragraph §3.X analysing HNSW as adjacent-not-conflicting prior art, and pre-empt the Phase-2 LSH continuation by noting "additional dependent or continuation claims directed to an LSH or graph-based recall index are out of scope of the present application."

### 5.4 Other HDC-domain prior art the memo overlooks

(a) **Kanerva 1996 ("Binary Spatter Codes," ICANN 1996).** The canonical BSC reference. The memo refers to BSC parenthetically in §2.2 but does not cite Kanerva 1996.

(b) **Gayler 1998 ("Multiplicative Binding, Representation Operators & Analogy," AAAI Fall Symposium).** Foundational VSA work on binding operators. Should be in §2.

(c) **Kleyko, Rahimi, Rachkovskij, Osipov, Rabaey (2018), "Classification and Recall with Binary Hyperdimensional Computing: Tradeoffs in Choice of Density and Mapping Characteristics," *IEEE TNNLS* 29(12): 5880-5898.** Direct prior art on binary HDC tradeoffs. Should be in §4.

(d) **Neubert, Schubert, Protzel (2019), "An Introduction to Hyperdimensional Computing for Robotics," *KI – Künstliche Intelligenz* 33: 319-330.** Tangential but worth a mention.

(e) **Mitrokhin, Sutor, Fermüller, Aloimonos (2019), "Learning sensorimotor control with neuromorphic sensors: Toward hyperdimensional active perception," *Science Robotics* 4(30).** HDC robotics application; tangential.

### 5.5 Verdict on Perspective 5

The HDC-domain coverage is *adequate but stale by ~3 years*: the memo over-relies on Imani 2017-2019, missing the Rahimi-anchor of level encoding and missing the post-2020 binary-HDC literature (Kleyko et al. 2018; Schlegel et al. 2022 fuller treatment). The most consequential correction is **Rahimi 2016 (ISLPED) as the level-encoder prior art**, which directly affects Claim 4's differentiation.

---

## Consolidated Revision Worklist

The following list is ordered by priority (P0 = blocker before filing; P1 = important; P2 = nice-to-have).

### P0 — must be completed before Form-2 filing

1. **Run a formal InPASS search and document the results in §5.1.** Replace the `[TBD]` with: query strings, dates, result counts, top-20 hit list, per-hit one-line differentiation. (Perspective 4 §4.1)

2. **Retrieve and read US 11,775,847 B2 and US 10,956,464 B1** (and the broader Imani / Pinecone patent portfolios) and update §5.2 with verified numbers, dates, assignees and a per-claim differentiation. Document the search methodology and date. (Perspective 3 §3.3)

3. **Add the missing prior-art categories** (Perspective 1 §1.2):
   - LSH / SimHash (Charikar 2002; Datar et al. 2004; Andoni & Indyk 2008) — affects Claims 1, 4, 6.
   - Rahimi 2016 (ISLPED) and Rahimi et al. 2017 (TCAS-I) — affects Claim 4.
   - HDF5 (Fletcher32 filter, chunking) — affects Claim 3.
   - Apache Parquet (magic footer, footer-first read) — affects Claim 3.
   - SQLite WAL — affects Claim 2.
   - Merkle trees / Git / IPFS content addressing — affects Claim 3.
   - Bloom filters / Count-Min / MinHash — affects Claims 1, 4.
   - SimHash text fingerprinting — affects Claim 6.
   - Kanerva 1996 (BSC) — foundational, affects Claims 1, 6.
   - NIST CAVS/CAVP precise reference (SP 800-140 series) — affects Claim 5.

4. **Reconcile the prior-art memo with `11-3k-defense.md`.** The §3(k) defence cites Imani HPCA 2017, Achlioptas 2001, Mikolov 2013, Pennington 2014, Devlin 2019. The prior-art memo cites none of these. The two filing documents must use the same closest-prior-art list. (Perspective 2; Perspective 4 §4.2)

5. **Reframe §6 differentiator language to lead with technical effect, not algorithmic mechanism.** Specifically rewrite rows 1, 2, 4 and 6. Mirror the language used in §3 of `11-3k-defense.md`. (Perspective 2)

### P1 — important; complete before agent review

6. **Add a "Closest single reference vs closest combination" split column to §6.** This is the form examiners use; presenting the matrix in this form will accelerate FER response. (Perspective 4 §4.2 (a))

7. **Add a "Differentiating limitation in claim-element form" column to §6.** Trace each differentiator to the exact words in `08-claims.md`. (Perspective 4 §4.2 (h))

8. **Add a §3.X paragraph on HNSW as adjacent-not-conflicting prior art** and pre-empt the Phase-2 LSH continuation. (Perspective 5 §5.3)

9. **Add DOIs for all academic references** per the audit table in Perspective 3 §3.1. Add page pins where any specific teaching is relied upon (Kanerva 1988 Chapter 3 majority operation; Plate 1995 §III binding via convolution; etc.).

10. **Soften the §5.1 "no Indian prior art identified to date" assertion** to "preliminary title-and-abstract scan of top-N InPASS hits returned no apparent anticipation; formal claim-by-claim reading is being undertaken by the patent agent." (Perspective 3 §3.2 (c))

11. **Remove or specify the vague reference to "other US patents assigned to Microsoft, Meta, IBM and Intel"** (§5.2 final bullet). Either name them or remove. (Perspective 3 §3.2 (b))

12. **Tighten the §11 verification statement** to enumerate exactly which citations have been verified against the original and which have not. (Perspective 3 §3.2 (d))

13. **Add §3(d) risk rows for Claims 1, 2, 3, 4 in §8.** Currently only Claim 5 has a §3(d) risk row. (Perspective 4 §4.3)

### P2 — nice-to-have; can be deferred

14. **Add Kanerva 1996 (BSC), Gayler 1998, Kleyko et al. 2018 (TNNLS), Plate 2003 monograph (with ISBN) to §2 / §4** as foundational background. (Perspective 5 §5.4)

15. **Add a sentence in §4.2 and §4.4 noting where Schlegel 2022 and Karunaratne 2020 do discuss binary-substrate / reproducibility issues** to pre-empt examiner combinations. (Perspective 5 §5.2 (d), (e))

16. **Date-stamp the vendor-documentation check** for Pinecone / Qdrant / Weaviate / FAISS / Milvus / Chroma in §3 (and re-check at filing time). (Perspective 3 §3.4 (b))

17. **Add CBOR (RFC 8949), FlatBuffers, Cap'n Proto, Bencode, Protobuf as enumerated comparators in §6 row 3** even if only to dismiss. (Perspective 1 §1.2 (6), (7), (9))

18. **Expand §1.5 query strings to include** the InPASS-specific Boolean syntax variations and add explicit queries for LSH, SimHash, Bloom-filter, Merkle-tree adjacency. (Perspective 4 §4.1)

---

## Report — summary for caller

**Missing prior-art categories identified (in order of threat severity):**

1. LSH / SimHash / sign-LSH (Charikar 2002; Datar et al. 2004; Andoni & Indyk 2008) — central threat to Claim 4, also to Claim 1 and Claim 6.
2. Rahimi 2016 ISLPED and Rahimi et al. 2017 TCAS-I — the *actual* foundational level-encoder prior art that the memo misattributes to Imani 2017.
3. SQLite WAL and write-ahead-logging literature — central threat to Claim 2 (replayable consolidation).
4. HDF5 Fletcher32 filter and Parquet magic footer — direct analogues for Claim 3's per-block integrity and footer-first streaming load.
5. Merkle trees / Git / IPFS content-addressed storage — direct prior art for per-block cryptographic integrity.
6. Bloom filters / Count-Min sketches / MinHash — adjacent probabilistic-binary-substrate prior art relevant to Claim 1.
7. Kanerva 1996 Binary Spatter Codes — foundational binary-HDC reference completely missing.
8. NIST CAVS/CAVP precise SP 800-140 reference — required for Claim 5.
9. CBOR / FlatBuffers / Cap'n Proto / Bencode — completeness gap for Claim 3 enumerated comparators.

**Critical [TO VERIFY] items the reviser must complete before filing:**

1. **US 11,775,847 B2 (Imani et al.)** — verify number, date, assignee, claim scope by retrieving from USPTO Patent Public Search. Run a full Imani inventor search and triage all results.
2. **US 10,956,464 B1 (Pinecone Systems)** — verify number and claim scope. Run a Pinecone assignee search.
3. **Indian Patent Office InPASS search (§5.1)** — currently `[TBD]`; this is a P0 blocker for filing. The memo cannot be relied upon for Form-3 / FER response without this being completed and documented.
4. **Imani DAC 2018 pagination** — flagged correctly in memo; action: confirm Article 108 / 6 pages / DOI 10.1145/3195970.3196060.
5. **IISc Bangalore and IIT Bombay HDC publications** (§4.6) — either cite specific papers or remove the paragraph.
6. **§11 sweeping "verified against well-known scholarly sources" claim** — narrow to enumerated list of actually-verified citations.

**Documents touched:** none outside `patent/critiques/round-2/`. No commits made.
