//! Python bindings for Smritidb. Wraps `smritidb-core` and exposes a flat
//! function surface plus a `Store` class that mirrors the canonical API
//! in SPEC.md §3. Built and packaged via maturin.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use pyo3::create_exception;
use pyo3::exceptions::{PyException, PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use smritidb_core as core;
use smritidb_core::PersistenceAdapter;

const DEFAULT_DIMENSION: usize = 10_000;
const DEFAULT_TOP_K: usize = 10;
const DEFAULT_MIN_SIM: f64 = 0.5;

// ---- module-level primitive functions ----

#[pyfunction]
#[pyo3(name = "random_hv")]
fn py_random_hv<'py>(py: Python<'py>, seed: &[u8], dim: usize) -> Bound<'py, PyBytes> {
    PyBytes::new_bound(py, &core::random_hv(seed, dim))
}

#[pyfunction]
#[pyo3(name = "similarity")]
fn py_similarity(a: &[u8], b: &[u8]) -> PyResult<f64> {
    if a.len() != b.len() {
        return Err(PyValueError::new_err(format!(
            "dimension mismatch: {} vs {}",
            a.len(),
            b.len()
        )));
    }
    Ok(core::similarity(a, b))
}

#[pyfunction]
#[pyo3(name = "bind")]
fn py_bind<'py>(py: Python<'py>, a: &[u8], b: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
    if a.len() != b.len() {
        return Err(PyValueError::new_err("dimension mismatch in bind"));
    }
    Ok(PyBytes::new_bound(py, &core::bind(a, b)))
}

#[pyfunction]
#[pyo3(name = "unbind")]
fn py_unbind<'py>(py: Python<'py>, a: &[u8], b: &[u8]) -> PyResult<Bound<'py, PyBytes>> {
    py_bind(py, a, b)
}

#[pyfunction]
#[pyo3(name = "bundle")]
fn py_bundle<'py>(py: Python<'py>, hvs: Vec<Vec<u8>>) -> PyResult<Bound<'py, PyBytes>> {
    if hvs.is_empty() {
        return Err(PyValueError::new_err("bundle requires at least one hypervector"));
    }
    let refs: Vec<&core::Hypervector> = hvs.iter().collect();
    Ok(PyBytes::new_bound(py, &core::bundle(&refs)))
}

#[pyfunction]
#[pyo3(name = "permute")]
fn py_permute<'py>(py: Python<'py>, hv: &[u8], k: i32) -> Bound<'py, PyBytes> {
    PyBytes::new_bound(py, &core::permute(hv, k))
}

#[pyfunction]
#[pyo3(name = "encode_string")]
fn py_encode_string<'py>(py: Python<'py>, s: &str, dim: usize) -> Bound<'py, PyBytes> {
    PyBytes::new_bound(py, &core::encode_string(s, dim))
}

#[pyfunction]
#[pyo3(name = "encode_embedding")]
fn py_encode_embedding<'py>(
    py: Python<'py>,
    embedding: Vec<f32>,
    dim: usize,
) -> Bound<'py, PyBytes> {
    PyBytes::new_bound(py, &core::encode_embedding(&embedding, dim))
}

// ---- Store class — SPEC §3 surface ----

#[derive(Clone)]
struct Item {
    id: String,
    key: Vec<u8>,
    value: Vec<u8>,
    tags: Vec<String>,
    created_at: f64,
    access_count: u64,
    last_accessed_at: f64,
}

#[pyclass]
struct Store {
    dimension: usize,
    items: HashMap<String, Item>,
    next_id: u64,
    default_top_k: usize,
    default_min_similarity: f64,
}

#[pymethods]
impl Store {
    #[new]
    #[pyo3(signature = (dimension = DEFAULT_DIMENSION, top_k = DEFAULT_TOP_K, min_similarity = DEFAULT_MIN_SIM))]
    fn new(dimension: usize, top_k: usize, min_similarity: f64) -> PyResult<Self> {
        if dimension < 1024 {
            return Err(PyValueError::new_err(format!(
                "dimension must be >= 1024 (got {dimension})"
            )));
        }
        Ok(Self {
            dimension,
            items: HashMap::new(),
            next_id: 0,
            default_top_k: top_k,
            default_min_similarity: min_similarity,
        })
    }

