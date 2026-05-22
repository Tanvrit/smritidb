#!/usr/bin/env bash
# End-to-end polyglot interop driver for Smritidb.
#
# Runs four readers in four different languages against a single SQLite
# database written in language number one. Prints a PASS / FAIL banner
# at each step and a summary at the end. Exits non-zero on any failure.

set -euo pipefail

# Always operate from the directory holding this script — keeps relative
# paths stable no matter where the caller cd'd to first.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$HERE"

# Allow running from a clean repo without installing the python wheel:
# `step1_python_write.py` already adds `packages/smritidb-py/python` to
# `sys.path`, but we mirror it here so any out-of-band tools (e.g. a
# debugger) get the same import surface.
export PYTHONPATH="${PYTHONPATH:-}:$(cd ../../packages/smritidb-py/python && pwd)"

banner() { printf '\n=== %s ===\n' "$1"; }

banner "Polyglot Smritidb interop demo"
echo "fixture path: /tmp/smritidb-polyglot.db"
echo "languages: Python (write) -> Rust -> Kotlin/JVM -> TypeScript (read)"

banner "Step 1: Python writes 100 items via PersistentStore.open_sqlite"
python3 step1_python_write.py

banner "Step 2: Rust reads via smritidb_core::open_persistent_store + SqliteAdapter"
( cd step2_rust_read && cargo run --release --quiet )

banner "Step 3: Kotlin/JVM reads via PersistentStore.openSqlite (UniFFI -> Rust)"
( cd step3_kotlin_read && gradle run -q --console=plain )

banner "Step 4: TypeScript reads the Rust schema directly + parses with readKmf"
node --import tsx step4_ts_read.ts

banner "ALL STEPS PASSED"
echo "Same SQLite file was opened bit-exactly across 4 language stacks."
