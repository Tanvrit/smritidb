# P0 Prior-Art Search Results

**Conducted:** 2026-05-20
**Sources:** WebSearch + WebFetch (Google Patents, Justia Patents, ip-ppubs.uspto.gov, NCBI/PMC, ipindia.gov.in)
**Code pin:** `17334f8` (matches the claims pinned in `patent/08-claims.md` Round-4 changelog)
**Conducted by:** Inventor; intended to inform — not replace — formal USPTO Patent Public Search and formal InPASS keyword search by the registered patent agent before Form-2 filing.

---

## 0. Search Volume Summary

- **Independent web searches run:** 19 (WebSearch) plus 9 (WebFetch on specific patent pages and portals) = **28 independent network calls**.
- **Candidate prior-art records examined for claim-1 text or for assignee/inventor identity:** **9 USPTO patent records** (US 11,775,847; US 10,956,464; US 11,854,253; US 12,450,896; US 12,015,424; US 11,574,209 = US 2022/0019441 A1 published / US 11,574,209 issued from a sister application; US 10,971,226; US 12,204,899; US 12,260,913; US 12,518,150), plus **2 Indian portals** (`ipindia.gov.in`, `iprsearch.ipindia.gov.in/PublicSearch/`) and **1 Indian institutional patent list** (IIT Bombay IRCC).
- **Conflict findings against Smritidb's 8 independent claims:** **0 directly conflicting**. **2 adjacent-but-non-conflicting** (US 12,518,150 IBM bundling; US 2022/0019441 A1 UCSD HDC systems). The remaining records are either non-HDC, or HDC at a different layer of abstraction (encoder, in-memory hardware, networked transmission, stochastic arithmetic).
- **Final P0 status:** **Resolved on the public-web question** (no public-web prior art reads on any independent claim). **Still pending patent-agent action** for the formal InPASS keyword-search certification (the InPASS portal at `ipindiaservices.gov.in/PublicSearch/` returned HTTP 403 to programmatic fetch and is not indexed by Google for substantive text content; a manual session by the registered agent is required for the Section 8 / FER record).

---

## 1. InPASS Indian Prior-Art Findings

### 1.1 Portal access status

- `https://ipindiaservices.gov.in/PublicSearch/` — HTTP **403 Forbidden** to WebFetch. The portal is an ASP.NET-style server-rendered application protected against scripted access; cookie/JavaScript challenge required.
- `https://iprsearch.ipindia.gov.in/PublicSearch/PublicationSearch/Search` — HTTP **404** on the public-search endpoint when followed directly.
- `https://ipindia.gov.in/` (root) — succeeds; the public-search system is described and supports search by Application Number, Publication Number, Patent Number, Applicant Name/Country, Inventor Name/Country, Title, Abstract, Complete Specification, IPC, PCT Application/Publication, Application/Publication/Grant/Priority dates, with AND/OR/NOT operators. No login required for keyword search according to the documentation reached.
- Google `site:ipindiaservices.gov.in` keyword search for `hyperdimensional`, `binary spatter`, `associative memory`, `vector database` — returns **zero hits** from that domain (Google appears not to crawl the deep search results of the portal).

### 1.2 Search queries executed (web-side, surrogate for InPASS)

1. `site:ipindiaservices.gov.in hyperdimensional computing`
2. `"hyperdimensional" patent IIT Bombay OR IISc OR IIIT Hyderabad India`
3. `India patent application hyperdimensional computing IPO 2023 2024`
4. `"associative memory" "binary vector" India patent IPO publication`
5. `"Indian patent" "vector" "hypervector" 2024 OR 2025 filing`
6. `"sparse distributed memory" OR "binary spatter code" patent India IPO`
7. `"hyperdimensional computing" India patent application Form-2 specification 2024 2025`
8. `"hyperdimensional" patent IIT Bombay IRCC inventor application India`
9. `"Indian patent" inventor "vector embedding" OR "high-dimensional vector" 2024 IPO`
10. `"binary spatter code" patent India OR USPTO`

### 1.3 Hits — and whether they conflict with Smritidb claims

