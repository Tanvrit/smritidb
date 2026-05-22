//! Smritidb C ABI.
//!
//! A thin, stable `extern "C"` surface over `smritidb_core::Store` and the
//! persistence adapters. Consumed by the Go, Dart, and .NET bindings, which
//! cannot use UniFFI's Kotlin/Swift code generators directly.
//!
//! ## Threading
//!
//! `SmritidbStore` is internally serialised via a `Mutex`, so multiple
//! threads may concurrently invoke methods on the same handle. The error
//! buffer (`smritidb_last_error`) is thread-local.
//!
//! ## Ownership
//!
//! - All `out_*` pointers are owned by the caller after a successful call.
//! - Strings are NUL-terminated UTF-8.
//! - Byte buffers (`SmritidbBytes`) are heap-allocated; release via
//!   `smritidb_free_bytes`.
//! - `SmritidbMatch` arrays are released via `smritidb_free_matches`.
//!
//! ## Status codes
//!
//! Every fallible function returns one of the `SMRITIDB_STATUS_*` constants
//! defined below (mirrored as `SmritidbStatus` in `smritidb.h`). `0` is
//! always success; any non-zero value indicates failure and a diagnostic is
//! available via `smritidb_last_error`.

#![allow(clippy::missing_safety_doc)]

use std::cell::RefCell;
use std::ffi::{c_char, c_int, CStr, CString};
use std::ptr;
use std::sync::Mutex;

use smritidb_core as core;

// ---------------------------------------------------------------------------
// Status codes
// ---------------------------------------------------------------------------

/// Success.
pub const SMRITIDB_STATUS_OK: c_int = 0;
/// One or more arguments were invalid (null pointer, non-UTF8, etc.).
pub const SMRITIDB_STATUS_INVALID_ARG: c_int = 1;
/// Dimension or value size mismatch.
pub const SMRITIDB_STATUS_DIMENSION_MISMATCH: c_int = 2;
/// Value exceeds the configured cap.
pub const SMRITIDB_STATUS_VALUE_TOO_LARGE: c_int = 3;
/// Item not found.
pub const SMRITIDB_STATUS_NOT_FOUND: c_int = 4;
/// Persistence (I/O, database) error.
pub const SMRITIDB_STATUS_PERSISTENCE: c_int = 5;
/// Snapshot corruption.
pub const SMRITIDB_STATUS_CORRUPTION: c_int = 6;
/// Catch-all internal error.
pub const SMRITIDB_STATUS_INTERNAL: c_int = 7;

// ---------------------------------------------------------------------------
// Thread-local error buffer
// ---------------------------------------------------------------------------

thread_local! {
    static LAST_ERROR: RefCell<Option<CString>> = const { RefCell::new(None) };
}

fn set_last_error<S: Into<String>>(msg: S) {
    let msg = msg.into();
    let c = CString::new(msg).unwrap_or_else(|_| CString::new("error").unwrap());
    LAST_ERROR.with(|slot| *slot.borrow_mut() = Some(c));
}

fn clear_last_error() {
    LAST_ERROR.with(|slot| *slot.borrow_mut() = None);
}

/// Returns a pointer to a NUL-terminated UTF-8 string describing the most
/// recent error on the current thread, or NULL if there has been no error
/// since the last successful call. The pointer is valid until the next call
/// into smritidb from the same thread.
#[no_mangle]
pub extern "C" fn smritidb_last_error() -> *const c_char {
    LAST_ERROR.with(|slot| match slot.borrow().as_ref() {
        Some(s) => s.as_ptr(),
        None => ptr::null(),
    })
}

// ---------------------------------------------------------------------------
// Opaque store handle
// ---------------------------------------------------------------------------

/// Opaque persistent store handle. Allocated via `smritidb_open_*`; freed via
/// `smritidb_close`.
#[allow(non_camel_case_types)]
pub struct SmritidbStore {
    inner: Mutex<core::Store>,
    adapter: std::sync::Arc<dyn core::PersistenceAdapter + Send + Sync>,
}

// ---------------------------------------------------------------------------
// Public POD types
// ---------------------------------------------------------------------------

