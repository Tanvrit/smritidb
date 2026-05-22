//! WebAssembly bindings — exposed only when the `wasm` feature is enabled.
//!
//! These wrap the pure Rust primitives in `hypervector` and `encode` so that
//! the TypeScript reference impl can drop in the wasm build as a transparent
//! 10–100× speedup on the hot paths while preserving the bit-exact semantics.
//!
//! ## Store handle protocol
//!
//! `wasm-bindgen` cannot ferry an opaque `&mut Store` across the JS / wasm
//! boundary safely — there's no Rust-side lifetime on the JS side, and the
//! closure-based approach `wasm-bindgen` uses for `&mut self` methods does
//! not compose well with a long-lived `Store` whose lifecycle outlives a
//! single JS call. We expose an integer **handle** API instead: each
//! `store_open_memory` returns a `u32` that JS holds; every other Store
//! method takes that handle as its first argument and we look the
//! corresponding `Store` up in a process-global `Mutex<HashMap<u32, Store>>`.
//!
//! This is safe on `wasm32-unknown-unknown` because the target is
//! single-threaded — the `Mutex` will never actually contend. We keep it
//! anyway because (a) it's the canonical Rust way to express interior
//! mutability across a `static`, and (b) it gives us a clean place to hang
//! a future threaded-wasm migration.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

use wasm_bindgen::prelude::*;

use crate::store::{Store, StoreConfig};
use crate::{
    bind as core_bind, bundle as core_bundle, encode_string as core_encode_string,
    hypervector::permute as core_permute, random_hv as core_random_hv,
    similarity as core_similarity,
};

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
    assert_eq!(
        hvs.len() % dim,
        0,
        "bundle: flat length must be a multiple of dim"
    );
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

// ---------------------------------------------------------------------------
// Store: handle-based API (see module docs for the rationale)
// ---------------------------------------------------------------------------

/// Shared registry of live in-memory `Store` instances, keyed by integer
/// handles. `wasm32-unknown-unknown` is single-threaded so the `Mutex` is
/// effectively a no-op; we keep it because `static mut` is unsound and
/// `RefCell` is not `Sync`.
fn stores() -> &'static Mutex<HashMap<u32, Store>> {
    static STORES: OnceLock<Mutex<HashMap<u32, Store>>> = OnceLock::new();
    STORES.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Monotonic handle allocator. Wraps at `u32::MAX`; in practice browsers
/// never open more than a handful of stores so wrap-around is academic.
static NEXT_HANDLE: AtomicU32 = AtomicU32::new(1);

fn allocate_handle() -> u32 {
    NEXT_HANDLE.fetch_add(1, Ordering::Relaxed)
}

/// Install a panic hook that surfaces Rust panics as `console.error`
/// messages instead of the opaque `RuntimeError: unreachable` wasm
/// otherwise raises. Idempotent — `console_error_panic_hook::set_once`
/// guards against repeated installation.
#[wasm_bindgen(start)]
pub fn _init_panic_hook() {
    console_error_panic_hook::set_once();
}

/// Open an in-memory store. Returns the integer handle the caller passes
/// to subsequent `store_*` functions. `dimension` defaults to 10 000 when
/// omitted (matches `StoreConfig::default()`).
#[wasm_bindgen(js_name = storeOpenMemory)]
pub fn store_open_memory(dimension: Option<u32>) -> Result<u32, JsError> {
    let mut config = StoreConfig::default();
    if let Some(d) = dimension {
        config.dimension = d as usize;
    }
    let store = Store::new(config).map_err(|e| JsError::new(&e.to_string()))?;
    let handle = allocate_handle();
    stores()
        .lock()
        .expect("stores mutex poisoned")
        .insert(handle, store);
    Ok(handle)
}

/// Insert (or replace) an item under a string cue. The cue is encoded to a
/// hypervector per SPEC §3.5. Returns the item's generated id.
#[wasm_bindgen(js_name = storePut)]
pub fn store_put(handle: u32, key: &str, value: &[u8]) -> Result<String, JsError> {
    let mut guard = stores().lock().expect("stores mutex poisoned");
    let store = guard
        .get_mut(&handle)
        .ok_or_else(|| JsError::new(&format!("invalid store handle: {handle}")))?;
    let item = store
        .put_string(key, value, &[], serde_json::Value::Null)
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(item.id)
}

