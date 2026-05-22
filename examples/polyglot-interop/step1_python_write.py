#!/usr/bin/env python3
"""Step 1 — Python writer for the polyglot interop demo.

Creates a SQLite-backed PersistentStore at `/tmp/smritidb-polyglot.db`,
inserts 100 ASCII items keyed `concept_N`, persists, and exits.

This script is the *producer* in the polyglot pipeline; Steps 2–4 each
open the same file in a different language and verify byte-identity.
"""

from __future__ import annotations

import os
import sys

# Allow running from the repo without installing the wheel: the freshly-built
# native module sits under `packages/smritidb-py/python/`.
HERE = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
PY_PKG = os.path.join(REPO_ROOT, "packages", "smritidb-py", "python")
if os.path.isdir(PY_PKG):
    sys.path.insert(0, PY_PKG)

from smritidb import PersistentStore  # noqa: E402

PATH = "/tmp/smritidb-polyglot.db"
N_ITEMS = 100
# Use a small dimension (matching the Python persistence test) so the snapshot
# stays bounded across the four readers. The dimension is embedded in the KMF
# header so every reader picks it up automatically.
DIMENSION = 8192


def main() -> int:
    # Clean any stale file (and SQLite side-cars) from a previous run.
    for suffix in ("", "-journal", "-wal", "-shm"):
        p = PATH + suffix
        if os.path.exists(p):
            os.unlink(p)

    store = PersistentStore.open_sqlite(PATH, dimension=DIMENSION)
    try:
        for i in range(N_ITEMS):
            store.put(f"concept_{i}", f"item value {i}".encode("ascii"))
        store.persist()
        size = store.size()
        if size != N_ITEMS:
            print(f"FAIL: expected {N_ITEMS} items, got {size}", file=sys.stderr)
            return 1
        # Sanity-check a recall round-trip *inside* Python so that any later
        # cross-language divergence is provably a language-boundary issue.
        hits = store.recall("concept_50", top_k=1, min_similarity=0.9)
        if not hits or hits[0]["value"] != b"item value 50":
            print(f"FAIL: in-process recall mismatch: {hits!r}", file=sys.stderr)
            return 1
    finally:
        store.close()

    print(f"PASS: Python wrote {N_ITEMS} items to {PATH}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