/// A heap-allocated byte buffer owned by the caller. Free via
/// `smritidb_free_bytes`.
#[repr(C)]
#[derive(Debug)]
pub struct SmritidbBytes {
    pub data: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl SmritidbBytes {
    fn from_vec(mut v: Vec<u8>) -> Self {
        v.shrink_to_fit();
        let len = v.len();
        let cap = v.capacity();
        let data = v.as_mut_ptr();
        std::mem::forget(v);
        Self { data, len, cap }
    }

}

/// A recall result. `id` is a NUL-terminated UTF-8 string; `value` is the raw
/// stored bytes. Both are owned by the caller; release the whole array via
/// `smritidb_free_matches`.
#[repr(C)]
#[derive(Debug)]
pub struct SmritidbMatch {
    pub id: *mut c_char,
    pub similarity: f64,
    pub value: SmritidbBytes,
    pub access_count: u32,
}

// ---------------------------------------------------------------------------
// Constructors
// ---------------------------------------------------------------------------

fn make_store(
    adapter: std::sync::Arc<dyn core::PersistenceAdapter + Send + Sync>,
    dimension: u32,
) -> Result<Box<SmritidbStore>, core::StoreError> {
    let mut config = core::StoreConfig::default();
    if dimension != 0 {
        config.dimension = dimension as usize;
    }
    let store = core::open_persistent_store(adapter.clone(), config)?;
    Ok(Box::new(SmritidbStore {
        inner: Mutex::new(store),
        adapter,
    }))
}

/// Open (or create) a SQLite-backed persistent store at `path`.
///
/// Returns NULL on failure; call `smritidb_last_error` for details. Pass
/// `dimension == 0` to use the core default (10_000).
#[no_mangle]
pub unsafe extern "C" fn smritidb_open_sqlite(
    path: *const c_char,
    dimension: u32,
) -> *mut SmritidbStore {
    clear_last_error();
    if path.is_null() {
        set_last_error("smritidb_open_sqlite: path is null");
        return ptr::null_mut();
    }
    let path = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("smritidb_open_sqlite: path is not valid UTF-8");
            return ptr::null_mut();
        }
    };
    let adapter = match core::persist::SqliteAdapter::open(path) {
        Ok(a) => std::sync::Arc::new(a) as std::sync::Arc<dyn core::PersistenceAdapter + Send + Sync>,
        Err(e) => {
            set_last_error(format!("sqlite open failed: {e}"));
            return ptr::null_mut();
        }
    };
    match make_store(adapter, dimension) {
        Ok(boxed) => Box::into_raw(boxed),
        Err(e) => {
            set_last_error(format!("store open failed: {e}"));
            ptr::null_mut()
        }
    }
}

/// Open an in-memory persistent store. Useful for tests.
#[no_mangle]
pub extern "C" fn smritidb_open_memory(dimension: u32) -> *mut SmritidbStore {
    clear_last_error();
    let adapter: std::sync::Arc<dyn core::PersistenceAdapter + Send + Sync> =
        std::sync::Arc::new(core::persist::MemoryAdapter::new());
    match make_store(adapter, dimension) {
        Ok(boxed) => Box::into_raw(boxed),
        Err(e) => {
            set_last_error(format!("store open failed: {e}"));
            ptr::null_mut()
        }
    }
}

// ---------------------------------------------------------------------------
// put / recall / size / close
// ---------------------------------------------------------------------------

fn status_for(err: &core::StoreError) -> c_int {
    match err {
        core::StoreError::DimensionMismatch { .. } => SMRITIDB_STATUS_DIMENSION_MISMATCH,
        core::StoreError::ValueTooLarge { .. } => SMRITIDB_STATUS_VALUE_TOO_LARGE,
        core::StoreError::NotFound(_) => SMRITIDB_STATUS_NOT_FOUND,
        core::StoreError::InvalidConfig(_) => SMRITIDB_STATUS_INVALID_ARG,
        core::StoreError::CorruptSnapshot(_) => SMRITIDB_STATUS_CORRUPTION,
        core::StoreError::Persistence(_) => SMRITIDB_STATUS_PERSISTENCE,
    }
}