/// Top-k recall by string cue. Returns a JSON-shaped array
/// (`[{id, similarity, value, tags, accessCount}, ...]`) — JS / Kotlin
/// lift it from a `JsValue`. We serialise rather than return a
/// `Vec<JsValue>` because `serde-wasm-bindgen` already handles the
/// binding generation cleanly and the result is almost always
/// immediately destructured on the JS side anyway.
///
/// `value` ships as a `Vec<u8>` (lands on JS as a `Uint8Array`) so the
/// common-test `Match.value` roundtrip works identically on JS, JVM,
/// and Native.
#[wasm_bindgen(js_name = storeRecall)]
pub fn store_recall(handle: u32, cue: &str, top_k: u32, min_sim: f64) -> Result<JsValue, JsError> {
    let mut guard = stores().lock().expect("stores mutex poisoned");
    let store = guard
        .get_mut(&handle)
        .ok_or_else(|| JsError::new(&format!("invalid store handle: {handle}")))?;
    let matches = store.recall_string(cue, top_k as usize, min_sim);
    // Project to a serialisable struct — `Match` holds `Item` which holds
    // a `serde_json::Value`; flat is friendlier to the JS consumer.
    #[derive(serde::Serialize)]
    struct WireMatch<'a> {
        id: &'a str,
        similarity: f64,
        value: &'a [u8],
        tags: &'a [String],
        #[serde(rename = "accessCount")]
        access_count: u32,
    }
    let wire: Vec<WireMatch<'_>> = matches
        .iter()
        .map(|m| WireMatch {
            id: &m.item.id,
            similarity: m.similarity,
            value: &m.item.value,
            tags: &m.item.tags,
            access_count: m.item.access_count,
        })
        .collect();
    serde_wasm_bindgen::to_value(&wire).map_err(|e| JsError::new(&e.to_string()))
}

/// Delete an item by id. Returns true iff a matching item existed.
#[wasm_bindgen(js_name = storeDelete)]
pub fn store_delete(handle: u32, id: &str) -> Result<bool, JsError> {
    let mut guard = stores().lock().expect("stores mutex poisoned");
    let store = guard
        .get_mut(&handle)
        .ok_or_else(|| JsError::new(&format!("invalid store handle: {handle}")))?;
    Ok(store.delete(id))
}

/// Total item count in the store.
#[wasm_bindgen(js_name = storeSize)]
pub fn store_size(handle: u32) -> Result<u32, JsError> {
    let guard = stores().lock().expect("stores mutex poisoned");
    let store = guard
        .get(&handle)
        .ok_or_else(|| JsError::new(&format!("invalid store handle: {handle}")))?;
    Ok(store.size() as u32)
}

/// Configured dimension.
#[wasm_bindgen(js_name = storeDimension)]
pub fn store_dimension(handle: u32) -> Result<u32, JsError> {
    let guard = stores().lock().expect("stores mutex poisoned");
    let store = guard
        .get(&handle)
        .ok_or_else(|| JsError::new(&format!("invalid store handle: {handle}")))?;
    Ok(store.dimension() as u32)
}

/// Run a consolidation pass. The in-memory adapter has no persistence
/// involvement, so the report is discarded.
#[wasm_bindgen(js_name = storeConsolidate)]
pub fn store_consolidate(handle: u32) -> Result<(), JsError> {
    let mut guard = stores().lock().expect("stores mutex poisoned");
    let store = guard
        .get_mut(&handle)
        .ok_or_else(|| JsError::new(&format!("invalid store handle: {handle}")))?;
    let _ = store.consolidate(None);
    Ok(())
}

/// Drop the store associated with `handle`. Idempotent — passing an
/// unknown handle is a no-op.
#[wasm_bindgen(js_name = storeClose)]
pub fn store_close(handle: u32) {
    let _ = stores()
        .lock()
        .expect("stores mutex poisoned")
        .remove(&handle);
}

/// Emit a KMF snapshot of the store's current substrate. Returns the
/// raw bytes that the browser-side persistence adapter (IndexedDB,
/// OPFS-sqlite-wasm, etc.) writes verbatim — the KMF format is the same
/// bit-identical wire format every Smritidb binding round-trips.
///
/// This is the Kotlin/JS side's hook for Phase D's IndexedDB adapter:
/// `persist()` calls `storeSnapshot(handle)` on the wasm boundary, then
/// writes the result via `IDBObjectStore.put({ id: 1, blob }, 1)`.
#[wasm_bindgen(js_name = storeSnapshot)]
pub fn store_snapshot(handle: u32) -> Result<Vec<u8>, JsError> {
    let guard = stores().lock().expect("stores mutex poisoned");
    let store = guard
        .get(&handle)
        .ok_or_else(|| JsError::new(&format!("invalid store handle: {handle}")))?;
    store
        .snapshot()
        .map_err(|e| JsError::new(&e.to_string()))
}

/// Replace the substrate of an existing store from a KMF snapshot,
/// preserving the same handle. Complement to `storeSnapshot`. The
/// snapshot's recorded dimension overrides the store's configured
/// dimension — same semantics as `Store::restore`.
///
/// Phase D usage: `openIndexedDb(dbName)` opens a fresh memory store,
/// reads the snapshot row out of IndexedDB, and pipes the bytes back in
/// via this entry point so the existing handle continues to work.
#[wasm_bindgen(js_name = storeRestoreFromSnapshot)]
pub fn store_restore_from_snapshot(handle: u32, bytes: &[u8]) -> Result<(), JsError> {
    let mut guard = stores().lock().expect("stores mutex poisoned");
    let store = guard
        .get_mut(&handle)
        .ok_or_else(|| JsError::new(&format!("invalid store handle: {handle}")))?;
    store
        .restore(bytes)
        .map_err(|e| JsError::new(&e.to_string()))
}
