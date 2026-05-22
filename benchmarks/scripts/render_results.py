#!/usr/bin/env python3
"""Render benchmarks/RESULTS.md from results/<binding>.json files.

Tolerates missing bindings (writes "N/A" with a footnote). Reads from
`benchmarks/results/` and writes `benchmarks/RESULTS.md`.

Usage:
    python3 scripts/render_results.py
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
from datetime import datetime, timezone

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, ".."))
RESULTS_DIR = os.path.join(ROOT, "results")
OUT_PATH = os.path.join(ROOT, "RESULTS.md")

BINDINGS = ["rust", "python", "kotlin", "typescript"]
PRIMITIVES = ["random_hv", "bundle", "bind", "similarity", "encode_string"]


def load(name: str) -> dict | None:
    p = os.path.join(RESULTS_DIR, f"{name}.json")
    if not os.path.exists(p):
        return None
    with open(p) as f:
        return json.load(f)


def fmt_ops(v: float | None) -> str:
    if v is None:
        return "N/A"
    return f"{v:,.0f}"


def fmt_ms(v: float | None) -> str:
    if v is None:
        return "N/A"
    if v < 0.01:
        return f"{v:.4f}"
    if v < 1:
        return f"{v:.3f}"
    return f"{v:.2f}"


def fmt_mbs(v: float | None) -> str:
    if v is None:
        return "N/A"
    return f"{v:,.1f}"


def get_commit() -> str:
    try:
        r = subprocess.run(
            ["git", "rev-parse", "HEAD"],
            cwd=ROOT,
            capture_output=True,
            text=True,
            check=True,
        )
        return r.stdout.strip()[:12]
    except Exception:
        return "(unknown)"


def get_host() -> str:
    try:
        import platform
        if sys.platform == "darwin":
            model = subprocess.run(
                ["sysctl", "-n", "machdep.cpu.brand_string"],
                capture_output=True, text=True, check=False,
            ).stdout.strip() or platform.machine()
        else:
            model = platform.processor() or platform.machine()
        return f"{platform.system()} {platform.release()}, {model}"
    except Exception:
        return "(unknown)"


def main() -> int:
    data = {b: load(b) for b in BINDINGS}
    missing = [b for b, d in data.items() if d is None]
    available = [b for b, d in data.items() if d is not None]

    if not available:
        print("ERROR: no benchmark results found in", RESULTS_DIR, file=sys.stderr)
        return 1

    lines: list[str] = []
    lines.append("# Smritidb Benchmark Results")
    lines.append("")
    now = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")
    lines.append(f"**Run on:** {now}")
    lines.append(f"**Host:** {get_host()}")
    lines.append(f"**Commit:** `{get_commit()}`")
    if missing:
        lines.append(f"**Missing bindings:** {', '.join(missing)} (see footnotes).")
    lines.append("")
    lines.append("Workload definition: see [WORKLOAD.md](./WORKLOAD.md).")
    lines.append("")
    lines.append("---")
    lines.append("")

    # -- primitives --
    lines.append("## Primitives (operations per second, higher is better)")
    lines.append("")
    lines.append("| Operation | Rust | Python | Kotlin/JVM | TypeScript |")
    lines.append("|---|---:|---:|---:|---:|")
    for op in PRIMITIVES:
        row = [op]
        for b in BINDINGS:
            d = data.get(b)
            if d is None:
                row.append("N/A")
                continue
            try:
                row.append(fmt_ops(d["primitives"][op]["ops_per_sec"]))
            except KeyError:
                row.append("N/A")
        lines.append("| " + " | ".join(row) + " |")
    lines.append("")

    # -- Rust-vs-other ratio --
    lines.append("### Rust vs other bindings ratio")
    lines.append("")
    if data.get("rust"):
        lines.append("| Operation | Python | Kotlin/JVM | TypeScript |")
        lines.append("|---|---:|---:|---:|")
        for op in PRIMITIVES:
            row = [op]
            rust_ops = data["rust"]["primitives"][op]["ops_per_sec"]
            for b in ["python", "kotlin", "typescript"]:
                d = data.get(b)
                if d is None:
                    row.append("N/A")
                    continue
                try:
                    other_ops = d["primitives"][op]["ops_per_sec"]
                    ratio = rust_ops / other_ops if other_ops > 0 else float("inf")
                    row.append(f"{ratio:.2f}x")
                except KeyError:
                    row.append("N/A")
            lines.append("| " + " | ".join(row) + " |")
        lines.append("")
        lines.append("Read as: Rust is N× faster than the named binding at this op.")
        lines.append("")

    # -- recall --
    lines.append("## Recall latency (milliseconds, lower is better)")
    lines.append("")
    lines.append("| N items | Rust p50 | Rust p99 | Python p50 | Python p99 | Kotlin/JVM p50 | Kotlin/JVM p99 | TS p50 | TS p99 |")
    lines.append("|---:|---:|---:|---:|---:|---:|---:|---:|---:|")
    for n in [100, 1_000, 10_000]:
        row = [str(n)]
        for b in BINDINGS:
            d = data.get(b)
            if d is None:
                row.extend(["N/A", "N/A"])
                continue
            rec = next((r for r in d.get("recall", []) if r.get("n") == n), None)
            if rec is None:
                row.extend(["N/A", "N/A"])
            else:
                row.append(fmt_ms(rec["p50_ms"]))
                row.append(fmt_ms(rec["p99_ms"]))
        lines.append("| " + " | ".join(row) + " |")
    lines.append("")

    # -- persistence --
    lines.append("## Persistence throughput (higher is better)")
    lines.append("")
    lines.append("| Operation | Unit | Rust | Python | Kotlin/JVM | TypeScript |")
    lines.append("|---|---|---:|---:|---:|---:|")

    def cell(b: str, key: str, field: str) -> str:
        d = data.get(b)
        if d is None:
            return "N/A"
        per = d.get("persistence", {})
        if not isinstance(per, dict):
            return "N/A"
        if per.get("available") is False:
            return "N/A[^kotlin-persist]"
        op = per.get(key)
        if not isinstance(op, dict):
            return "N/A"
        v = op.get(field)
        if field == "mb_per_sec":
            return fmt_mbs(v)
        if field == "ops_per_sec":
            return fmt_ops(v)
        return str(v)

    lines.append("| snapshot_write | MB/s | " + " | ".join(
        cell(b, "snapshot_write", "mb_per_sec") for b in BINDINGS) + " |")
    lines.append("| snapshot_read | MB/s | " + " | ".join(
        cell(b, "snapshot_read", "mb_per_sec") for b in BINDINGS) + " |")
    lines.append("| wal_append | ops/s | " + " | ".join(
        cell(b, "wal_append", "ops_per_sec") for b in BINDINGS) + " |")
    lines.append("")

    # -- memory --
    lines.append("## Memory footprint")
    lines.append("")
    lines.append("| Binding | Bytes/item | vs theoretical floor (1250 B = 10000 bits / 8) |")
    lines.append("|---|---:|---:|")
    for b in BINDINGS:
        d = data.get(b)
        if d is None:
            lines.append(f"| {b} | N/A | N/A |")
            continue
        m = d.get("memory", {})
        per = m.get("bytes_per_item")
        floor = m.get("theoretical_floor_bytes_per_item", 1250)
        if per is None or floor in (None, 0):
            lines.append(f"| {b} | N/A | N/A |")
        else:
            ratio = per / floor
            lines.append(f"| {b} | {per:,.1f} | {ratio:.2f}× |")
    lines.append("")
    lines.append(
        "The theoretical floor counts only the binary hypervector key (D=10000 bits = 1250 B). "
        "Real bindings add metadata (uuid, value bytes, tags, map overhead), so 1.5–3× the floor "
        "is normal. Ratios above 5× indicate runtime overhead worth investigating."
    )
    lines.append("")
    lines.append(
        "_Caveat: on macOS the system allocator and memory compressor often keep RSS flat "
        "across allocations under ~50 MB, so readings <100 B/item should be treated as "
        "\"below RSS resolution\" rather than \"zero overhead\". The TypeScript number is a "
        "V8 heapUsed sample taken after `--expose-gc` collection, which is more reliable but "
        "undercounts off-heap Uint8Array backing buffers. Run on Linux for the most reliable "
        "per-item numbers._"
    )
    lines.append("")

    # -- footnotes --
    if any(b in missing for b in BINDINGS) or any(
        (d.get("persistence", {}).get("available") is False) for d in data.values() if d
    ):
        lines.append("---")
        lines.append("")
        lines.append("### Footnotes")
        lines.append("")
        for b in missing:
            lines.append(
                f"- `{b}`: results file `results/{b}.json` was missing at render time; "
                f"this binding's toolchain probably failed during `run-all.sh`."
            )
        # Per-binding notes from the JSON itself.
        for b in BINDINGS:
            d = data.get(b)
            if d is None:
                continue
            per = d.get("persistence", {})
            if isinstance(per, dict):
                if per.get("available") is False:
                    note = per.get("note", "(no note)")
                    lines.append(f"[^kotlin-persist]: `{b}` persistence: {note}")
                wa = per.get("wal_append", {})
                if isinstance(wa, dict) and "note" in wa:
                    lines.append(f"- `{b}` wal_append: {wa['note']}")
        lines.append("")

    with open(OUT_PATH, "w") as f:
        f.write("\n".join(lines))
    print(f"wrote {OUT_PATH}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
