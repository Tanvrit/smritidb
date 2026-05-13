//! Python bindings for Smritidb. Wraps `smritidb-core` and exposes a flat
//! function surface plus a `Store` class that mirrors the canonical API
//! in SPEC.md §3. Built and packaged via maturin.

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use smritidb_core as core;

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
    Err(PyValueError::new_err("value must be a string or bytes"))
}

fn now_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

// ---- module entry point ----

#[pymodule]
fn _native(_py: Python<'_>, m: &Bound<PyModule>) -> PyResult<()> {
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
    Ok(())
}
