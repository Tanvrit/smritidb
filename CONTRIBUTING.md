# Contributing to Kanerva

Thanks for being here. Kanerva is in **Phase 0** (spec & math validation), so the most useful contributions right now are conceptual rather than code-level. Once Phase 1 (TS reference implementation) starts, the contribution surface broadens.

## How to help, by phase

| Phase | What we need |
|---|---|
| **0 — Spec** (current) | Spec review, math gotchas, real-world use-case proposals via issues. |
| **1 — TS reference** | Code review, test coverage, doc improvements, demo apps. |
| **2 — Rust core + Wasm** | Performance work, profiling, SIMD optimizations. |
| **3 — Native bindings** | Bindings expertise (UniFFI, pyo3, napi-rs). |
| **4 — Persistence** | Adapter implementations, durability testing. |
| **5 — Ecosystem** | Examples, integrations, documentation, advocacy. |

The current phase is tracked in the [README](README.md) status badge.

## Ground rules

- **Be kind.** We follow the [Contributor Covenant](CODE_OF_CONDUCT.md). Disagree about ideas, not people.
- **Spec drives code, not the other way around.** Behavior changes start with a `SPEC.md` proposal (an issue tagged `spec`). The reference implementations are downstream of the spec.
- **The three load-bearing properties** (fuzzy content-addressing, holographic distribution, Hebbian consolidation) are the project's identity. Proposals that erode any of them need an unusually strong justification.
- **Tests for new behavior, regardless of language.** Untested code is unmerged code.
- **No surprise dependencies.** Adding a runtime dep needs a one-paragraph rationale in the PR description.

## Setup (when Phase 1 is live)

```bash
# Node ≥22 (see .nvmrc), pnpm ≥9
pnpm install
pnpm test
pnpm --filter core-ts dev

# Site
pnpm --filter web dev
```

For the Phase 0 notebooks:

```bash
cd notebooks
python -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
jupyter notebook
```

## Filing an issue

- **Bug**: include version, platform, repro steps, expected vs. actual.
- **Feature**: describe the *use case* first, then the proposed API. Use cases beat API designs.
- **Spec proposal**: open a `Discussions` thread before a PR. Spec changes are deliberately friction-heavy.

## Pull requests

1. Fork, branch from `main`.
2. Make the change. Keep PRs focused — one concept per PR.
3. Update docs and tests in the same PR.
4. Run `pnpm test && pnpm lint && pnpm typecheck` locally.
5. Open the PR. CI must be green before review.
6. Squash-merge on approval. The maintainer trailer is added automatically.

## Commit messages

Conventional Commits, prefix optional:

```
feat(core-ts): add bind() and unbind() for HD vectors
fix(core-ts): correct similarity threshold off-by-one
docs(spec): clarify cleanup-memory semantics under degradation
```

Keep the first line under ~70 characters. Use the body for *why*, not *what*.

## Security

Please do **not** open public issues for security vulnerabilities. See [`SECURITY.md`](SECURITY.md) for the disclosure process.

## License

By contributing, you agree your contributions are licensed under the [Apache License 2.0](LICENSE), including its explicit patent grant.