    #[pyo3(signature = (key, value, id = None, tags = None))]
    fn put(
        &mut self,
        key: PyObject,
        value: PyObject,
        id: Option<String>,
        tags: Option<Vec<String>>,
        py: Python<'_>,
    ) -> PyResult<String> {
        let hv = to_hypervector(py, key, self.dimension)?;
        let bytes = to_value_bytes(py, value)?;
        let now = now_ms();
        let resolved_id = if let Some(id) = id {
            id
        } else {
            self.next_id += 1;
            format!("item-{:020}", self.next_id)
        };
        let existing = self.items.get(&resolved_id).cloned();
        let item = Item {
            id: resolved_id.clone(),
            key: hv,
            value: bytes,
            tags: tags.unwrap_or_default(),
            created_at: existing.as_ref().map_or(now, |it| it.created_at),
            access_count: existing.as_ref().map_or(0, |it| it.access_count),
            last_accessed_at: now,
        };
        self.items.insert(resolved_id.clone(), item);
        Ok(resolved_id)
    }

    #[pyo3(signature = (cue, top_k = None, min_similarity = None))]
    fn recall<'py>(
        &mut self,
        py: Python<'py>,
        cue: PyObject,
        top_k: Option<usize>,
        min_similarity: Option<f64>,
    ) -> PyResult<Vec<Bound<'py, PyDict>>> {
        let cue_hv = to_hypervector(py, cue, self.dimension)?;
        let k = top_k.unwrap_or(self.default_top_k);
        let min_sim = min_similarity.unwrap_or(self.default_min_similarity);

        let mut hits: Vec<(String, f64)> = self
            .items
            .iter()
            .map(|(id, it)| (id.clone(), core::similarity(&it.key, &cue_hv)))
            .filter(|(_, s)| *s >= min_sim)
            .collect();
        hits.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.0.cmp(&b.0))
        });
        hits.truncate(k);

        let now = now_ms();
        let mut out = Vec::with_capacity(hits.len());
        for (id, sim) in hits {
            if let Some(item) = self.items.get_mut(&id) {
                item.access_count += 1;
                item.last_accessed_at = now;
            }
            let item = &self.items[&id];
            let d = PyDict::new_bound(py);
            d.set_item("id", item.id.clone())?;
            d.set_item("similarity", sim)?;
            d.set_item("value", PyBytes::new_bound(py, &item.value))?;
            d.set_item("tags", item.tags.clone())?;
            d.set_item("access_count", item.access_count)?;
            out.push(d);
        }
        Ok(out)
    }

    fn delete(&mut self, id: String) -> bool {
        self.items.remove(&id).is_some()
    }

    fn get<'py>(&self, py: Python<'py>, id: String) -> PyResult<Bound<'py, PyDict>> {
        let item = self
            .items
            .get(&id)
            .ok_or_else(|| PyKeyError::new_err(format!("item {id} not found")))?;
        let d = PyDict::new_bound(py);
        d.set_item("id", item.id.clone())?;
        d.set_item("value", PyBytes::new_bound(py, &item.value))?;
        d.set_item("tags", item.tags.clone())?;
        d.set_item("created_at", item.created_at)?;
        d.set_item("access_count", item.access_count)?;
        d.set_item("last_accessed_at", item.last_accessed_at)?;
        Ok(d)
    }

    fn size(&self) -> usize {
        self.items.len()
    }

    #[getter]
    fn spec_version(&self) -> &'static str {
        core::SPEC_VERSION
    }

    #[getter]
    fn dimension(&self) -> usize {
        self.dimension
    }
}

// ---- exceptions ----

create_exception!(
    _native,
    SmritidbError,
    PyException,
    "Base class for smritidb native errors."
);
create_exception!(
    _native,
    PersistenceError,
    SmritidbError,
    "Raised when the underlying persistence adapter fails."
);
create_exception!(
    _native,
    StoreError,
    SmritidbError,
    "Raised when a Store operation fails (dimension mismatch, value cap, etc.)."
);

