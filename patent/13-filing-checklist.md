# IPO Filing Checklist

This checklist is provided as an internal working document for the inventor and the applicant. It is not part of the Complete Specification filed with the Indian Patent Office. Statutory references are to the Patents Act 1970 (as amended) and the Patents Rules 2003 (as amended).

## Pre-Filing

- [ ] Confirm Tanvrit Private Limited's DPIIT Startup Recognition status for the startup-tier fee discount under the Fourth Schedule of the Patents (Amendment) Rules 2016.
- [ ] Confirm the applicant's CIN, registered office address, and authorised signatory for Form 1 and Form 5.
- [ ] Confirm the inventor's full residential address and nationality for Form 5.
- [ ] Choose between filing pro se or engaging a registered patent agent (the latter requires Form 26 Power of Attorney).
- [ ] Decide on the priority claim: first filing in India (no priority claim) versus convention application claiming priority from a foreign filing.
- [ ] Conduct a final prior-art search on the Indian Patent Office InPASS database (https://ipindiaservices.gov.in/PublicSearch/) and on Google Patents, Espacenet, and Lens.org. Save results to `patent/prior-art-search/` with date stamps.
- [ ] Confirm whether any pre-filing public disclosure has occurred within 12 months (grace period under §31 of the Patents Act 1970) and document the said disclosure for the prior-art section.
- [ ] Identify the territorial jurisdiction (New Delhi, Mumbai, Chennai, or Kolkata branch of the IPO) by reference to the applicant's registered office address.
- [ ] Reserve a working budget of approximately Rs. 1,00,000 to Rs. 2,00,000 covering filing, examination, agent fees, and drawings preparation.

## Forms to be Filed Concurrently

- [ ] Form 1 — Application for Grant of Patent. Fee: Rs. 1,600 (startup or small entity, e-filing), Rs. 8,000 (other applicants). To be signed by the authorised signatory of Tanvrit Private Limited.
- [ ] Form 2 — Complete Specification (this patent package, assembled into a single PDF per Rule 8 and Rule 9 of the Patents Rules 2003). Sequence: Title, Preamble, Field, Background, Objects, Summary, Drawings (with reference numerals), Detailed Description, Claims, Abstract.
- [ ] Form 3 — Statement and Undertaking under Section 8 of the Patents Act 1970, disclosing any corresponding foreign filings (if none, the form so states).
- [ ] Form 5 — Declaration as to Inventorship under Rule 13(6), naming Vivek Singh as the sole inventor and Tanvrit Private Limited as the assignee.
- [ ] Form 26 — Power of Attorney, only if a registered patent agent files on behalf of the applicant.

## Forms Filed Subsequently

- [ ] Form 9 — Request for Early Publication under Rule 24A. Fee: Rs. 2,500 (startup). Strongly recommended in order to enable faster examination and to crystallise the priority position publicly.
- [ ] Form 18 — Request for Examination under §11B of the Patents Act 1970. Fee: Rs. 4,000 (startup). MUST be filed within 48 months of the priority date or filing date, whichever is earlier. Failure to do so causes the application to be treated as withdrawn.

## Formal Requirements for the Complete Specification

- [ ] Drawings prepared on A4-sized sheets in PDF format, line drawings only, no shading, no colour, with reference numerals at a height of 5 millimetres (Rule 15 of the Patents Rules 2003).
- [ ] Complete Specification typed in 1.5-line spacing, 12-point font, with a left margin of 4 centimetres (Rule 9(1)).
- [ ] Abstract limited to not more than 150 words, on a separate page, with at least one reference to a representative drawing figure (Rule 13(7)).
- [ ] Claims commenced on a new page; each claim numbered consecutively; dependent claims explicitly referring back to the claim or claims from which they depend.
- [ ] Indian English orthography throughout (`characterised`, `optimised`, `organisation`, `realised`).
- [ ] Each page of the Specification numbered consecutively at the bottom centre.
- [ ] Sequence listing not applicable (no biological sequences in this invention).
- [ ] Source-code listings, if included as an appendix, in monospaced font and clearly marked as non-limiting embodiments.

## Subject-Matter Eligibility Pre-Check (§3(k) of the Patents Act 1970)

- [ ] Confirm that the claims, as drafted, recite a technical means and produce a technical effect that is not merely the execution of an algorithm in the abstract (see `11-3k-defense.md`).
- [ ] Confirm that the claims tie their operations to specific hardware artefacts (processor, memory, persistent storage) and to a specific industrial application (associative memory database).
- [ ] Cross-check the technical effects (bit-exact reproducibility, reduced footprint, reduced I/O bandwidth, fault tolerance, cryptographic integrity) against the language of the Computer-Related Inventions Guidelines 2017 of the Indian Patent Office.

## Post-Filing

- [ ] Record the filing receipt and the application number; track the application via the IPO e-portal (https://ipindiaservices.gov.in/PublicSearch/).
- [ ] Monitor for the First Examination Report (FER) — typically issued 18-24 months after the filing of Form 18.
- [ ] Prepare the FER response using `11-3k-defense.md` as the substantive foundation, addressing any objections under §3(k), novelty, inventive step, and clarity.
- [ ] Respond to the FER within the statutory period of six months (extendable by three months on payment of fees under Rule 138).
- [ ] Attend any hearing scheduled by the Controller; prepare written submissions in advance.
- [ ] Consider filing a Patent Cooperation Treaty (PCT) application within 12 months of the priority date for international protection. PCT filing fee at the Indian receiving office is approximately Rs. 17,600 (startup) plus the international fees.
- [ ] Consider filing a divisional application under §16 if any subject matter is identified during examination as relating to a distinct invention.

## Costs (Indicative, Startup Rate, Subject to Revision)

| Item | Fee |
|---|---|
| Form 1 (filing, startup) | Rs. 1,600 |
| Form 2 (Complete Specification, up to 30 pages) | included in filing fee |
| Each additional page beyond 30 | Rs. 160 |
| Each additional claim beyond 10 | Rs. 320 |
| Form 9 (Request for Early Publication, startup) | Rs. 2,500 |
| Form 18 (Request for Examination, startup) | Rs. 4,000 |
| Patent agent fee (if engaged) | Rs. 50,000 – Rs. 1,50,000 |
| Drawings preparation (formal, by draftsperson) | Rs. 5,000 – Rs. 15,000 |
| **Total estimated, startup tier** | **Rs. 65,000 – Rs. 1,75,000** |

## After Grant

- [ ] Pay annual renewal fees under §53 and the First Schedule of the Patents Rules 2003. The startup rate begins at Rs. 800 per annum (years 3-6) and escalates thereafter.
- [ ] The patent term is 20 years from the filing date of the Complete Specification (§53(1)).
- [ ] Mark commercial embodiments with the patent number to put third parties on constructive notice.
- [ ] Monitor third-party activity and, where appropriate, send pre-litigation notices; the Apache-2.0 patent grant on the source code does not, in the applicant's submission, extend to closed-source reimplementations that do not themselves grant a reciprocal patent licence.

## Document-Package Assembly (Pre-Filing)

- [ ] Render `06-drawings-list.md` to formal A4 PDF drawings with consistent reference numerals.
- [ ] Concatenate Sections 01 through 09, the drawings, the prior-art section, and the §3(k) defence in the order required by IPO Form 2.
- [ ] Verify total page count and adjust additional-page fees accordingly.
- [ ] Have the inventor and the applicant's authorised signatory sign Form 1 and Form 5 (digital signatures permitted for e-filing).
- [ ] Submit through the IPO e-filing portal (https://ipindiaonline.gov.in/) and retain the acknowledgement receipt.
