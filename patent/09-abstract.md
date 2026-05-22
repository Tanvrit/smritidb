<!-- Round 4 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Coverage: added the conformance-gating system (Claim 31) and the permutation-positional N-gram text encoder (Claim 35) to the enumeration, so that all six inventive families of the Round-4 claim set are reflected. The Round-2 abstract enumerated only four of the six families, which under-represented the claim scope.
- Polish: tightened the "high storage input-output cost" and "substantial memory footprint" phrasing into a single technical-problem clause to reclaim the words needed for the two additional inventions, holding the final body word count to 150 or fewer.
- Verified Indian English orthography retained throughout (characterised, quantised, footprint).
- Verified terminal closing form "Reference is invited to Fig. 1 of the accompanying drawings." preserved verbatim.
- Round-2 changelog retained below for audit trail.

Round 2 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Orthography: quantized → quantised
- Polish: replaced terminal "Figure: Fig. 1" with conventional IPO closing form "Reference is invited to Fig. 1 of the accompanying drawings."
-->

# Abstract

A computer-implemented system and method for a binary-hypervector associative memory store characterised by deterministic cross-implementation byte-identical state. Conventional vector databases suffer from silent numerical drift across language runtimes and substantial memory footprint owing to floating-point representations. The disclosed system addresses these problems by: a deterministic tiebreaker for bundle operations using a cryptographic hash over a domain-separation tag, dimension, bit index and multiplicity; a packed-bit wire format with per-block hash integrity and a directional magic trailer; a replayable Hebbian consolidation procedure with bounded similarity drift; a thermometer-quantised random projection encoder for bounded floating-point inputs; a load-time conformance-gating apparatus admitting only byte-identical implementations; and a permutation-positional n-gram text encoder. The combination yields bit-exact interoperability between native, managed and browser runtimes, an approximately thirty-two-fold reduction in memory footprint relative to single-precision vectors, and verifiable fault tolerance through corruption detection. Reference is invited to Fig. 1 of the accompanying drawings.

---

## REFINEMENT-LOG (Round 4)

### Round-4 changes applied

1. **Coverage extended to all six inventive families of the Round-4 claim set.** The Round-2 abstract enumerated four of six families (deterministic tiebreaker, packed-bit wire format, replayable Hebbian consolidation, thermometer-quantised encoder). Round 4 adds:
   - the **conformance-gating apparatus** (independent Claim 31 of `08-claims.md`, system-form per the Round-2 OpenTV reframing), articulated as "a load-time conformance-gating apparatus that admits only byte-identical implementations"; and
   - the **permutation-positional n-gram text encoder** (independent Claim 35), articulated as "a permutation-positional n-gram text encoder".
2. **Phrasing tightened** to hold the body word count to no more than 150. The Round-2 "high storage input-output cost owing to floating-point representations, and substantial memory footprint when persisting large embedding collections" was consolidated to "a substantial memory footprint owing to floating-point representations" — preserving the same technical problem statement (the 32× footprint baseline is the principal quantitative effect) and reclaiming words for the two added inventions.
3. **Tiebreaker description sharpened** from "data, index and count" to "a domain-separation tag, dimension, bit index and multiplicity", matching the BLAKE3 input tuple actually recited in Claim 1(d) of `08-claims.md` (`tag || D || i || n`) so that the abstract does not silently misstate the invention.
4. **Magic trailer described as "directional"** to track the Round-2 promotion into Claim 18 of the directional integrity marker (trailer "FMK\0" being the byte-reversal of header "KMF\0").
5. **Closing form preserved verbatim.** "Reference is invited to Fig. 1 of the accompanying drawings." carried forward without amendment.
6. **Indian English orthography verified throughout.** characterised, quantised, footprint, organised — all -ised forms retained.

### Word-count check

- Round-2 body: 147 words.
- Round-4 body: 146 words (verified by `awk` count over the body paragraph).
- IPO ceiling: 150 words (Rule 13(7)(b) of the Patents Rules, 2003).
- Margin: 4 words to spare.

### Critic-perspective audit

- **Coverage critic.** All six inventive families now appear: tiebreaker (Claims 1, 7), wire format (Claims 18, 23), Hebbian consolidation (Claim 9), thermometer encoder (Claim 27), conformance gating (Claim 31), text encoder (Claim 35). No invention is over-promised; the technical effects stated (cross-platform interoperability, 32× footprint reduction, corruption detection) are all directly grounded in the body of one or more independent claims.
- **Indian-English critic.** Orthography passes (characterised, quantised, organised, footprint). No Americanisms detected.
- **IPO-form critic.** Single-paragraph form retained; opens with "A computer-implemented system and method"; closes with the conventional "Reference is invited to Fig. 1 of the accompanying drawings."; word count at 146 (under the 150 ceiling).
- **Patent-attorney critic.** The "approximately thirty-two-fold reduction" is phrased in words (per IPO convention of avoiding numerals in the abstract where reasonably practicable) and tracks the 40000/1250 ratio at D = 10000 stated in `11-3k-defense.md` §3.3.

### Residual concerns

1. **Word budget close to ceiling.** The abstract sits at 146 words against the 150-word ceiling of Rule 13(7)(b). Any future round that adds a further inventive family, or that tightens a claim wording in a way that needs reflection in the abstract, will have only 4 words of headroom.
2. **No mention of unity-of-invention concept.** The abstract enumerates six families but does not articulate the single inventive concept (BLAKE3-anchored deterministic byte-identity) that unifies them. The IPO does not require this in the abstract — the unity-of-invention concept is articulated in the Explanatory Note of `08-claims.md` — but the patent agent may consider adding a single-clause framing if Rule 13(7)(b) permits.