/// Insert (or update) an item keyed by the UTF-8 string `key`. Returns
/// `SMRITIDB_STATUS_OK` on success and writes a heap-allocated NUL-terminated
/// id to `*out_id`. The caller MUST free `*out_id` via `smritidb_free_string`.
#[no_mangle]
pub unsafe extern "C" fn smritidb_put(
    store: *mut SmritidbStore,
    key: *const c_char,
    value: *const u8,
    value_len: usize,
    out_id: *mut *mut c_char,
) -> c_int {
    clear_last_error();
    if store.is_null() || key.is_null() || out_id.is_null() {
        set_last_error("smritidb_put: null argument");
        return SMRITIDB_STATUS_INVALID_ARG;
    }
    let store = &*store;
    let key = match CStr::from_ptr(key).to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("smritidb_put: key is not valid UTF-8");
            return SMRITIDB_STATUS_INVALID_ARG;
        }
    };
    let value_slice: &[u8] = if value.is_null() || value_len == 0 {
        &[]
    } else {
        std::slice::from_raw_parts(value, value_len)
    };

    let mut guard = match store.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            set_last_error("smritidb_put: store mutex poisoned");
            return SMRITIDB_STATUS_INTERNAL;
        }
    };
    match guard.put_string(key, value_slice, &[], serde_json::Value::Null) {
        Ok(item) => {
            let id_c = match CString::new(item.id) {
                Ok(c) => c,
                Err(_) => {
                    set_last_error("smritidb_put: id contained interior NUL");
                    return SMRITIDB_STATUS_INTERNAL;
                }
            };
            *out_id = id_c.into_raw();
            SMRITIDB_STATUS_OK
        }
        Err(e) => {
            let code = status_for(&e);
            set_last_error(format!("put failed: {e}"));
            code
        }
    }
}

/// Recall the top-K matches for the UTF-8 cue. Writes a heap-allocated array
/// of `SmritidbMatch` of length `*out_count` to `*out_matches`. Free via
/// `smritidb_free_matches(*out_matches, *out_count)`.
#[no_mangle]
pub unsafe extern "C" fn smritidb_recall(
    store: *mut SmritidbStore,
    cue: *const c_char,
    top_k: u32,
    min_similarity: f64,
    out_matches: *mut *mut SmritidbMatch,
    out_count: *mut usize,
) -> c_int {
    clear_last_error();
    if store.is_null() || cue.is_null() || out_matches.is_null() || out_count.is_null() {
        set_last_error("smritidb_recall: null argument");
        return SMRITIDB_STATUS_INVALID_ARG;
    }
    let store = &*store;
    let cue = match CStr::from_ptr(cue).to_str() {
        Ok(s) => s,
        Err(_) => {
            set_last_error("smritidb_recall: cue is not valid UTF-8");
            return SMRITIDB_STATUS_INVALID_ARG;
        }
    };
    let mut guard = match store.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            set_last_error("smritidb_recall: store mutex poisoned");
            return SMRITIDB_STATUS_INTERNAL;
        }
    };
    let hits = guard.recall_string(cue, top_k as usize, min_similarity);

    let mut out: Vec<SmritidbMatch> = Vec::with_capacity(hits.len());
    for m in hits {
        let id_c = match CString::new(m.item.id) {
            Ok(c) => c.into_raw(),
            Err(_) => {
                // Free anything we've already allocated.
                for m in out.drain(..) {
                    if !m.id.is_null() {
                        let _ = CString::from_raw(m.id);
                    }
                    if !m.value.data.is_null() {
                        let _ = Vec::from_raw_parts(m.value.data, m.value.len, m.value.cap);
                    }
                }
                set_last_error("smritidb_recall: item id contained interior NUL");
                return SMRITIDB_STATUS_INTERNAL;
            }
        };
        let value = SmritidbBytes::from_vec(m.item.value);
        out.push(SmritidbMatch {
            id: id_c,
            similarity: m.similarity,
            value,
            access_count: m.item.access_count,
        });
    }

    out.shrink_to_fit();
    let len = out.len();
    let ptr = out.as_mut_ptr();
    // The accompanying free fn rebuilds the Vec with the original capacity,
    // so we must ensure capacity == length (shrink_to_fit above).
    debug_assert_eq!(out.capacity(), len);
    std::mem::forget(out);
    *out_matches = ptr;
    *out_count = len;
    SMRITIDB_STATUS_OK
}

/// Number of items currently in the store.
#[no_mangle]
pub unsafe extern "C" fn smritidb_size(store: *const SmritidbStore) -> usize {
    if store.is_null() {
        return 0;
    }
    let store = &*store;
    match store.inner.lock() {
        Ok(g) => g.size(),
        Err(_) => 0,
    }
}

/// Hypervector dimension this store was opened with.
#[no_mangle]
pub unsafe extern "C" fn smritidb_dimension(store: *const SmritidbStore) -> u32 {
    if store.is_null() {
        return 0;
    }
    let store = &*store;
    match store.inner.lock() {
        Ok(g) => g.dimension() as u32,
        Err(_) => 0,
    }
}

