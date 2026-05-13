# Security Policy

## Supported Versions

Kanerva is pre-release (Phase 0). No version has reached general availability. Security fixes will land on `main` and be tagged once the 0.1.0 release ships.

| Version | Supported |
|---------|-----------|
| `main` (pre-release) | ✅ |
| Released versions | Latest minor only, until 1.0.0 |

## Reporting a Vulnerability

**Do not open public issues for security vulnerabilities.**

Please email **security@kanervalabs.com** (or, until that mailbox exists, the maintainer listed in `MAINTAINERS.md`) with:

- Affected component(s) and version(s)
- A description of the vulnerability and its impact
- Reproduction steps or proof-of-concept
- Any suggested mitigation

You will receive an acknowledgement within **72 hours**. We will work with you on a fix and a coordinated disclosure timeline — typical window is **90 days** from acknowledgement, shorter for actively exploited issues.

We do not currently offer a bug bounty. We do credit reporters in release notes unless anonymity is requested.

## Threat model (early sketch)

Because Kanerva stores data, the relevant threats are:

- **Recall poisoning** — an attacker with write access introduces vectors that pull legitimate recalls toward malicious values. Mitigated by capability-scoped writes and audit logs.
- **Cleanup memory inversion** — an attacker with read access uses repeated probes to reconstruct stored items. Mitigated by query rate limits and differential-privacy-aware recall (research).
- **Persistence-layer compromise** — standard backend security applies to the chosen adapter (IndexedDB, SQLite, S3, etc.).
- **Wire-format spoofing** — once the open wire format ships, signed snapshots will be the recommended path for production deployments.

A full threat model lives in `docs/SECURITY.md` and tracks changes via PR review.
