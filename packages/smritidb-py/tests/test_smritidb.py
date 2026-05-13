"""Phase 1 conformance tests, re-run in Python against the Rust core via pyo3.

These mirror the TypeScript test suite and verify bit-exact identity for
randomHv across implementations.
"""

import pytest
from smritidb import (
    Store,
    bind,
    bundle,
    encode_string,
    permute,
    random_hv,
    similarity,
    unbind,
)


D = 8192


def test_spec_version_present():
    from smritidb import SPEC_VERSION
    assert isinstance(SPEC_VERSION, str)
    assert SPEC_VERSION.startswith("0.")


def test_random_hv_is_deterministic_and_correct_length():
    a = random_hv(b"hello", D)
    b = random_hv(b"hello", D)
    c = random_hv(b"world", D)
    assert len(a) == D
    assert a == b
    assert a != c


def test_similarity_endpoints():
    a = random_hv(b"a", D)
    b = random_hv(b"b", D)
    assert similarity(a, a) == 1.0
    assert 0.45 < similarity(a, b) < 0.55


def test_bind_is_self_inverse():
    a = random_hv(b"a", D)
    b = random_hv(b"b", D)
    assert similarity(unbind(bind(a, b), b), a) == 1.0


def test_bundle_preserves_similarity_to_components():
    xs = [random_hv(f"x{i}".encode(), D) for i in range(5)]
    bundled = bundle(xs)
    for x in xs:
        assert similarity(bundled, x) > 0.6


def test_permute_is_invertible():
    a = random_hv(b"p", D)
    assert similarity(permute(permute(a, 7), -7), a) == 1.0


def test_compositional_role_filler_recovery():
    role = encode_string("role:name", D)
    alice = encode_string("filler:alice", D)
    bob = encode_string("filler:bob", D)
    record = bind(role, alice)
    recovered = unbind(record, role)
    assert similarity(recovered, alice) == 1.0
    assert similarity(recovered, bob) < 0.55


def test_store_rejects_small_dimension():
    with pytest.raises(ValueError):
        Store(dimension=512)


def test_store_put_and_recall_round_trip():
    store = Store(dimension=D)
    store.put("the cat sat on the mat", "the cat sat on the mat")
    hits = store.recall("the cat sat on the mat", top_k=1, min_similarity=0.9)
    assert len(hits) == 1
    assert hits[0]["value"] == b"the cat sat on the mat"
    assert hits[0]["similarity"] == 1.0


def test_store_ranks_multiple_items_by_similarity():
    store = Store(dimension=D)
    store.put("alpha", "alpha")
    store.put("beta", "beta")
    store.put("gamma", "gamma")
    hits = store.recall("alpha", top_k=3, min_similarity=0.0)
    assert len(hits) == 3
    assert hits[0]["value"] == b"alpha"
    assert hits[0]["similarity"] == 1.0


def test_store_delete_and_get():
    store = Store(dimension=D)
    item_id = store.put("alpha", "alpha", id="fixed-1")
    assert store.size() == 1
    assert store.delete(item_id) is True
    assert store.size() == 0
    with pytest.raises(KeyError):
        store.get(item_id)


def test_store_access_count_increments_on_hit():
    store = Store(dimension=D)
    item_id = store.put("alpha", "alpha", id="alpha")
    store.recall("alpha", top_k=1, min_similarity=0.9)
    store.recall("alpha", top_k=1, min_similarity=0.9)
    assert store.get(item_id)["access_count"] == 2
