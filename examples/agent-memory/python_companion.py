#!/usr/bin/env python3
"""Python companion to the TS agent-memory demo.

After running `npm start` (or `pnpm demo`) in this directory, a SQLite file
lives at /tmp/agent-memory.db containing the agent's long-term memories.
That file uses the canonical `smritidb_snapshot` schema — the same schema
the Rust core and every language binding writes — so the bytes the TS demo
just wrote are directly readable from Python with no conversion.

This script opens that file via `smritidb.PersistentStore.open_sqlite()`
(PyO3 -> Rust core -> SqliteAdapter), reports how many memories it sees,
and runs a couple of recall queries. The two language stacks meet in the
middle at the KMF snapshot byte string stored at id=1.

Usage:

    # First write the file with the TS demo:
    cd examples/agent-memory && npm start

    # Then read it from Python:
    python3 examples/agent-memory/python_companion.py
"""
from __future__ import annotations

import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO_ROOT = os.path.abspath(os.path.join(HERE, "..", ".."))
PY_PKG = os.path.join(REPO_ROOT, "packages", "smritidb-py", "python")
if os.path.isdir(PY_PKG):
    sys.path.insert(0, PY_PKG)

try:
    from smritidb import PersistentStore  # noqa: E402
except ImportError as e:
    print(
        "FAIL: could not import smritidb. Build the Python extension first:\n"
        "  cd packages/smritidb-py && maturin develop --release\n"
        f"(import error: {e})",
        file=sys.stderr,
    )
    sys.exit(1)

PATH = os.environ.get("SMRITIDB_AGENT_MEMORY_PATH", "/tmp/agent-memory.db")
DIMENSION = 8192


def main() -> int:
    if not os.path.exists(PATH):
        print(
            f"FAIL: {PATH} not found. Run `npm start` in examples/agent-memory first.",
            file=sys.stderr,
        )
        return 1

    print(f"opening {PATH} via PersistentStore.open_sqlite(...)")
    with PersistentStore.open_sqlite(PATH, dimension=DIMENSION) as store:
        size = store.size()
        print(f"  loaded {size} memories written by the TypeScript demo.\n")

        queries = [
            "what does the user like in writing style?",
            "what is Smritidb?",
            "what does 'ship it' mean to the user?",
        ]
        for q in queries:
            print(f"query: {q}")
            hits = store.recall(q, top_k=3, min_similarity=0.0)
            if not hits:
                print("  (no matches above floor)")
                continue
            for h in hits:
                text = h["value"].decode("utf-8", errors="replace")
                tags = ", ".join(h["tags"]) if h["tags"] else "—"
                print(f"  {h['similarity']:.2f}  [{tags}]  {text}")
            print()

    print("done. Same SQLite file, two languages, one schema.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
