use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smritidb_core::{bind, bundle, encode_string, random_hv, similarity};

fn bench_random_hv(c: &mut Criterion) {
    c.bench_function("random_hv D=10000", |b| {
        b.iter(|| random_hv(black_box(b"seed"), 10000));
    });
}

fn bench_similarity(c: &mut Criterion) {
    let a = random_hv(b"a", 10000);
    let bv = random_hv(b"b", 10000);
    c.bench_function("similarity D=10000", |b| {
        b.iter(|| similarity(black_box(&a), black_box(&bv)));
    });
}

fn bench_bind(c: &mut Criterion) {
    let a = random_hv(b"a", 10000);
    let bv = random_hv(b"b", 10000);
    c.bench_function("bind D=10000", |b| {
        b.iter(|| bind(black_box(&a), black_box(&bv)));
    });
}

fn bench_bundle(c: &mut Criterion) {
    let xs: Vec<_> = (0..7).map(|i| random_hv(format!("x{i}").as_bytes(), 10000)).collect();
    let refs: Vec<&Vec<u8>> = xs.iter().collect();
    c.bench_function("bundle n=7 D=10000", |b| {
        b.iter(|| bundle(black_box(&refs)));
    });
}

fn bench_recall_100k(c: &mut Criterion) {
    // The hot path: linear-scan recall over a populated cleanup memory.
    let n = 100_000;
    let dim = 10000;
    let store: Vec<_> = (0..n)
        .map(|i| encode_string(&format!("item_{i}"), dim))
        .collect();
    let cue = encode_string("item_42", dim);
    c.bench_function("recall@1 N=100k D=10000 (brute-force)", |b| {
        b.iter(|| {
            let mut best_i = 0usize;
            let mut best_s = f64::NEG_INFINITY;
            for (i, hv) in store.iter().enumerate() {
                let s = similarity(black_box(hv), black_box(&cue));
                if s > best_s {
                    best_s = s;
                    best_i = i;
                }
            }
            (best_i, best_s)
        });
    });
}

criterion_group!(
    benches,
    bench_random_hv,
    bench_similarity,
    bench_bind,
    bench_bundle,
    bench_recall_100k,
);
criterion_main!(benches);