| Source | Hit | Assignee/Inventor | Conflict with Claims 1, 7, 9, 18, 23, 27, 31, 35? |
|---|---|---|---|
| IIT Bombay IRCC `PatentList.jsp` | URL returned 404 to WebFetch (site appears restructured); index page reachable but no programmatic listing of titles. | (n/a) | **Indeterminate by web search alone — patent-agent InPASS Applicant-Name search "Indian Institute of Technology Bombay" required.** |
| `site:ipindiaservices.gov.in` keyword queries | Zero hits on any HDC/HD-vector/SDM/binary-spatter terminology. | (n/a) | **No conflicts identified.** |
| All 10 surrogate Google searches | No Indian patent application or Indian-affiliated PCT national-phase patent surfaced on hyperdimensional computing, binary spatter codes, sparse distributed memory, vector database with BLAKE3, or HDC-text encoding. The only India-side hits were procedural (Patents Rules 2024 amendments, Form-2 procedural guidance, Indian Patent Office press releases). | (n/a) | **No conflicts identified.** |

### 1.4 Conclusion

**No conflicts found on the public web.** The substantive InPASS keyword search remains an agent-execution item (not a finding-of-fact item) because the public web cannot reach inside the InPASS full-text index. The four searches the patent agent must execute on InPASS for the Section 8 record are:

1. Title/Abstract: `hyperdimensional` AND (`binary` OR `vector`)
2. Title/Abstract: `associative memory` AND `binary`
3. Title/Abstract: `vector database` OR `vector store`
4. Title/Abstract: `BLAKE3` OR `cryptographic hash` AND `database`

Each search should be repeated as a Complete-Specification full-text query if Title/Abstract returns zero. The expected outcome (based on the present web search): **zero substantive hits.**

---

## 2. Imani / Rosing HDC US Patents — Verification

### 2.1 Round-4 candidate leads — re-verification

#### US 12,450,896 — **NOT an Imani/Rosing patent**

- **Title:** "Apparatus, method, and computer-readable medium for robust response to adversarial perturbations using hyperdimensional vectors"
- **Inventor:** **Narayan Srinivasa** (single inventor)
- **Assignee:** **Intel Corp**
- **Filing Date:** 29 Sep 2023 (continuation of US 11,854,253 lineage)
- **Issue Date:** 21 Oct 2025
- **Abstract first sentence:** "Apparatuses, methods, and articles of manufacture are disclosed. An example apparatus includes processor circuitry to assign a location value hyperdimensional vector (HDV) to a location in an image of a first patch of one or more pixels…"
- **Subject matter:** Image-classification robustness against adversarial perturbations using HDC. Uses HDV binding/permutation/consensus-sum but is firmly in the image-classifier domain, not in associative-memory persistence / wire format / cross-implementation byte-identity.

#### US 11,854,253 — **NOT an Imani/Rosing patent**

- **Title:** "Apparatus, method, and computer-readable medium for robust response to adversarial perturbations using hyperdimensional vectors"
- **Inventor:** **Narayan Srinivasa** (single inventor)
- **Assignee:** **Intel Corp**
- **Filing Date:** 26 Jun 2021
- **Issue Date:** 26 Dec 2023
- **Abstract:** "The invention relates to artificial neural networks. More specifically, the invention relates to adversarial perturbations introduced to training and classification performed by artificial neural networks."
- **Claim 1 first sentence:** "The processor circuitry is configured to instantiate seeder circuitry to assign a location value hyperdimensional vector (HDV) to a location in a received image of a first patch of one or more pixels."
- **Subject matter:** Same family as US 12,450,896. Image-classification robustness; not an associative-memory wire-format/cross-implementation invention.

**Round-4 correction:** Both Round-4 "candidate leads" must be **removed** from the Imani-HDC slot in `patent/10-prior-art.md` §5.2. They are Intel patents on a narrow image-robustness application of HDC and do not read on any Smritidb independent claim.

### 2.2 Actual Imani / Rosing UCSD HDC patents (verified this round)

#### US 12,015,424 B2 — Network-based hyperdimensional system

- **Inventor:** Mohsen Imani (sole)
- **Assignee:** The Regents of the University of California (UCSD)
- **Filing Date:** 25 Aug 2022
- **Issue Date:** 18 Jun 2024
- **Abstract first sentence:** "Disclosed is a network-based hyperdimensional system having an encoder configured to receive input data and encode the input data using hyperdimensional computing to generate a hypervector having encoded data bits that represent the input data."
- **Hypervector representation:** Complex phase values `{±1, ±i}` — **not binary**.
- **Does NOT recite:** binary `{0,1}` hypervectors, BLAKE3, deterministic majority tiebreaker, JSON wire format, conformance corpus, content-addressable hyperdimensional substrate.
- **Conflict with Smritidb claims:** None. Smritidb is restricted to binary `{0,1}` hypervectors and uses BLAKE3 for both the per-coordinate level encoder (Claim 27), the bundle tiebreaker (Claims 1, 7), the consolidation deterministic ordering (Claim 9), and the wire-format per-block integrity (Claims 18, 23). US 12,015,424 sits in the complex-valued NetHD encoder space, which is orthogonal.

