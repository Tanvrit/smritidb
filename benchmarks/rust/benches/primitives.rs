//! Extended primitives benchmark — Rust binding.
//!
//! This mirrors the workload defined in `benchmarks/WORKLOAD.md` and runs
//! it through Criterion for fine-grained per-op timings. The companion
//! `cargo run --bin smritidb-bench` produces the JSON consumed by
//! `run-all.sh`; the Criterion HTML reports here are auxiliary.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use smritidb_core::{bind, bundle, encode_string, random_hv, similarity};

const D: usize = 10_000;

fn bench_random_hv(c: &mut Criterion) {
    let mut group = c.benchmark_group("random_hv");
    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("D", D), |b| {
        b.iter(|| random_hv(black_box(b"seed"), D));
    });
    group.finish();
}

fn bench_similarity(c: &mut Criterion) {
    let a = random_hv(b"a", D);
    let bv = random_hv(b"b", D);
    let mut group = c.benchmark_group("similarity");
    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("D", D), |b| {
        b.iter(|| similarity(black_box(&a), black_box(&bv)));
    });
    group.finish();
}

fn bench_bind(c: &mut Criterion) {
    let a = random_hv(b"a", D);
    let bv = random_hv(b"b", D);
    let mut group = c.benchmark_group("bind");
    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("D", D), |b| {
        b.iter(|| bind(black_box(&a), black_box(&bv)));
    });
    group.finish();
}

fn bench_bundle(c: &mut Criterion) {
    let a = random_hv(b"a", D);
    let bv = random_hv(b"b", D);
    let refs: Vec<&Vec<u8>> = vec![&a, &bv];
    let mut group = c.benchmark_group("bundle");
    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("D pair", D), |b| {
        b.iter(|| bundle(black_box(&refs)));
    });
    group.finish();
}

fn bench_encode_string(c: &mut Criterion) {
    // Vary the input per iteration to defeat any caching the encoder might do
    // and to reflect a realistic workload where strings differ.
    let inputs: Vec<String> = (0..1024).map(|i| format!("item_{i}")).collect();
    let mut group = c.benchmark_group("encode_string");
    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("D", D), |b| {
        let mut i = 0usize;
        b.iter(|| {
            let s = &inputs[i % inputs.len()];
            i = i.wrapping_add(1);
            encode_string(black_box(s.as_str()), D)
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_random_hv,
    bench_similarity,
    bench_bind,
    bench_bundle,
    bench_encode_string,
);
criterion_main!(benches);
