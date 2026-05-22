# Form-2 Filing Procedure — Indian Patent Office

**Subject:** Complete Specification (Form 2) for "A System and Method for Bit-Exact Cross-Implementation Persistent Associative Memory Using Binary Hyperdimensional Vectors"
**Applicant:** Tanvrit Private Limited
**Inventor:** Vivek Singh
**Pinned to SHA:** `17334f8`
**Procedure date:** 2026-05-21

This document is a step-by-step procedural guide for converting the assembled Markdown specification (`patent/FORM-2-COMPLETE-SPECIFICATION.md`) into a filable PDF and lodging the Complete Specification with the Indian Patent Office ("IPO") via the IP India e-filing portal. The procedure assumes the applicant is a recognised startup under the Startup India scheme (per the Patents Rules 2003 First Schedule, startup fee tier); large-entity applicants should adjust the fee figures by approximately ten-fold.

---

## Step 1 — Generate the Form-2 PDF from Markdown

The recommended toolchain is **pandoc** (universal document converter) coupled with **xelatex** (the PDF engine that supports Unicode, OpenType fonts, and the customary IPO typesetting conventions).

### 1.1 Verify toolchain availability

On the filing workstation, run:

```bash
which pandoc && which xelatex
```

If both binaries resolve, proceed to §1.2. If either is missing, install via your platform package manager:

- **macOS (Homebrew):** `brew install pandoc && brew install --cask mactex` (the MacTeX install ships xelatex; allow ~5 GB download).
- **Debian / Ubuntu:** `sudo apt install pandoc texlive-xetex texlive-fonts-recommended texlive-fonts-extra`.
- **Windows:** install pandoc from the official MSI at https://pandoc.org and MiKTeX or TeX Live from https://miktex.org.

**Toolchain status at SHA `17334f8` on the development workstation (2026-05-21):** `pandoc` is NOT installed, `xelatex` is NOT installed. The PDF must be generated on a workstation where the toolchain is provisioned, or installed before the filing run. See §1.4 for an alternative LibreOffice-based path.

### 1.2 Run the pandoc conversion

From the repository root, execute:

```bash
pandoc patent/FORM-2-COMPLETE-SPECIFICATION.md \
  --pdf-engine=xelatex \
  -V geometry:a4paper \
  -V geometry:left=4cm,right=2cm,top=2cm,bottom=2cm \
  -V fontsize=12pt \
  -V mainfont="Times New Roman" \
  -V linestretch=1.5 \
  -V pagestyle=plain \
  -V documentclass=article \
  -V colorlinks=true \
  --toc \
  --toc-depth=3 \
  -o patent/FORM-2-COMPLETE-SPECIFICATION.pdf
```

Verify the output:

```bash
ls -lh patent/FORM-2-COMPLETE-SPECIFICATION.pdf
# Expected: 1–3 MB depending on whether figures are rasterised
```

If "Times New Roman" is unavailable on the host (common on Linux without `ttf-mscorefonts-installer`), substitute `--mainfont="EB Garamond"` (a free OpenType serif font shipped with TeX Live) or `--mainfont="Liberation Serif"` (a metrically-compatible drop-in for Times New Roman).

### 1.3 Verify PDF rendering

Open the PDF and check:

- Page numbers appear in the footer (centre or right).
- Margins match the specification (4 cm left, 2 cm elsewhere).
- The Table of Contents was generated and lists Sections 1–8 with correct page numbers.
- ASCII-art figures in §5 are not clipped (some ASCII figures are wide; if any figure is clipped at the right margin, add `\\fontsize{10pt}{12pt}\\selectfont` to a `\\begin{verbatim}` block, or use `--variable fontsize=11pt` to give more room).
- All seven figures (Fig. 1 through Fig. 7) appear intact.
- Page count is approximately 100–115 (per the page audit in `patent/FORM-2-PAGE-AUDIT.md`).

### 1.4 Alternative: LibreOffice export

If the pandoc + xelatex toolchain is not available, the Markdown may be converted to OpenDocument first (`pandoc patent/FORM-2-COMPLETE-SPECIFICATION.md -o patent/FORM-2.odt`), opened in LibreOffice Writer, manually adjusted for margins, line spacing, and font, and exported to PDF via File → Export As PDF. The xelatex path is preferred because it produces consistent, repeatable typesetting from a script; the LibreOffice path is acceptable for one-off filing but introduces a manual review step that may admit typographical drift.

---

## Step 2 — Print, sign, and paginate

The IPO requires the Complete Specification to be filed as a PDF, but customary practice is to also retain a physically signed paper copy for the applicant's records.

### 2.1 Print on A4 paper