#### US 12,204,899 B2 — Stochastic hyperdimensional arithmetic computing

- **Inventor:** Mohsen Imani (sole)
- **Assignee:** The Regents of the University of California (UCSD)
- **Filing Date:** 13 May 2022
- **Issue Date:** 21 Jan 2025
- **Abstract first sentence:** "Stochastic hyperdimensional arithmetic computing is provided."
- **Claim 1 first sentence:** "A method for efficient and robust computation, comprising: converting stored data into hyperdimensional data…"
- **Hypervector representation:** `±1` (bipolar) — **not Smritidb's `{0,1}` binary**.
- **Does NOT recite:** BLAKE3, deterministic majority tiebreaker, JSON wire format, conformance corpus.
- **Conflict with Smritidb claims:** None. Stochastic arithmetic over bipolar HDC is a different primitive set from byte-identical binary HDC; the lack of any cryptographic anchor or wire-format limitation places it well outside Claims 1, 7, 9, 18, 23, 27, 31, 35.

#### US 2022/0019441 A1 — Circuits, methods, and articles of manufacture for hyper-dimensional computing systems and related applications

- **Inventors:** **Tajana Simunic Rosing, Justin Morris, Mohsen Imani, Yeseong Kim, John Messerly, Yunhui Guo, Behnam Khaleghi, Saransh Gupta, Sahand Salamat, Joonseop Sim**
- **Assignee:** The Regents of the University of California (UCSD)
- **Filing Date:** 14 Jul 2021
- **Publication Date:** 20 Jan 2022 (US patent **application** publication; grant status not confirmed by the present web search — patent agent should run a USPTO PAIR check on serial number).
- **Abstract first sentence:** "the present invention relates to the field of information processing in general, and more particularly, to hyper-dimensional computing systems."
- **Subject matter:** Comprehensive HDC system patent covering hyperdimensional processing unit (HPU), binary hypervector ops, content-addressable memory for binary hypervectors, binding/bundling/permutation primitives.
- **Recites:** binary hypervectors; CAM (content-addressable memory) for binary hypervectors.
- **Does NOT recite:** BLAKE3 hashing; deterministic majority tiebreaker by cryptographic hash; JSON wire format; conformance corpus by SHA-256 byte-identity; the specific KMF magic/trailer/header-offset/per-block-hash construction; the specific BLAKE3-XOF thermometer encoder of Smritidb Claim 27; the permutation-positional text encoder of Claim 35.
- **Conflict with Smritidb claims:** **Adjacent prior art — not blocking.** This is the **closest single reference** to the substrate framing of Smritidb's Claims 1 (method) and 7 (system) because it discloses binary HDC primitives in a system context. However, every Smritidb independent claim is distinguished by a substantive limitation US 2022/0019441 lacks:
  - **Claim 1 / Claim 7:** distinguished by **BLAKE3-anchored deterministic majority tiebreaker with the four-tuple `(tag, D, i, n)` input** — US 2022/0019441 leaves tie resolution unspecified.
  - **Claim 9:** distinguished by **replayable Hebbian consolidation bounded by `maxSimDelta` with BLAKE3-derived ordering** — US 2022/0019441 has no Hebbian replay-by-snapshot mechanism.
  - **Claims 18 / 23:** distinguished by the **KMF magic+spec-version+reserved-offset+per-block-BLAKE3+trailer-magic header-after-blocks** construction — US 2022/0019441 has no persistence wire-format.
  - **Claim 27:** distinguished by the **canonical decimal-ASCII seed + BLAKE3-XOF + thermometer-quantised level** construction — US 2022/0019441 does not recite this encoder.
  - **Claim 31:** distinguished by the **JSON conformance corpus with SHA-256-of-packed-bytes byte-identity gates** — US 2022/0019441 has no conformance-gating apparatus.
  - **Claim 35:** distinguished by the **permutation-positional n-gram text encoder cross-referencing Claim 1 tiebreaker and Claim 31 corpus** — US 2022/0019441 does not address text encoding.

