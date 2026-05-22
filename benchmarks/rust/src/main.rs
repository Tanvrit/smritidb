//! Cross-binding benchmark runner — Rust portion.
//!
//! Emits `results/rust.json` matching the schema in `benchmarks/WORKLOAD.md`.
//! This is the file `run-all.sh` consumes; the criterion benches are
//! auxiliary HTML reports.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use serde::Serialize;
use smritidb_core::{
    bind, bundle, encode_string, open_persistent_store, persist::PersistenceAdapter,
    persist::SqliteAdapter, persist_store, random_hv, similarity, Store, StoreConfig,
};

const D: usize = 10_000;
const RANDOM_HV_BATCH: usize = 1_000;
const BUNDLE_BATCH: usize = 100;
const BIND_BATCH: usize = 100;
const SIMILARITY_BATCH: usize = 10_000;
const ENCODE_STRING_BATCH: usize = 10_000;
const RECALL_QUERIES: usize = 1_000;
const PERSIST_N: usize = 10_000;
const WAL_APPENDS: usize = 1_000;
const WARMUP_ITERS: usize = 3;

#[derive(Serialize)]
struct PrimitiveResult {
    ops_per_sec: f64,
    iters: usize,
    wall_seconds: f64,
}

#[derive(Serialize)]
struct RecallResult {
    n: usize,
    queries: usize,
    p50_ms: f64,
    p99_ms: f64,
    p999_ms: f64,
    insert_seconds: f64,
}

#[derive(Serialize)]
struct PersistOp {
    n: usize,
    bytes: u64,
    wall_seconds: f64,
    mb_per_sec: f64,
}

#[derive(Serialize)]
struct WalResult {
    n: usize,
    wall_seconds: f64,
    ops_per_sec: f64,
}

#[derive(Serialize)]
struct PersistenceResult {
    snapshot_write: PersistOp,
    snapshot_read: PersistOp,
    wal_append: WalResult,
}

#[derive(Serialize)]
struct MemoryResult {
    n_items: usize,
    rss_before_bytes: u64,
    rss_after_bytes: u64,
    bytes_per_item: f64,
    theoretical_floor_bytes_per_item: usize,
}

#[derive(Serialize)]
struct HostInfo {
    os: String,
    arch: String,
}

#[derive(Serialize)]
struct Primitives {
    random_hv: PrimitiveResult,
    bundle: PrimitiveResult,
    bind: PrimitiveResult,
    similarity: PrimitiveResult,
    encode_string: PrimitiveResult,
}