Print the PDF single-sided on plain white A4 paper (80 gsm or heavier). Each page should bear a page number in the footer (already inserted in Step 1).

### 2.2 Inventor signature

The inventor (Vivek Singh) signs at the foot of each page in the bottom-right margin, and signs the Inventor signature block on the final page of the specification immediately following §8 Abstract. Practice varies — some IPO branches accept a single signed cover page and the signed claims; others require a signed footer on every page. The conservative practice is to sign every page.

### 2.3 Applicant signature

The duly authorised signatory of Tanvrit Private Limited (typically the director or company secretary) signs the Applicant signature block on the final page. The applicant signature is countersigned by the company seal if available.

### 2.4 Scan back to PDF (if signing paper)

Scan the signed paper specification at 300 dpi as a single PDF document. Verify the scan is legible (use OCR if available) and the file is under 50 MB (the IPO e-filing portal's per-attachment ceiling).

Alternatively, sign electronically using a Class III Digital Signature Certificate ("DSC") issued by a Controller-of-Certifying-Authorities-licensed agency (e.g., eMudhra, Sify, Capricorn); this is the path required for e-filing in any event (see §4 below).

---

## Step 3 — Prepare the accompanying forms

The Complete Specification (Form 2) is filed together with several other forms. Each is a separate PDF or e-form within the IP India e-filing portal.

### 3.1 Form 1 — Application for Grant of Patent

- Title of the invention (copy verbatim from §0 / §1 of the assembled Form 2).
- Applicant: Tanvrit Private Limited, [registered office TBD].
- Inventor: Vivek Singh, [address TBD].
- Declaration of inventorship: tick the relevant box (sole inventor; no co-inventors).
- Choose the applicable filing track: "Ordinary application" (not a PCT national-phase entry, not a divisional).
- Pay the prescribed application fee. For startup tier, this is Rs. 1,600 (Form 1 fee — see First Schedule).

### 3.2 Form 3 — Statement and Undertaking under §8

- §8 of the Patents Act 1970 requires the applicant to disclose all corresponding foreign applications and to undertake to update the IPO as new foreign applications are filed.
- For the present application (no prior foreign filing as of 2026-05-21): Form 3 is filed with **NIL** corresponding-application table, plus the undertaking to update within six months of any future foreign filing.

### 3.3 Form 5 — Declaration as to Inventorship

- A declaration by the applicant (Tanvrit Private Limited) that the named inventor (Vivek Singh) is the true and first inventor of the invention claimed in the specification, and that the applicant has acquired rights from the inventor by [delete as applicable: assignment / employment / agreement] dated [TBD].
- The assignment agreement between Vivek Singh and Tanvrit Private Limited should be executed and dated **before** the Form 5 filing date. Counsel should review this before filing.

### 3.4 Form 9 (optional) — Request for Early Publication

- §11A of the Patents Act 1970 provides that an application is published 18 months after the priority date by default. Form 9 requests early publication, which can occur within approximately one month of filing.
- Fee (startup tier): Rs. 2,500.
- Recommendation: **file Form 9** so that the specification enters the public domain (and the §3(k) clock starts) immediately, giving infringement-watch standing without waiting 18 months.

### 3.5 Form 18 — Request for Examination

- Must be filed within 48 months of the priority date (filing date for ordinary applications).
- Fee (startup tier): Rs. 4,000 for a Standard Request for Examination, or Rs. 8,000 for an Expedited Request (which requires the applicant to be a startup, female, small entity, or eligible-country applicant; Tanvrit qualifies as a startup).
- Recommendation: **file Form 18 expedited** at filing time to compress the examination timeline (target First Examination Report within 6 months rather than the standard 31 months).

### 3.6 Form 28 (if claiming startup status)

- Form 28 is the certification that the applicant is a recognised startup under the DPIIT Startup India scheme.
- Required as a precondition for all startup-tier fee reductions throughout the application's life-cycle. Attach the DPIIT-issued startup-recognition certificate.

---

## Step 4 — File via the IP India e-filing portal

### 4.1 Portal access

URL: https://ipindiaonline.gov.in/epatentfiling/

Login requires a registered patent-agent account or a registered applicant account. The applicant may file in person (Tanvrit Private Limited's authorised signatory) or via a registered patent agent (recommended for the first filing; engage a Senior Patent Agent with HDC/software-patent experience).

### 4.2 Begin a new application

In the portal:

1. Select **"Patent Application Form"** → **"Ordinary Application"**.
2. Fill in the Form 1 fields (mirroring §3.1 above).
3. Upload Form 2 (the signed PDF from §2 above).
4. Upload Form 3, Form 5, optional Form 9, Form 18, and Form 28 in the corresponding attachment slots.
5. Upload the DPIIT startup-recognition certificate as an annexure.
6. Optionally upload the Apache-2.0 source-code archive (`git archive 17334f8 -o smritidb-17334f8.tar.gz`) as an "Annexure: Reference Implementation"; this is **not required** by the IPO but is useful evidence of constructive reduction-to-practice on the priority date.

### 4.3 Apply the digital signature

Each uploaded PDF is signed using the applicant's (or patent agent's) Class III DSC token (USB hardware key). The portal will prompt for the DSC PIN once per filing session.

### 4.4 Pay fees

Fees are paid online via NEFT, RTGS, or internet banking through the portal's payment gateway. Total expected fee (startup tier, summed):

| Form / item                                          | Startup fee (Rs.) |
|------------------------------------------------------|-------------------|
| Form 1 (application)                                 | 1,600             |
| Form 2 excess pages (~80 × Rs. 160)                   | ~12,800           |
| Form 2 excess claims (26 × Rs. 320)                   | 8,320             |
| Form 9 (early publication, optional)                  | 2,500             |
| Form 18 (expedited examination, optional)             | 8,000             |
| **Approximate total (with Form 9 + Form 18 expedited)** | **~33,220**     |
| **Approximate total (without Form 9 + Form 18)**       | **~22,720**     |

Add ~Rs. 500–2,000 for the digital signature, e-stamp duty, and any patent-agent professional fee (which is separately negotiated and typically Rs. 30,000–80,000 for a first ordinary filing).

### 4.5 Submit and capture the acknowledgement

Upon successful payment, the portal generates an acknowledgement receipt with:

- Application number (format: `NNNNNN/2026/XXX` where `XXX` is the branch suffix).
- Filing date and time (this is the priority date for §13 prior-art purposes).
- Digitally signed acknowledgement PDF.

Save the acknowledgement PDF in `patent/00-filing-acknowledgement.pdf`. The application number must be quoted in all subsequent correspondence with the IPO.

---

## Step 5 — Post-filing housekeeping

### 5.1 Record the priority date

Record the priority date in `patent/00-cover.md` (replacing the `[TBD]` placeholder).

### 5.2 Calendar critical dates

| Event                                              | Date relative to priority date |
|----------------------------------------------------|-------------------------------|
| Publication under §11A (default)                   | priority + 18 months          |
| Publication under Form 9 (early publication)       | ~1 month after filing         |
| Last date to file Form 18 (Request for Examination)| priority + 48 months          |
| Last date to file a divisional application         | grant or 6 months after FER, whichever is later |
| Annuity payable                                    | year 3, 4, 5, ... (after priority date) |

### 5.3 Update the source repository

Add a `PATENTS.md` file at the repository root recording:

- The IPO application number and filing date.
- The pinned source SHA (`17334f8`) used in the specification.
- A statement that the Apache-2.0 patent grant of the project's `LICENSE` co-exists with the IPO application; the patent grant is exercised against downstream users to the maximum extent compatible with the Apache-2.0 §3 grant.

### 5.4 Foreign filing strategy

Within 12 months of the Indian priority date, the applicant should decide whether to:

- File a PCT international application (preserves Paris-Convention priority across 153 contracting states) — recommended if international markets are a near-term consideration.
- File direct national applications in priority jurisdictions (US via the USPTO, EU via the EPO, JP via the JPO, etc.).
- Allow the priority window to lapse (Indian-only protection).

The PCT route is the most cost-efficient for early-stage applicants; engage a foreign-filing patent agent at month 9 to allow sufficient preparation time.

---

## Step 6 — Examination response (anticipated 6–18 months post-filing)

When the IPO issues its First Examination Report ("FER"), the applicant has **6 months** (extendable by 3 months on payment of the extension fee) to file a response. The response should:

- Address every §3(k) objection (computer-programme-per-se) by pointing to the technical-effect framing throughout §6 Detailed Description, particularly the *Ferid Allani* and *Microsoft Technology Licensing* anchoring at the head of §6.
- Address every novelty / inventive-step objection by reference to the prior-art distinguishing paragraphs already drafted in `patent/10-prior-art.md` (which was deliberately NOT included in the Form 2 filing for §10(4) reasons but is available verbatim for the examination response).
- Address every unity-of-invention (§16) objection by reference to the Explanatory Note at the head of §7 Claims.

Recommended counsel: a Senior Patent Agent with experience in software / cryptography / hyperdimensional-computing matters before the IPO. The internal working documents in `patent/` (not filed) — particularly `10-prior-art.md`, `11-3k-defense.md`, `12-examples.md`, and `REFINEMENT-LOG.md` — provide the full audit trail of the drafting process and should be shared with counsel as briefing material.

---

*End of Filing Procedure. Pinned to SHA `17334f8`.*
