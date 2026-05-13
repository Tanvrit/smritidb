"""Smritidb — an open biology-inspired associative memory layer.

Python bindings over the Rust core. Same primitives as the TypeScript
reference, bit-exact across implementations. See https://smritidb.com.
"""

from __future__ import annotations

from ._native import (
    SPEC_VERSION,
    Store,
    bind,
    bundle,
    encode_embedding,
    encode_string,
    permute,
    random_hv,
    similarity,
    unbind,
)

__all__ = [
    "SPEC_VERSION",
    "Store",
    "bind",
    "bundle",
    "encode_embedding",
    "encode_string",
    "permute",
    "random_hv",
    "similarity",
    "unbind",
]

__version__ = "0.1.0"
