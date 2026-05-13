# kanerva (core-ts)

TypeScript reference implementation of [Kanerva](https://github.com/REPLACE_WITH_OWNER/kanerva) — the open biology-inspired associative memory layer.

**Status:** Phase 0 — spec only. The real code lands in Phase 1, against [`SPEC.md`](../../SPEC.md).

## Install (when published)

```bash
npm install kanerva
```

## Quick example (target API)

```ts
import { Kanerva } from "kanerva";

const store = new Kanerva({ dimension: 10000, backend: "memory" });

await store.put("the cat sat on the mat", { tags: ["sentence"] });

const hits = await store.recall("cat on mat", { topK: 5 });
// → [{ item: { value: "the cat sat on the mat", ... }, similarity: 0.91 }, ...]
```

See [`docs/MANIFESTO.md`](../../docs/MANIFESTO.md) for the project's thesis and [`SPEC.md`](../../SPEC.md) for the wire-level contract every binding implements.

## License

MIT.
