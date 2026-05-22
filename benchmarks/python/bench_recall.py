#!/usr/bin/env python3
"""Recall latency benchmark — Python binding.

Builds a Store with N items, runs 1000 recall queries, reports p50/p99/p999.
"""

from __future__ import annotations

import statistics
import time
from typing import Any

from smritidb import Store, encode_string

D = 10_000
RECALL_NS = [100, 1_000, 10_000]
RECALL_QUERIES = 1_000
WARMUP_ITERS = 3


def _percentile(samples: list[float], p: float) -> float:
    if not samples:
        return 0.0
    s = sorted(samples)
    idx = round((len(s) - 1) * p)
    return s[min(idx, len(s) - 1)]


def run() -> list[dict[str, Any]]:
    print("[2/4] recall (Python)")
    out: list[dict[str, Any]] = []
    for n in RECALL_NS:
        store = Store(dimension=D)
        t0 = time.perf_counter()
        for i in range(n):
            store.put(f"item_{i}", f"value for item {i}")
        insert_seconds = time.perf_counter() - t0

        # Pre-encode the cues so the timed region measures recall, not encode.
        cues = [encode_string(f"item_{i % n}", D) for i in range(RECALL_QUERIES)]

        # Warm-up.
        for _ in range(WARMUP_ITERS):
            store.recall(cues[0], top_k=10, min_similarity=0.5)

        samples_ms = []
        for c in cues:
            t = time.perf_counter()
            store.recall(c, top_k=10, min_similarity=0.5)
            samples_ms.append((time.perf_counter() - t) * 1000.0)

        p50 = _percentile(samples_ms, 0.50)
        p99 = _percentile(samples_ms, 0.99)
        p999 = _percentile(samples_ms, 0.999)
        print(
            f"  N={n:>5} insert={insert_seconds:.3f}s "
            f"p50={p50:.3f}ms p99={p99:.3f}ms p999={p999:.3f}ms"
        )
        out.append({
            "n": n,
            "queries": len(cues),
            "p50_ms": p50,
            "p99_ms": p99,
            "p999_ms": p999,
            "insert_seconds": insert_seconds,
            "mean_ms": statistics.fmean(samples_ms),
        })
    return out


if __name__ == "__main__":
    import json, sys
    json.dump({"recall": run()}, sys.stdout, indent=2)
    print()
