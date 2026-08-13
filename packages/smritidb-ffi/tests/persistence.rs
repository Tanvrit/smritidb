//! Integration tests for the `PersistentStore` UniFFI surface.
//!
//! These exercise the Rust-side API directly; the Kotlin / Swift bindings
//! generated from the same UDL go through identical entry points, so passing
//! here implies the generated FFI is structurally sound.

use smritidb_ffi::{AdapterKind, PersistenceConfig, PersistentStore, StoreOptions};

fn opts() -> StoreOptions {
    StoreOptions {
        // 0 => inherit core defaults (dim = 10_000, etc).
        dimension: 0,
        value_cap_bytes: 0,
        default_top_k: 0,
        default_min_similarity: 0.0,
    }
}

#[test]
fn persistent_store_sqlite_round_trip() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    // `into_temp_path` releases the file handle but keeps the path reserved
    // for cleanup on drop, so SQLite can open it cleanly on every platform.
    let path_holder = tmp.into_temp_path();
    let path = path_holder.to_str().unwrap().to_string();

    let store = PersistentStore::open_sqlite(path.clone(), opts()).unwrap();
    for i in 0..1000 {
        store
            .put(format!("key{}", i), vec![i as u8], vec![], None)
            .unwrap();
    }
    store.persist().unwrap();
    assert_eq!(store.size().unwrap(), 1000);
    // Force-drop so SQLite releases its handle before we reopen.
    drop(store);

    let store2 = PersistentStore::open_sqlite(path.clone(), opts()).unwrap();
    assert_eq!(store2.size().unwrap(), 1000);

    let matches = store2.recall("key500".to_string(), 5, 0.5).unwrap();
    assert!(!matches.is_empty(), "expected at least one recall hit");
    assert!(
        matches[0].similarity > 0.99,
        "best match similarity was {}",
        matches[0].similarity
    );
}

#[test]
fn persistent_store_file_round_trip() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir
        .path()
        .join("ffi-store.kmf")
        .to_str()
        .unwrap()
        .to_string();

    let store = PersistentStore::open_file(path.clone(), opts()).unwrap();
    let id = store
        .put(
            "alpha".to_string(),
            b"alpha-value".to_vec(),
            vec!["greek".to_string()],
            Some("{\"source\":\"ffi-test\"}".to_string()),
        )
        .unwrap();
    store
        .put("beta".to_string(), b"beta-value".to_vec(), vec![], None)
        .unwrap();
    store.persist().unwrap();
    assert_eq!(store.size().unwrap(), 2);

    let fetched = store.get(id.clone()).unwrap();
    assert_eq!(fetched.value, b"alpha-value".to_vec());
    assert_eq!(fetched.tags, vec!["greek".to_string()]);

    drop(store);

    let store2 = PersistentStore::open_file(path, opts()).unwrap();
    assert_eq!(store2.size().unwrap(), 2);
    let hits = store2.recall("alpha".to_string(), 1, 0.9).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].value, b"alpha-value".to_vec());
}

#[test]
fn persistent_store_memory_does_not_persist_across_instances() {
    let store_a = PersistentStore::open_memory(opts()).unwrap();
    store_a
        .put("alpha".to_string(), b"a".to_vec(), vec![], None)
        .unwrap();
    store_a.persist().unwrap();
    assert_eq!(store_a.size().unwrap(), 1);

    // A fresh MemoryAdapter is independent — verifies the constructor really
    // creates a new adapter rather than aliasing process-global state.
    let store_b = PersistentStore::open_memory(opts()).unwrap();
    assert_eq!(store_b.size().unwrap(), 0);
}

#[test]
fn open_via_persistence_config_sqlite() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.into_temp_path();
    let path_str = path.to_str().unwrap().to_string();

    let store = PersistentStore::open(
        PersistenceConfig {
            kind: AdapterKind::Sqlite,
            path: Some(path_str.clone()),
        },
        opts(),
    )
    .unwrap();
    store
        .put("alpha".to_string(), b"a".to_vec(), vec![], None)
        .unwrap();
    store.persist().unwrap();
    drop(store);

    let store2 = PersistentStore::open(
        PersistenceConfig {
            kind: AdapterKind::Sqlite,
            path: Some(path_str),
        },
        opts(),
    )
    .unwrap();
    assert_eq!(store2.size().unwrap(), 1);
}

#[test]
fn open_file_requires_path_via_config() {
    let err = PersistentStore::open(
        PersistenceConfig {
            kind: AdapterKind::File,
            path: None,
        },
        opts(),
    )
    .err()
    .expect("opening File adapter without a path should fail");
    // The exact variant doesn't matter as much as the fact that it's an error.
    let _ = err;
}

#[test]
fn delete_and_consolidate_smoke() {
    let store = PersistentStore::open_memory(opts()).unwrap();
    let id = store
        .put("solo".to_string(), b"x".to_vec(), vec![], None)
        .unwrap();
    assert_eq!(store.size().unwrap(), 1);
    // Consolidation on a singleton store is a no-op but must not error.
    let _ = store.consolidate().unwrap();
    assert!(store.delete(id.clone()));
    assert_eq!(store.size().unwrap(), 0);
    assert!(!store.delete(id));
    // Idempotent close.
    store.close().unwrap();
}
