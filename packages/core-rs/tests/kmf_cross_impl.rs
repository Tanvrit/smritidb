//! Cross-implementation KMF conformance test.
//!
//! Loads `tests/conformance/kmf_fixture.bin` — a KMF blob emitted by the
//! TypeScript reference — and verifies the Rust reader recovers the exact
//! items described in `tests/conformance/kmf_fixture.json`.
//!
//! Then re-writes the same logical snapshot via the Rust writer and asserts
//! the bytes are byte-identical to the TS-produced blob. This is the
//! tightest possible test of KMF parity.
//!
//! Regenerate the fixture with:
//!
//!     cd packages/core-ts && pnpm build
//!     node tests/conformance/emit_kmf_fixture.mjs

use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::Value as JsonValue;
use smritidb_core::{random_hv, read_kmf, write_kmf, KmfItem, KmfSnapshot};

#[derive(Debug, Deserialize)]
struct FixtureSpec {
    dimension: usize,
    created_at: i64,
    items: Vec<FixtureItem>,
}

#[derive(Debug, Deserialize)]
struct FixtureItem {
    id: String,
    seed: String,
    value: String,
    tags: Vec<String>,
    metadata: JsonValue,
    created_at: i64,
    access_count: u32,
    last_accessed_at: i64,
}

fn fixture_paths() -> (PathBuf, PathBuf) {
    let mut bin = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bin.pop();
    bin.pop();
    bin.push("tests/conformance/kmf_fixture.bin");
    let mut spec = bin.clone();
    spec.set_extension("json");
    (bin, spec)
}

fn load_fixture() -> (Vec<u8>, FixtureSpec) {
    let (bin_path, spec_path) = fixture_paths();
    let bytes = fs::read(&bin_path).unwrap_or_else(|e| {
        panic!(
            "could not read {}: {e}. Regenerate with `node tests/conformance/emit_kmf_fixture.mjs`.",
            bin_path.display()
        )
    });
    let spec_raw = fs::read_to_string(&spec_path).expect("read kmf_fixture.json");
    let spec: FixtureSpec = serde_json::from_str(&spec_raw).expect("parse kmf_fixture.json");
    (bytes, spec)
}

#[test]
fn rust_reads_ts_produced_kmf() {
    let (bytes, spec) = load_fixture();
    let snap = read_kmf(&bytes).expect("read TS-produced KMF");
    assert_eq!(snap.dimension, spec.dimension);
    assert_eq!(snap.created_at, spec.created_at);
    assert_eq!(snap.items.len(), spec.items.len());

    for (got, want) in snap.items.iter().zip(spec.items.iter()) {
        let expected_key = random_hv(want.seed.as_bytes(), spec.dimension);
        assert_eq!(got.id, want.id);
        assert_eq!(got.key, expected_key, "key for {} differs", want.id);
        assert_eq!(got.value, want.value.as_bytes());
        assert_eq!(got.tags, want.tags);
        assert_eq!(got.metadata, want.metadata);
        assert_eq!(got.created_at, want.created_at);
        assert_eq!(got.access_count, want.access_count);
        assert_eq!(got.last_accessed_at, want.last_accessed_at);
    }
}

#[test]
fn rust_writer_matches_ts_writer_byte_for_byte() {
    let (ts_bytes, spec) = load_fixture();

    // Reconstruct the same logical snapshot in Rust.
    let mut items: Vec<KmfItem> = Vec::with_capacity(spec.items.len());
    for it in &spec.items {
        items.push(KmfItem {
            id: it.id.clone(),
            key: random_hv(it.seed.as_bytes(), spec.dimension),
            value: it.value.as_bytes().to_vec(),
            tags: it.tags.clone(),
            metadata: it.metadata.clone(),
            created_at: it.created_at,
            access_count: it.access_count,
            last_accessed_at: it.last_accessed_at,
        });
    }
    let snap = KmfSnapshot {
        dimension: spec.dimension,
        created_at: spec.created_at,
        items,
    };
    let rs_bytes = write_kmf(&snap).expect("write KMF from Rust");
    assert_eq!(
        rs_bytes.len(),
        ts_bytes.len(),
        "byte length differs: rs={} ts={}",
        rs_bytes.len(),
        ts_bytes.len()
    );
    if rs_bytes != ts_bytes {
        // Find the first divergence to make debugging tractable.
        for (i, (a, b)) in rs_bytes.iter().zip(ts_bytes.iter()).enumerate() {
            if a != b {
                panic!(
                    "byte mismatch at offset {i}: rs=0x{a:02x} ts=0x{b:02x}\n\
                     context rs: {:?}\ncontext ts: {:?}",
                    &rs_bytes[i.saturating_sub(8)..(i + 8).min(rs_bytes.len())],
                    &ts_bytes[i.saturating_sub(8)..(i + 8).min(ts_bytes.len())],
                );
            }
        }
    }
}
