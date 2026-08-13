//! Persistence benchmark — SQLite snapshot write/read throughput.
//!
//! Inserts PERSIST_N=10000 items, persists to a temp SQLite file, measures
//! the wall-clock and blob size. Reopens the file and re-loads to measure
//! read throughput.

use std::sync::Arc;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use smritidb_core::{
    open_persistent_store, persist::PersistenceAdapter, persist::SqliteAdapter, persist_store,
    Store, StoreConfig,
};

const D: usize = 10_000;
const PERSIST_N: usize = 10_000;

fn build_store() -> Store {
    let mut store = Store::new(StoreConfig {
        dimension: D,
        ..Default::default()
    })
    .expect("store");
    for i in 0..PERSIST_N {
        let key = format!("item_{i}");
        let value = format!("value for item {i}");
        store
            .put_string(&key, value.as_bytes(), &[], serde_json::Value::Null)
            .expect("put");
    }
    store
}

fn bench_snapshot_write(c: &mut Criterion) {
    // We construct a fresh store every iteration to avoid amortising the build
    // cost across multiple persist() calls. Criterion's `iter_with_setup`
    // excludes the setup closure from the measured region.
    let mut group = c.benchmark_group("snapshot_write");
    group.throughput(Throughput::Elements(PERSIST_N as u64));
    group.sample_size(10);
    group.bench_function(BenchmarkId::new("N", PERSIST_N), |b| {
        b.iter_with_setup(
            || {
                let dir = tempfile::tempdir().expect("tempdir");
                let path = dir.path().join("bench.db");
                let store = build_store();
                (dir, path, store)
            },
            |(_dir, path, store)| {
                let adapter = SqliteAdapter::open(path.to_str().unwrap()).expect("open");
                persist_store(black_box(&store), &adapter).expect("persist");
            },
        );
    });
    group.finish();
}

fn bench_snapshot_read(c: &mut Criterion) {
    // Prepare a snapshot once, then measure repeated open_persistent_store calls.
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("bench.db");

    let store = build_store();
    {
        let adapter = SqliteAdapter::open(path.to_str().unwrap()).expect("open");
        persist_store(&store, &adapter).expect("persist");
    }

    let blob_bytes = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
    println!("snapshot_read: blob_bytes = {blob_bytes}");

    let mut group = c.benchmark_group("snapshot_read");
    group.throughput(Throughput::Bytes(blob_bytes));
    group.sample_size(10);
    group.bench_function(BenchmarkId::new("N", PERSIST_N), |b| {
        b.iter(|| {
            let adapter: Arc<dyn PersistenceAdapter> =
                Arc::new(SqliteAdapter::open(path.to_str().unwrap()).expect("open"));
            let restored = open_persistent_store(
                adapter,
                StoreConfig {
                    dimension: D,
                    ..Default::default()
                },
            )
            .expect("restore");
            black_box(restored.size());
        });
    });
    group.finish();
}

fn bench_wal_append(c: &mut Criterion) {
    // Per-op WAL append: each iteration appends one KMF-encoded delta. The
    // delta is precomputed outside the timed region so we measure the adapter,
    // not the serializer.
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("wal.db");
    let adapter = SqliteAdapter::open(path.to_str().unwrap()).expect("open");

    // A small (single-item) delta KMF blob. We snapshot a 1-item store to get
    // a realistic small KMF payload.
    let mut delta = Store::new(StoreConfig {
        dimension: D,
        ..Default::default()
    })
    .expect("store");
    delta
        .put_string("delta", b"delta value", &[], serde_json::Value::Null)
        .expect("put");
    let delta_bytes = delta.snapshot().expect("snapshot");

    let mut group = c.benchmark_group("wal_append");
    group.throughput(Throughput::Elements(1));
    group.bench_function("single-item delta", |b| {
        b.iter(|| {
            adapter.append_wal(black_box(&delta_bytes)).expect("wal");
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_snapshot_write,
    bench_snapshot_read,
    bench_wal_append
);
criterion_main!(benches);
