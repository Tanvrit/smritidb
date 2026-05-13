//! WebAssembly bindings — exposed only when the `wasm` feature is enabled.
//!
//! These wrap the pure Rust primitives in `hypervector` and `encode` so that
//! the TypeScript reference impl can drop in the wasm build as a transparent
//! 10–100× speedup on the hot paths while preserving the bit-exact semantics.

use wasm_bindgen::prelude::*;

use crate::{bind as core_bind, bundle as core_bundle, encode_string as core_encode_string,
            hypervector::permute as core_permute, random_hv as core_random_hv,
            similarity as core_similarity};

#[wasm_bindgen(js_name = randomHv)]
pub fn random_hv(seed: &[u8], dim: usize) -> Vec<u8> {
    core_random_hv(seed, dim)
}

#[wasm_bindgen(js_name = bind)]
pub fn bind(a: &[u8], b: &[u8]) -> Vec<u8> {
    core_bind(a, b)
}

#[wasm_bindgen(js_name = unbind)]
pub fn unbind(a: &[u8], b: &[u8]) -> Vec<u8> {
    core_bind(a, b)
}

#[wasm_bindgen(js_name = similarity)]
pub fn similarity(a: &[u8], b: &[u8]) -> f64 {
    core_similarity(a, b)
}

#[wasm_bindgen(js_name = permute)]
pub fn permute(hv: &[u8], k: i32) -> Vec<u8> {
    core_permute(hv, k)
}

/// `hvs` is a flat `Uint8Array` where every `dim` consecutive bytes is one
/// hypervector — easier on the JS boundary than passing a `Vec<Uint8Array>`.
#[wasm_bindgen(js_name = bundle)]
pub fn bundle(hvs: &[u8], dim: usize) -> Vec<u8> {
    assert!(dim > 0);
    assert_eq!(hvs.len() % dim, 0, "bundle: flat length must be a multiple of dim");
    let count = hvs.len() / dim;
    let owned: Vec<Vec<u8>> = (0..count)
        .map(|i| hvs[i * dim..(i + 1) * dim].to_vec())
        .collect();
    let refs: Vec<&Vec<u8>> = owned.iter().collect();
    core_bundle(&refs)
}

#[wasm_bindgen(js_name = encodeString)]
pub fn encode_string(s: &str, dim: usize) -> Vec<u8> {
    core_encode_string(s, dim)
}

#[wasm_bindgen(js_name = specVersion)]
pub fn spec_version() -> String {
    crate::SPEC_VERSION.to_string()
}
