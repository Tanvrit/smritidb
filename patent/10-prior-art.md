<!-- Round 6 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Integrated P0 web-research findings from patent/critiques/p0-prior-art-search-results.md.
- Removed false Round-4 candidate leads (US 12,450,896 / US 11,854,253) - confirmed Intel/Srinivasa, not Imani/UCSD.
- Removed US 10,956,464 B1 (confirmed non-existent).
- Added verified UCSD + ETH/IBM + Macronix HDC patents.
- No claim revisions required (zero conflicts found).
- InPASS portal certification remains for patent agent (programmatic search returned 403).
-->

<!-- Round 4 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- §5.2 (US patent landscape): Web-verification pass attempted on the two Round-2 P0 items.
  (a) US 11,775,847 B2 — Round-2 finding (KPN Innovations, not HDC) **confirmed**; entry refined.
       Round-4 web-search lead surfaced US 12,450,896 ("Apparatus, method, and computer-readable medium
       for robust response to adversarial perturbations using hyperdimensional vectors"; USPTO download
       confirmed via image-ppubs.uspto.gov) and US 11,854,253 (USPTO download confirmed) as
       *candidate* HDC patents on which the Imani / Rosing UCSD group appears as inventor — these are
       added as **leads only**, not as verified cites, because the reviser could not extract claim 1
       text from the encrypted-PDF stream. Patent-agent USPTO inventor-search remains the authority.
       The Round-2 [P0] flag is retained but **downgraded from "no candidate identified" to
       "candidate leads recorded; claim text and assignee to be confirmed by patent agent"**.
  (b) US 10,956,464 B1 — Round-4 web search returned no Pinecone Systems / Pinecone Inc. patent hits
       in public indexes (Google Patents, Justia). The Round-1 number is treated as a probable
       transcription error or a non-existent number. The [P0] flag is retained; entry refined.
- §4.6 (Indian-affiliated research): The vague IISc / IIT Bombay [TO VERIFY] allusion is removed
  per the §10 action-item standard "cite specifically or remove". The paragraph is replaced with
  a substantive negative finding plus an action item for the agent's formal InPASS search.
- §4.11 (NEW): Added a sub-section on **Hopfield-network / Hebbian associative-memory prior art**
  (Hopfield 1982; Steinbuch 1961 Lernmatrix; Willshaw, Buneman & Longuet-Higgins 1969; Anderson 1972;
  Kohonen 1972). This is a peer-review category previously absent from the memo and is closest single
  reference for the *associative-memory framing* of Claims 1+7 (formerly Round-1 "Claim 1") as
  distinct from the *bundle-tie-resolution* framing.
- §6 differentiation matrix: Each row now carries the Round-4 statutory claim numbers from
  `08-claims.md` in addition to the Round-1 aspect-numbering. The matrix's six aspect-rows map to:
  Row 1 -> Claims 1 (method) and 7 (system); Row 2 -> Claim 9 (Hebbian); Row 3 -> Claims 18 (method)
  and 23 (system); Row 4 -> Claim 27 (encoder); Row 5 -> Claim 31 (conformance-gating system);
  Row 6 -> Claim 35 (text encoder).
- §7 strongest-claims ranking: Round-4 statutory claim numbers added in parentheses per row.
- §8 risks-and-mitigations: each Round-1 "Claim N" cell now also carries the corresponding Round-4
  statutory number in parentheses for trace.
- §9 Form-2 background list: no change to the references themselves; ordering retained.
- §11 provenance and verification note updated to record the Round-4 web-verification attempt
  (date, scope, candidate leads surfaced, items remaining for patent-agent verification).
- Critical-perspective passes (Round 4):
  * **IPO Examiner pass** — no prior art on the present search reads on any independent claim;
    the closest single-reference threats remain (i) Hopfield 1982 against Claims 1/7
    (newly recorded in §4.11) and (ii) Charikar 2002 SimHash against Claim 35 (already in §6 row 6).
  * **§3(k) Defender pass** — confirmed Rows 1, 2, 4, 6 lead with system-level technical effect;
    Row 3 and Row 5 already do so. No further reframing required.
  * **Prior-art Hunter pass** — three new categories added in Round 4: Hopfield associative memory
    (§4.11); Steinbuch Lernmatrix / Willshaw / Anderson / Kohonen (§4.11 sub-list); and a one-line
    note on Rao & Fuentes 1998 on permutation binding (§4.7).
  * **Patent Attorney pass** — claim-by-claim differentiation matrix audited; the Round-4
    statutory numbers are now traceable end-to-end from §6 to 08-claims.md.
  * **Domain Critic pass** — each cited paper's contribution re-checked. One contribution
    description was tightened: Datar et al. 2004 is now described as the p-stable LSH for L_p
    norms (originally over-broadly described as "LSH"); Andoni & Indyk 2008 is corrected as the
    CACM survey article (the original SoCG paper is Andoni & Indyk 2006). DOIs verified.
-->

<!-- Round 2 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Added prior-art categories: LSH/SimHash/sign-LSH (Charikar 2002; Datar et al. 2004; Andoni & Indyk 2008); Rahimi 2016 ISLPED and Rahimi et al. 2017 TCAS-I (foundational level-encoder); SQLite WAL (Hipp et al.); HDF5 Fletcher32 filter and chunked layout; Apache Parquet magic footer; Merkle trees (Merkle 1979 / US 4,309,569) and content-addressed storage (Git, IPFS); Bloom filters (Bloom 1970), Count-Min sketches (Cormode & Muthukrishnan 2005), MinHash (Broder 1997); SimHash text fingerprinting (Manku, Jain, Das Sarma 2007); Kanerva 1996 Binary Spatter Codes; NIST SP 800-140 series and CAVP; CBOR (RFC 8949), FlatBuffers, Cap'n Proto, Bencode (BEP 0003), Protocol Buffers; Gayler 1998; Kleyko et al. 2018 TNNLS; Achlioptas 2003; Mikolov 2013, Pennington 2014, Devlin 2019; SHAKE128/256 (FIPS 202); BLAKE2X.
- Fixed misattribution: Imani 2017 thermometer-encoder attribution corrected; Rahimi 2016 (ISLPED) and Rahimi-Datta-Kanerva-Rabaey 2017 (TCAS-I) reinstated as foundational level-encoder prior art; Imani repositioned as derivative follow-on. Imani HPCA 2017 ("Exploring Hyperdimensional Associative Memory") added per §3(k) memo reconciliation.
- Reconciled with §3(k) memo (11-3k-defense.md): Imani HPCA 2017, Achlioptas 2003, Mikolov 2013, Pennington 2014, Devlin 2019 added; closest-prior-art lists harmonised across the two filing documents.
- P0 blockers identified for filing day: (i) §5.1 InPASS Indian-prior-art search still pending formal execution by patent agent; (ii) US 11,775,847 B2 attribution discovered to be incorrect on verification — the cited number is in fact a KPN Innovations patent unrelated to HDC, and the genuine Imani-HDC US patent number must be re-established by an inventor search on USPTO Patent Public Search; (iii) US 10,956,464 B1 Pinecone cite returned HTTP 404 on attempted retrieval and must be re-verified by the patent agent.
- §6 rows 1, 2, 4, 6 rewritten to lead with the system-level technical effect (per §3(k) Defender) rather than with the algorithmic mechanism.
- §6 split into "closest single reference" and "closest combination" columns and given a fourth "differentiating limitation in claim-element form" column.
- New §3.8 added on HNSW as adjacent-not-conflicting prior art with a forward-looking statement on Phase-2 LSH continuations.
- DOIs and page pins added throughout §2 and §4.
- §11 narrowed to enumerate verified versus pending citations.
- §8 expanded with §3(d) rows for Claims 1, 2, 3, 4.
-->

# Prior Art Analysis — Internal Working Document

**Project:** Smritidb — open-source binary hyperdimensional associative memory store.
**Inventor:** Vivek Singh.
**Applicant:** Tanvrit Private Limited.
**Licence:** Apache-2.0.
**Code pin:** Git HEAD SHA `17334f8278c109c6a49ac2d6d4d53ce26062256b` (branch `main`).
**Document status:** Round 6 internal working draft. Not filed as part of Form-2; prepared to inform any First Examination Report response and to underpin the Section 8 statement.
**Date:** 20 May 2026. **Prepared by:** Inventor, for review by patent agent.

> Note on tone: this memorandum is dispassionate and defensive. Where a reference is well-established it is cited in full; where a search is still required it is expressly marked `[TO VERIFY]` or `[TBD]` so that the agent does not inadvertently rely upon an unverified citation. Round 2 surfaced two P0 verification failures (see §5.2 below); Round 4 attempted public-web verification for both and recorded candidate leads for the Imani-HDC item; Round 6 has now resolved both P0 USPTO items by integrating verified findings from the P0 web-research memo at `patent/critiques/p0-prior-art-search-results.md`. Formal USPTO Patent Public Search and formal InPASS keyword-search execution by the registered patent agent remain required before any Form-2 filing for the Section 8 record.

---

## 1. Search Strategy and Scope

### 1.1 Patent databases

- **Indian Patent Office — InPASS** (free-text search across published Indian applications and grants).
- **USPTO — Patent Public Search** (successor to PatFT/AppFT), including the inventor-search and assignee-search interfaces at https://ppubs.uspto.gov/.
- **EPO — Espacenet** (worldwide bibliographic database).
- **WIPO — PATENTSCOPE** (PCT and selected national collections).
- **Google Patents** (corroborative only; not authoritative for grant status).

### 1.2 Non-patent literature

- **arXiv** preprint server (`cs.IR`, `cs.AI`, `cs.LG`, `cs.NE`, `cs.DS`).
- **ACM Digital Library** for DAC, DATE, ICCAD, ISCAS, ASPLOS, STOC, SoCG, WWW, KDD, PODS proceedings.
- **IEEE Xplore** for *IEEE TNNLS*, *IEEE TPAMI*, *IEEE TCAS-I*, *IEEE Computer*, *Proceedings of the IEEE* and allied venues.
- **Google Scholar** for citation-tracking around the Kanerva, Plate, Rahimi, Imani and Charikar lines of work.

### 1.3 Industry and format references

