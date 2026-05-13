//! UniFFI binding crate. Generates Kotlin and Swift surfaces over the same
//! `smritidb-core` primitives the TypeScript and Python bindings use.
//!
//! Use `uniffi-bindgen generate` to emit foreign-language bindings:
//!
//! ```sh
//! cargo build --release
//! cargo run --bin uniffi-bindgen -- generate src/smritidb.udl \
//!     --language kotlin --out-dir bindings/kotlin
//! cargo run --bin uniffi-bindgen -- generate src/smritidb.udl \
//!     --language swift --out-dir bindings/swift
//! ```

use std::collections::HashMap;

use parking_lot::Mutex;
use smritidb_core as core;
use thiserror::Error;

uniffi::include_scaffolding!("smritidb");

// ---- module-level primitives ----

pub fn spec_version() -> String {
    core::SPEC_VERSION.to_string()
}

pub fn random_hv(seed: Vec<u8>, dim: u32) -> Vec<u8> {
    core::random_hv(&seed, dim as usize)
}

pub fn similarity(a: Vec<u8>, b: Vec<u8>) -> Result<f64, SmritidbError> {
    if a.len() != b.len() {
        return Err(SmritidbError::DimensionMismatch);
    }
    Ok(core::similarity(&a, &b))
}

pub fn bind(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, SmritidbError> {
    if a.len() != b.len() {
        return Err(SmritidbError::DimensionMismatch);
    }
    Ok(core::bind(&a, &b))
}

pub fn unbind(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, SmritidbError> {
    bind(a, b)
}

pub fn bundle(hvs: Vec<Vec<u8>>) -> Result<Vec<u8>, SmritidbError> {
    if hvs.is_empty() {
        return Err(SmritidbError::EmptyInput);
    }
    let refs: Vec<&core::Hypervector> = hvs.iter().collect();
    Ok(core::bundle(&refs))
}

pub fn permute(hv: Vec<u8>, k: i32) -> Vec<u8> {
    core::permute(&hv, k)
}

pub fn encode_string(s: String, dim: u32) -> Vec<u8> {
    core::encode_string(&s, dim as usize)
}

pub fn encode_embedding(embedding: Vec<f32>, dim: u32) -> Vec<u8> {
    core::encode_embedding(&embedding, dim as usize)
}

// ---- Error type ----

#[derive(Error, Debug)]
pub enum SmritidbError {
    #[error("dimension mismatch")]
    DimensionMismatch,
    #[error("invalid config")]
    InvalidConfig,
    #[error("value too large for the configured cap")]
    ValueTooLarge,
    #[error("item not found")]
    NotFound,
    #[error("empty input")]
    EmptyInput,
}

// ---- Store class ----

pub struct Match {
    pub id: String,
    pub similarity: f64,
    pub value: Vec<u8>,
    pub tags: Vec<String>,
    pub access_count: u32,
}

struct ItemState {
    key: Vec<u8>,
    value: Vec<u8>,
    tags: Vec<String>,
    access_count: u32,
}

struct StoreState {
    dimension: usize,
    items: HashMap<String, ItemState>,
    next_id: u64,
}

pub struct Store {
    inner: Mutex<StoreState>,
}

impl Store {
    pub fn new(dimension: u32) -> Result<Self, SmritidbError> {
        let dim = dimension as usize;
        if dim < 1024 {
            return Err(SmritidbError::InvalidConfig);
        }
        Ok(Self {
            inner: Mutex::new(StoreState {
                dimension: dim,
                items: HashMap::new(),
                next_id: 0,
            }),
        })
    }

    pub fn put(
        &self,
        key: String,
        value: Vec<u8>,
        tags: Vec<String>,
    ) -> Result<String, SmritidbError> {
        let mut state = self.inner.lock();
        let dim = state.dimension;
        let hv = core::encode_string(&key, dim);
        state.next_id += 1;
        let id = format!("item-{:020}", state.next_id);
        state.items.insert(
            id.clone(),
            ItemState {
                key: hv,
                value,
                tags,
                access_count: 0,
            },
        );
        Ok(id)
    }

    pub fn recall(
        &self,
        cue: String,
        top_k: u32,
        min_similarity: f64,
    ) -> Result<Vec<Match>, SmritidbError> {
        let mut state = self.inner.lock();
        let cue_hv = core::encode_string(&cue, state.dimension);

        let mut hits: Vec<(String, f64)> = state
            .items
            .iter()
            .map(|(id, it)| (id.clone(), core::similarity(&it.key, &cue_hv)))
            .filter(|(_, s)| *s >= min_similarity)
            .collect();
        hits.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.0.cmp(&b.0))
        });
        hits.truncate(top_k as usize);

        let mut out = Vec::with_capacity(hits.len());
        for (id, sim) in hits {
            if let Some(item) = state.items.get_mut(&id) {
                item.access_count += 1;
                out.push(Match {
                    id: id.clone(),
                    similarity: sim,
                    value: item.value.clone(),
                    tags: item.tags.clone(),
                    access_count: item.access_count,
                });
            }
        }
        Ok(out)
    }

    pub fn delete(&self, id: String) -> bool {
        self.inner.lock().items.remove(&id).is_some()
    }

    pub fn size(&self) -> u32 {
        self.inner.lock().items.len() as u32
    }

    pub fn dimension(&self) -> u32 {
        self.inner.lock().dimension as u32
    }

    pub fn spec_version(&self) -> String {
        core::SPEC_VERSION.to_string()
    }
}