fn map_store_err(err: core::StoreError) -> PyErr {
    match err {
        core::StoreError::NotFound(id) => PyKeyError::new_err(format!("item {id} not found")),
        core::StoreError::DimensionMismatch { .. }
        | core::StoreError::ValueTooLarge { .. }
        | core::StoreError::InvalidConfig(_) => StoreError::new_err(err.to_string()),
        core::StoreError::CorruptSnapshot(_) => StoreError::new_err(err.to_string()),
        core::StoreError::Persistence(inner) => PersistenceError::new_err(inner.to_string()),
    }
}

fn map_persistence_err(err: core::PersistenceError) -> PyErr {
    PersistenceError::new_err(err.to_string())
}

// ---- PersistentStore class ----

/// A Store backed by a `PersistenceAdapter` from the Rust core. Wraps the
/// adapter in an `Arc` and the `core::Store` in a `Mutex` so the same Python
/// object can be safely used from multiple threads (and so PyO3's `&self`
/// methods can mutate the underlying store).
#[pyclass(name = "PersistentStore", module = "smritidb._native")]
struct PyPersistentStore {
    inner: Mutex<Option<core::Store>>,
    adapter: Mutex<Option<Arc<dyn PersistenceAdapter>>>,
    dimension: usize,
}

impl PyPersistentStore {
    fn build(
        adapter: Arc<dyn PersistenceAdapter>,
        dimension: Option<usize>,
    ) -> PyResult<Self> {
        let mut config = core::StoreConfig::default();
        if let Some(d) = dimension {
            config.dimension = d;
        }
        // Open against the adapter — replays snapshot + WAL into the in-memory
        // store. The configured dimension wins only if the snapshot does not
        // override it (Store::restore mutates config.dimension from the blob).
        let store = core::open_persistent_store(adapter.clone(), config).map_err(map_store_err)?;
        let dim = store.dimension();
        Ok(Self {
            inner: Mutex::new(Some(store)),
            adapter: Mutex::new(Some(adapter)),
            dimension: dim,
        })
    }

    fn with_store<R>(&self, f: impl FnOnce(&mut core::Store) -> PyResult<R>) -> PyResult<R> {
        let mut guard = self
            .inner
            .lock()
            .map_err(|_| SmritidbError::new_err("store mutex poisoned"))?;
        let store = guard
            .as_mut()
            .ok_or_else(|| SmritidbError::new_err("store is closed"))?;
        f(store)
    }

}

#[pymethods]
impl PyPersistentStore {
    /// Open a SQLite-backed PersistentStore at `path`. Creates the database
    /// file and schema on first open. Pass `dimension` to override the default
    /// (10_000); ignored when an existing snapshot is replayed — the snapshot's
    /// own dimension wins.
    #[staticmethod]
    #[pyo3(signature = (path, dimension = None))]
    fn open_sqlite(path: String, dimension: Option<usize>) -> PyResult<Self> {
        let adapter = core::persist::SqliteAdapter::open(&path).map_err(map_persistence_err)?;
        Self::build(Arc::new(adapter), dimension)
    }

    /// Open a filesystem-backed PersistentStore at `path`. The snapshot is
    /// written to `<path>` and the WAL to `<path>.wal`.
    #[staticmethod]
    #[pyo3(signature = (path, dimension = None))]
    fn open_file(path: String, dimension: Option<usize>) -> PyResult<Self> {
        let adapter = core::persist::FileSystemAdapter::new(&path);
        Self::build(Arc::new(adapter), dimension)
    }

    /// Open an ephemeral in-memory PersistentStore. Useful for tests — the
    /// adapter is dropped when the store is closed.
    #[staticmethod]
    #[pyo3(signature = (dimension = None))]
    fn open_memory(dimension: Option<usize>) -> PyResult<Self> {
        let adapter = core::persist::MemoryAdapter::new();
        Self::build(Arc::new(adapter), dimension)
    }

    /// Insert or replace an item.
    #[pyo3(signature = (key, value, id = None, tags = None, metadata = None))]
    fn put(
        &self,
        py: Python<'_>,
        key: PyObject,
        value: PyObject,
        id: Option<String>,
        tags: Option<Vec<String>>,
        metadata: Option<PyObject>,
    ) -> PyResult<String> {
        let dim = self.dimension;
        let hv = to_hypervector(py, key, dim)?;
        let bytes = to_value_bytes(py, value)?;
        let tag_vec = tags.unwrap_or_default();
        let meta = match metadata {
            Some(obj) => py_to_json(py, obj)?,
            None => serde_json::Value::Null,
        };
        self.with_store(|store| {
            let item = store
                .put(hv, bytes, tag_vec, meta, id)
                .map_err(map_store_err)?;
            Ok(item.id)
        })
    }

