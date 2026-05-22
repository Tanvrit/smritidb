#!/usr/bin/env python3
"""Run every Python benchmark sub-script in this directory and emit
`results/python.json` in the cross-binding schema.
"""

from __future__ import annotations

import json
import os
import platform
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

import bench_persistence  # noqa: E402
import bench_primitives  # noqa: E402
import bench_recall  # noqa: E402


def main() -> int:
    print("Smritidb benchmark — Python binding")
    print(f"  Python {sys.version.split()[0]} on {platform.platform()}")

    primitives_out = bench_primitives.run()
    recall_out = bench_recall.run()
    persistence_out = bench_persistence.run()
    memory_out = bench_persistence.run_memory()

    output = {
        "binding": "python",
        "version": "0.1.0",
        "spec_version": primitives_out["spec_version"],
        "host": {
            "os": platform.system().lower(),
            "arch": platform.machine().lower(),
        },
        "primitives": primitives_out["results"],
        "recall": recall_out,
        "persistence": persistence_out,
        "memory": memory_out,
    }

    out_dir = os.path.abspath(os.path.join(HERE, "..", "results"))
    os.makedirs(out_dir, exist_ok=True)
    out_path = os.path.join(out_dir, "python.json")
    with open(out_path, "w") as f:
        json.dump(output, f, indent=2)
    print(f"wrote {out_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
