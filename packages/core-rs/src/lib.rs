//! Smritidb core — binary hyperdimensional computing primitives.
//!
//! Reference implementation of the operations defined in `SPEC.md`
//! §1 (mathematical substrate) and §3 (canonical API), in Rust.
//!
//! Same semantics as the TypeScript reference; designed to be bit-exact
//! across both implementations so a snapshot written by one can be read by
//! the other.
//!
//! ## Features
//!
//! - `wasm`: enables `wasm-bindgen` glue for the browser. Native consumers
//!   leave this off.
//!
//! ## Spec version
//!
//! This crate currently targets SPEC.md v0.1.0-draft.

#![warn(missing_debug_implementations, rust_2018_idioms)]

pub mod encode;
pub mod hypervector;

#[cfg(feature = "wasm")]
pub mod wasm;

pub const SPEC_VERSION: &str = "0.1.0-draft";

pub use encode::{encode_embedding, encode_string};
pub use hypervector::{bind, bundle, permute, random_hv, similarity, Hypervector};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spec_version_advertised() {
        assert_eq!(SPEC_VERSION, "0.1.0-draft");
    }

    #[test]
    fn similarity_self_equals_one() {
        let a = random_hv(b"hello", 4096);
        assert!((similarity(&a, &a) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn similarity_random_pair_near_half() {
        let a = random_hv(b"a", 8192);
        let b = random_hv(b"b", 8192);
        let s = similarity(&a, &b);
        assert!(s > 0.45 && s < 0.55, "expected ~0.5, got {s}");
    }

    #[test]
    fn bind_self_inverse() {
        let a = random_hv(b"a", 4096);
        let b = random_hv(b"b", 4096);
        let round = bind(&bind(&a, &b), &b);
        assert!((similarity(&a, &round) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn random_hv_is_deterministic() {
        let a = random_hv(b"seed", 4096);
        let b = random_hv(b"seed", 4096);
        assert_eq!(a, b);
    }

    #[test]
    fn bundle_preserves_similarity_to_components() {
        let xs: Vec<_> = (0..5)
            .map(|i| random_hv(format!("x{i}").as_bytes(), 4096))
            .collect();
        let refs: Vec<&Hypervector> = xs.iter().collect();
        let bundled = bundle(&refs);
        for x in &xs {
            let s = similarity(&bundled, x);
            assert!(s > 0.6, "expected bundle similarity > 0.6 to component, got {s}");
        }
    }

    #[test]
    fn permute_is_invertible() {
        let a = random_hv(b"p", 4096);
        let round = permute(&permute(&a, 7), -7);
        assert!((similarity(&a, &round) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn compositional_recall() {
        let dim = 4096;
        let role = encode_string("role:name", dim);
        let alice = encode_string("filler:alice", dim);
        let bob = encode_string("filler:bob", dim);
        let carol = encode_string("filler:carol", dim);
        let record = bind(&role, &alice);
        let recovered = bind(&record, &role);
        let s_alice = similarity(&recovered, &alice);
        let s_bob = similarity(&recovered, &bob);
        let s_carol = similarity(&recovered, &carol);
        assert!(s_alice > 0.99, "alice should be ~1.0, got {s_alice}");
        assert!(s_bob < 0.55);
        assert!(s_carol < 0.55);
    }
}