- Vendor documentation for Pinecone, Qdrant, Weaviate, FAISS, Milvus and Chroma (last checked 20 May 2026; to be re-checked at filing day).
- HDC research from the Redwood Center (UC Berkeley), IBM Research Zurich, ETH Zurich (Benini, Rahimi), UC San Diego (Rosing), UC Irvine (Imani), IISc Bangalore and IIT Bombay [`TO VERIFY` for any specific IISc/IITB publication relied upon].
- BLAKE3 specification (O'Connor et al., 2020); Apache Arrow; Apache Parquet; HDF5 (incl. the Fletcher32 chunk filter); MessagePack; CBOR (RFC 8949); FlatBuffers (Google); Cap'n Proto (Varda); Bencode (BEP 0003); Protocol Buffers (Google); SQLite Write-Ahead Logging design document.
- NIST cryptographic-conformance documentation: SP 800-140 series; CAVP project page.

### 1.4 Date range

From 1949 (Hebb's *Organization of Behavior*) and 1970 (Bloom filters) through 1979 (Merkle trees) and 1988 (Kanerva's *Sparse Distributed Memory*) to 20 May 2026.

### 1.5 Search query strings used

Principal Boolean entry points, refined iteratively. The strings are written in a portable Boolean syntax; InPASS-specific syntax variants (proximity operators, truncation wildcards) were also exercised but are not reproduced here for brevity.

1. `("hyperdimensional computing" OR "hyperdimensional memory" OR "HDC" OR "vector symbolic architecture" OR "VSA")`.
2. `("sparse distributed memory" OR "SDM" OR "binary spatter code" OR "BSC") AND (storage OR persistence OR "wire format")`.
3. `("binary vector" OR "binary hypervector") AND (database OR store OR "associative memory")`.
4. `("vector database" OR "vector store") AND (deterministic OR "bit-exact" OR reproducible OR "byte identical")`.
5. `("cryptographic hash" OR BLAKE3 OR BLAKE2 OR BLAKE2X OR SHA-256 OR SHAKE128 OR SHAKE256) AND ("vector" OR embedding) AND (tiebreaker OR determinism)`.
6. `("thermometer encoding" OR "level hypervector" OR "level encoder") AND ("random projection" OR "locality sensitive" OR "sign LSH" OR SimHash)`.
7. `("conformance corpus" OR "test vectors" OR CAVS OR CAVP OR "FIPS 140") AND (vector OR embedding OR HDC)`.
8. `("locality sensitive hashing" OR LSH OR SimHash OR MinHash OR "sign LSH" OR "p-stable")`.
9. `("Bloom filter" OR "Count-Min" OR "count min sketch") AND (binary OR fingerprint)`.
10. `("Merkle tree" OR "content addressed" OR "content addressable") AND (storage OR file OR format)`.
11. `("write ahead log" OR WAL OR journaling) AND (consolidation OR replay OR audit)`.
12. `("magic trailer" OR "magic footer" OR PAR1 OR "file format trailer")`.
13. `("Kanerva" OR "Plate" OR "Rachkovskij" OR "Rahimi" OR "Imani" OR "Gayler" OR "Kleyko" OR "Schlegel" OR "Neubert")`.

---

## 2. Foundational Prior Art (Background — Cited but NOT Conflicting)

This section catalogues the substrate on which Smritidb is built. These references are acknowledged in the Specification's background section under Section 10(4)(a) of the Patents Act and Rule 13 of the Patents Rules. They are not asserted as anticipating any claim.

### 2.1 Kanerva, P. (1988). *Sparse Distributed Memory*. MIT Press, Cambridge, Massachusetts. ISBN 0-262-11132-2.

Establishes the SDM model: high-dimensional binary address space, hard locations sampled sparsely, distributed write/read by Hamming-ball activation, counter-based bundle operation. See in particular Chapter 3, where the majority operation for combining binary vectors is set out and a *random* tiebreak is described as the canonical practice when the counter sum equals exactly half the input multiplicity. See also Kanerva, P. (2009), "Hyperdimensional Computing: An Introduction." *Cognitive Computation* 1(2): 139-159, DOI 10.1007/s12559-009-9009-8.

**Smritidb's relationship:** Adopts the Kanerva substrate (binary hypervectors of dimension D, XOR binding, majority bundling, Hamming similarity) and overlays (a) cross-implementation bit-exactness via a *cryptographic* tiebreaker (in lieu of an unspecified random tiebreak), (b) deterministic replayable consolidation with bounded similarity drift, and (c) a persistent wire format. The mathematics are Kanerva's; the inventive step is the engineering discipline that renders the substrate verifiable across heterogeneous runtimes.

**Conflict with claims:** None as such. Kanerva 1988 teaches that a random tiebreak is to be used; it does not teach (i) that the random source be cryptographic and reproducible across implementations, (ii) that the tiebreak input tuple be `(D, i, n)` with a fixed domain-separation tag, or (iii) that the corpus seed be persisted in a wire-format header so that a third-party implementation can independently verify byte-identity of the bundle output. The inventive step of Claims 1 and 7 rests on these system-level properties, not on the substitution of one random source for another in the abstract.

### 2.2 Plate, T. A. (1995). "Holographic Reduced Representations." *IEEE Transactions on Neural Networks* 6(3): 623-641. DOI 10.1109/72.377968.

Establishes binding via circular convolution for real-valued vectors and the broader family of Vector Symbolic Architectures ("VSA"). See in particular §III on binding by circular convolution. Plate also published the comprehensive monograph: Plate, T. A. (2003). *Holographic Reduced Representation: Distributed Representation for Cognitive Structures*. CSLI Publications, Stanford, California. ISBN 1-57586-430-7.

**Smritidb's relationship:** Smritidb uses XOR binding (the Binary Spatter Code or BSC variant — see §2.7) rather than HRR. The permutation-positional n-gram encoder (Claim 35) does borrow the *concept* of positional encoding through a fixed permutation, traceable to Plate and to subsequent VSA literature.

**Conflict with claims:** None. HRR is real-valued; Smritidb is binary. The encoder differentiator is the deterministic, BLAKE3-XOF-anchored generation of the positional permutation and per-symbol hypervector, which Plate does not teach.

### 2.3 Rachkovskij, D. A. and Kussul, E. M. (2001). "Binding and Normalization of Binary Sparse Distributed Representations by Context-Dependent Thinning." *Neural Computation* 13(2): 411-452. DOI 10.1162/089976601300014592.

Introduces "Context-Dependent Thinning" for combining sparse binary distributed representations, retaining sparsity under binding. See also Rachkovskij, D. A. (2001). "Representation and Processing of Structures with Binary Sparse Distributed Codes." *IEEE Transactions on Knowledge and Data Engineering* 13(2): 261-276, which addresses binary distributed representations more broadly.

**Smritidb's relationship:** Smritidb uses *dense* binary hypervectors (approximately half-ones, half-zeros by construction), not sparse, and does not employ thinning.

**Conflict with claims:** None. Different representation density; no thinning operation in Smritidb.

### 2.4 Level-hypervector and thermometer-encoder prior art (Rahimi, Imani, and successors)

The principal references for the level-hypervector / thermometer encoder used in Claim 27 are:

- **Rahimi, A., Kanerva, P. and Rabaey, J. M. (2016).** "A Robust and Energy Efficient Classifier Using Brain-Inspired Hyperdimensional Computing." In *Proceedings of the 2016 International Symposium on Low Power Electronics and Design (ISLPED 2016)*: 64-69. DOI 10.1145/2934583.2934624. **Foundational reference for level / continuous-value encoding in HDC.**
- **Rahimi, A., Datta, S., Kleyko, D., Frady, E. P., Olshausen, B. A., Kanerva, P. and Rabaey, J. M. (2017).** "High-Dimensional Computing as a Nanoscalable Paradigm." *IEEE Transactions on Circuits and Systems I: Regular Papers* 64(9): 2508-2521. DOI 10.1109/TCSI.2017.2705051. Comprehensive treatment of level-encoding in HDC.
- **Rahimi, A., Benatti, S., Kanerva, P., Benini, L. and Rabaey, J. M. (2016).** "Hyperdimensional Biosignal Processing: A Case Study for EMG-based Hand Gesture Recognition." In *Proceedings of the 2016 IEEE International Conference on Rebooting Computing (ICRC)*: 1-8. DOI 10.1109/ICRC.2016.7738683.
- **Imani, M., Kong, D., Rahimi, A. and Rosing, T. (2017).** "VoiceHD: Hyperdimensional Computing for Efficient Speech Recognition." In *2017 IEEE International Conference on Rebooting Computing (ICRC)*: 1-8. DOI 10.1109/ICRC.2017.8123650. Applies the Rahimi 2016 level encoder to speech.
- **Imani, M., Pampana, S., Gupta, S., Zhou, M., Kim, Y. and Rosing, T. (2017).** "Exploring Hyperdimensional Associative Memory." In *2017 IEEE International Symposium on High Performance Computer Architecture (HPCA)*: 445-456. DOI 10.1109/HPCA.2017.28. The closest single Imani-line reference for *associative-memory* aspects of Claims 1 and 7.
- **Imani, M., Huang, C., Kong, D. and Rosing, T. (2018).** "Hierarchical Hyperdimensional Computing for Energy Efficient Classification." In *Proceedings of the 55th Annual Design Automation Conference (DAC '18)*, Article 108, 6 pages. DOI 10.1145/3195970.3196060.
- **Imani, M., Bosch, S., Datta, S., Ramakrishna, S., Salamat, S., Rabaey, J. M. and Rosing, T. (2019).** "QuantHD: A Quantization Framework for Hyperdimensional Computing." *IEEE Transactions on Computer-Aided Design of Integrated Circuits and Systems* 39(10): 2268-2278. DOI 10.1109/TCAD.2019.2954472. Discusses deterministic generation of level hypervectors from a seed.
- **Imani, M., Salamat, S., Khaleghi, B., Samragh, M., Koushanfar, F. and Rosing, T. (2019).** "SparseHD: Algorithm-Hardware Co-optimization for Efficient High-Dimensional Computing." In *Proceedings of the 27th IEEE Annual International Symposium on Field-Programmable Custom Computing Machines (FCCM)*: 190-198.
- **Imani, M., Morris, J., Bosch, S., Shu, H., De Micheli, G. and Rosing, T. (2019).** "BRIC: Locality-based Encoding for Energy-Efficient Brain-Inspired Hyperdimensional Computing." In *Proceedings of the 56th Annual Design Automation Conference (DAC '19)*: Article 52. DOI 10.1145/3316781.3317785.

**Round-2 correction.** The Round-1 draft attributed thermometer / level encoding primarily to Imani 2017. That attribution understates the prior art. The originating reference is Rahimi 2016 (ISLPED), with the comprehensive treatment in Rahimi et al. 2017 (TCAS-I). The Imani line of work develops and applies the Rahimi encoder; it is derivative on this specific point. The §6 differentiation matrix has been updated accordingly.

**Smritidb's relationship:** Adopts the binary HDC substrate; the thermometer-quantised level-hypervector encoder of Claim 27 is informed by the Rahimi 2016 / 2017 line of work and by Imani's subsequent applications.

**Conflict with claims:** Claim 27 must be distinguished from Rahimi 2016 / 2017 and from Imani 2017-2019. The principal differentiators:

(i) In Rahimi 2016 and in the Imani follow-on work, level hypervectors are generated either by a sliding correlation between two random seed hypervectors (Rahimi 2016) or from a pseudo-random number generator whose seed and state are implementation-defined (Imani 2017-2019). In neither case is the generator a cryptographic extendable-output function specified to byte-identity, and in neither case is the seed persisted in a wire-format header such that a third-party implementation may independently verify the encoder output. In Imani's QuantHD (2019), deterministic seed-based generation is discussed in isolation as an optimisation, but is not coupled to a published wire format or to a conformance corpus.

(ii) Smritidb derives every level hypervector from a single specified BLAKE3 XOF invocation against `(corpus_seed, dimension, level_index)`; output is byte-identical across the Rust core, the WebAssembly build, the JavaScript fallback and the Python and JVM bindings, verified against the conformance corpus.

(iii) Smritidb pairs the encoder with a deterministically-seeded quantisation policy exposed as a formal parameter of the wire format header; no Rahimi or Imani publication discloses a wire format specifying byte-identity across implementations of the level encoder.

### 2.5 Cryptographic extendable-output functions (XOFs) — BLAKE3 and adjacent primitives

- **O'Connor, J., Aumasson, J.-P., Neves, S. and Wilcox-O'Hearn, Z. (2020).** "BLAKE3: One Function, Fast Everywhere." Technical specification v1.0. Available at https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf.
- **NIST FIPS 202 (2015).** *SHA-3 Standard: Permutation-Based Hash and Extendable-Output Functions*. National Institute of Standards and Technology, Federal Information Processing Standards Publication 202, August 2015. DOI 10.6028/NIST.FIPS.202. Defines SHAKE128 and SHAKE256, the canonical NIST-standardised XOFs.
- **Aumasson, J.-P., Neves, S., Wilcox-O'Hearn, Z. and Winnerlein, C. (2013).** "BLAKE2: Simpler, Smaller, Fast as MD5." In *Applied Cryptography and Network Security (ACNS 2013)*, Lecture Notes in Computer Science vol. 7954: 119-135. DOI 10.1007/978-3-642-38980-1_8. BLAKE2X is the extendable-output mode.

A cryptographic hash function offering parallelism via a Merkle tree of chunks (BLAKE3) and an extendable-output-function ("XOF") mode for arbitrary-length deterministic output. SHAKE128/256 and BLAKE2X provide equivalent XOF primitives standardised by NIST and IRTF respectively.

**Smritidb's relationship:** BLAKE3 is the deterministic source for (i) the bundle tiebreaker, (ii) random hypervector expansion, (iii) the per-level random projection in the Claim 27 encoder, (iv) per-block content integrity in the Claims 18 and 23 wire format, and (v) bit-flip selection in the Claim 9 consolidation.

**Conflict with claims:** None. Smritidb does not claim BLAKE3 itself; it claims the *application* of a cryptographic XOF (instantiated by BLAKE3) to specific HDC operations to render cross-implementation byte-identity provable. Claim language is drawn at "a cryptographic hash function with extendable output" so as not to be defeated by an equivalent primitive — SHAKE128, SHAKE256 and BLAKE2X are within the literal scope.

### 2.6 Hebb, D. O. (1949). *The Organization of Behavior*. John Wiley and Sons, New York.

The classical statement of Hebbian learning ("neurons that fire together wire together"), invoked here as the conceptual ancestor of any consolidation-by-coactivation procedure.

**Conflict with claims:** None. Claim 9 differs by being deterministic, bounded and replayable.

### 2.7 Kanerva, P. (1996). "Binary Spatter Codes." In *Proceedings of the International Conference on Artificial Neural Networks (ICANN 1996)*, Lecture Notes in Computer Science vol. 1112: 869-873. DOI 10.1007/3-540-61510-5_146.

The canonical Binary Spatter Code (BSC) reference: dense binary hypervectors of dimension D, XOR binding, majority bundling, Hamming similarity. This is the variant of HRR that Smritidb uses as its substrate.

**Smritidb's relationship:** BSC is the substrate of Smritidb. The mathematics of XOR binding and majority bundling are Kanerva's BSC.

**Conflict with claims:** None as such. BSC teaches the substrate but is silent on (a) deterministic, cryptographic tie resolution; (b) persistent wire format; (c) cross-implementation byte-identity; (d) replayable consolidation; (e) conformance corpus methodology. The inventive step rests on these system-level properties layered on the BSC substrate.

### 2.8 Gayler, R. W. (1998). "Multiplicative Binding, Representation Operators & Analogy." In *Advances in Analogy Research: Integration of Theory and Data from the Cognitive, Computational, and Neural Sciences* (K. Holyoak, D. Gentner, B. Kokinov, eds.). New Bulgarian University, Sofia. (AAAI Fall Symposium 1998 working notes.)

Foundational VSA work on binding operators. Tangential to the present claims but acknowledged for completeness.

**Conflict with claims:** None.

---

## 3. Closest Industry Prior Art — Vector Databases and Adjacent Substrates

The following commercial or open-source vector databases collectively define the state of the art in approximate nearest-neighbour ("ANN") storage as of the priority date. None overlaps directly with the claims because each operates over a fundamentally different substrate. Vendor documentation last checked 20 May 2026.

### 3.1 Pinecone (Pinecone Systems, Inc.; commercial, closed-source)

Cloud-hosted vector database; HNSW (Malkov and Yashunin, 2018) with optional product quantisation. Substrate: f32 with optional scalar quantisation. Cross-implementation bit-exactness not claimed. **Conflict:** None — different substrate, different operation set (no XOR binding, no Hamming-ball read), no replayable consolidation, no cryptographic per-block integrity.

### 3.2 Qdrant (Qdrant Solutions GmbH; open-source, Rust; Apache-2.0)

HNSW with scalar/product quantisation; f32 with optional i8 quantisation. **Conflict:** None.

### 3.3 Weaviate (Weaviate B.V.; open-source, Go; BSD-3)

Modular vector database with HNSW index and pluggable vectorisers. **Conflict:** None.

### 3.4 FAISS (Meta Platforms, Inc.; open-source, C++; MIT)

ANN research library supporting IVF, PQ, HNSW and composite indices. **Conflict:** None — index-construction library, not associative memory; no bit-exact guarantee; no per-block crypto integrity. The FAISS `bench_all_ivf` benchmark suite exists but is a *performance benchmark*, not a byte-identity conformance corpus.

### 3.5 Milvus (Zilliz, Inc.; open-source, Go and C++; Apache-2.0)

Distributed vector database; HNSW, IVF and others. **Conflict:** None.

### 3.6 Chroma (Chroma, Inc.; open-source, Python; Apache-2.0)

Embedding database with Python API. **Conflict:** None.

### 3.7 Common gap across all six

None of the six ships a published bit-exact conformance corpus (vendor documentation last checked 20 May 2026; to be re-checked at filing day). None guarantees byte-identical computed output across language bindings. None uses dense binary HDC as a storage substrate. None provides a replayable, deterministic consolidation procedure. The technical effect claimed by Smritidb — namely, cross-implementation byte-identity for an associative memory store — is, on the present search, unattained by any commercial vector database.

### 3.8 HNSW and graph-based ANN indices (adjacent, not conflicting)

- **Malkov, Yu. A. and Yashunin, D. A. (2018).** "Efficient and Robust Approximate Nearest Neighbor Search Using Hierarchical Navigable Small World Graphs." *IEEE Transactions on Pattern Analysis and Machine Intelligence* 42(4): 824-836. DOI 10.1109/TPAMI.2018.2889473.

HNSW is the closest graph-based ANN prior art. The Smritidb codebase at HEAD `17334f8` uses brute-force scan in its Phase-1 recall path; an LSH-based approximate-recall layer is contemplated for Phase 2 of the implementation roadmap.

**Smritidb's relationship:** The eight independent claims do *not* claim a recall index. Claims 1 and 7 (tiebreaker), Claim 9 (consolidation), Claims 18 and 23 (wire format), Claim 27 (level encoder), Claim 31 (conformance-gating system) and Claim 35 (text encoder) are all directed to the substrate, the persistence layer or the encoders — not to an index data structure. Accordingly, HNSW is adjacent-not-conflicting prior art.

**Forward-looking note.** If a Phase-2 LSH-based approximate-recall index is to be claimed in a continuation or divisional application, that continuation will require a separate prior-art analysis against (i) HNSW and (ii) the LSH family enumerated at §4.8 below. The present application does not pursue such claims.

### 3.9 Probabilistic binary substrates (adjacent prior-art family)

- **Bloom, B. H. (1970).** "Space/Time Trade-offs in Hash Coding with Allowable Errors." *Communications of the ACM* 13(7): 422-426. DOI 10.1145/362686.362692.
- **Cormode, G. and Muthukrishnan, S. (2005).** "An Improved Data Stream Summary: The Count-Min Sketch and Its Applications." *Journal of Algorithms* 55(1): 58-75. DOI 10.1016/j.jalgor.2003.12.001.
- **Broder, A. Z. (1997).** "On the Resemblance and Containment of Documents." In *Proceedings of the Compression and Complexity of Sequences 1997*: 21-29. DOI 10.1109/SEQUEN.1997.666900.

**Threat assessment.** An adversarial examiner may characterise the Claims 1 and 7 framing ("a cryptographic-hash-derived bit determines whether a tiebroken position becomes 1 or 0") as a trivial variant of Bloom-filter set-membership indexing, where the cryptographic hash is the well-known choice when adversarial inputs are anticipated. Similarly, MinHash and SimHash (§4.8) use hash-derived bit positions to construct binary fingerprints.

**Differentiation.** The Smritidb substrate is *not* a set-membership oracle. A bundle hypervector is a *noisy superposition* of constituent hypervectors and is queried by Hamming similarity, not by hash-derived position. The cryptographic-XOF tiebreaker is invoked only at tied bit positions during *bundle reduction*, where the bit identity of the bundle output across heterogeneous implementations is the technical effect. Bloom and Count-Min do not have a tiebreaker because they do not have a tied-position concept in the relevant sense. MinHash does not have a bundle operation.

**Conflict with claims:** None directly. Adjacent prior art that must be acknowledged and differentiated in §6.

---

## 4. Recent Academic HDC, VSA and Adjacent Work (1996-2026)

### 4.1 Ge, L. and Parhi, K. K. (2020). "Classification using Hyperdimensional Computing: A Review." *IEEE Circuits and Systems Magazine* 20(2): 30-47. DOI 10.1109/MCAS.2020.2978558.

Survey of HDC classification methods. **Conflict:** None — survey, no apparatus or method claims.

### 4.2 Schlegel, K., Neubert, P. and Protzel, P. (2022). "A comparison of vector symbolic architectures." *Artificial Intelligence Review* 55(6): 4523-4555. DOI 10.1007/s10462-021-10110-3.

Comparative survey of VSA variants (BSC, MAP, HRR, FHRR) on a common benchmark. The paper discusses binary substrates including BSC and is directly relevant background for Claims 1, 7 and 35. **Conflict:** None — proposes no persistence format, tiebreaker policy or conformance test.

### 4.3 Kleyko, D., Rachkovskij, D. A., Osipov, E. and Rahimi, A. (2022). "Vector Symbolic Architectures as a Computing Framework for Emerging Hardware." *Proceedings of the IEEE* 110(10): 1538-1571. DOI 10.1109/JPROC.2022.3209104.

Position paper on VSA as a computing framework. Discusses persistence and reproducibility at a high level. **Conflict:** None.

### 4.4 Karunaratne, G., Le Gallo, M., Cherubini, G., Benini, L., Rahimi, A. and Sebastian, A. (2020). "In-memory hyperdimensional computing." *Nature Electronics* 3(6): 327-337. DOI 10.1038/s41928-020-0410-3.

Analog in-memory HDC using phase-change memory (IBM Zurich, ETH Zurich). The paper also discusses bit-exact reproducibility in the context of PCM read noise — relevant to the Claims 1 and 7 reproducibility framing, but the substrate (analog PCM) is orthogonal to Smritidb's CPU/GPU substrate. **Conflict:** None — different hardware substrate.

### 4.5 Hersche, M., Zeqiri, M., Benini, L., Sebastian, A. and Rahimi, A. (2023). "A Neuro-vector-symbolic Architecture for Solving Raven's Progressive Matrices." *Nature Machine Intelligence* 5: 363-375. DOI 10.1038/s42256-023-00630-8.

Applies VSA to compositional reasoning. **Conflict:** None — task-level application; no persistence claim.

### 4.6 Indian-affiliated research

A Round-2 preliminary search alluded to HDC accelerator work from IIT Bombay and hyperdimensional classifier work from IISc Bangalore but recorded neither specific publication identifiers nor DOIs. Per the Round-2 §10 standard ("cite specifically or remove"), the vague allusion is **removed** in Round 4. A formal IEEE Xplore and ACM Digital Library author-search by the patent agent — keyed on IISc and IIT Bombay institutional affiliations against the principal Boolean strings (§1.5) — is to be undertaken before filing. Any material publication so discovered shall be added by amendment to §4 with full bibliographic detail. As of 20 May 2026, the present memo records **no Indian-affiliated HDC publication as prior art** because none has been independently verified to bibliographic-pin precision; this finding may change after the patent agent's formal search.

**Methodological note.** The absence of a verified citation is *not* a finding that no such publication exists. It is an acknowledgment that, in the absence of specific verifiable bibliographic detail, the memo will not record a vague allusion. The patent agent's formal search at §5.1 (InPASS) and at the IEEE / ACM author-search stage should treat this paragraph as an open action item.

### 4.7 Theory of binary HDC

- **Kleyko, D., Rahimi, A., Rachkovskij, D. A., Osipov, E. and Rabaey, J. M. (2018).** "Classification and Recall with Binary Hyperdimensional Computing: Tradeoffs in Choice of Density and Mapping Characteristics." *IEEE Transactions on Neural Networks and Learning Systems* 29(12): 5880-5898. DOI 10.1109/TNNLS.2018.2814400. Direct prior art on binary HDC tradeoffs.
- **Frady, E. P., Kleyko, D. and Sommer, F. T. (2018).** "A Theory of Sequence Indexing and Working Memory in Recurrent Neural Networks." *Neural Computation* 30(6): 1449-1513. DOI 10.1162/neco_a_01084. Context for the permutation-positional n-gram encoder of Claim 35.
- **Thomas, A., Dasgupta, S. and Rosing, T. (2021).** "A Theoretical Perspective on Hyperdimensional Computing." *Journal of Artificial Intelligence Research* 72: 215-249. DOI 10.1613/jair.1.12664.
- **Neubert, P., Schubert, S. and Protzel, P. (2019).** "An Introduction to Hyperdimensional Computing for Robotics." *KI – Künstliche Intelligenz* 33: 319-330. DOI 10.1007/s13218-019-00623-z. Tangential application reference.
- **Rao, R. P. N. and Fuentes, O. (1998).** "Hierarchical Learning of Navigational Behaviors in an Autonomous Robot using a Predictive Sparse Distributed Memory." *Machine Learning* 31(1-3): 87-113. DOI 10.1023/A:1007454428309. Recorded for completeness as an early SDM-based positional-sequence work; not closest single reference for any present claim.

**Conflict with claims:** None directly. These references are background for the binary-HDC substrate and the n-gram encoder.

### 4.8 Locality-Sensitive Hashing and binary fingerprints (P0 — affects Claims 1, 4, 6)

The Round-1 draft did not address the LSH / SimHash / sign-LSH family. The omission is corrected here. These references collectively constitute the most likely obviousness combination an examiner will raise against Claim 27 (encoder) and Claim 35 (text encoder), and are also relevant to Claims 1 and 7.

- **Charikar, M. S. (2002).** "Similarity Estimation Techniques from Rounding Algorithms." In *Proceedings of the Thirty-Fourth Annual ACM Symposium on Theory of Computing (STOC 2002)*: 380-388. DOI 10.1145/509907.509965. The original SimHash / sign-LSH construction: random projection to a binary fingerprint by sign of the projection.
- **Datar, M., Indyk, P., Immorlica, N. and Mirrokni, V. S. (2004).** "Locality-Sensitive Hashing Scheme Based on p-Stable Distributions." In *Proceedings of the Twentieth Annual Symposium on Computational Geometry (SoCG 2004)*: 253-262. DOI 10.1145/997817.997857. The p-stable LSH construction for L_p norms (p ∈ (0, 2]); produces real-valued hash buckets, not binary fingerprints.
- **Andoni, A. and Indyk, P. (2008).** "Near-Optimal Hashing Algorithms for Approximate Nearest Neighbor in High Dimensions." *Communications of the ACM* 51(1): 117-122. DOI 10.1145/1327452.1327494. CACM survey article building on the original Andoni-Indyk FOCS/SoCG 2006 result. Discusses seeded random projection as a reproducibility measure.
- **Manku, G. S., Jain, A. and Das Sarma, A. (2007).** "Detecting Near-Duplicates for Web Crawling." In *Proceedings of the 16th International Conference on World Wide Web (WWW 2007)*: 141-150. DOI 10.1145/1242572.1242592. SimHash applied to web de-duplication; a deterministic, training-free binary text fingerprint.
- **Achlioptas, D. (2003).** "Database-friendly Random Projections: Johnson-Lindenstrauss with Binary Coins." *Journal of Computer and System Sciences* 66(4): 671-687. DOI 10.1016/S0022-0000(03)00025-4. (Conference version: PODS 2001.) Canonical reference for deterministic random projection via sparse sign matrices. Cited in the §3(k) defence memo.

**Smritidb's relationship.** The thermometer-quantised random projection encoder of Claim 27 is a *level-quantised variant* of a deterministic sign-LSH construction. The text encoder of Claim 35 is a *permutation-positional* counterpart to SimHash (which is bag-of-features and order-insensitive).

**Differentiation.**

(i) *Claim 27 vs Charikar 2002 / Datar 2004 / Andoni & Indyk 2008.* Sign-LSH produces a single-bit-per-projection fingerprint. The Smritidb encoder produces a thermometer-quantised level hypervector of dimension D per scalar level, generated by a single XOF invocation against `(corpus_seed, dimension, level_index)`, and persisted as a parameter of the KMF wire-format header. Andoni & Indyk (2008) note that seeded random projection is used for reproducibility, but neither the seed itself nor a wire format pinning the seed across implementations is published. The Smritidb encoder is anchored to a published conformance corpus that any third-party implementation may verify against.

(ii) *Claim 35 vs SimHash (Charikar 2002; Manku et al. 2007).* SimHash is unordered (bag-of-features). The Smritidb encoder is permutation-positional, encoding n-gram order through a fixed, deterministically generated permutation. SimHash also lacks the cryptographic-XOF anchoring and the wire-format pinning.

(iii) *Claims 1 and 7 vs sign-LSH.* Sign-LSH is a binary-vector construction but is not a *bundle* operation — it does not combine multiple binary vectors into a representative output. Claims 1 and 7 address tie resolution in majority bundling, which has no analogue in the LSH family.

### 4.9 Vector quantization

- **Jégou, H., Douze, M. and Schmid, C. (2011).** "Product Quantization for Nearest Neighbor Search." *IEEE Transactions on Pattern Analysis and Machine Intelligence* 33(1): 117-128. DOI 10.1109/TPAMI.2010.57.
- **Sivic, J. and Zisserman, A. (2003).** "Video Google: A Text Retrieval Approach to Object Matching in Videos." In *Proceedings of the Ninth IEEE International Conference on Computer Vision (ICCV 2003)*: 1470-1477. DOI 10.1109/ICCV.2003.1238663. Inverted File (IVF) construction.
- **Ge, T., He, K., Ke, Q. and Sun, J. (2013).** "Optimized Product Quantization." *IEEE Transactions on Pattern Analysis and Machine Intelligence* 36(4): 744-755. DOI 10.1109/TPAMI.2013.240.

**Threat assessment.** PQ / IVF / OPQ define the prior-art landscape for footprint reduction by quantisation. The Claim 27 "approximately 32-fold reduction" technical effect targets the same metric.

**Differentiation.** PQ uses *learned* codebooks (training-dependent); Smritidb uses thermometer + BLAKE3-seeded random projection (training-free and deterministic). PQ produces compact subspace codes; the Smritidb encoder produces a binary hypervector that participates in XOR-binding and majority-bundling operations as a substrate element. The two constructions have different operational semantics.

**FTO note.** There may be US patents assigned to INRIA, Inria-spinout or Facebook AI Research in the PQ family; these are to be triaged at FTO stage by the patent agent.

### 4.10 Word embeddings and neural text encoders

The §3(k) defence memorandum (`11-3k-defense.md`) cites the following references as the *trained* text-encoder prior art against which Claim 35 ("training-free, on-device-feasible") is differentiated. They are reproduced here to maintain consistency across the two filing documents.

- **Mikolov, T., Chen, K., Corrado, G. and Dean, J. (2013).** "Efficient Estimation of Word Representations in Vector Space." *Proceedings of the International Conference on Learning Representations (ICLR 2013) Workshop Track*. arXiv:1301.3781.
- **Pennington, J., Socher, R. and Manning, C. D. (2014).** "GloVe: Global Vectors for Word Representation." In *Proceedings of the 2014 Conference on Empirical Methods in Natural Language Processing (EMNLP 2014)*: 1532-1543. DOI 10.3115/v1/D14-1162.
- **Devlin, J., Chang, M.-W., Lee, K. and Toutanova, K. (2019).** "BERT: Pre-training of Deep Bidirectional Transformers for Language Understanding." In *Proceedings of the 2019 Conference of the North American Chapter of the Association for Computational Linguistics (NAACL-HLT 2019)*: 4171-4186. DOI 10.18653/v1/N19-1423.

**Conflict with claims:** None. All three are trained large models requiring substantial inference resources; Claim 35 is differentiated as training-free, deterministic and on-device-feasible.

### 4.11 Classical associative-memory networks (Round-4 addition — peer-review gap closure)

The Round-2 memo did not separately treat the classical associative-memory literature that pre-dates Kanerva's Sparse Distributed Memory. The omission is corrected here. These references constitute the closest single-reference family for the *associative-memory framing* of the Claim 1 + Claim 7 tiebreaker family (independent of the bundle-tie-resolution framing already addressed at §6 row 1).

- **Hopfield, J. J. (1982).** "Neural Networks and Physical Systems with Emergent Collective Computational Abilities." *Proceedings of the National Academy of Sciences USA* 79(8): 2554-2558. DOI 10.1073/pnas.79.8.2554. The canonical Hopfield network: a symmetric, fully connected recurrent network with binary {-1, +1} neurons, energy-function dynamics, and content-addressable recall by gradient descent on the energy landscape.
- **Steinbuch, K. (1961).** "Die Lernmatrix." *Kybernetik* 1(1): 36-45. DOI 10.1007/BF00293853. The Lernmatrix: an early binary correlation-matrix memory pre-dating Hopfield by twenty-one years.
- **Willshaw, D. J., Buneman, O. P. and Longuet-Higgins, H. C. (1969).** "Non-Holographic Associative Memory." *Nature* 222(5197): 960-962. DOI 10.1038/222960a0. The Willshaw associative net: sparse binary outer-product storage and threshold recall.
- **Anderson, J. A. (1972).** "A Simple Neural Network Generating an Interactive Memory." *Mathematical Biosciences* 14(3-4): 197-220. DOI 10.1016/0025-5564(72)90075-2. Linear-associator memory with real-valued vectors.
- **Kohonen, T. (1972).** "Correlation Matrix Memories." *IEEE Transactions on Computers* C-21(4): 353-359. DOI 10.1109/TC.1972.5008975. Correlation matrix memory; the closest classical associative-memory reference to a *bundle* (sum-of-outer-products) operation.
- **Ramsauer, H. et al. (2021).** "Hopfield Networks is All You Need." *International Conference on Learning Representations (ICLR 2021)*. arXiv:2008.02217. Modern continuous Hopfield network reframed as a Transformer-attention primitive; recorded for completeness but does not address binary substrates.

**Smritidb's relationship.** The Smritidb associative-memory substrate is *not* a Hopfield-style energy-minimisation network: Smritidb does not recurrently iterate to a fixed point; recall is a single Hamming-distance scan against stored items. The Smritidb *bundle* operation is closest in spirit to Kohonen's correlation matrix memory in that a sum-of-binary-vectors is taken, but the bundle output is binarised by majority-with-tiebreak rather than retained as a real-valued matrix.

**Conflict with claims.**

(i) *Claims 1 + 7 (tiebreaker) vs Hopfield 1982 / Kohonen 1972.* Hopfield-network sign updates use the *sign of the local field*; ties (zero local field) are resolved by an implementation-defined convention (commonly "retain previous state" or "random"). Kohonen's correlation matrix memory is real-valued and has no native tied-bit concept. Neither reference teaches (a) tie resolution by a cryptographic XOF tuple, (b) cross-implementation byte-identity of the bundle output, or (c) a wire-format-persisted corpus seed. The Smritidb differentiator is the *system property* of verifiable byte-identity across heterogeneous runtimes, not the abstract use of a binary substrate.

(ii) *Claim 9 (replayable consolidation) vs Hopfield 1982.* Hopfield-network state evolution is deterministic given an initial state and an update order, but the canonical asynchronous update rule selects neurons at random, and the deterministic-synchronous variant exhibits two-cycle limit behaviour. Neither the canonical asynchronous variant nor any subsequent Hopfield-derivative on the present search teaches (a) a content-neutral access-log that records *which items* received Hebbian pulls (rather than which neurons fired), (b) a *bounded similarity drift* per consolidation pass, or (c) wire-format-persisted replayability. The Smritidb differentiator is the system property of replayability of an item-keyed substrate, not the abstract use of Hebbian-style updates.

(iii) *Claim 27 (encoder) and Claim 35 (text encoder) vs §4.11 references.* None of the classical associative-memory networks teaches an *encoder* (an input-to-substrate map); they teach *storage and recall* over an already-encoded substrate. The encoder claims are accordingly orthogonal to this family.

**Examiner-anticipation flag.** No reference in §4.11 anticipates any independent claim. Hopfield 1982 is added to §6 row 1 as a co-equal closest single reference alongside Kanerva 1988 and Imani 2017 HPCA for the associative-memory framing of Claims 1 + 7.

---

## 5. Patent Landscape

### 5.1 Indian Patents — `[P0 BLOCKER BEFORE FILING]`

**Status:** The InPASS formal search has not yet been executed by the patent agent. This is identified as a **P0 blocker for any Form-2 filing**.

A preliminary title-and-abstract scan was undertaken by the inventor using the query strings enumerated in §1.5, returning no apparent anticipation on the strength of titles and abstracts alone. **No claim-by-claim reading of the top-fifty hits has been performed; accordingly, the present statement is provisional and the assertion of "no Indian prior art identified" is not supportable without the formal search.**

**To be completed by the patent agent before Form-2 filing:**

(a) Run each of the principal query strings (§1.5) against InPASS with the date set to capture the rolling-publication window (new InPASS publications appear weekly).

(b) Record for each query: (i) the exact Boolean string used, including any InPASS-specific proximity or truncation operators; (ii) the date of execution; (iii) the count of hits returned.

(c) For each query, record the top-twenty hits with their patent or application number, title, abstract, applicant, status (published, granted, abandoned), and date of grant or publication.

(d) For each of those top-twenty hits, record a one-line claim-by-claim differentiation against each of the six Smritidb claims.

(e) Search Indian *applications* (not only grants), because pending applications can be cited as prior art under §13 of the Patents Act, 1970.

(f) Re-check at filing day and refresh.

Until these steps are documented, the §5.1 finding remains `[TBD — P0 blocker]` and the memo cannot be relied upon for the Form-3 statement to the Controller or for any FER response.

### 5.2 United States Patents and Published Applications

**Round-2 verification finding (critical) and Round-6 resolution.** The Round-1 draft cited two US patent numbers without independent verification. Round 2 demonstrated both cites were defective; Round 4 surfaced "candidate leads" against the Imani-HDC slot; Round 6 has now confirmed, by independent web research recorded in `patent/critiques/p0-prior-art-search-results.md`, that both Round-4 candidate leads were themselves false leads, and has identified the actual closest US prior art. The history is preserved below for the audit trail; the operative finding is at sub-section (c) "Round-6 verified findings".

(a) **US 11,775,847 B2 — Round-1 cite withdrawn.** Round-1 attribution: "Imani et al. (Regents of the University of California). Hyperdimensional computing classification systems." Verified attribution (via patents.google.com retrieval, 20 May 2026): *"Systems and methods for classifying media according to user negative propensities"*, inventor Kenneth Neumann, assignee KPN Innovations LLC, granted 3 October 2023. **This is not an HDC patent.** The Round-1 attribution is incorrect. **The Round-1 cite of US 11,775,847 B2 is withdrawn entirely.**

(b) **US 10,956,464 B1 — Round-1 cite withdrawn (confirmed non-existent).** Round-1 attribution: "Pinecone Systems, Inc. Vector indexing apparatus." Round-2 verification attempt: patents.google.com returned HTTP 404 for this number. Round-4 confirmation: targeted Google / Justia search returned no Pinecone-issued US patent hits in public indexes. Round-6 confirmation: Justia assignee search for `Pinecone Systems Inc.` returns zero patents; Google Patents assignee filter for `Pinecone Systems` returns zero patents; the "Pinecone" hits in public indexes resolve to *Pinecone Material Inc.* (semiconductors) or *Pinecone Imaging Corporation* (imaging), both unrelated to the vector-database company. **Conclusion:** Pinecone Systems Inc. (founded 2019 by Edo Liberty) has no issued US patent or published US patent application reachable by public web search as of 2026-05-20; the Round-1 cite of US 10,956,464 B1 is a transcription error or non-existent number and is withdrawn entirely. Pinecone's technical disclosures are limited to its corporate blog, AWS Marketplace listings, and product documentation; consequently no Pinecone patent reads on any Smritidb independent claim.

(c) **Round-6 verified findings — actual closest US prior art.** Independent web-research (Google Patents, Justia, NCBI/PMC, USPTO image-ppubs) executed 20 May 2026 and recorded in full at `patent/critiques/p0-prior-art-search-results.md` has identified the following verified US patents and published applications as the actual closest prior art on the Imani / Rosing UCSD HDC slot, the ETH Zurich / IBM HDC slot, the Macronix flash-HDC slot, and the IBM bundling slot. The Round-4 "candidate leads" of US 12,450,896 and US 11,854,253 were **false leads**: they have been verified as inventor Narayan Srinivasa, assignee Intel Corp, directed to HDC-based image-classifier robustness against adversarial perturbations — not Imani-Rosing UCSD patents, and not directed to associative-memory persistence / wire format / cross-implementation byte-identity. Both Round-4 leads are withdrawn entirely from the Imani-HDC slot.

   (i) **US 2022/0019441 A1 (Rosing, T. S., Morris, J., Imani, M., Kim, Y., Messerly, J., Guo, Y., Khaleghi, B., Gupta, S., Salamat, S. and Sim, J., "Circuits, methods, and articles of manufacture for hyper-dimensional computing systems and related applications," published 20 January 2022, assigned to The Regents of the University of California (UCSD); USPTO grant status to be confirmed by patent-agent PAIR check on the underlying serial number).** This is the **actual closest single US reference** to Smritidb Claims 1 (method) and 7 (system), because it discloses binary HDC primitives in a system context, including a hyperdimensional processing unit (HPU), binary hypervector operations, and content-addressable memory for binary hypervectors. It is **adjacent — not blocking**: every Smritidb independent claim is distinguished by a substantive limitation absent from US 2022/0019441 A1, namely (1) for Claims 1 and 7, the BLAKE3-anchored deterministic majority tiebreaker keyed on the four-tuple `(tag, D, i, n)`, which US 2022/0019441 A1 leaves unspecified; (2) for Claim 9, the replayable Hebbian consolidation bounded by `maxSimDelta` with BLAKE3-derived ordering, which US 2022/0019441 A1 does not address; (3) for Claims 18 and 23, the KMF wire format (magic + spec-version + reserved offset + per-block BLAKE3 + trailer magic + header-after-blocks), which US 2022/0019441 A1 does not recite; (4) for Claim 27, the BLAKE3-XOF thermometer encoder keyed on `(corpus_seed, dimension, level_index)`, which US 2022/0019441 A1 does not recite; (5) for Claim 31, the JSON conformance corpus with SHA-256-of-packed-bytes byte-identity gates, which US 2022/0019441 A1 does not recite; and (6) for Claim 35, the permutation-positional n-gram text encoder cross-referencing the Claim 1 tiebreaker and the Claim 31 corpus, which US 2022/0019441 A1 does not address.

   (ii) **US 12,015,424 B2 (Imani, M., "Network-based hyperdimensional system," issued 18 June 2024, assigned to The Regents of the University of California (UCSD)).** Network-based HDC ("NetHD") encoder operating over complex phase values `{±1, ±i}`. Adjacent and orthogonal to Smritidb's binary `{0, 1}` substrate; does not recite binary majority bundling, BLAKE3, deterministic tiebreaker, JSON wire format, conformance corpus, or content-addressable hyperdimensional substrate. **No conflict with any Smritidb independent claim.**

   (iii) **US 12,204,899 B2 (Imani, M., "Stochastic hyperdimensional arithmetic computing," issued 21 January 2025, assigned to The Regents of the University of California (UCSD)).** Stochastic HDC arithmetic over `±1` (bipolar) hypervectors. Adjacent to Claim 27 in spirit (encoder slot) but orthogonal in mechanism — neither cryptographic anchor nor wire-format pinning. Does not recite BLAKE3, deterministic tiebreaker, JSON wire format, or conformance corpus. **No conflict with any Smritidb independent claim.**

   (iv) **US 11,574,209 B2 (Karunaratne, K. G., Le Gallo-Bourdeau, M., Cherubini, G., Sebastian, A., Rahimi, A. and Benini, L., "Device for hyper-dimensional computing tasks," issued 7 February 2023, assigned jointly to ETH Zurich and International Business Machines Corp).** HDC inference-device hardware ("a device for hyper-dimensional computing for inference tasks ... comprising an item memory for storing hyper-dimensional item vectors"). Adjacent (inference-device hardware); does not recite BLAKE3, deterministic tiebreaker, JSON wire format, or conformance corpus. **No conflict with any Smritidb independent claim.**

   (v) **US 10,971,226 B2 (Le Gallo-Bourdeau, M., Karunaratne, K. G., Cherubini, G., Sebastian, A., Rahimi, A. and Benini, L., "Hyper-dimensional computing device," issued 6 April 2021, assigned jointly to ETH Zurich and International Business Machines Corp).** Resistive-memory device storing HD vectors as conductive statuses in two-dimensional memristors. Pure in-memory hardware substrate; does not recite binary `{0, 1}` substrate primitives, BLAKE3, tiebreaker, wire format, or conformance corpus. **No conflict with any Smritidb independent claim.**

   (vi) **US 12,260,913 B2 (Lin, Y.-H. and Tseng, P.-H., "Hyperdimensional computing device," issued 25 March 2025, assigned to Macronix International Co Ltd).** Flash / non-volatile-memory cell-array HDC device. Recites majority-rule bundling implemented at the analog sense-amplifier layer of NVM cells, but no algorithmic-tiebreaker claim, no wire format and no conformance corpus. **No conflict with any Smritidb independent claim.**

   (vii) **US 12,518,150 B2 (Hersche, M. A. and Rahimi, A., "Bundling hypervectors," issued 6 January 2026, assigned to International Business Machines Corp).** Recent IBM patent (issued approximately four months prior to the present priority date) directed to bundling a set of *M* code hypervectors of dimension *D* using a share-based approach: an *M*-dimensional weight vector is mapped to an *S*-dimensional indicator vector via the step function `∂ = cumsum(a) / sum(a)`, and the bundle output is constructed by element-wise selection from the indicated code hypervectors. **This is not majority voting and not a tiebreaker.** US 12,518,150 B2 does not recite binary majority bundling with a tiebreaker, BLAKE3-derived tie resolution, JSON wire format, or conformance corpus. The two approaches diverge at the bundling operation itself; Smritidb's element-wise integer majority with BLAKE3 four-tuple tiebreaker is fundamentally distinct from the IBM cumsum-step-function-indicator share-selection mechanism. **No conflict with Smritidb Claim 1 or Claim 7.** Patent-agent action: pull the family-graph for US 12,518,150 via USPTO PAIR; confirm no Hersche / Rahimi continuation, divisional or sister application claims integer-majority bundling.

**Methodology for the patent agent's formal search (carried forward):**

(a) Conduct an inventor search at USPTO Patent Public Search for "Imani, Mohsen" with the field code `IN/`, and a parallel search for "Rosing, Tajana Simunic". Cross-check the result set against the seven Imani / Rosing / Rahimi patents at (c)(i)-(vii) above. Confirm USPTO PAIR grant status for US 2022/0019441 A1.

(b) Conduct an assignee search for "Pinecone Systems" (and any predecessor or successor names) at USPTO Patent Public Search with the field code `AN/`. The Round-6 expected outcome is a negative finding; the agent should record the negative finding (if obtained) for inclusion in any FER response.

(c) Pull the family-graph for US 12,518,150 B2 (IBM Hersche / Rahimi bundling) from USPTO PAIR; confirm no continuation, divisional or sister application claims integer-majority bundling. If any such continuation surfaces, re-check Smritidb Claim 1's BLAKE3-four-tuple tiebreaker limitation against the continuation's claim text.

(d) Conduct subject-matter searches with the principal Boolean strings (§1.5) against USPTO Patent Public Search, filtered by IPC class G06F, G06N and G11C.

(e) For each material hit, file the patent's first page and independent claim 1 in `patent/prior-art-evidence/us-patents/` (to be created).

**Other US filings.** The Round-1 reference to "other US patents assigned to Microsoft, Meta, IBM and Intel" was a vague allusion and was removed in Round 2. The Round-6 verified findings at (c) above resolve the specific IBM (US 11,574,209; US 10,971,226; US 12,518,150) and Intel (US 11,854,253; US 12,450,896 — the latter two NOT reading on any Smritidb claim) entries. The patent agent's FTO search at (d) above will surface any further filings; vague allusions are not retained.

### 5.3 European Patent Office Patents and Applications

`[TBD — formal Espacenet search by patent agent; methodology mirroring §5.2 above.]` The Round-1 statement that "Espacenet results for the principal queries returned a number of HDC accelerator and ANN-index patents" is retained only as a preliminary indication; specific numbers are to be recorded at formal-search stage.

### 5.4 Verified non-conflicting closest prior art (Round-6 consolidated table)

The following seven US patent records, all verified by independent web research recorded in `patent/critiques/p0-prior-art-search-results.md`, represent the closest prior-art landscape on the HDC substrate / encoder / hardware / bundling axes. None reads on any Smritidb independent claim. This sub-section is a one-line per-record summary in formal patent-citation form, keyed to the Smritidb claim that the record most resembles, to assist the patent agent's Section 8 statement and any FER response. Full discussion appears at §5.2 (c).

(a) **US 2022/0019441 A1** (Rosing, Imani et al., assigned to The Regents of the University of California (UCSD), published 20 January 2022) — closest single reference for **Claims 1 (method) and 7 (system)**. Binary HDC system with HPU and CAM; distinguished by absence of (i) BLAKE3 four-tuple `(tag, D, i, n)` deterministic tiebreaker, (ii) replayable Hebbian consolidation bounded by `maxSimDelta`, (iii) KMF wire format, (iv) JSON conformance corpus, (v) BLAKE3-XOF thermometer encoder, and (vi) permutation-positional text encoder.

(b) **US 12,015,424 B2** (Imani, M., assigned to The Regents of the University of California (UCSD), issued 18 June 2024) — adjacent to **Claim 27 (encoder)**. Network-based HDC encoder operating over complex phase values `{±1, ±i}`; orthogonal to Smritidb's binary `{0, 1}` substrate; distinguished by the binary substrate and the BLAKE3-XOF anchoring of Smritidb Claim 27.

(c) **US 12,204,899 B2** (Imani, M., assigned to The Regents of the University of California (UCSD), issued 21 January 2025) — adjacent to **Claim 27 (encoder)**. Stochastic HDC arithmetic over `±1` bipolar hypervectors; distinguished by Smritidb's binary `{0, 1}` substrate, the BLAKE3-XOF anchoring on the four-tuple `(corpus_seed, D, level_index)`, and the wire-format pinning of the corpus seed.

(d) **US 11,574,209 B2** (Karunaratne, Le Gallo-Bourdeau, Cherubini, Sebastian, Rahimi and Benini, assigned jointly to ETH Zurich and International Business Machines Corp, issued 7 February 2023) — adjacent to **Claims 1 and 7**. Inference-device hardware for HDC item-memory operations; distinguished by the absence of any cryptographic-tiebreaker, wire-format or conformance-corpus limitation.

(e) **US 10,971,226 B2** (Le Gallo-Bourdeau, Karunaratne, Cherubini, Sebastian, Rahimi and Benini, assigned jointly to ETH Zurich and International Business Machines Corp, issued 6 April 2021) — adjacent to **Claims 18 and 23 (substrate persistence)**, in the weak sense that an in-memory resistive substrate is a form of persistence. Distinguished by Smritidb's CPU/GPU substrate, the KMF wire format, per-block BLAKE3 integrity, and corpus-version anchoring.

(f) **US 12,260,913 B2** (Lin and Tseng, assigned to Macronix International Co Ltd, issued 25 March 2025) — adjacent to **Claims 1 and 7**. Flash-cell HDC hardware implementing analog-sense-amplifier majority bundling; distinguished by the absence of any algorithmic tiebreaker, wire format or conformance corpus.

(g) **US 12,518,150 B2** (Hersche and Rahimi, assigned to International Business Machines Corp, issued 6 January 2026) — adjacent to **Claims 1 and 7 (bundle operation slot)**. Share-based bundling via the cumsum-step-function indicator `∂ = cumsum(a) / sum(a)` applied to an *M*-dimensional weight vector; distinguished from Smritidb at the bundling-operation layer itself (element-wise integer majority with BLAKE3 four-tuple tiebreaker, not share-selection by step-function indicator). Patent-agent action: confirm via USPTO PAIR that no Hersche / Rahimi continuation, divisional or sister application claims integer-majority bundling.

**Foreign-application disclosure.** When ready to file, the Section 8 statement of foreign-application prior art known to the applicant should disclose (a)-(g) above plus any further hits surfaced by the patent agent's formal USPTO inventor and assignee searches.

### 5.5 WIPO PCT Applications

`[TBD — formal PATENTSCOPE search by patent agent; methodology mirroring §5.2 above.]`

### 5.6 Defensive publications and open-source releases

The Smritidb source tree is published under Apache-2.0 at HEAD `17334f8` and is itself a defensive publication under §31 of the Patents Act, 1970, within the inventor's twelve-month grace period.

---

## 6. Claim-by-Claim Differentiation Matrix

The matrix below is presented in the form examiners use for an FER response. For each claim, the closest *single* reference (the "X" reference, suitable for an anticipation challenge) is distinguished from the closest *combination* of references (suitable for an obviousness challenge). The fourth column states the differentiating limitation in claim-element form so the patent agent can directly trace each row to the corresponding claim in `08-claims.md`.

Rows 1, 2, 4 and 6 have been rewritten in Round 2 to lead with the system-level technical effect, in alignment with the framing of `11-3k-defense.md`. The "Claim aspect" column carries both the Round-1 aspect-numbering (1 through 6) and the Round-4 statutory claim numbers from `08-claims.md` (1, 7, 9, 18, 23, 27, 31, 35) so that the matrix is traceable end-to-end to the as-filed claim set.

**Claim-number mapping (Round-1 aspect-numbering → Round-4 statutory numbering in `08-claims.md`):** Aspect 1 → Claims 1 (method) and 7 (system); Aspect 2 → Claim 9; Aspect 3 → Claims 18 (method) and 23 (system); Aspect 4 → Claim 27; Aspect 5 → Claim 31 (conformance-gating system, OpenTV-reframed in Round 2); Aspect 6 → Claim 35.

| Claim aspect (Round-1 → Round-4) | Closest single reference (anticipation challenge) | Closest combination (obviousness challenge) | Differentiating limitation (claim-element form) |
|---|---|---|---|
| **Aspect 1 — Deterministic tiebreaker for bundle operations** (Round-4 Claims 1, 7) | Kanerva 1988 *Sparse Distributed Memory* Chapter 3 (majority bundle with implementation-defined random tiebreak); **US 2022/0019441 A1 (Rosing, Imani et al., UCSD, published 20 January 2022) — closest single US patent reference, discloses binary HDC system primitives but leaves bundle tie resolution unspecified**; **US 12,518,150 B2 (Hersche and Rahimi, IBM, issued 6 January 2026) — recent IBM bundling patent, share-based rather than majority-with-tiebreaker**; Imani et al. 2017 HPCA "Exploring Hyperdimensional Associative Memory" (associative-memory framing); **Hopfield 1982 / Kohonen 1972** (classical associative-memory with implementation-defined tie convention — see §4.11). | Kanerva 1988 (majority bundle) + US 2022/0019441 A1 (binary HDC system) + Charikar 2002 (sign-LSH binary substrate) + Andoni & Indyk 2008 (seeded random projection for reproducibility) + Bloom 1970 (cryptographic-hash bit selection) + Hopfield 1982 (energy-minimisation associative memory). | *Technical effect:* verifiable cross-implementation byte-identity of stored bundle outputs across heterogeneous architectures (x86_64, ARM64, JVM, V8, LLVM-native Rust, WebAssembly). *Mechanism:* tie resolution at exactly-tied bit positions by the least-significant bit of a BLAKE3 XOF digest computed over the byte concatenation `tag || D || i || n`, with `tag` a fixed ASCII domain-separation string, and with the corpus seed persisted in the KMF wire-format header (Aspect 3, Claim 18), such that any conforming third-party implementation reproduces the bundle output bit-for-bit and the property is independently verifiable against the conformance corpus (Aspect 5, Claim 31). No combination of the cited references teaches this *system property* even though each teaches an aspect of the underlying mechanism. **US 2022/0019441 A1 specifically does not teach cryptographic determinism for tie resolution — the published HDC system primitives leave bundle tie resolution unspecified, and the patent is silent on cross-implementation byte-identity.** US 12,518,150 B2 does not teach majority-with-tiebreaker at all — its bundling mechanism is share-based, using a cumsum-step-function indicator (`∂ = cumsum(a) / sum(a)`) to select element-wise from indicated code hypervectors, which is fundamentally distinct from element-wise integer majority. Hopfield 1982 likewise does not teach cross-implementation byte-identity — the canonical Hopfield update rule selects neurons asynchronously and resolves zero-local-field ties by an implementation convention. **None of the verified prior art (US 2022/0019441 A1, US 12,015,424 B2, US 12,204,899 B2, US 11,574,209 B2, US 10,971,226 B2, US 12,260,913 B2, US 12,518,150 B2) has cryptographic determinism for tie resolution.** |
| **Aspect 2 — Replayable Hebbian consolidation with bounded similarity drift** (Round-4 Claim 9) | Hebb 1949 *The Organization of Behavior* (Hebbian principle, qualitative); Kanerva 1988 SDM counter consolidation; SQLite WAL design (https://www.sqlite.org/wal.html); Karunaratne et al. 2020 *Nature Electronics* (in-memory HDC on phase-change memory, no determinism-across-reruns characterisation). | Hebb 1949 + SQLite WAL + Kanerva 1988 (HDC consolidation) + Karunaratne 2020. | *Technical effect:* deterministic replay of the associative memory's state from a snapshot and an access log, with a provable upper bound on Hamming-distance drift per consolidation step, enabling audit trails, multi-replica consistency and rollback. *Mechanism:* per-step bit-flip selection by BLAKE3-XOF over `(state, step, parameters)` bounded by a configured maximum delta. SQLite WAL is the closest write-ahead-log prior art, but it is content-neutral (it logs database pages) and provides no notion of *bounded similarity drift in a binary-vector substrate*. Kanerva consolidation is stochastic and not replayable. Karunaratne 2020 does not characterise determinism across re-runs. No prior art on the present search combines (i) cryptographic determinism, (ii) bounded drift, and (iii) replayability in a binary-HDC consolidation step. |
| **Aspect 3 — Kanerva-Memory-Format ("KMF") packed-bit wire format** (Round-4 Claims 18, 23) | Apache Parquet (magic footer `PAR1`, footer-first read pattern); HDF5 (chunking with the Fletcher32 checksum filter, per https://docs.hdfgroup.org/hdf5/v1_14/group___f_l_e_t_c_h_e_r32.html). | Apache Arrow (columnar packing, IPC framing) + HDF5 Fletcher32 (per-chunk checksums) + Merkle 1979 / IPFS (content-addressed integrity) + Git object store (BLAKE-family per-block hashing) + MessagePack / CBOR (RFC 8949) / FlatBuffers / Cap'n Proto / Bencode / Protocol Buffers (generic binary serialisation comparators). | *Technical effect:* fault detection, footer-first streaming load, and cross-implementation interoperability of stored binary-hypervector memory state, with corruption locatable to a single block by cryptographic hash rather than by CRC. *Mechanism:* the conjunction of (i) `D` mod 64 = 0 column-major bit packing tuned for word-aligned XOR and popcount operations, (ii) per-block BLAKE3 cryptographic content hash (not CRC, not Fletcher32), (iii) a magic trailer with a corpus-version field at the end of the file, and (iv) an explicit header pointer to the conformance corpus version. No prior format combines all four: Parquet has the magic footer (i) and (iii) but not (ii) or (iv); HDF5 has (i) and (ii) only in the weaker Fletcher32 form and lacks (iii)-(iv); Arrow has (i) but neither (ii) nor (iii) nor (iv); Git and IPFS have (ii) in a generic-blob form but lack (i), (iii) and (iv); CBOR, FlatBuffers, Cap'n Proto, Bencode and Protocol Buffers are generic serialisation comparators and lack (i), (ii) and (iv). The claim is a *combination claim* requiring all four limitations. |
| **Aspect 4 — Thermometer-quantised random projection encoder** (Round-4 Claim 27) | Rahimi, Kanerva, Rabaey 2016 ISLPED "A Robust and Energy Efficient Classifier Using Brain-Inspired Hyperdimensional Computing" (foundational level encoder); Rahimi et al. 2017 TCAS-I "High-Dimensional Computing as a Nanoscalable Paradigm" (comprehensive treatment); Imani 2017-2019 line of work (derivative encoder applications); **US 12,204,899 B2 (Imani, UCSD, "Stochastic hyperdimensional arithmetic computing," issued 21 January 2025) — closest single US patent reference on the encoder slot, bipolar `±1` substrate with stochastic arithmetic but no cryptographic anchor**; **US 12,015,424 B2 (Imani, UCSD, "Network-based hyperdimensional system," issued 18 June 2024) — adjacent on complex `{±1, ±i}` substrate**. | Rahimi 2016 (level encoder) + US 12,204,899 B2 (stochastic bipolar HDC arithmetic) + US 12,015,424 B2 (complex-valued NetHD encoder) + Charikar 2002 (sign-LSH random projection) + Achlioptas 2003 (database-friendly random projections, sparse sign matrices) + Andoni & Indyk 2008 (seeded random projection) + Jégou et al. 2011 (PQ as a contrast for footprint reduction). | *Technical effect:* approximately 32-fold reduction in storage footprint relative to f32, preserving cosine similarity as Hamming similarity within a bounded error envelope verifiable against the conformance corpus. *Mechanism:* every level hypervector is derived from a single specified BLAKE3 XOF invocation against `(corpus_seed, dimension, level_index)`, with the corpus seed persisted in the KMF wire-format header (Aspect 3, Claim 18) and the bounded error envelope persisted in the conformance corpus (Aspect 5, Claim 31). Rahimi's encoder uses a sliding correlation between two random seed hypervectors with implementation-defined RNG state; the Imani follow-on work and US 12,204,899 B2 use PRNG seeding (bipolar `±1` for the stochastic variant) without a wire format and without a cryptographic XOF anchor; US 12,015,424 B2 uses a complex-valued `{±1, ±i}` substrate orthogonal to Smritidb's binary `{0, 1}` substrate. **None of the verified prior art (US 12,015,424 B2, US 12,204,899 B2, and the other five HDC patents) uses a cryptographic XOF for level-hypervector generation.** The combined prior art teaches each constituent technique in isolation; it does not teach the integrated technical effect of *training-free, cryptographically anchored, wire-format-portable* encoding verifiable against a corpus. |
| **Aspect 5 — Conformance corpus methodology / Conformance-gating system** (Round-4 Claim 31) | NIST CAVP (Cryptographic Algorithm Validation Program) test-vector suites under FIPS 140-3 and the SP 800-140 series. | NIST SP 800-140C, SP 800-140D and SP 800-140F (algorithm-validation requirements) + the FAISS `bench_all_ivf` benchmark suite (a *performance* benchmark, not a byte-identity corpus) + Pinecone-style integration test suites (functional, not bit-exact). | *Technical effect:* enabling verifiable, contractual cross-binding interoperability of an approximate-vector data structure across a plurality of implementations, by reference to a versioned, bit-exact test-vector corpus that is itself persisted as a KMF artefact and *gates load-time admission* into the family of conformant runtimes (Round-2 OpenTV reframing, see `08-claims.md` Claim 31). *Mechanism:* application of the cryptographic-validation test-vector methodology (NIST CAVP / SP 800-140) — fixed input/expected-output pairs that any conforming implementation must reproduce bit-for-bit — to an approximate-vector index for the first time on the present search. NIST CAVP targets *deterministic* cryptographic primitives with no acceptable error envelope; Claim 31 extends the methodology to an *approximate* index that nevertheless admits byte-identity at the encoded substrate level. The corpus is itself a data structure, persisted in KMF (Aspect 3, Claim 18), distinguishing the claim from a method-of-testing in the abstract. |
| **Aspect 6 — Permutation-positional binary n-gram encoder** (Round-4 Claim 35) | Charikar 2002 SimHash (deterministic, training-free binary text fingerprint, bag-of-features); Manku et al. 2007 (SimHash applied to web crawling). | SimHash (Charikar 2002) + Plate 1995 HRR (positional encoding via convolution, real-valued) + classical character-n-gram bag-of-features + Kanerva 2009 (HDC permutation primitive) + Frady-Kleyko-Sommer 2018 (sequence indexing in recurrent networks) + Mikolov 2013 Word2Vec / Pennington 2014 GloVe / Devlin 2019 BERT (trained text encoders, cited for completeness as the *non-applicable* prior art). | *Technical effect:* training-free, deterministic, order-preserving text representation in a fixed-size binary substrate, with operating-point feasible on resource-constrained on-device hardware (mobile, embedded, edge) on which trained neural encoders cannot fit within memory or energy budgets. *Mechanism:* the conjunction of (i) a binary substrate (BSC), (ii) permutation-positional encoding with a permutation derived deterministically by BLAKE3-XOF, and (iii) per-symbol hypervector generation by BLAKE3-XOF against `(corpus_seed, dimension, symbol)`. SimHash is bag-of-features and lacks (ii); Plate is real-valued and lacks (i); Word2Vec / GloVe / BERT are *trained* and accordingly lack the training-free feasibility property; Frady et al. 2018 is real-valued and recurrent-network-flavoured, lacking (i). The combination is not taught by any single reference or by the closest combinations on the present search. |

**Examiner-anticipation flag.** For each of the six aspect-rows above, the inventor's view is that no single reference anticipates and no combination renders the corresponding statutory claim obvious to a person skilled in the art at the priority date. The patent agent is invited to re-test this view against the closest-combination column on receipt of any FER.

**Round-6 cross-row consistency note.** The seven verified US HDC patents catalogued at §5.4 have been cross-checked against every row of the matrix. The verified patents update Row 1 (US 2022/0019441 A1 and US 12,518,150 B2 added) and Row 4 (US 12,204,899 B2 and US 12,015,424 B2 added) as set out above. Rows 2, 3, 5 and 6 are unchanged because (i) no verified HDC patent recites Hebbian consolidation, (ii) no verified HDC patent recites a wire format with magic + per-block hash + corpus-version anchoring, (iii) no verified HDC patent recites a conformance corpus with byte-identity gates, and (iv) no verified HDC patent recites a text encoder. The differentiation matrix is internally consistent.

---

## 7. Strongest Independent Claims (After Differentiation)

The following ranking is the inventor's preliminary view of claim strength, by reference to (i) the novelty floor over the closest prior art, (ii) the inventive step (non-obviousness) over the closest combinations, and (iii) the technical effect demonstrated by the implementation pinned at HEAD `17334f8`. The ranking is offered to assist prosecution strategy; all six claims are pursued.

1. **Aspect 1 — Deterministic tiebreaker (Round-4 Claims 1, 7).** STRONGEST. No prior art identified that addresses cross-implementation byte-identity for HDC bundle operations. The technical effect — verifiable interoperability across the Rust core, WebAssembly, JavaScript, Python and JVM bindings — is concrete and reproducible against the conformance corpus. Hopfield 1982 (noted in §4.11) does not weaken this ranking: it teaches an energy-minimisation associative-memory framework but does not address cross-implementation byte-identity, which is the technical effect on which Claims 1 and 7 turn. **The Round-6 verified prior art (§5.4) reinforces this ranking:** US 2022/0019441 A1 (the closest single US patent reference) discloses binary HDC primitives in a system context but leaves bundle tie resolution unspecified; US 12,518,150 B2 (the recent IBM bundling patent) uses a fundamentally different bundling mechanism (share-based cumsum-step-function indicator selection) rather than majority-with-tiebreaker. None of the seven verified US HDC patents has cryptographic determinism for tie resolution, and none discloses cross-implementation byte-identity at the bundle operation.
2. **Aspect 3 — KMF wire format (Round-4 Claims 18, 23).** STRONG. Format-level patents are common, and the patent agent will be alert to obviousness combinations of Arrow + HDF5 Fletcher32 + Merkle / Git / IPFS content-addressed integrity. The defensive posture is to draw the claim narrowly around the *four-element combination* of per-block BLAKE3 integrity (not CRC, not Fletcher32), magic trailer for streaming load with corpus-version field, column-major bit packing tuned for `D` mod 64 = 0, and explicit conformance-corpus pointer.
3. **Aspect 2 — Replayable consolidation (Round-4 Claim 9).** STRONG. The combination of determinism, bounded drift and replay from log is unique on the present search; SQLite WAL is the closest log-and-replay prior art but is content-neutral. The drift bound is a verifiable engineering invariant.
4. **Aspect 5 — Conformance corpus / conformance-gating system (Round-4 Claim 31).** MEDIUM. The claim is methodology-flavoured but Round-2 reframing as a system (OpenTV reframing) anchors it in the load-time gating action. The §3(d) and §3(k) risks are managed by drafting around the *artefact* (the corpus is a data structure persisted in KMF) and the *system property* (the system is gated by the corpus). The novelty depends on whether NIST CAVP or any equivalent (FIPS 140-3, SP 800-140) is held to read on it; the inventor's view is that the cryptographic-validation analogy reinforces non-obviousness without anticipating, because no prior art applies the methodology to an approximate vector substrate.
5. **Aspect 4 — Encoder (Round-4 Claim 27).** MEDIUM. Closest to Rahimi 2016 and Imani 2017-2019, with an obviousness threat from the LSH / SimHash family. The differentiator (BLAKE3-anchored level generation and wire-format pinning) must be drawn sharply; the inventor will accept narrowing if necessary to preserve grant.
6. **Aspect 6 — N-gram encoder (Round-4 Claim 35).** MEDIUM. The combination of SimHash and Plate 1995 is the principal obviousness threat. The defensive position is the three-element conjunction set out in §6 above and the absence of any single reference that teaches all three.

---

## 8. Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Rahimi 2016 (ISLPED) and the Imani follow-on work may render Aspect 4 / Claim 27 obvious by combination with a separate teaching of cryptographic seeding — in particular Charikar 2002 (sign-LSH), Achlioptas 2003 (database-friendly random projections) or Andoni & Indyk 2008 (seeded random projection for reproducibility). | Sharpen Claim 27 around (i) the specific tuple `(corpus_seed, D, level_index)` over which the XOF is invoked, (ii) the storage of the corpus seed in the KMF header, and (iii) the bit-identity property verified against the conformance corpus. Emphasise that the Rahimi / Imani encoders are *not* portable and that the LSH family does not produce a level-quantised output. |
| Generic file-format prior art (Arrow, Parquet, HDF5, CBOR, FlatBuffers, Cap'n Proto, Bencode, Protobuf) may be combined with cryptographic-integrity teachings (Merkle trees, signed archives, Git, IPFS content-addressing) to attack Aspect 3 / Claims 18 and 23 on obviousness grounds. | Emphasise the four-element combination (column-major `D` mod 64 = 0 bit packing; per-block BLAKE3 integrity; magic trailer with corpus-version field; explicit conformance-corpus pointer) and the absence of any single prior-art format that combines all four. Parquet has the footer and the magic but not BLAKE3 integrity; HDF5 has Fletcher32 chunks but not the magic trailer or the corpus pointer; Git and IPFS have content-addressing but are substrate-agnostic. |
| SQLite WAL (Hipp et al., https://www.sqlite.org/wal.html) may be combined with Kanerva consolidation to attack Aspect 2 / Claim 9 on obviousness grounds. | Distinguish on grounds that SQLite WAL is content-neutral (logs database pages), provides no notion of *bounded similarity drift in a binary-vector substrate* and does not teach cryptographic-XOF bit-selection. The technical effect of Claim 9 is the binding of bounded drift to the replayability property, which SQLite WAL does not address. |
| Bloom filter / Count-Min / MinHash / SimHash may be combined to argue that hash-derived bit selection in a binary substrate is generally known, attacking Aspect 1 / Claims 1, 7 (tiebreaker) and Aspect 6 / Claim 35 (text encoder) on obviousness grounds. | For Aspect 1: emphasise that Bloom and Count-Min are set-membership oracles, not bundle operations, and that the tiebreaker is invoked only at *tied* positions during reduction. For Aspect 6: emphasise that SimHash is bag-of-features and order-insensitive, whereas the Smritidb text encoder is permutation-positional. |
| **(Round 4 — NEW)** Hopfield 1982 / Kohonen 1972 / Steinbuch 1961 / Willshaw 1969 may be cited against Aspect 1 / Claims 1, 7 as the closest single-reference family for the *associative-memory framing*. | Distinguish on the system-level technical effect: classical associative-memory networks (Hopfield in particular) resolve tied local fields by an implementation-defined convention ("retain previous state" or "random") and do not address cross-implementation byte-identity. Smritidb's tiebreaker is cryptographically deterministic and produces byte-identical output across heterogeneous runtimes — a system property absent from the entire §4.11 family. |
| **(Round 6 — NEW)** US 2022/0019441 A1 (Rosing, Imani et al., UCSD) may be cited against Aspect 1 / Claims 1, 7 as the closest single US patent reference (binary HDC system primitives in a hyperdimensional processing unit with content-addressable memory). | Distinguish on the BLAKE3 four-tuple `(tag, D, i, n)` deterministic tiebreaker, which US 2022/0019441 A1 does not recite; on the corpus-seed-in-wire-format-header system property, which US 2022/0019441 A1 does not recite; and on the cross-implementation byte-identity technical effect verifiable against the conformance corpus (Claim 31), which US 2022/0019441 A1 does not address. US 2022/0019441 A1 leaves bundle tie resolution unspecified. |
| **(Round 6 — NEW)** US 12,518,150 B2 (Hersche and Rahimi, IBM, "Bundling hypervectors," issued 6 January 2026) may be cited as recent prior art on the bundle-operation slot. | Distinguish on the bundling mechanism itself: US 12,518,150 B2 uses share-based bundling via cumsum-step-function indicator selection (`∂ = cumsum(a) / sum(a)`) on an *M*-dimensional weight vector, which is not majority voting and has no tied-position concept. Smritidb Claim 1 / Claim 7 is directed to element-wise integer majority with BLAKE3 four-tuple tiebreaker — a fundamentally distinct operation. Patent-agent action: confirm via USPTO PAIR that no Hersche / Rahimi continuation or divisional claims integer-majority bundling. |
| **(Round 6 — NEW)** US 12,015,424 B2 (Imani, UCSD, "Network-based hyperdimensional system") and US 12,204,899 B2 (Imani, UCSD, "Stochastic hyperdimensional arithmetic computing") may be cited against Aspect 4 / Claim 27 (encoder) as the closest UCSD encoder-line US patents. | Distinguish on the substrate: US 12,015,424 B2 uses complex `{±1, ±i}` phase values, US 12,204,899 B2 uses bipolar `±1` stochastic arithmetic, and Smritidb uses binary `{0, 1}` hypervectors generated by BLAKE3 XOF on the four-tuple `(corpus_seed, D, level_index)`. None of the verified UCSD encoder patents uses a cryptographic XOF for level-hypervector generation, and none pins the corpus seed in a wire-format header. |
| Methodology claims (Aspect 5 / Claim 31) may attract §3(d) ("mere new use of a known process") or §3(k) ("computer programme per se") objections under the Patents Act, 1970. | Frame the claim as a technical artefact (the corpus itself is a data structure, persisted in KMF, with a specified generation procedure) and a system property (the system *is verifiable* against the corpus and *gated* by it at load time per the OpenTV reframing in Claim 31), not as a method of testing in the abstract. See the §3(k) defence memorandum at `patent/11-3k-defense.md`. |
| §3(k) attack on all algorithm-flavoured claims. | Per the §3(k) defence memorandum, all preambles are drafted with a "technical-effect" framing — verifiable cross-implementation interoperability, approximately 32-fold memory-footprint reduction, corruption detection by per-block hash — that locates the invention in the technical (rather than the abstract-algorithmic) domain. The §6 matrix has been re-cast in Round 2 to lead with the technical effect rather than the algorithmic mechanism. |
| §3(d) attack on Aspect 1 / Claims 1, 7 — characterised as "new use of BLAKE3". | §3(d) is principally a pharmaceutical doctrine. To pre-empt any generalised reading, the response is that BLAKE3 is used in Claims 1 and 7 *as a deterministic source for a tiebreaker in a binary-HDC bundle operation* — a use that produces the technical effect of cross-implementation byte-identity and is therefore not a "mere" new use. Per the CRI Guidelines 2017 §4.5 and *Ferid Allani* (Delhi HC, 2019), demonstrable technical effect defeats §3(d) on the same logic as it defeats §3(k). |
| §3(d) attack on Aspect 2 / Claim 9 — characterised as "new use of Hebbian learning". | The Claim 9 consolidation is *not* Hebbian learning. Hebbian learning is stochastic; Claim 9 is deterministic, bounded and replayable. The transformation from a stochastic rule to a verifiable engineering primitive is a substantive technical contribution and produces a system-level property (replayable state) that the underlying Hebbian rule does not. |
| §3(d) attack on Aspect 3 / Claims 18, 23 — characterised as "new use of a binary file format". | The KMF format is not a "new use" of an existing format; it is a new format characterised by a *combination* of four limitations (§6 Aspect 3 row), at least one of which (per-block BLAKE3 integrity tuned for binary-hypervector blocks with corpus-version anchoring) is not found in any prior file format. |
| §3(d) attack on Aspect 4 / Claim 27 — characterised as "new use of random projection". | The Claim 27 encoder is characterised by the conjunction of thermometer quantisation, BLAKE3-XOF anchoring and wire-format portability; the technical effect is footprint reduction with a corpus-verifiable bounded error envelope. This is not a "mere" new use of random projection; it is a structured combination that produces a system-level property absent from the cited prior art. |
| Section 8 statement of foreign filings will be required if the Applicant pursues a PCT or non-Indian national filing. | Maintain a current list of all foreign filings and update Form-3 within the statutory window. |
| Open-source publication of the Smritidb codebase predates filing. | The Applicant relies upon the inventor's grace period under §31 of the Patents Act, 1970 (twelve months for the inventor's own publication), and the corresponding USPTO §102(b)(1)(A) grace period. The patent agent to confirm the grace-period analysis for each jurisdiction of interest. |

---

## 9. Specific Reference List for Form-2 §4 (Background of the Invention)

The following references are proposed for express citation in the Form-2 background section, ordered by descending relevance. Full bibliographic detail is provided in §2 and §4 above; this list is for the patent agent's convenience.

1. Kanerva, P. (1988). *Sparse Distributed Memory*. MIT Press. ISBN 0-262-11132-2.
2. Kanerva, P. (2009). "Hyperdimensional Computing: An Introduction." *Cognitive Computation* 1(2): 139-159. DOI 10.1007/s12559-009-9009-8.
3. Kanerva, P. (1996). "Binary Spatter Codes." *ICANN 1996*, LNCS 1112: 869-873. DOI 10.1007/3-540-61510-5_146.
4. Plate, T. A. (1995). "Holographic Reduced Representations." *IEEE TNN* 6(3): 623-641. DOI 10.1109/72.377968.
5. Rahimi, A., Kanerva, P. and Rabaey, J. M. (2016). "A Robust and Energy Efficient Classifier Using Brain-Inspired Hyperdimensional Computing." *ISLPED 2016*: 64-69. DOI 10.1145/2934583.2934624.
6. Rahimi, A. et al. (2017). "High-Dimensional Computing as a Nanoscalable Paradigm." *IEEE TCAS-I* 64(9): 2508-2521. DOI 10.1109/TCSI.2017.2705051.
7. Imani, M. et al. (2017). "Exploring Hyperdimensional Associative Memory." *HPCA 2017*: 445-456. DOI 10.1109/HPCA.2017.28.
8. Imani, M. et al. (2017). "VoiceHD." *ICRC 2017*: 1-8. DOI 10.1109/ICRC.2017.8123650.
9. Imani, M. et al. (2018). "Hierarchical Hyperdimensional Computing for Energy Efficient Classification." *DAC 2018*, Article 108. DOI 10.1145/3195970.3196060.
10. O'Connor, J., Aumasson, J.-P., Neves, S. and Wilcox-O'Hearn, Z. (2020). "BLAKE3: One Function, Fast Everywhere." Project specification.
11. NIST (2015). *FIPS 202: SHA-3 Standard*. DOI 10.6028/NIST.FIPS.202.
12. Charikar, M. S. (2002). "Similarity Estimation Techniques from Rounding Algorithms." *STOC 2002*: 380-388. DOI 10.1145/509907.509965.
13. Datar, M., Indyk, P., Immorlica, N. and Mirrokni, V. S. (2004). "Locality-Sensitive Hashing Scheme Based on p-Stable Distributions." *SoCG 2004*: 253-262. DOI 10.1145/997817.997857.
14. Andoni, A. and Indyk, P. (2008). "Near-Optimal Hashing Algorithms for Approximate Nearest Neighbor in High Dimensions." *CACM* 51(1): 117-122. DOI 10.1145/1327452.1327494.
15. Achlioptas, D. (2003). "Database-friendly Random Projections." *JCSS* 66(4): 671-687. DOI 10.1016/S0022-0000(03)00025-4.
16. Bloom, B. H. (1970). "Space/Time Trade-offs in Hash Coding with Allowable Errors." *CACM* 13(7): 422-426. DOI 10.1145/362686.362692.
17. Merkle, R. C. (1979). "A Certified Digital Signature." US Patent 4,309,569 (filed 1979, granted 5 January 1982). Original Merkle-tree construction.
18. SQLite Consortium (Hipp, D. R. et al.). "Write-Ahead Logging." https://www.sqlite.org/wal.html.
19. The HDF Group. "HDF5 Fletcher32 Filter." https://docs.hdfgroup.org/hdf5/v1_14/group___f_l_e_t_c_h_e_r32.html.
20. Apache Parquet project. "Parquet File Format Specification" v2.4.
21. Internet Engineering Task Force (Bormann, C. and Hoffman, P.). *RFC 8949: Concise Binary Object Representation (CBOR)*. December 2020.
22. NIST. SP 800-140 series (SP 800-140A through SP 800-140F). https://csrc.nist.gov/projects/cryptographic-module-validation-program.
23. Rachkovskij, D. A. and Kussul, E. M. (2001). "Binding and Normalization of Binary Sparse Distributed Representations by Context-Dependent Thinning." *Neural Computation* 13(2): 411-452. DOI 10.1162/089976601300014592.
24. Kleyko, D. et al. (2018). "Classification and Recall with Binary Hyperdimensional Computing." *IEEE TNNLS* 29(12): 5880-5898. DOI 10.1109/TNNLS.2018.2814400.
25. Malkov, Yu. A. and Yashunin, D. A. (2018). "Efficient and Robust Approximate Nearest Neighbor Search Using Hierarchical Navigable Small World Graphs." *IEEE TPAMI* 42(4): 824-836. DOI 10.1109/TPAMI.2018.2889473.
26. Jégou, H., Douze, M. and Schmid, C. (2011). "Product Quantization for Nearest Neighbor Search." *IEEE TPAMI* 33(1): 117-128. DOI 10.1109/TPAMI.2010.57.
27. Kleyko, D. et al. (2022). "Vector Symbolic Architectures as a Computing Framework for Emerging Hardware." *Proceedings of the IEEE* 110(10): 1538-1571. DOI 10.1109/JPROC.2022.3209104.

---

## 10. Action Items for Filing Day

- [ ] **P0** — Run formal InPASS searches per the methodology of §5.1; record results with dates, screen-captures and per-hit differentiation. **Blocker.** (Round-6 status: substantively de-risked by public-web surrogate search returning zero Indian HDC patent hits across 10 queries; formal portal session by the patent agent still required for the Section 8 record because `ipindiaservices.gov.in/PublicSearch/` returned HTTP 403 to programmatic fetch.)
- [x] **P0 (USPTO — Imani-HDC) — Round-6 closed.** Round-1 cite US 11,775,847 B2 was confirmed in Round 2 as a KPN Innovations patent unrelated to HDC. Round-4 candidate leads US 12,450,896 and US 11,854,253 have been verified in Round 6 as Intel / Narayan Srinivasa image-classifier patents, not Imani-Rosing UCSD HDC patents — both Round-4 leads are withdrawn. The actual closest single US patent reference for the Imani-Rosing UCSD HDC slot is **US 2022/0019441 A1** (binary HDC system, HPU, CAM). See §5.2 (c) and §5.4. Patent-agent residual: USPTO PAIR check on the underlying serial number of US 2022/0019441 A1 to confirm grant status.
- [x] **P0 (USPTO — Pinecone) — Round-6 closed.** Round-1 cite US 10,956,464 B1 confirmed non-existent. Pinecone Systems Inc. has no US patent on file reachable by public web search as of 2026-05-20. See §5.2 (b).
- [ ] **Patent-agent residual** — Run USPTO Patent Public Search inventor queries for `Imani, Mohsen`, `Rosing, Tajana Simunic` and `Rahimi, Abbas`; run assignee queries for `Pinecone Systems` (and variants); pull USPTO PAIR family-graph for US 12,518,150 B2 (IBM Hersche / Rahimi bundling) to confirm no continuation or divisional claims integer-majority bundling. Capture for the Section 8 record and any FER response.
- [ ] Run Espacenet bulk search; export the top one hundred title-and-abstract hits and triage per §5.3 methodology.
- [ ] Run PATENTSCOPE searches for PCT applications per §5.5 methodology.
- [x] Round-4: Removed vague Indian-affiliated HDC publication allusion at §4.6 per "cite specifically or remove" standard. Replaced with positive negative-finding paragraph and an open action item.
- [ ] Execute the IEEE Xplore and ACM Digital Library author-search keyed on IISc Bangalore and IIT Bombay institutional affiliations (replacing the Round-1 [TO VERIFY] flag removed in Round 4).
- [ ] Re-check vendor documentation for Pinecone, Qdrant, Weaviate, FAISS, Milvus and Chroma at filing day to confirm no vendor has, in the intervening period, shipped a bit-exact conformance corpus.
- [ ] Cite the top references from §9 as references in Form-2 §4 (Background).
- [ ] Prepare an Information-Disclosure-equivalent listing for any related search disclosures discovered after the priority date.
- [ ] Confirm with the patent agent the grace-period analysis under §31 of the Patents Act, 1970 in respect of the Apache-2.0 open-source publication of the Smritidb codebase at HEAD `17334f8`.
- [ ] Decide on the PCT versus direct national strategy; if PCT, prepare Form-3 (Statement and Undertaking) accordingly.
- [ ] If the Phase-2 LSH-based approximate-recall index is to be claimed, prepare a continuation or divisional analysis against HNSW (Malkov & Yashunin 2018) and the LSH family (§4.8); the present application does not pursue such claims.

---

## 11. Provenance and Verification Note

**Citations independently verified against the original publications and confirmed accurate as to author, year, venue and pagination:**

- Kanerva 1988 (ISBN); Kanerva 2009 (DOI); Kanerva 1996 (DOI); Plate 1995 (DOI); Rachkovskij & Kussul 2001 (DOI); BLAKE3 v1.0 specification; NIST FIPS 202 (DOI); Bloom 1970 (DOI); Charikar 2002 (DOI); Datar et al. 2004 (DOI); Andoni & Indyk 2008 (DOI); Achlioptas 2003 (DOI); Hebb 1949; Malkov & Yashunin 2018 (DOI); Jégou et al. 2011 (DOI); Ge & Parhi 2020 (DOI); Schlegel et al. 2022 (DOI); Kleyko et al. 2022 (DOI); Karunaratne et al. 2020 (DOI); Hersche et al. 2023 (DOI); Imani VoiceHD 2017 (DOI); Imani HPCA 2017 (DOI); Imani DAC 2018 (DOI, Article 108, 6 pages); Rahimi 2016 ISLPED (DOI); Rahimi et al. 2017 TCAS-I (DOI); Kleyko et al. 2018 TNNLS (DOI); Frady-Kleyko-Sommer 2018 (DOI); Thomas-Dasgupta-Rosing 2021 (DOI); Mikolov 2013 (arXiv); Pennington 2014 (DOI); Devlin 2019 (DOI); Manku et al. 2007 (DOI); Cormode & Muthukrishnan 2005 (DOI); Broder 1997 (DOI); Sivic & Zisserman 2003 (DOI); Ge et al. 2013 OPQ (DOI); Aumasson et al. 2013 BLAKE2 (DOI); RFC 8949 CBOR; SQLite WAL documentation; HDF5 Fletcher32 documentation; Apache Parquet specification; Apache Arrow documentation; NIST SP 800-140 series.

**Round-4 additions to the verified-citation list (20 May 2026):** Hopfield 1982 (DOI 10.1073/pnas.79.8.2554); Kohonen 1972 (DOI 10.1109/TC.1972.5008975); Willshaw, Buneman & Longuet-Higgins 1969 (DOI 10.1038/222960a0); Steinbuch 1961 (DOI 10.1007/BF00293853); Anderson 1972 (DOI 10.1016/0025-5564(72)90075-2); Ramsauer et al. 2021 (arXiv:2008.02217); Rao & Fuentes 1998 (DOI 10.1023/A:1007454428309). All DOIs verified via crossref / publisher landing pages (Round-4 reviser cross-checked DOI strings only; full PDFs not independently read in Round 4 — full-text reads recommended for the patent agent if any of these references becomes load-bearing in an FER response).

**Round-6 additions to the verified-citation list (20 May 2026):** US 2022/0019441 A1 (Rosing, Imani et al., UCSD; verified by Google Patents / USPTO image-ppubs fetch); US 12,015,424 B2 (Imani, UCSD; verified); US 12,204,899 B2 (Imani, UCSD; verified); US 11,574,209 B2 (Karunaratne et al., ETH Zurich + IBM; verified); US 10,971,226 B2 (Le Gallo-Bourdeau et al., ETH Zurich + IBM; verified); US 12,260,913 B2 (Lin and Tseng, Macronix; verified); US 12,518,150 B2 (Hersche and Rahimi, IBM; verified). Verification recorded in full at `patent/critiques/p0-prior-art-search-results.md`; the patent agent is invited to re-verify each via USPTO Patent Public Search and capture the first page and claim 1 of each into `patent/prior-art-evidence/us-patents/` (to be created).

**Round-6 confirmation of false leads (now removed):** Round-4 candidate leads US 12,450,896 and US 11,854,253 have been verified as Intel / Narayan Srinivasa image-classifier patents (HDC-based adversarial-perturbation robustness; not Imani-Rosing UCSD patents and not directed to associative-memory persistence / wire format / cross-implementation byte-identity). Both are withdrawn from the Imani-HDC slot. The Round-1 cite of US 10,956,464 B1 is confirmed non-existent and is withdrawn.

**Citations awaiting verification before reliance in any filed document:**

- Indian Patent Office InPASS top-twenty hits and per-hit differentiation (§5.1). `[P0]` Round-6 status: substantively de-risked by public-web surrogate search returning zero Indian HDC patent hits across 10 queries; formal portal session by the patent agent still required for the Section 8 record because `ipindiaservices.gov.in/PublicSearch/` returned HTTP 403 to programmatic fetch.
- IISc Bangalore and IIT Bombay HDC publications (§4.6) — Round-2 [TO VERIFY] flag converted to a positive Round-4 finding of "no Indian-affiliated HDC publication independently verified to bibliographic-pin precision"; the patent agent's formal author-search by IEEE / ACM author-index plus InPASS is the remaining action.
- Indian-affiliated HDC patent applications at InPASS (§5.1).
- Espacenet hits for the principal queries (§5.3).
- PATENTSCOPE PCT hits for the principal queries (§5.5).
- USPTO PAIR check on US 2022/0019441 A1 to confirm grant status (Round-6 web verification confirmed publication but not grant).
- USPTO PAIR family-graph check on US 12,518,150 B2 to confirm no Hersche / Rahimi continuation or divisional claims integer-majority bundling.
- Imani QuantHD 2019 (TCAD): DOI 10.1109/TCAD.2019.2954472 — verified by citation database; full text not independently read in Round 2.
- Imani SparseHD 2019 (FCCM) and BRIC 2019 (DAC): pagination verified by citation database; full text not independently read in Round 2.
- Gayler 1998 (AAAI Fall Symposium working notes): verified by citation; not independently read in Round 2.

**Round-4 reviser's note on web verification.** The Round-4 pass attempted public-web verification of the two P0 US-patent items using WebSearch and WebFetch tools (search providers: Google Search via WebSearch; HTTP fetches against patents.justia.com, image-ppubs.uspto.gov and patents.google.com). The justia and patents.google.com servers returned HTTP 403 on direct fetch — these are anti-bot-protected sites and are not amenable to programmatic WebFetch. Indirect verification via search-snippet text plus uspto image-ppubs PDF download links was used instead. The reviser could *not* extract structured claim text from the image-only USPTO PDFs (the streams are CCITT-fax-encoded image objects requiring OCR). Accordingly, the Round-4 candidate-lead findings are **suggestive but not authoritative**, and the original [P0] flags are retained on the agent's worklist. Authoritative verification requires the patent agent's manual execution of USPTO Patent Public Search at https://ppubs.uspto.gov/pubwebapp/ followed by examination of each result's claim-text page.

End of memorandum. To be reviewed by patent agent. Round 6 has resolved both USPTO P0 items by independent web research (see `patent/critiques/p0-prior-art-search-results.md`); the remaining InPASS P0 item is substantively de-risked by the public-web surrogate search but a formal portal session by the patent agent is still required for the Section 8 record. Any further round prior to Form-2 filing should address: (i) the formal InPASS portal session once executed, (ii) the USPTO PAIR check on US 2022/0019441 A1 grant status and the US 12,518,150 B2 family-graph, (iii) any prior art discovered between Round 6 and the filing date, and (iv) any reframing that arises from the Round-4 §4.11 Hopfield / classical associative-memory addition or the Round-6 §5.4 verified-prior-art table (e.g., whether dependent claims under Claims 1 and 7 should expressly recite the "cross-implementation byte-identity" technical effect to strengthen the differentiation against Hopfield 1982 and US 2022/0019441 A1).
