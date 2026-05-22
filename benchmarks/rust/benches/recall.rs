//! Recall latency benchmark — Rust binding.
//!
//! Builds an in-memory store of N items keyed `item_{i}` and runs a sweep of
//! recall queries against it. Criterion reports mean / std-dev per query;
//! the companion `cargo run --bin smritidb-bench` produces the p50/p99/p999
//! that the cross-binding RESULTS.md table consumes.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use smritidb_core::{encode_string, Store, StoreConfig};

const D: usize = 10_000;

fn build_store(n: usize) -> Store {
    let mut store = Store::new(StoreConfig {
        dimension: D,
        ..Default::default()
    })
    .expect("store");
    for i in 0..n {
        let key = format!("item_{i}");
        let value = format!("value for item {i}");
        store
            .put_string(&key, value.as_bytes(), &[], serde_json::Value::Null)
            .expect("put");
    }
    store
}

fn bench_recall_sweep(c: &mut Criterion) {
    let mut group = c.benchmark_group("recall");
    for n in [100usize, 1_000, 10_000] {
        let mut store = build_store(n);
        // Pre-encode 1000 cues to cycle through. Encoding happens outside
        // the measured region so we time recall in isolation.
        let cues: Vec<_> = (0..1000)
            .map(|i| encode_string(&format!("item_{}", i % n), D))
            .collect();
        let mut idx = 0usize;
        group.throughput(Throughput::Elements(1));
        group.bench_function(BenchmarkId::new("N", n), |b| {
            b.iter(|| {
                let cue = &cues[idx % cues.len()];
                idx = idx.wrapping_add(1);
                let hits = store.recall(black_box(cue), 10, 0.5);
                black_box(hits.len());
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_recall_sweep);
criterion_main!(benches);