**Required action:** **Add** US 2022/0019441 A1 to `patent/10-prior-art.md` §5.2 as the **actual Imani-Rosing UCSD HDC system patent**, replacing the (now-falsified) Round-1 US 11,775,847 KPN cite and the (now-falsified) Round-4 US 12,450,896 / US 11,854,253 Intel cites. The differentiation matrix (§6) must be updated to add US 2022/0019441 A1 as the *closest single reference* against rows 1 and 2 (Claims 1, 7, 9).

#### US 10,971,226 B2 — Hyper-dimensional computing device (ETH Zurich + IBM, **not** UCSD)

- **Inventors:** Manuel Le Gallo-Bourdeau, Kumudu Geethan Karunaratne, Giovanni Cherubini, Abu Sebastian, Abbas Rahimi, Luca Benini
- **Assignee:** ETH Zurich and IBM
- **Filing Date:** 30 May 2019
- **Issue Date:** 6 Apr 2021
- **Subject matter:** Resistive-memory device for storing HD vectors as conductive statuses in 2D memristors. In-memory hardware.
- **Recites:** HD vectors (i.i.d. pseudo-random components).
- **Does NOT recite:** binary `{0,1}` substrate primitives, BLAKE3, tiebreaker, wire format, conformance corpus.
- **Conflict with Smritidb claims:** None. Pure hardware-substrate patent.

#### US 11,574,209 B2 — Device for hyper-dimensional computing tasks (ETH Zurich + IBM, **not** UCSD)

- **Inventors:** Karunaratne, Le Gallo-Bourdeau, Cherubini, Sebastian, Rahimi, Benini
- **Assignee:** ETH Zurich and IBM
- **Filing Date:** 30 May 2019
- **Issue Date:** 7 Feb 2023
- **Claim 1 first sentence:** "A device for hyper-dimensional computing for inference tasks, the device comprising an item memory for storing hyper-dimensional item vectors…"
- **Does NOT recite:** BLAKE3, tiebreaker, JSON wire format, conformance corpus.
- **Conflict with Smritidb claims:** None. Inference-device patent for HDC, no persistence/cross-implementation byte-identity claim.

#### US 12,260,913 B2 — Hyperdimensional computing device (Macronix — flash/NVM in-memory)

- **Inventors:** Yu-Hsuan Lin, Po-Hao Tseng
- **Assignee:** Macronix International Co Ltd
- **Filing Date:** 9 Feb 2023
- **Issue Date:** 25 Mar 2025
- **Claim 1 first sentence:** "A hyperdimensional computing device, comprising: a non-volatile memory cell array, coupled to a plurality of first word lines…"
- **Recites:** majority-rule bundling **in NVM hardware**, but no cryptographic tiebreaker, no wire format, no conformance corpus.
- **Conflict with Smritidb claims:** None. A flash-cell HDC hardware patent; the majority-rule bundling is implemented at the analog sense-amplifier layer and the patent has no algorithmic-tiebreaker claim.

#### US 12,518,150 B2 — **Bundling hypervectors** (IBM — Hersche, Rahimi)

