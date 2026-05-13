"""Type stubs for the native extension module."""
from __future__ import annotations

from typing import Iterable, Optional, TypedDict, Union

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
