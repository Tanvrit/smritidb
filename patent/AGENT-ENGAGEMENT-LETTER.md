# Patent Filing Engagement — Smritidb

**To:** [Registered Patent Agent — to be filled in]
**From:** Vivek Singh, for Tanvrit Private Limited
**Date:** 2026-05-21
**Re:** Engagement to file Patent Application at the Indian Patent Office (IPO) — Smritidb associative memory system

---

Dear Sir / Madam,

I write on behalf of Tanvrit Private Limited (the **Applicant**), an Indian company in the process of confirming its DPIIT Startup recognition, to engage your good offices for the preparation, finalisation and filing of a complete patent application at the Indian Patent Office in respect of an invention titled "Smritidb" (the **Invention**), of which I, Vivek Singh, am the sole inventor.

The drafting package is filing-ready in substance. The Applicant seeks your professional assistance for the procedural, formal and certification steps that only a registered patent agent can discharge, and for review of the drafted Complete Specification against IPO conformance requirements before submission. The particulars are set out below.

## 1. Background

Smritidb is an open-source, computer-implemented **associative memory system** built on a **binary hyperdimensional computing (HDC) substrate**. Symbols, keys, sequences and structured records are represented as 8,192-bit binary hypervectors; recall, similarity, binding and unbinding are reduced to bitwise XOR, popcount-based Hamming distance and majority-vote bundling, executed in constant time per operation against the population of stored hypervectors. The store is consolidated through a **replayable Hebbian update** rule that is deterministic in the order and content of writes, and is persisted to a **Kanonical Memory Format (KMF)** — a self-describing binary wire format carrying a magic header, version, deterministic parameter block, hypervector payload and a BLAKE3 content hash that anchors cryptographic integrity across processes, languages and machines.

The inventive contribution lies not in HDC per se, which is a known mathematical idiom, but in the **system-level engineering** that makes a hyperdimensional memory behave as a reliable artefact of software infrastructure: a **deterministic encoding pipeline** (canonical string normalisation, fixed permutation tables, seeded basis vectors derived from a domain-separated keyed hash) producing the same hypervector for the same input across the Rust, TypeScript/WebAssembly and Python implementations; **cross-implementation bit-exactness** asserted as a runtime invariant rather than an aspiration, verified by a conformance corpus of paired (input, expected-hash) vectors; **replayable Hebbian consolidation**, in which a store can be reconstructed from its append-only write log to the same byte-level KMF image; and **persistence with cryptographic integrity**, in which the BLAKE3 hash of the canonical payload is computed on write and verified on read, with corruption surfacing as an error rather than a silent semantic drift. These properties — taken together — yield a tangible technical effect: an associative memory whose state is reproducible, auditable and portable across heterogeneous runtimes, a guarantee not previously made for HDC systems and not achievable by the prior art identified to date.

The Invention is released to the public under the Apache-2.0 licence, which carries an express patent grant; the Applicant nevertheless seeks patent protection to anchor a defensive position and to provide a stable legal basis for downstream commercial deployment.

## 2. Scope of Engagement

The Applicant requests the Agent to undertake the following tasks:

1. **Section 8 / Form 3 declaration.** Advise on prior foreign filings (none are anticipated at first filing; an undertaking under Section 8(1) and a Form 3 statement of nil foreign filings will accordingly be required).
2. **Form 1 preparation.** Complete Form 1 with the Applicant and Inventor details. Confirm Startup-category fee eligibility based on Tanvrit Private Limited's DPIIT registration certificate (to be furnished separately).
3. **Form 2 review and finalisation.** Review the drafted Complete Specification at `patent/FORM-2-COMPLETE-SPECIFICATION.md`. Verify IPO-format conformance per the Patents Rules, 2003 (in particular Rule 9: 12-point font, 1.5 line spacing, 4 cm left margin, A4 paper). Verify the claims structure (36 claims total, 8 independents). Verify that the Abstract does not exceed 150 words.
4. **Drawings preparation.** The drafting package includes Figures 1–7 as ASCII mock-ups at `patent/06-drawings-list.md`. The Agent is requested to arrange formal A4 black-and-white drawings in conformity with the Patents Rules, 2003 (Rule 15: line drawings only, 5 mm reference numerals, no shading or colour).
5. **InPASS formal prior-art search.** A surrogate web search has been conducted (see `patent/critiques/p0-prior-art-search-results.md`); the formal InPASS keyword search remains a required certification step to be discharged by the Agent.
6. **USPTO and Espacenet supplementary searches.** Verified candidate prior art for the Section 3(k) and Section 2(1)(ja) defences is listed at `patent/10-prior-art.md`, §5.2 and §5.4. The Agent is requested to retrieve the full claim text for the cited UCSD, ETH Zurich / IBM Zurich and Macronix HDC patents so that distinguishing limitations can be confirmed on the face of the issued claims.
7. **Form 5 Declaration of Inventorship.** Standard form, to be executed by the inventor.
8. **e-Filing.** Submit Forms 1, 2, 3 and 5 (and optionally Form 9 for early publication) via the IP India e-portal at https://ipindiaonline.gov.in/epatentfiling/.
9. **Form 18 (Request for Examination).** File within 48 months of the priority date (latest by 2030-05-21 if the application is filed on the date of this letter).
10. **First Examination Report (FER) response preparation.** When the IPO First Examination Report issues, the Agent is requested to use the working documents at `patent/10-prior-art.md`, `patent/11-3k-defense.md` and `patent/12-examples.md` as the substantive basis for the response.

