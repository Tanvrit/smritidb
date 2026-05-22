# Form-2 Page Audit

**Source:** `patent/FORM-2-COMPLETE-SPECIFICATION.md`
**Pinned to SHA:** `17334f8`
**Audit date:** 2026-05-21
**Typesetting target:** A4 paper, 12 pt Times New Roman / Garamond, 1.5-line spacing, 4 cm left margin, 2 cm right/top/bottom margins (per Patents Rules 2003 Rule 9).

At the said typesetting density, an A4 page accommodates approximately 250 words of running prose. ASCII-art diagrams, pseudocode blocks, tables, and claim-style indentation occupy more vertical space per word than running prose; the page-count estimates below are therefore lower bounds and may rise modestly when the document is actually typeset (particularly in §5 Drawings and §6 Detailed Description).

## Per-section word and page-equivalent count (measured)

| §   | Section                                | Words   | Pages (~250 wpp) |
|-----|----------------------------------------|---------|------------------|
| —   | Cover preamble + Table of Contents     | 140     | 0.6              |
| 1   | Field of the Invention                 | 81      | 0.3              |
| 2   | Background of the Invention            | 1,194   | 4.8              |
| 3   | Objects of the Invention               | 531     | 2.1              |
| 4   | Summary of the Invention               | 1,734   | 6.9              |
| 5   | Brief Description of the Drawings      | 1,889   | 7.6              |
| 6   | Detailed Description of the Invention  | 13,473  | 53.9             |
| 7   | Claims                                 | 5,252   | 21.0             |
| 8   | Abstract                               | 179     | 0.7              |
|     | **Total (running prose basis)**        | **24,473** | **~97.9**     |

*Note: the abstract word count above (179) includes the section heading and the closing-signature lines that are split out into Section 8 by the regex; the formal IPO abstract body itself is **146 words**, well under the Rule 13(7)(b) ceiling of 150.*

## Adjusted page estimate (accounting for ASCII figures, tables, pseudocode)

The §5 Brief Description of the Drawings contains seven ASCII-art figures, each of which occupies considerably more vertical space than a 250-wpp count predicts. The §6 Detailed Description contains numerous pseudocode blocks (bundle, bind, permute, similarity, randomHv, encodeString, encodeEmbedding, encodeWordNgrams, pullCloser, cleanupSearch, put, recall, flagColdItems, KMF read-loop, etc.) and several tables (Item fields, KMF header schema, conformance-corpus categories). Empirically, these expand each page by roughly 20–30 %.

Adjusted estimate: **approximately 110 typeset pages** at 12 pt 1.5-line A4 with the specified margins. This matches the prior estimate given in the brief (~110 pages).

## IPO fee implications

- IPO Form 2 filing fee includes the first **30 pages** of the Complete Specification at no per-page surcharge.
- Each additional page (page 31 onwards) incurs a per-page fee of **Rs. 160** for a small entity / startup, or Rs. 800 / Rs. 1,600 / Rs. 4,000 for large entity tiers (see First Schedule to the Patents Rules 2003).
- For startup-tier filing (most likely for Tanvrit Private Limited):
  - Additional pages: ~110 − 30 = ~80 pages
  - Excess-page fee: ~80 × Rs. 160 = **~Rs. 12,800**
- For large entity filing (counterfactual):
  - Excess-page fee: ~80 × Rs. 1,600 = ~Rs. 1,28,000

(Excess-claim fees are separate: 36 total claims minus the 10 free claims = 26 excess claims, at Rs. 320 per excess claim for startup tier = ~Rs. 8,320. This audit does not double-count that figure.)

## Trimming recommendations (optional, if cost reduction is desired)

The single largest section is **§6 Detailed Description at 13,473 words (~54 pages)**. The IPO sufficiency requirement under §10(4) is satisfied so long as a PHOSITA can construct the invention from the specification; sufficiency does not require exhaustive worked examples or every prior-art distinction paragraph. The following candidate reductions, in priority order, would each cut roughly 1,000–2,000 words without disturbing §10(4) sufficiency:

1. **§6.11 Worked Examples (1,200+ words).** The four worked examples (tiebreaker, embedding round-trip, consolidation step, KMF round-trip) reinforce the algorithms but a PHOSITA can re-derive any of them from the §6.3–§6.8 specification text. Trim to two examples (tiebreaker and KMF round-trip) for ~600 words.
2. **§6.5.2 Distinguishing paragraph against Charikar / Achlioptas (≈300 words).** This is doctrinally useful for examination response but is not §10(4) material; can be reserved for the examination-stage written submission. Saves ~300 words.
3. **§6.7 Hebbian-precedent prior-art paragraph (≈200 words).** Same observation: distinguishing language belongs in the examiner-response, not the body. Saves ~200 words.
4. **§6.10 Standards-body-precedent paragraph (≈150 words).** Same observation. Saves ~150 words.
5. **§6.8.7 "contemplated" attic and zstd subsections (~600 words).** These describe Phase-2 apparatus not present at SHA `17334f8`; they support dependent claims 22 and 24 but a brief one-paragraph disclosure would suffice. Saves ~400 words if compressed.
6. **§6.12 Industrial Applicability (~600 words).** The §10(4)(e) requirement is satisfied by a much shorter recital. Trim from six application paragraphs to three (LLM agent memory, semantic search, on-device RAG). Saves ~300 words.

**Cumulative trimming potential:** approximately 1,750–2,000 words (~7–8 pages) without disturbing §10(4) sufficiency or §3(k) defensibility. This would bring the document to approximately 102 pages and reduce the additional-page fee to approximately 72 × Rs. 160 = Rs. 11,520 (saving Rs. ~1,280 for the startup tier).

## Recommendation

**File at the current ~110-page length.** The additional-page fee at startup tier (~Rs. 12,800) is small relative to the §3(k) defensibility benefit of the long technical-effect treatment and the extensive worked examples. Trimming should be considered only if the applicant elects to file under a higher entity tier where the per-page fee is 10× higher.

If trimming is elected, prioritise items (1) and (5) from the list above; do not trim items (2)–(4) before the first examination report, as the prior-art distinguishing paragraphs may be cited verbatim in the Reply to FER.

## Abstract word-count verification

The IPO Patents Rules 2003 Rule 13(7)(b) ceiling on the abstract body is **150 words**. The assembled abstract body in §8 is **146 words** (verified against the Round-4 changelog in `09-abstract.md`). Margin: 4 words. PASS.

## Claims-count verification

- Independent claims: 8 (at claim numbers 1, 7, 9, 18, 23, 27, 31, 35).
- Total claims: 36 (contiguous numbering 1..36).
- IPO fee includes 10 claims; excess: 26 claims.
- Excess-claim fee (startup tier): 26 × Rs. 320 = Rs. 8,320.