#[derive(Serialize)]
struct Output {
    binding: &'static str,
    version: &'static str,
    spec_version: &'static str,
    host: HostInfo,
    primitives: Primitives,
    recall: Vec<RecallResult>,
    persistence: PersistenceResult,
    memory: MemoryResult,
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let idx = ((sorted.len() as f64 - 1.0) * p).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

fn time_block<F: FnMut()>(iters: usize, mut f: F) -> f64 {
    // Warm-up.
    for _ in 0..WARMUP_ITERS {
        f();
    }
    let start = Instant::now();
    for _ in 0..iters {
        f();
    }
    start.elapsed().as_secs_f64()
}

fn measure(name: &str, iters: usize, body: impl FnMut()) -> PrimitiveResult {
    let wall = time_block(iters, body);
    let ops = iters as f64 / wall;
    println!("  {name}: {ops:>12.0} ops/s ({wall:.3}s for {iters} iters)");
    PrimitiveResult {
        ops_per_sec: ops,
        iters,
        wall_seconds: wall,
    }
}

fn bench_primitives() -> Primitives {
    println!("[1/4] primitives");

    // random_hv — vary the seed per iteration.
    let seeds: Vec<Vec<u8>> = (0..RANDOM_HV_BATCH)
        .map(|i| format!("hv_{i}").into_bytes())
        .collect();
    let mut i = 0usize;
    let random_hv_r = measure("random_hv", RANDOM_HV_BATCH, || {
        std::hint::black_box(random_hv(&seeds[i % seeds.len()], D));
        i = i.wrapping_add(1);
    });

    // Pre-generate pairs for bundle / bind / similarity.
    let pairs: Vec<(Vec<u8>, Vec<u8>)> = (0..SIMILARITY_BATCH)
        .map(|i| {
            (
                random_hv(format!("a{i}").as_bytes(), D),
                random_hv(format!("b{i}").as_bytes(), D),
            )
        })
        .collect();

    let mut i = 0usize;
    let bundle_r = measure("bundle", BUNDLE_BATCH, || {
        let (a, b) = &pairs[i % pairs.len()];
        let refs: Vec<&Vec<u8>> = vec![a, b];
        std::hint::black_box(bundle(&refs));
        i = i.wrapping_add(1);
    });

    let mut i = 0usize;
    let bind_r = measure("bind", BIND_BATCH, || {
        let (a, b) = &pairs[i % pairs.len()];
        std::hint::black_box(bind(a, b));
        i = i.wrapping_add(1);
    });

    let mut i = 0usize;
    let similarity_r = measure("similarity", SIMILARITY_BATCH, || {
        let (a, b) = &pairs[i % pairs.len()];
        std::hint::black_box(similarity(a, b));
        i = i.wrapping_add(1);
    });

    let inputs: Vec<String> = (0..ENCODE_STRING_BATCH).map(|i| format!("item_{i}")).collect();
    let mut i = 0usize;
    let encode_string_r = measure("encode_string", ENCODE_STRING_BATCH, || {
        std::hint::black_box(encode_string(&inputs[i % inputs.len()], D));
        i = i.wrapping_add(1);
    });

    Primitives {
        random_hv: random_hv_r,
        bundle: bundle_r,
        bind: bind_r,
        similarity: similarity_r,
        encode_string: encode_string_r,
    }
}

fn bench_recall() -> Vec<RecallResult> {
    println!("[2/4] recall");
    let mut out = Vec::new();
    for &n in &[100usize, 1_000, 10_000] {
        let mut store = Store::new(StoreConfig {
            dimension: D,
            ..Default::default()
        })
        .expect("store");
        let insert_start = Instant::now();
        for i in 0..n {
            store
                .put_string(
                    &format!("item_{i}"),
                    format!("value for item {i}").as_bytes(),
                    &[],
                    serde_json::Value::Null,
                )
                .expect("put");
        }
        let insert_seconds = insert_start.elapsed().as_secs_f64();

        let cues: Vec<_> = (0..RECALL_QUERIES)
            .map(|i| encode_string(&format!("item_{}", i % n), D))
            .collect();

        // Warm-up — first iteration tends to be noisy.
        for c in cues.iter().take(WARMUP_ITERS) {
            let _ = store.recall(c, 10, 0.5);
        }

        let mut samples = Vec::with_capacity(cues.len());
        for c in &cues {
            let t0 = Instant::now();
            let _ = store.recall(c, 10, 0.5);
            samples.push(t0.elapsed().as_secs_f64() * 1000.0);
        }
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p50 = percentile(&samples, 0.50);
        let p99 = percentile(&samples, 0.99);
        let p999 = percentile(&samples, 0.999);
        println!(
            "  N={n:>5} insert={insert_seconds:.3}s p50={p50:.3}ms p99={p99:.3}ms p999={p999:.3}ms"
        );

        out.push(RecallResult {
            n,
            queries: cues.len(),
            p50_ms: p50,
            p99_ms: p99,
            p999_ms: p999,
            insert_seconds,
        });
    }
    out
}

fn bench_persistence() -> PersistenceResult {
    println!("[3/4] persistence");

    let dir = tempfile::tempdir().expect("tempdir");
    let write_path = dir.path().join("write.db");
    let read_path = dir.path().join("read.db");
    let wal_path = dir.path().join("wal.db");

    // -- snapshot_write --
    let mut store = Store::new(StoreConfig {
        dimension: D,
        ..Default::default()
    })
    .expect("store");
    for i in 0..PERSIST_N {
        store
            .put_string(
                &format!("item_{i}"),
                format!("value for item {i}").as_bytes(),
                &[],
                serde_json::Value::Null,
            )
            .expect("put");
    }
    let adapter = SqliteAdapter::open(write_path.to_str().unwrap()).expect("open");
    let t0 = Instant::now();
    persist_store(&store, &adapter).expect("persist");
    let write_wall = t0.elapsed().as_secs_f64();
    adapter.close().ok();
    let blob_bytes = fs::metadata(&write_path).map(|m| m.len()).unwrap_or(0);
    let write_mbs = (blob_bytes as f64 / 1_000_000.0) / write_wall;
    println!(
        "  snapshot_write N={PERSIST_N} bytes={blob_bytes} wall={write_wall:.3}s = {write_mbs:.1} MB/s"
    );

    // -- snapshot_read --
    // Re-persist into read_path to isolate read measurement (then we re-read it).
    let adapter_r = SqliteAdapter::open(read_path.to_str().unwrap()).expect("open");
    persist_store(&store, &adapter_r).expect("persist");
    adapter_r.close().ok();
    let read_bytes = fs::metadata(&read_path).map(|m| m.len()).unwrap_or(0);

    // Warm-up.
    for _ in 0..WARMUP_ITERS {
        let a: Arc<dyn PersistenceAdapter> =
            Arc::new(SqliteAdapter::open(read_path.to_str().unwrap()).expect("open"));
        let _ = open_persistent_store(
            a,
            StoreConfig {
                dimension: D,
                ..Default::default()
            },
        )
        .expect("restore");
    }

    let t0 = Instant::now();
    let a: Arc<dyn PersistenceAdapter> =
        Arc::new(SqliteAdapter::open(read_path.to_str().unwrap()).expect("open"));
    let restored = open_persistent_store(
        a,
        StoreConfig {
            dimension: D,
            ..Default::default()
        },
    )
    .expect("restore");
    let read_wall = t0.elapsed().as_secs_f64();
    assert_eq!(restored.size(), PERSIST_N);
    let read_mbs = (read_bytes as f64 / 1_000_000.0) / read_wall;
    println!(
        "  snapshot_read  N={PERSIST_N} bytes={read_bytes} wall={read_wall:.3}s = {read_mbs:.1} MB/s"
    );

    // -- wal_append --
    let wal_adapter = SqliteAdapter::open(wal_path.to_str().unwrap()).expect("open");
    // Tiny single-item KMF delta to append.
    let mut delta = Store::new(StoreConfig {
        dimension: D,
        ..Default::default()
    })
    .expect("store");
    delta
        .put_string("delta", b"x", &[], serde_json::Value::Null)
        .expect("put");
    let delta_bytes = delta.snapshot().expect("snapshot");

    for _ in 0..WARMUP_ITERS {
        wal_adapter.append_wal(&delta_bytes).expect("wal");
    }
    let t0 = Instant::now();
    for _ in 0..WAL_APPENDS {
        wal_adapter.append_wal(&delta_bytes).expect("wal");
    }
    let wal_wall = t0.elapsed().as_secs_f64();
    let wal_ops = WAL_APPENDS as f64 / wal_wall;
    println!("  wal_append    N={WAL_APPENDS} wall={wal_wall:.3}s = {wal_ops:.0} ops/s");

    PersistenceResult {
        snapshot_write: PersistOp {
            n: PERSIST_N,
            bytes: blob_bytes,
            wall_seconds: write_wall,
            mb_per_sec: write_mbs,
        },
        snapshot_read: PersistOp {
            n: PERSIST_N,
            bytes: read_bytes,
            wall_seconds: read_wall,
            mb_per_sec: read_mbs,
        },
        wal_append: WalResult {
            n: WAL_APPENDS,
            wall_seconds: wal_wall,
            ops_per_sec: wal_ops,
        },
    }
}

#[cfg(target_os = "macos")]
fn rss_bytes() -> u64 {
    // mach_task_basic_info via the libc crate would be cleaner, but we want
    // zero extra deps. Shell out to `ps` for the same answer.
    let pid = std::process::id();
    let out = std::process::Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output();
    if let Ok(o) = out {
        if let Ok(s) = std::str::from_utf8(&o.stdout) {
            if let Ok(kb) = s.trim().parse::<u64>() {
                return kb * 1024;
            }
        }
    }
    0
}

#[cfg(target_os = "linux")]
fn rss_bytes() -> u64 {
    if let Ok(s) = std::fs::read_to_string("/proc/self/status") {
        for line in s.lines() {
            if let Some(rest) = line.strip_prefix("VmRSS:") {
                let kb: u64 = rest
                    .trim()
                    .split_whitespace()
                    .next()
                    .and_then(|x| x.parse().ok())
                    .unwrap_or(0);
                return kb * 1024;
            }
        }
    }
    0
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn rss_bytes() -> u64 {
    0
}

fn bench_memory() -> MemoryResult {
    println!("[4/4] memory");
    let before = rss_bytes();
    let mut store = Store::new(StoreConfig {
        dimension: D,
        ..Default::default()
    })
    .expect("store");
    for i in 0..PERSIST_N {
        store
            .put_string(
                &format!("item_{i}"),
                format!("value for item {i}").as_bytes(),
                &[],
                serde_json::Value::Null,
            )
            .expect("put");
    }
    let after = rss_bytes();
    let delta = after.saturating_sub(before);
    let per_item = delta as f64 / PERSIST_N as f64;
    println!(
        "  RSS before={before}B after={after}B delta={delta}B = {per_item:.1} bytes/item"
    );
    // Keep `store` alive past the RSS sample.
    std::hint::black_box(&store);
    MemoryResult {
        n_items: PERSIST_N,
        rss_before_bytes: before,
        rss_after_bytes: after,
        bytes_per_item: per_item,
        theoretical_floor_bytes_per_item: D / 8,
    }
}

fn main() {
    let out_path: PathBuf = env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("../results/rust.json"));

    println!("Smritidb benchmark — Rust binding");
    println!("  D={D}, PERSIST_N={PERSIST_N}, WARMUP_ITERS={WARMUP_ITERS}");

    let primitives = bench_primitives();
    let recall = bench_recall();
    let persistence = bench_persistence();
    let memory = bench_memory();

    let out = Output {
        binding: "rust",
        version: env!("CARGO_PKG_VERSION"),
        spec_version: smritidb_core::SPEC_VERSION,
        host: HostInfo {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
        },
        primitives,
        recall,
        persistence,
        memory,
    };

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).ok();
    }
    fs::write(&out_path, serde_json::to_vec_pretty(&out).unwrap()).expect("write json");
    println!("wrote {}", out_path.display());
}