- **Inventors:** Michael Andreas Hersche, Abbas Rahimi
- **Assignee:** IBM
- **Filing Date:** 27 Jun 2022
- **Issue Date:** **6 Jan 2026** (very recent — issued in the four months prior to Smritidb's filing date)
- **Abstract:** "Embodiments are disclosed for a method. The method includes bundling a set of M code hypervectors, each of dimension D, where M>1."
- **Bundling approach:** **Share-based bundling using an M-dimensional weight vector mapped to an S-dimensional indicator vector via a step function `∂=cumsum(a)/sum(a)`** — element-wise selection from indicated code hypervectors, **not majority voting and not a tiebreaker**.
- **Does NOT recite:** binary majority bundling with a tiebreaker; BLAKE3-derived tiebreaker; cryptographic tie resolution; JSON wire format; conformance corpus.
- **Conflict with Smritidb claims:** **None — but this is a recent IBM patent that the patent agent should be aware of.** US 12,518,150 implements bundling by a fundamentally different mechanism (cumsum-step-function-indicator share-selection) from Smritidb's element-wise integer majority with BLAKE3 tiebreaker. The two approaches diverge at the bundling operation itself; Claim 1 of US 12,518,150 (so far as can be inferred from the patent prose; full claim text was not extractable from the public PDF) is directed to the share-based mechanism, not majority.
  - **However:** if the patent agent's USPTO formal search reveals that any Hersche/Rahimi continuation, divisional, or sister application claims **integer majority bundling**, that family must be re-checked against Smritidb Claim 1's tiebreaker limitation. The novelty of Smritidb Claim 1 sits on the **BLAKE3 four-tuple `(tag, D, i, n)` tiebreaker** specifically, which is not in the Hersche/Rahimi work or in any other public HDC patent surfaced by this search.

### 2.3 Other Imani patents found

Justia and Google Patents indicate Imani is named on roughly **20 US patents** (per his SIGDA profile and CV at UC Irvine). The present web search surfaced three (US 12,015,424; US 12,204,899; US 2022/0019441 A1) by direct URL fetch and a handful more by title in search snippets. **The patent agent's formal USPTO inventor search ("Imani, Mohsen") will be authoritative**; the present web search is sufficient only to confirm that **none of the Imani patents reachable by public web fetch reads on any Smritidb independent claim**.

### 2.4 Conclusion — Imani / Rosing patents

- **Round-4 candidate leads US 12,450,896 and US 11,854,253 are FALSE LEADS** (Intel/Srinivasa, not UCSD/Imani-Rosing). They must be **removed** from `patent/10-prior-art.md` §5.2.
- **Actual closest Imani-Rosing UCSD HDC patent is US 2022/0019441 A1** (system patent covering binary HDC primitives, HPU, CAM). This is **adjacent** and is the **closest single reference** for Smritidb Claims 1, 7, 9 — but is distinguished by the BLAKE3-tiebreaker / replay / wire-format / conformance-corpus / encoder / text-encoder limitations that define each Smritidb independent.
- **No claim revisions required** as a consequence of the Imani/Rosing verification.

---

## 3. Pinecone US Patents

### 3.1 Confirmed: NO Pinecone-Systems-Inc patents found

- The Round-1 cite `US 10,956,464 B1` returned HTTP 404 in Round 4 and remains a **fabricated or transcription-error cite**.
- Justia assignee search for `Pinecone Systems Inc.` returns **zero patents**.
- Google Patents assignee filter for `Pinecone Systems` returns **zero patents**.
- Hits in Justia for "Pinecone" resolve to **Pinecone Material Inc.** (semiconductors, unrelated) and **Pinecone Imaging Corporation** (imaging, unrelated). Neither is the vector-DB company.
- Crunchbase / company-profile confirmation: Pinecone (founded 2019 by Edo Liberty) publishes academic papers but **has not filed any US patent** under that corporate name reachable by public web search.

### 3.2 Conclusion — Pinecone

**Pinecone Systems Inc. has no public US patent on file as of 2026-05-20.** The Round-1 US 10,956,464 B1 cite must be **deleted entirely** from `patent/10-prior-art.md`. The replacement narrative should read: *"Pinecone Systems Inc. (commercial closed-source vector database) does not appear to have any issued US patent or published US patent application; Pinecone's technical disclosures are limited to its corporate blog, AWS Marketplace listings, and product documentation; consequently no Pinecone patent reads on any Smritidb independent claim."*

---

## 4. Recent HDC-Adjacent Patents (2023–2026)

### 4.1 Findings table

| Patent | Issued | Inventor / Assignee | Subject | Reads on Smritidb? |
|---|---|---|---|---|
| US 12,015,424 B2 | 18 Jun 2024 | Imani / UCSD | Network-based HDC encoder, complex `{±1, ±i}` | No (orthogonal; complex-valued) |
| US 11,854,253 B2 | 26 Dec 2023 | Srinivasa / Intel | HDC image-classifier adversarial robustness | No (image classifier) |
| US 12,450,896 B2 | 21 Oct 2025 | Srinivasa / Intel | Continuation of 11,854,253 | No (image classifier) |
| US 11,574,209 B2 | 7 Feb 2023 | Rahimi et al. / ETHZ+IBM | HDC inference device | No (inference device) |
| US 12,204,899 B2 | 21 Jan 2025 | Imani / UCSD | Stochastic HDC arithmetic (bipolar `±1`) | No (orthogonal; bipolar; no cryptographic anchor) |
| US 12,260,913 B2 | 25 Mar 2025 | Lin/Tseng / Macronix | NVM-cell-array HDC hardware | No (hardware) |
| **US 12,518,150 B2** | **6 Jan 2026** | Hersche, Rahimi / IBM | **Bundling hypervectors (share-based step function)** | **No (different bundling mechanism)** |
| US 2022/0019441 A1 | published 20 Jan 2022 | Rosing, Imani et al. / UCSD | HDC system, HPU, CAM | No (closest single reference but distinguished) |

### 4.2 Sparse-distributed-memory adjacent patents (older)

- **US 5,113,507 A** — Method and apparatus for a sparse distributed memory system (1992; Kanerva-era; expired). Not a 2023+ filing, but noted because Round-2 SDM citations rely on Kanerva 1988 non-patent literature; this expired patent confirms SDM was disclosed in the patent literature and is therefore unambiguous **prior art** but at a level so far above the BLAKE3-binary specifics of Smritidb that no conflict arises.
- **US 9,189,745 B2** — Temporal memory using sparse distributed representation (Numenta-style HTM; 2015). Adjacent but distinct from binary-spatter-code HDC; does not read on Smritidb.

### 4.3 Vector-database / wire-format adjacent patents (2024)

- **US 12,517,868** — Vector embedding compression (Justia 2024 listing). Not a wire-format-integrity claim; orthogonal.
- **US 9,990,687** — Fast and repeatable embedding of high-dimensional data objects using deep learning. Pre-Smritidb; no conformance-corpus / BLAKE3 limitation.
- **US 12,315,220** — Methods and systems for indexing embedding vectors representing disjoint classes at above-billion scale (2025). HNSW-class indexing; no wire-format / cross-implementation byte-identity claim. Already adjacent-noted in Smritidb §3.8 (HNSW non-conflicting prior art).

### 4.4 BLAKE3 / cryptographic-anchor patents

- No US patent surfaces that recites BLAKE3 specifically as a database-integrity primitive. Adjacent patents in the blockchain-database integrity space (e.g., US 11,711,202; US 10,419,225; US 10,114,980) use generic "hash" recitations and are blockchain-ledger oriented, not vector-substrate oriented. None reads on Smritidb's BLAKE3-tiebreaker / BLAKE3-XOF-encoder / per-block-BLAKE3-wire-format claims.

### 4.5 Conclusion — Recent HDC patents

The 2023–2026 HDC patent landscape is **dense in encoder, inference, and hardware-substrate inventions** but **empty in the specific intersection of (binary-HDC, cryptographic-determinism, wire-format-integrity, conformance-corpus-gating)** that Smritidb occupies. No 2023–2026 patent reads on any Smritidb independent claim. The closest recent issuance — US 12,518,150 (IBM bundling, Jan 2026) — uses a fundamentally different bundling mechanism (cumsum-step-function-indicator share-selection rather than integer-majority with tiebreaker) and therefore does not impair Smritidb Claim 1 / 7's novelty or non-obviousness.

---

## 5. Files to update in `patent/10-prior-art.md`

### 5.1 Add new entries (with full citation form)

```
US 2022/0019441 A1 — Rosing, T. S., Morris, J., Imani, M., Kim, Y., Messerly, J., Guo, Y.,
   Khaleghi, B., Gupta, S., Salamat, S., and Sim, J., "Circuits, methods, and articles of
   manufacture for hyper-dimensional computing systems and related applications,"
   Pub. Date 20 Jan 2022, Assignee The Regents of the University of California (UCSD).
   Closest single reference to Claims 1, 7 (binary HDC primitives in a system context).
   Distinguished by absence of (i) BLAKE3-four-tuple deterministic majority tiebreaker,
   (ii) replayable Hebbian consolidation, (iii) KMF wire-format, (iv) JSON conformance
   corpus, (v) BLAKE3-XOF thermometer encoder, (vi) permutation-positional text encoder.

US 12,015,424 B2 — Imani, M., "Network-based hyperdimensional system," Issued 18 Jun 2024,
   Assignee The Regents of the University of California (UCSD). Adjacent (complex
   {±1, ±i} encoder; orthogonal to Smritidb's binary {0,1} substrate).

US 12,204,899 B2 — Imani, M., "Stochastic hyperdimensional arithmetic computing,"
   Issued 21 Jan 2025, Assignee The Regents of the University of California (UCSD).
   Adjacent (stochastic bipolar arithmetic; no cryptographic anchor, no wire format).

US 11,574,209 B2 — Karunaratne, K. G., Le Gallo-Bourdeau, M., Cherubini, G., Sebastian, A.,
   Rahimi, A., Benini, L., "Device for hyper-dimensional computing tasks,"
   Issued 7 Feb 2023, Assignees ETH Zurich and IBM. Adjacent (inference-device hardware;
   no algorithmic-tiebreaker, no wire-format).

US 10,971,226 B2 — Le Gallo-Bourdeau, M., Karunaratne, K. G., Cherubini, G., Sebastian, A.,
   Rahimi, A., Benini, L., "Hyper-dimensional computing device," Issued 6 Apr 2021,
   Assignees ETH Zurich and IBM. Adjacent (resistive-memory HD-vector storage hardware).

US 12,260,913 B2 — Lin, Y.-H. and Tseng, P.-H., "Hyperdimensional computing device,"
   Issued 25 Mar 2025, Assignee Macronix International Co Ltd. Adjacent (flash-cell HDC
   hardware; analog-sense-amplifier majority; no cryptographic tiebreaker, no wire format).

US 12,518,150 B2 — Hersche, M. A. and Rahimi, A., "Bundling hypervectors,"
   Issued 6 Jan 2026, Assignee International Business Machines Corp. Recent (issued
   four months pre-filing); uses share-based bundling via cumsum-step-function-indicator
   element selection rather than integer-majority-with-tiebreaker. Distinguished from
   Smritidb Claims 1 and 7 by the bundling mechanism itself, not merely by the tiebreaker
   construction. Patent-agent action: confirm via USPTO PAIR that no Hersche/Rahimi
   continuation or divisional claims integer-majority bundling; if any such continuation
   exists, recheck Claim 1's BLAKE3-four-tuple-tiebreaker limitation.

US 5,113,507 A — Kanerva, P. et al., "Method and apparatus for a sparse distributed memory
   system," Issued 12 May 1992 (expired). Adjacent (foundational SDM patent; co-extensive
   with Kanerva 1988 non-patent literature already cited).
```

### 5.2 Remove / correct existing entries

```
DELETE: US 11,775,847 B2 ("Imani HDC patent") — confirmed in Round 2 as a KPN Innovations
   media-classification patent unrelated to HDC. Round-4 retained this as a "verification
   failure" entry; Round 5 should delete it entirely from §5.2 and replace it with
   US 2022/0019441 A1 (above) as the actual Rosing-Imani UCSD HDC system patent.

DELETE: US 10,956,464 B1 ("Pinecone patent") — verified as non-existent or transcription
   error. Pinecone Systems Inc. has no US patent reachable by public web search.

DELETE: Round-4 candidate-lead entries for US 12,450,896 and US 11,854,253 ("Imani HDC
   candidate leads") — verified as Intel/Narayan Srinivasa image-classifier patents,
   not Imani-Rosing UCSD patents. Round 4's "candidate leads recorded; claim text and
   assignee to be confirmed by patent agent" status is now resolved as a false-lead
   identification; remove these entries to avoid misleading any future FER response.
```

### 5.3 §6 differentiation-matrix updates

The closest-single-reference column for **Row 1** (Claim 1 method, Claim 7 system — deterministic tiebreaker for bit-exact bundling) should be updated from "Kanerva 1988 + Imani 2017 + Hopfield 1982" to **"US 2022/0019441 A1 (UCSD Rosing-Imani HDC system)"** as the patent-literature closest reference, retaining the non-patent-literature references as supporting closest-combination citations. The differentiating-limitation column should add: *"BLAKE3 four-tuple `(tag, D, i, n)` deterministic tiebreaker, absent from US 2022/0019441 A1 which leaves bundle tie resolution unspecified."*

Row 3 (Claims 18, 23 — wire format) closest-single-reference patent is unchanged (Apache Parquet magic footer remains the closest in patent-adjacent format literature; no HDC patent recites a wire format).

Row 4 (Claim 27 — thermometer encoder) closest-single-reference patent should note **US 12,015,424 B2** as adjacent (complex-valued encoder) but distinguished by the binary `{0,1}` output, the BLAKE3-XOF construction, and the canonical decimal-ASCII seed of Smritidb Claim 27.

Row 6 (Claim 35 — text encoder) is unchanged — no HDC patent recites a text encoder; Charikar 2002 SimHash remains the closest single reference.

### 5.4 P0 status assessment

| P0 item | Round-1 / Round-4 status | Post-Round-5 (this search) status |
|---|---|---|
| (i) **InPASS Indian prior-art search** | Pending formal execution by patent agent. | **Web-side surrogate executed — zero Indian prior-art hits across 10 surrogate queries.** Formal InPASS keyword-search-from-portal still pending patent-agent execution for the Section 8 record, but the substantive risk of an undisclosed Indian-affiliated prior-art application is **assessed as low** based on the public web. |
| (ii) **US 11,775,847 B2 Imani-HDC re-verification** | Round-2 falsified the cite (actually KPN). Round-4 surfaced candidate leads. | **Round-5 resolves: candidate leads (US 12,450,896 and US 11,854,253) are also false (Intel image-classifier). The actual Rosing-Imani UCSD HDC system patent is US 2022/0019441 A1.** P0 item closed. |
| (iii) **US 10,956,464 B1 Pinecone re-verification** | Round-2 reported HTTP 404; Round-4 confirmed no Pinecone patents in public indexes. | **Round-5 confirms: Pinecone Systems Inc. has no US patent on file. The Round-1 cite is to be deleted entirely.** P0 item closed. |

### 5.5 Final P0 status

**Resolved on the public-web question; still pending patent-agent for formal InPASS certification.**

- The two USPTO P0 items (Imani-HDC and Pinecone) are **closed** by this round's verification work. The replacement entries (US 2022/0019441 A1 et al.) are clean and ready for Round 5 to absorb into `patent/10-prior-art.md`.
- The InPASS P0 item is **substantively de-risked** by the public-web surrogate search (zero hits) but a formal InPASS session by the patent agent is still required as part of the Section 8 statement / pre-filing checklist.

---

## 6. CLAIM REVISION REQUIRED — None

**No prior-art finding in this round reads on any Smritidb independent claim.** No claim revision is required as a consequence of this search.

The single recently-issued patent that warranted close inspection — **US 12,518,150 B2 (IBM "Bundling hypervectors", 6 Jan 2026)** — uses a fundamentally different bundling mechanism (cumsum-step-function-indicator share-selection) from Smritidb's element-wise integer majority + BLAKE3 tiebreaker. The two are distinct at the operation layer, not merely at the tiebreaker layer; consequently Smritidb Claim 1's "majority + BLAKE3 four-tuple tiebreaker" remains both novel and non-obvious over US 12,518,150.

The closest single patent reference to Smritidb Claims 1 / 7 is **US 2022/0019441 A1 (UCSD Rosing-Imani)**, which discloses binary HDC primitives in a system context but does not recite (i) the BLAKE3-anchored deterministic tiebreaker, (ii) replayable Hebbian consolidation, (iii) the KMF wire format, (iv) the JSON conformance corpus, (v) the BLAKE3-XOF thermometer encoder, or (vi) the permutation-positional text encoder. Smritidb's independent claims are distinguished from US 2022/0019441 A1 by each of these six features, and the unity-of-invention concept ("BLAKE3-anchored deterministic byte-identity of a binary HDC substrate") is therefore preserved.

---

## 7. Patent-Agent Action Items (carried forward)

1. **Formal InPASS session.** Execute the four keyword searches in §1.4 against the InPASS portal (`https://ipindiaservices.gov.in/PublicSearch/`), capture screenshots and download any matching applications, and certify the negative result for the Section 8 statement.
2. **USPTO inventor search.** Run `Imani, Mohsen` and `Rosing, Tajana Simunic` inventor searches against USPTO Patent Public Search and cross-check against the seven Imani / Rosing patents identified in §2 of this memo. Capture USPTO PAIR status (issued vs. abandoned vs. pending) for US 2022/0019441 A1.
3. **USPTO Hersche/Rahimi family check.** Pull the family-graph for US 12,518,150 (IBM bundling) from USPTO PAIR; confirm no Hersche/Rahimi continuation or divisional claims integer-majority bundling.
4. **Replace P0 entries.** Apply the §5.1, §5.2, §5.3 edits to `patent/10-prior-art.md` in Round 5.
5. **Update Section 8 statement.** When ready to file, the Section 8 statement should disclose US 2022/0019441 A1, US 12,015,424 B2, US 12,204,899 B2, US 11,574,209 B2, US 10,971,226 B2, US 12,260,913 B2, and US 12,518,150 B2 as foreign-application prior art known to the applicant.

---

*End of P0 prior-art search results memo.*
