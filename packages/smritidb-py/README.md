# smritidb (Python)

Python bindings for [Smritidb](https://smritidb.com) — the open biology-inspired associative memory standard. Built on the Rust core via [pyo3](https://pyo3.rs) + [maturin](https://maturin.rs).

## Install

```bash
pip install smritidb
```

Wheels for CPython 3.9+ on Linux / macOS / Windows × x86_64 + aarch64.

## Quick example

```python
from smritidb import Store

store = Store(dimension=10000)
store.put("the cat sat on the mat", "the cat sat on the mat")
store.put("a bird in the hand", "a bird in the hand")

hits = store.recall("cat on mat", top_k=3, min_similarity=0.0)
for h in hits:
    print(h["value"].decode(), "->", round(h["similarity"], 4))
```

## Primitives

```python
from smritidb import random_hv, similarity, bind, unbind, bundle, permute, encode_string

a = random_hv(b"role", 8192)
b = random_hv(b"filler", 8192)
bound = bind(a, b)
assert similarity(unbind(bound, a), b) == 1.0   # XOR is self-inverse
```

## Build from source

```bash
pip install maturin
maturin develop --release            # build + install in your venv
pytest tests/                        # run conformance suite
```

## License

Apache-2.0. See the repo root [`LICENSE`](../../LICENSE) and [`NOTICE`](../../NOTICE).
