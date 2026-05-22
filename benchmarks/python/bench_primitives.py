#!/usr/bin/env python3
"""Primitive throughput benchmark — Python binding.

Times every primitive in the canonical workload (see ../WORKLOAD.md) and
returns a dict matching the cross-binding JSON schema. Importable so the
combined `run_all.py` can call it without re-importing the native module.
"""

from __future__ import annotations

import time
from typing import Any

from smritidb import (
    SPEC_VERSION,
    bind,
    bundle,
    encode_string,
    random_hv,
    similarity,
)

D = 10_000
RANDOM_HV_BATCH = 1_000
BUNDLE_BATCH = 100
BIND_BATCH = 100
SIMILARITY_BATCH = 10_000
ENCODE_STRING_BATCH = 10_000
WARMUP_ITERS = 3


def _measure(name: str, iters: int, body) -> dict[str, Any]:
    # Warm-up rounds — defeat any first-call import / lazy-init cost.
    for _ in range(WARMUP_ITERS):
        body(0)
    t0 = time.perf_counter()
    for i in range(iters):
        body(i)
    wall = time.perf_counter() - t0
    ops = iters / wall
    print(f"  {name}: {ops:>12,.0f} ops/s ({wall:.3f}s for {iters} iters)")
    return {
        "ops_per_sec": ops,
        "iters": iters,
        "wall_seconds": wall,
    }


def run() -> dict[str, Any]:
    print("[1/4] primitives (Python)")

    # random_hv with varied seed.
    seeds = [f"hv_{i}".encode() for i in range(RANDOM_HV_BATCH)]
    def _hv(i: int) -> None:
        random_hv(seeds[i % len(seeds)], D)
    r_hv = _measure("random_hv", RANDOM_HV_BATCH, _hv)

    # Pre-generate pairs (outside the timed region) so the timed loop measures
    # the operation alone, not the random_hv setup.
    a_pool = [random_hv(f"a{i}".encode(), D) for i in range(SIMILARITY_BATCH)]
    b_pool = [random_hv(f"b{i}".encode(), D) for i in range(SIMILARITY_BATCH)]

    def _bundle(i: int) -> None:
        bundle([a_pool[i % len(a_pool)], b_pool[i % len(b_pool)]])
    r_bundle = _measure("bundle", BUNDLE_BATCH, _bundle)

    def _bind(i: int) -> None:
        bind(a_pool[i % len(a_pool)], b_pool[i % len(b_pool)])
    r_bind = _measure("bind", BIND_BATCH, _bind)

    def _sim(i: int) -> None:
        similarity(a_pool[i % len(a_pool)], b_pool[i % len(b_pool)])
    r_sim = _measure("similarity", SIMILARITY_BATCH, _sim)

    inputs = [f"item_{i}" for i in range(ENCODE_STRING_BATCH)]
    def _enc(i: int) -> None:
        encode_string(inputs[i], D)
    r_enc = _measure("encode_string", ENCODE_STRING_BATCH, _enc)

    return {
        "spec_version": SPEC_VERSION,
        "results": {
            "random_hv": r_hv,
            "bundle": r_bundle,
            "bind": r_bind,
            "similarity": r_sim,
            "encode_string": r_enc,
        },
    }


if __name__ == "__main__":
    out = run()
    import json, sys
    json.dump(out, sys.stdout, indent=2)
    print()