/// Force-persist any pending in-memory state to the backing adapter.
#[no_mangle]
pub unsafe extern "C" fn smritidb_persist(store: *mut SmritidbStore) -> c_int {
    clear_last_error();
    if store.is_null() {
        set_last_error("smritidb_persist: null store");
        return SMRITIDB_STATUS_INVALID_ARG;
    }
    let store = &*store;
    let guard = match store.inner.lock() {
        Ok(g) => g,
        Err(_) => {
            set_last_error("smritidb_persist: store mutex poisoned");
            return SMRITIDB_STATUS_INTERNAL;
        }
    };
    match core::persist_store(&*guard, store.adapter.as_ref()) {
        Ok(()) => SMRITIDB_STATUS_OK,
        Err(e) => {
            set_last_error(format!("persist failed: {e}"));
            SMRITIDB_STATUS_PERSISTENCE
        }
    }
}

/// Close and free the store. The pointer MUST NOT be used afterwards.
#[no_mangle]
pub unsafe extern "C" fn smritidb_close(store: *mut SmritidbStore) {
    if store.is_null() {
        return;
    }
    let boxed = Box::from_raw(store);
    let _ = boxed.adapter.close();
    // boxed dropped here.
}

// ---------------------------------------------------------------------------
// Memory release helpers
// ---------------------------------------------------------------------------

/// Free a string returned via an `out_*` parameter.
#[no_mangle]
pub unsafe extern "C" fn smritidb_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}

/// Free a byte buffer returned via an `out_*` parameter or as part of a
/// `SmritidbMatch` (when peeled out separately).
#[no_mangle]
pub unsafe extern "C" fn smritidb_free_bytes(bytes: SmritidbBytes) {
    if bytes.data.is_null() {
        return;
    }
    let _ = Vec::from_raw_parts(bytes.data, bytes.len, bytes.cap);
}

/// Free an array of `SmritidbMatch` returned by `smritidb_recall`.
#[no_mangle]
pub unsafe extern "C" fn smritidb_free_matches(matches: *mut SmritidbMatch, count: usize) {
    if matches.is_null() || count == 0 {
        return;
    }
    let v = Vec::from_raw_parts(matches, count, count);
    for m in v {
        if !m.id.is_null() {
            let _ = CString::from_raw(m.id);
        }
        if !m.value.data.is_null() {
            let _ = Vec::from_raw_parts(m.value.data, m.value.len, m.value.cap);
        }
    }
}

// ---------------------------------------------------------------------------
// Spec version
// ---------------------------------------------------------------------------

/// Return a pointer to the static NUL-terminated spec version string. The
/// pointer is static and never freed.
#[no_mangle]
pub extern "C" fn smritidb_spec_version() -> *const c_char {
    // Build a static CStr at runtime by leaking a CString once. We keep it
    // in a static OnceLock so we only allocate once per process.
    static SPEC: std::sync::OnceLock<CString> = std::sync::OnceLock::new();
    SPEC.get_or_init(|| CString::new(core::SPEC_VERSION).expect("spec version has no NUL"))
        .as_ptr()
}

// ---------------------------------------------------------------------------
// Tests — exercise the C surface from Rust.
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn cstr(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    #[test]
    fn open_memory_put_recall_roundtrip() {
        unsafe {
            let store = smritidb_open_memory(0);
            assert!(!store.is_null());

            for (k, v) in [("alpha", b"a"), ("beta", b"b"), ("gamma", b"c")] {
                let key = cstr(k);
                let mut out_id: *mut c_char = ptr::null_mut();
                let rc = smritidb_put(store, key.as_ptr(), v.as_ptr(), v.len(), &mut out_id);
                assert_eq!(rc, SMRITIDB_STATUS_OK);
                assert!(!out_id.is_null());
                smritidb_free_string(out_id);
            }

            assert_eq!(smritidb_size(store), 3);

            let cue = cstr("alpha");
            let mut matches: *mut SmritidbMatch = ptr::null_mut();
            let mut count: usize = 0;
            let rc = smritidb_recall(store, cue.as_ptr(), 5, 0.0, &mut matches, &mut count);
            assert_eq!(rc, SMRITIDB_STATUS_OK);
            assert!(count >= 1, "expected at least one match");
            smritidb_free_matches(matches, count);

            smritidb_close(store);
        }
    }

    #[test]
    fn last_error_is_set_on_failure() {
        unsafe {
            // Passing a null path triggers INVALID_ARG.
            let s = smritidb_open_sqlite(ptr::null(), 0);
            assert!(s.is_null());
            let msg = smritidb_last_error();
            assert!(!msg.is_null());
        }
    }
}
