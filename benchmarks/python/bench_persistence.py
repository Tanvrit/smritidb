#!/usr/bin/env python3
"""Persistence throughput benchmark — Python binding.

Measures SQLite snapshot write/read MB/s and (where the Python binding
exposes it) WAL append ops/s. The Python `PersistentStore` only exposes
`persist()`, which writes a full snapshot — there's no direct WAL append
API on the Python surface today. We fall back to repeated `persist()`
calls and surface that as a snapshot-rewrite-rate so the table has a
number to compare against the other bindings.
"""

from __future__ import annotations

import os
import resource
import tempfile
import time
from typing import Any

from smritidb import PersistentStore

D = 10_000
PERSIST_N = 10_000
WAL_APPENDS = 1_000
WARMUP_ITERS = 3


def _populate(store: PersistentStore, n: int) -> None:
    for i in range(n):
        store.put(f"item_{i}", f"value for item {i}".encode())


def _cleanup(path: str) -> None:
    for suffix in ("", "-journal", "-wal", "-shm"):
        p = path + suffix
        if os.path.exists(p):
            os.unlink(p)


def run() -> dict[str, Any]:
    print("[3/4] persistence (Python)")

    with tempfile.TemporaryDirectory() as tmp:
        write_path = os.path.join(tmp, "write.db")
        read_path = os.path.join(tmp, "read.db")
        wal_path = os.path.join(tmp, "wal.db")

        # -- snapshot_write --
        _cleanup(write_path)
        store = PersistentStore.open_sqlite(write_path, dimension=D)
        _populate(store, PERSIST_N)
        t0 = time.perf_counter()
        store.persist()
        write_wall = time.perf_counter() - t0
        store.close()
        write_bytes = os.path.getsize(write_path)
        write_mbs = (write_bytes / 1_000_000) / write_wall
        print(
            f"  snapshot_write N={PERSIST_N} bytes={write_bytes} "
            f"wall={write_wall:.3f}s = {write_mbs:.1f} MB/s"
        )

        # -- snapshot_read --
        _cleanup(read_path)
        store_r = PersistentStore.open_sqlite(read_path, dimension=D)
        _populate(store_r, PERSIST_N)
        store_r.persist()
        store_r.close()
        read_bytes = os.path.getsize(read_path)

        for _ in range(WARMUP_ITERS):
            s = PersistentStore.open_sqlite(read_path)
            s.close()

        t0 = time.perf_counter()
        s = PersistentStore.open_sqlite(read_path)
        read_wall = time.perf_counter() - t0
        assert s.size() == PERSIST_N
        s.close()
        read_mbs = (read_bytes / 1_000_000) / read_wall
        print(
            f"  snapshot_read  N={PERSIST_N} bytes={read_bytes} "
            f"wall={read_wall:.3f}s = {read_mbs:.1f} MB/s"
        )

        # -- wal_append (degraded — Python binding does not expose append_wal
        # directly; surrogate uses repeated persist() at small N for
        # rate-of-snapshots-per-sec). Document this in RESULTS.md footnote.
        _cleanup(wal_path)
        wal_store = PersistentStore.open_sqlite(wal_path, dimension=D)
        wal_store.put("seed", b"x")  # small steady-state store
        for _ in range(WARMUP_ITERS):
            wal_store.persist()
        t0 = time.perf_counter()
        for _ in range(WAL_APPENDS):
            wal_store.persist()
        wal_wall = time.perf_counter() - t0
        wal_ops = WAL_APPENDS / wal_wall
        wal_store.close()
        print(
            f"  wal_append*   N={WAL_APPENDS} wall={wal_wall:.3f}s = {wal_ops:.0f} ops/s "
            "(surrogate: repeated persist())"
        )

    return {
        "snapshot_write": {
            "n": PERSIST_N,
            "bytes": write_bytes,
            "wall_seconds": write_wall,
            "mb_per_sec": write_mbs,
        },
        "snapshot_read": {
            "n": PERSIST_N,
            "bytes": read_bytes,
            "wall_seconds": read_wall,
            "mb_per_sec": read_mbs,
        },
        "wal_append": {
            "n": WAL_APPENDS,
            "wall_seconds": wal_wall,
            "ops_per_sec": wal_ops,
            "note": "Python binding has no direct append_wal; this measures repeated full persist() instead.",
        },
    }


def _rss_bytes() -> int:
    """Best-effort current process RSS in bytes.

    `ru_maxrss` from resource is a *high-water mark* (never shrinks), which
    makes delta measurements unreliable. We shell out to `ps` for current
    RSS on macOS, parse /proc/self/status on Linux, and fall back to
    `ru_maxrss` elsewhere.
    """
    import os
    import subprocess
    import sys

    if sys.platform == "darwin":
        try:
            out = subprocess.check_output(
                ["ps", "-o", "rss=", "-p", str(os.getpid())], text=True
            ).strip()
            return int(out) * 1024
        except Exception:
            pass
    elif sys.platform.startswith("linux"):
        try:
            with open("/proc/self/status") as f:
                for line in f:
                    if line.startswith("VmRSS:"):
                        return int(line.split()[1]) * 1024
        except Exception:
            pass
    # Fallback: ru_maxrss. Scale: bytes on macOS, KiB on Linux.
    scale = 1 if sys.platform == "darwin" else 1024
    return resource.getrusage(resource.RUSAGE_SELF).ru_maxrss * scale


def run_memory() -> dict[str, Any]:
    """Insert PERSIST_N items into an in-memory Store, measure RSS delta.

    Caveat on macOS: the system allocator (malloc + memory compressor) tends
    to keep RSS flat across small allocations — the 10000×1250 B ≈ 12 MB
    store often fits inside the interpreter's pre-grown arena. We still
    report the delta but a 0 reading on macOS doesn't mean "zero overhead";
    it means "below RSS measurement resolution". The Rust runner has the
    same limitation. The honest per-item floor is calculated by counting
    the dominant data structure: a `bytes` of D bytes per item.
    """
    print("[4/4] memory (Python)")
    import gc
    gc.collect()
    before = _rss_bytes()
    from smritidb import Store
    store = Store(dimension=D)
    for i in range(PERSIST_N):
        store.put(f"item_{i}", f"value for item {i}")
    gc.collect()
    after = _rss_bytes()
    delta = max(0, after - before)
    per_item = delta / PERSIST_N
    # Synthetic floor: every item carries a D-byte key bytes object,
    # a value, a UUID, plus dict / HashMap overhead. We surface this
    # alongside the noisy RSS measurement so the table has *something*
    # meaningful when the OS allocator is compressing under us.
    synthetic = float(D)  # at minimum, the key
    print(
        f"  RSS before={before}B after={after}B delta={delta}B = {per_item:.1f} bytes/item "
        f"(synthetic floor: {synthetic:.0f} B/item)"
    )
    # Keep store alive past the sample.
    _ = store.size()
    return {
        "n_items": PERSIST_N,
        "rss_before_bytes": before,
        "rss_after_bytes": after,
        "bytes_per_item": per_item,
        "synthetic_floor_bytes_per_item": synthetic,
        "theoretical_floor_bytes_per_item": D // 8,
    }


if __name__ == "__main__":
    import json, sys
    out = {"persistence": run(), "memory": run_memory()}
    json.dump(out, sys.stdout, indent=2)
    print()