    /// Recall the top-k items most similar to `cue`. Returns a list of dicts.
    #[pyo3(signature = (cue, top_k = None, min_similarity = None))]
    fn recall<'py>(
        &self,
        py: Python<'py>,
        cue: PyObject,
        top_k: Option<usize>,
        min_similarity: Option<f64>,
    ) -> PyResult<Vec<Bound<'py, PyDict>>> {
        let cue_hv = to_hypervector(py, cue, self.dimension)?;
        self.with_store(|store| {
            let k = top_k.unwrap_or(store.config().default_top_k);
            let min_sim = min_similarity.unwrap_or(store.config().default_min_similarity);
            let hits = store.recall(&cue_hv, k, min_sim);
            let mut out = Vec::with_capacity(hits.len());
            for m in hits {
                let d = PyDict::new_bound(py);
                d.set_item("id", m.item.id.clone())?;
                d.set_item("similarity", m.similarity)?;
                d.set_item("value", PyBytes::new_bound(py, &m.item.value))?;
                d.set_item("tags", m.item.tags.clone())?;
                d.set_item("access_count", m.item.access_count)?;
                out.push(d);
            }
            Ok(out)
        })
    }

    /// Delete an item by id. Returns True if removed.
    fn delete(&self, id: String) -> PyResult<bool> {
        self.with_store(|store| Ok(store.delete(&id)))
    }

    /// Fetch an item by id. Returns None if not found.
    fn get<'py>(&self, py: Python<'py>, id: String) -> PyResult<Option<Bound<'py, PyDict>>> {
        self.with_store(|store| match store.get(&id) {
            Ok(item) => {
                let d = PyDict::new_bound(py);
                d.set_item("id", item.id.clone())?;
                d.set_item("value", PyBytes::new_bound(py, &item.value))?;
                d.set_item("tags", item.tags.clone())?;
                d.set_item("created_at", item.created_at)?;
                d.set_item("access_count", item.access_count)?;
                d.set_item("last_accessed_at", item.last_accessed_at)?;
                Ok(Some(d))
            }
            Err(core::StoreError::NotFound(_)) => Ok(None),
            Err(err) => Err(map_store_err(err)),
        })
    }

    fn size(&self) -> PyResult<usize> {
        self.with_store(|store| Ok(store.size()))
    }

    #[getter]
    fn dimension(&self) -> usize {
        self.dimension
    }

    #[getter]
    fn spec_version(&self) -> &'static str {
        core::SPEC_VERSION
    }

    /// Run a consolidation pass over the live substrate. Returns a dict with
    /// the report fields (`pairs_pulled`, `bits_flipped`, `cold_items_flagged`).
    fn consolidate<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        self.with_store(|store| {
            let report = store.consolidate(None);
            let d = PyDict::new_bound(py);
            d.set_item("pairs_pulled", report.pairs_pulled)?;
            d.set_item("bits_flipped", report.bits_flipped)?;
            d.set_item("cold_items_flagged", report.cold_items_flagged)?;
            Ok(d)
        })
    }

    /// Snapshot the current substrate to the underlying adapter and truncate
    /// the WAL.
    fn persist(&self) -> PyResult<()> {
        // Hold both locks for the duration of the snapshot+truncate so we
        // observe a consistent view.
        let store_guard = self
            .inner
            .lock()
            .map_err(|_| SmritidbError::new_err("store mutex poisoned"))?;
        let adapter_guard = self
            .adapter
            .lock()
            .map_err(|_| SmritidbError::new_err("adapter mutex poisoned"))?;
        let store = store_guard
            .as_ref()
            .ok_or_else(|| SmritidbError::new_err("store is closed"))?;
        let adapter = adapter_guard
            .as_ref()
            .ok_or_else(|| SmritidbError::new_err("store is closed"))?;
        core::persist_store(store, adapter.as_ref()).map_err(map_store_err)
    }

    /// Close the underlying adapter and drop the in-memory store. Idempotent.
    /// After close, further calls raise SmritidbError.
    fn close(&self) -> PyResult<()> {
        // Drop the store first so any borrows release, then close the adapter.
        if let Ok(mut g) = self.inner.lock() {
            *g = None;
        }
        if let Ok(mut g) = self.adapter.lock() {
            if let Some(a) = g.take() {
                a.close().map_err(map_persistence_err)?;
            }
        }
        Ok(())
    }

    fn __enter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    #[pyo3(signature = (exc_type = None, exc_value = None, traceback = None))]
    fn __exit__(
        &self,
        exc_type: Option<PyObject>,
        exc_value: Option<PyObject>,
        traceback: Option<PyObject>,
    ) -> PyResult<bool> {
        let _ = (exc_type, exc_value, traceback);
        self.close()?;
        Ok(false)
    }
}

