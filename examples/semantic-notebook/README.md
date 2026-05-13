# semantic-notebook

A one-screen Smritidb walkthrough. Demonstrates three things:

1. **Fuzzy semantic recall** — store a handful of sentences, query by partial phrases, get back the nearest stored sentence ranked by similarity.
2. **Compositional bind / unbind** — bundle three role-filler pairs into one hypervector, then unbind any role to recover its filler against a candidate set.
3. **Holographic degradation** — recall@1 over a 200-item corpus with cue corruption from 0% to 50%.

```bash
cd examples/semantic-notebook
pnpm install
pnpm demo            # in-memory walkthrough
pnpm demo:persistent # FS adapter — open, populate, persist, cold-restore
```

The example imports directly from `@tanvrit/smritidb` (workspace-linked here; the same code works from `npm install @tanvrit/smritidb`).
