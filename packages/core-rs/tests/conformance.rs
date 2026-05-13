//! Cross-binding conformance suite. Verifies that the Rust core produces
//! byte-for-byte the same outputs as the TypeScript reference for the
//! corpus in `tests/conformance/golden.json`.

use std::fs;
use std::path::PathBuf;

use serde::Deserialize;
use sha2::{Digest, Sha256};
use smritidb_core::{
    bind, bundle, encode_bag_of_words, encode_char_ngrams, encode_string, random_hv, similarity,
    TextEncodingOptions, SPEC_VERSION,
};

#[derive(Debug, Deserialize)]
struct Golden {
    spec_version: String,
    random_hv: Vec<RandomHvEntry>,
    encode_string: Vec<EncodeStringEntry>,
    similarity_pairs: Vec<SimilarityPair>,
    bind_round_trip: Vec<BindRoundTrip>,
    bundle: Vec<BundleEntry>,
    text_bag_of_words: Vec<TextEntry>,
    text_char_ngrams: Vec<TextNgramsEntry>,
}

#[derive(Debug, Deserialize)]
struct RandomHvEntry {
    seed_utf8: String,
    dim: usize,
    sha256: String,
}

#[derive(Debug, Deserialize)]
struct EncodeStringEntry {
    input: String,
    dim: usize,
    sha256: String,
}

#[derive(Debug, Deserialize)]
struct SimilarityPair {
    a_seed: String,
    b_seed: String,
    dim: usize,
    expected: f64,
}

#[derive(Debug, Deserialize)]
struct BindRoundTrip {
    a_seed: String,
    b_seed: String,
    dim: usize,
    expected_similarity_to_a: f64,
}

#[derive(Debug, Deserialize)]
struct BundleEntry {
    seeds: Vec<String>,
    dim: usize,
    sha256: String,
}

#[derive(Debug, Deserialize)]
struct TextEntry {
    text: String,
    dim: usize,
    sha256: String,
}

#[derive(Debug, Deserialize)]
struct TextNgramsEntry {
    text: String,
    dim: usize,
    n: usize,
    sha256: String,
}

fn load_golden() -> Golden {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("tests/conformance/golden.json");
    let raw = fs::read_to_string(&path).expect("read golden.json");
    serde_json::from_str(&raw).expect("parse golden.json")
}

fn hex_sha256(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

#[test]
fn conformance_spec_version() {
    let g = load_golden();
    assert_eq!(SPEC_VERSION, g.spec_version);
}

#[test]
fn conformance_random_hv() {
    let g = load_golden();
    for r in &g.random_hv {
        let hv = random_hv(r.seed_utf8.as_bytes(), r.dim);
        assert_eq!(hex_sha256(&hv), r.sha256, "randomHv({:?}, {})", r.seed_utf8, r.dim);
    }
}

#[test]
fn conformance_encode_string() {
    let g = load_golden();
    for r in &g.encode_string {
        let hv = encode_string(&r.input, r.dim);
        assert_eq!(hex_sha256(&hv), r.sha256, "encodeString({:?}, {})", r.input, r.dim);
    }
}

#[test]
fn conformance_similarity_pairs() {
    let g = load_golden();
    for r in &g.similarity_pairs {
        let a = random_hv(r.a_seed.as_bytes(), r.dim);
        let b = random_hv(r.b_seed.as_bytes(), r.dim);
        let s = similarity(&a, &b);
        assert!(
            (s - r.expected).abs() < 1e-9,
            "sim({:?}, {:?}) = {}, expected {}",
            r.a_seed,
            r.b_seed,
            s,
            r.expected
        );
    }
}

#[test]
fn conformance_bind_round_trip() {
    let g = load_golden();
    for r in &g.bind_round_trip {
        let a = encode_string(&r.a_seed, r.dim);
        let b = encode_string(&r.b_seed, r.dim);
        let round = bind(&bind(&a, &b), &b);
        let s = similarity(&round, &a);
        assert!(
            (s - r.expected_similarity_to_a).abs() < 1e-9,
            "bind round-trip sim {} != {}",
            s,
            r.expected_similarity_to_a
        );
    }
}

#[test]
fn conformance_bundle() {
    let g = load_golden();
    for r in &g.bundle {
        let hvs: Vec<_> = r.seeds.iter().map(|s| random_hv(s.as_bytes(), r.dim)).collect();
        let refs: Vec<&Vec<u8>> = hvs.iter().collect();
        let out = bundle(&refs);
        assert_eq!(hex_sha256(&out), r.sha256, "bundle({:?})", r.seeds);
    }
}

#[test]
fn conformance_bag_of_words() {
    let g = load_golden();
    let opts = TextEncodingOptions::default();
    for r in &g.text_bag_of_words {
        let hv = encode_bag_of_words(&r.text, r.dim, opts);
        assert_eq!(hex_sha256(&hv), r.sha256, "encodeBagOfWords({:?})", r.text);
    }
}

#[test]
fn conformance_char_ngrams() {
    let g = load_golden();
    for r in &g.text_char_ngrams {
        let hv = encode_char_ngrams(&r.text, r.dim, r.n, true);
        assert_eq!(hex_sha256(&hv), r.sha256, "encodeCharNgrams({:?})", r.text);
    }
}
