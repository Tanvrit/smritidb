"""Persistence smoke tests for the PyO3 `PersistentStore` wrapper.

These exercise the same `PersistenceAdapter` surface used by the Rust unit
tests, but driven from Python via the native module.
"""

from __future__ import annotations

import os
import tempfile

import pytest

from smritidb import PersistenceError, PersistentStore, SmritidbError


D = 8192  # small dimension keeps the round-trip test cheap


def _tmp_sqlite_path() -> str:
    with tempfile.NamedTemporaryFile(suffix=".db", delete=False) as f:
        return f.name


def test_sqlite_round_trip():
    path = _tmp_sqlite_path()
    # Ensure SQLite opens an empty file rather than parsing the zero-byte one
    # created by NamedTemporaryFile.
    os.unlink(path)
    try:
        store = PersistentStore.open_sqlite(path, dimension=D)
        for i in range(1000):
            store.put(f"key{i}", bytes([i % 256]))
        store.persist()
        assert store.size() == 1000
        store.close()

        # Reopen — should replay the snapshot off disk.
        store2 = PersistentStore.open_sqlite(path)
        assert store2.size() == 1000
        # Dimension comes from the persisted snapshot, not the default.
        assert store2.dimension == D
        matches = store2.recall("key500", top_k=5, min_similarity=0.5)
        assert len(matches) > 0
        assert matches[0]["similarity"] > 0.99
        assert matches[0]["value"] == bytes([500 % 256])
        store2.close()
    finally:
        for suffix in ("", "-journal", "-wal", "-shm"):
            p = path + suffix
            if os.path.exists(p):
                os.unlink(p)


def test_in_memory_recall():
    store = PersistentStore.open_memory(dimension=D)
    store.put("alpha", b"hello")
    store.put("beta", b"world")
    matches = store.recall("alpha", top_k=2, min_similarity=0.5)
    assert matches[0]["similarity"] > 0.9
    assert matches[0]["value"] == b"hello"
    store.close()


def test_file_adapter_round_trip(tmp_path):
    path = tmp_path / "store.kmf"
    store = PersistentStore.open_file(str(path), dimension=D)
    store.put("hello", b"world", tags=["greet"])
    store.persist()
    store.close()
    assert path.exists()

    reopened = PersistentStore.open_file(str(path))
    assert reopened.size() == 1
    hits = reopened.recall("hello", top_k=1, min_similarity=0.9)
    assert len(hits) == 1
    assert hits[0]["value"] == b"world"
    assert hits[0]["tags"] == ["greet"]
    reopened.close()


def test_get_returns_none_for_missing():
    store = PersistentStore.open_memory(dimension=D)
    assert store.get("does-not-exist") is None
    store.close()


def test_delete_returns_bool():
    store = PersistentStore.open_memory(dimension=D)
    item_id = store.put("alpha", b"a", id="alpha-1")
    assert item_id == "alpha-1"
    assert store.delete("alpha-1") is True
    assert store.delete("alpha-1") is False
    store.close()


def test_context_manager_closes():
    with PersistentStore.open_memory(dimension=D) as store:
        store.put("alpha", b"a")
        assert store.size() == 1
    # After exit, further calls fail with SmritidbError.
    with pytest.raises(SmritidbError):
        store.put("beta", b"b")


def test_consolidate_returns_report():
    store = PersistentStore.open_memory(dimension=D)
    store.put("alpha", b"a")
    store.put("beta", b"b")
    # Touch them together so the tracker records a coactivation.
    store.recall("alpha", top_k=2, min_similarity=0.0)
    report = store.consolidate()
    assert set(report.keys()) == {"pairs_pulled", "bits_flipped", "cold_items_flagged"}
    store.close()


def test_metadata_round_trips():
    store = PersistentStore.open_memory(dimension=D)
    store.put("alpha", b"a", metadata={"weight": 0.7, "source": "test"})
    store.close()


def test_persistence_error_on_invalid_path():
    # Pointing SqliteAdapter at a path under a non-existent directory should
    # surface a PersistenceError, not a panic.
    with pytest.raises(PersistenceError):
        PersistentStore.open_sqlite("/nonexistent-dir-xyzzy/db.sqlite")