// ---- helpers ----

fn to_hypervector(py: Python<'_>, obj: PyObject, dim: usize) -> PyResult<Vec<u8>> {
    if let Ok(s) = obj.extract::<String>(py) {
        return Ok(core::encode_string(&s, dim));
    }
    if let Ok(b) = obj.downcast_bound::<PyBytes>(py) {
        let v = b.as_bytes().to_vec();
        if v.len() != dim {
            return Err(PyValueError::new_err(format!(
                "hypervector length {} does not match dimension {}",
                v.len(),
                dim
            )));
        }
        return Ok(v);
    }
    if let Ok(v) = obj.extract::<Vec<f32>>(py) {
        return Ok(core::encode_embedding(&v, dim));
    }
    Err(PyValueError::new_err(
        "cue must be a string, bytes (hypervector), or list of floats (embedding)",
    ))
}

fn to_value_bytes(py: Python<'_>, obj: PyObject) -> PyResult<Vec<u8>> {
    if let Ok(s) = obj.extract::<String>(py) {
        return Ok(s.into_bytes());
    }
    if let Ok(b) = obj.downcast_bound::<PyBytes>(py) {
        return Ok(b.as_bytes().to_vec());
    }
    if let Ok(v) = obj.extract::<Vec<u8>>(py) {
        return Ok(v);
    }
    Err(PyValueError::new_err(
        "value must be a string, bytes, or bytes-like sequence",
    ))
}

/// Convert a Python object into a `serde_json::Value`. We do this by round-
/// tripping through the standard library `json` module so we accept anything
/// Python's own `json.dumps` would accept (dicts, lists, primitives). Returning
/// `Null` for `None` keeps parity with the TS reference.
fn py_to_json(py: Python<'_>, obj: PyObject) -> PyResult<serde_json::Value> {
    if obj.is_none(py) {
        return Ok(serde_json::Value::Null);
    }
    let json_mod = py.import_bound("json")?;
    let s: String = json_mod.call_method1("dumps", (obj,))?.extract()?;
    serde_json::from_str(&s)
        .map_err(|e| PyValueError::new_err(format!("metadata must be JSON-serialisable: {e}")))
}

fn now_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

// ---- module entry point ----

#[pymodule]
fn _native(py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
    m.add("SPEC_VERSION", core::SPEC_VERSION)?;
    m.add_function(wrap_pyfunction!(py_random_hv, m)?)?;
    m.add_function(wrap_pyfunction!(py_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(py_bind, m)?)?;
    m.add_function(wrap_pyfunction!(py_unbind, m)?)?;
    m.add_function(wrap_pyfunction!(py_bundle, m)?)?;
    m.add_function(wrap_pyfunction!(py_permute, m)?)?;
    m.add_function(wrap_pyfunction!(py_encode_string, m)?)?;
    m.add_function(wrap_pyfunction!(py_encode_embedding, m)?)?;
    m.add_class::<Store>()?;
    m.add_class::<PyPersistentStore>()?;
    m.add("SmritidbError", py.get_type_bound::<SmritidbError>())?;
    m.add("PersistenceError", py.get_type_bound::<PersistenceError>())?;
    m.add("StoreError", py.get_type_bound::<StoreError>())?;
    Ok(())
}
