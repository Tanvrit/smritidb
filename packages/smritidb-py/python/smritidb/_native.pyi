"""Type stubs for the native extension module."""
from __future__ import annotations

from types import TracebackType
from typing import Any, Iterable, Optional, Type, TypedDict, Union

SPEC_VERSION: str

CueLike = Union[str, bytes, list[float]]
ValueLike = Union[str, bytes]


class Match(TypedDict):
    id: str
    similarity: float
    value: bytes
    tags: list[str]
    access_count: int


class Item(TypedDict):
    id: str
    value: bytes
    tags: list[str]
    created_at: float
    access_count: int
    last_accessed_at: float


class ConsolidationReport(TypedDict):
    pairs_pulled: int
    bits_flipped: int
    cold_items_flagged: int


class SmritidbError(Exception):
    """Base class for native smritidb errors."""


class PersistenceError(SmritidbError):
    """Raised when the underlying persistence adapter fails."""


class StoreError(SmritidbError):
    """Raised when a Store operation fails."""


def random_hv(seed: bytes, dim: int) -> bytes: ...
def similarity(a: bytes, b: bytes) -> float: ...
def bind(a: bytes, b: bytes) -> bytes: ...
def unbind(a: bytes, b: bytes) -> bytes: ...
def bundle(hvs: Iterable[bytes]) -> bytes: ...
def permute(hv: bytes, k: int) -> bytes: ...
def encode_string(s: str, dim: int) -> bytes: ...
def encode_embedding(embedding: list[float], dim: int) -> bytes: ...


class Store:
    spec_version: str
    dimension: int

    def __init__(
        self,
        dimension: int = 10000,
        top_k: int = 10,
        min_similarity: float = 0.5,
    ) -> None: ...

    def put(
        self,
        key: CueLike,
        value: ValueLike,
        id: Optional[str] = ...,
        tags: Optional[list[str]] = ...,
    ) -> str: ...

    def recall(
        self,
        cue: CueLike,
        top_k: Optional[int] = ...,
        min_similarity: Optional[float] = ...,
    ) -> list[Match]: ...

    def delete(self, id: str) -> bool: ...
    def get(self, id: str) -> Item: ...
    def size(self) -> int: ...


class PersistentStore:
    """Store backed by a Rust-core persistence adapter.

    Construct via one of the staticmethod openers — `open_sqlite`,
    `open_file`, or `open_memory`. Supports the context manager protocol;
    `close()` releases the underlying adapter.
    """

    dimension: int
    spec_version: str

    @staticmethod
    def open_sqlite(path: str, dimension: Optional[int] = ...) -> "PersistentStore": ...

    @staticmethod
    def open_file(path: str, dimension: Optional[int] = ...) -> "PersistentStore": ...

    @staticmethod
    def open_memory(dimension: Optional[int] = ...) -> "PersistentStore": ...

    def put(
        self,
        key: CueLike,
        value: ValueLike,
        id: Optional[str] = ...,
        tags: Optional[list[str]] = ...,
        metadata: Optional[Any] = ...,
    ) -> str: ...

    def recall(
        self,
        cue: CueLike,
        top_k: Optional[int] = ...,
        min_similarity: Optional[float] = ...,
    ) -> list[Match]: ...

    def delete(self, id: str) -> bool: ...
    def get(self, id: str) -> Optional[Item]: ...
    def size(self) -> int: ...
    def consolidate(self) -> ConsolidationReport: ...
    def persist(self) -> None: ...
    def close(self) -> None: ...

    def __enter__(self) -> "PersistentStore": ...
    def __exit__(
        self,
        exc_type: Optional[Type[BaseException]],
        exc_value: Optional[BaseException],
        traceback: Optional[TracebackType],
    ) -> bool: ...