The Applicant draws the Agent's particular attention to the **Section 3(k) exclusion** for "computer programmes per se". The Complete Specification has been drafted around the technical-effect framing established by *Ferid Allani v. Union of India* (Delhi High Court, 2019), reaffirmed in *Microsoft Technology Licensing LLC v. Assistant Controller of Patents* (Delhi High Court, 2023) and *OpenTV Inc. v. Controller of Patents* (Delhi High Court, 2023). The Section 3(k) survival brief at `patent/11-3k-defense.md` cites seven Indian decisions and articulates the technical effects — cryptographic integrity of memory state, bit-exact reproducibility across heterogeneous runtimes, replayable consolidation — that are advanced as taking the Invention outside the "per se" exclusion.

## 3. Drafting Package Provided

The Applicant has prepared the following materials, all held in the git repository at `<repo URL — TBD>`, branch `<branch — TBD>`, pinned to commit `17334f8`:

| File | Purpose |
|---|---|
| `patent/FORM-2-COMPLETE-SPECIFICATION.md` | Filing-ready Form 2 (~24,500 words, ~110 A4 pages at IPO format). |
| `patent/FORM-2-PAGE-AUDIT.md` | Word-count to page-count audit, with excess-page fee estimate. |
| `patent/FORM-2-FILING-PROCEDURE.md` | Filing procedure, including pandoc-based PDF conversion. |
| `patent/00-cover.md` through `patent/13-filing-checklist.md` | The 14 working artefacts forming the refinement audit trail. |
| `patent/REFINEMENT-LOG.md` | Six refinement rounds and final read-aloud notes. |
| `patent/10-prior-art.md` | Prior-art memo (internal — to underpin the FER response). |
| `patent/11-3k-defence.md` (`11-3k-defense.md` on disk) | Section 3(k) survival brief citing seven Indian decisions. |
| `patent/critiques/p0-prior-art-search-results.md` | Web-search-derived P0 prior-art findings. |

For the Agent's situational awareness: the drafting has been carried through five refinement rounds, a final read-aloud pass, integration of P0 prior-art findings, and a cross-document consistency audit. The Applicant does not anticipate further substantive revisions on its side and offers the package as a stable basis for the Agent's review.

## 4. Open Items Requiring Agent Input

1. Applicant address (registered office of Tanvrit Private Limited).
2. Inventor address.
3. Priority date (intended: date of filing of this complete specification, with no convention claim).
4. Patent agent's Power of Attorney on Form 26, if the Agent is to file on the Applicant's behalf.

## 5. Fee Estimate (Startup tier, per First Schedule)

| Item | Estimated Fee |
|---|---|
| Form 1 (e-filing) | Rs. 1,600 |
| Form 2 (up to 30 pages) | included |
| Excess pages (~80 over 30) | ~Rs. 12,800 |
| Form 3 (Section 8) | nil |
| Form 5 (Inventor declaration) | nil |
| Form 9 (early publication, optional) | Rs. 2,500 |
| Form 18 (Request for Examination) | Rs. 4,000 |
| Form 26 (POA, if Agent files) | nil |
| Drawings preparation (formal) | Rs. 5,000–15,000 (vendor estimate) |
| **Subtotal — government fees** | **~Rs. 21,000–24,000** |
| **Patent agent professional fee** | **TBD by Agent quote** |

The Agent is requested to provide a professional-fee quotation against the Scope set out at §2, distinguishing the one-time filing fee from the contingent FER response fee, so that the Applicant may approve engagement on a settled basis.

## 6. Confidentiality

The underlying source code of Smritidb is publicly visible under the Apache-2.0 licence in the Applicant's public git repository. The drafted Complete Specification is, however, **not** yet published. The Applicant requests the Agent to treat the Form 2 and all working artefacts in `patent/` as confidential until the Applicant expressly authorises publication — whether by Form 9 early publication or by automatic publication 18 months after the priority date under Section 11A. The Applicant is content for the Agent to treat the Apache-2.0-licensed source repository as in the public domain for the purposes of the prior-art analysis.

## 7. Timeline

The Applicant requests the Agent to:

- Confirm engagement within **5 business days** of receipt of this letter, together with the professional-fee quotation referred to at §5.
- Complete Form 2 review and IPO-format finalisation within **15 business days** of confirmation.
- File the application within **30 business days** of engagement, subject to the Applicant having furnished the addresses at §4 and executed Form 26.

The Applicant is content to extend any of the foregoing periods on reasonable cause shown by the Agent.

## 8. Contact

**Inventor and Applicant Representative:**

Vivek Singh
Email: ervivek40@gmail.com
Postal address: [TBD]

I should be obliged if you would acknowledge receipt of this letter and confirm willingness to take up the engagement at your earliest convenience. Should you require any clarification on the technical content prior to confirming, I am available to meet (in person or by video conference) and to walk you through the drafting package.

Yours faithfully,

\

\

**Vivek Singh**
Inventor, for and on behalf of Tanvrit Private Limited

---

**Enclosures:**

1. `FORM-2-COMPLETE-SPECIFICATION.md` (PDF to be generated by the Agent, or by the Applicant via pandoc/xelatex, on confirmation of engagement).
2. All working artefacts in the `patent/` directory of the repository.
3. Web-search prior-art findings at `patent/critiques/p0-prior-art-search-results.md`.
4. Public source repository link: [TBD].
