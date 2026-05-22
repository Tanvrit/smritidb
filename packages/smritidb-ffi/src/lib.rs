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
use std::sync::Arc;

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

/// FFI-facing error type. UniFFI lowers this to a Kotlin sealed class
/// (`SmritidbException`) and a Swift enum (`SmritidbError`) with associated
/// `message: String` values where applicable.
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
    #[error("io error: {message}")]
    Io { message: String },
    #[error("database error: {message}")]
    Database { message: String },
    #[error("corruption: {message}")]
    Corruption { message: String },
    #[error("encoding error: {message}")]
    Encoding { message: String },
    #[error("error: {message}")]
    Other { message: String },
}

impl From<core::PersistenceError> for SmritidbError {
    fn from(value: core::PersistenceError) -> Self {
        match value {
            core::PersistenceError::Io(err) => SmritidbError::Io {
                message: err.to_string(),
            },
            core::PersistenceError::Database(msg) => SmritidbError::Database { message: msg },
            core::PersistenceError::Corruption(msg) => SmritidbError::Corruption { message: msg },
            core::PersistenceError::Other(msg) => SmritidbError::Other { message: msg },
        }
    }
}

impl From<core::StoreError> for SmritidbError {
    fn from(value: core::StoreError) -> Self {
        match value {
            core::StoreError::DimensionMismatch { .. } => SmritidbError::DimensionMismatch,
            core::StoreError::ValueTooLarge { .. } => SmritidbError::ValueTooLarge,
            core::StoreError::NotFound(_) => SmritidbError::NotFound,
            core::StoreError::InvalidConfig(_) => SmritidbError::InvalidConfig,
            core::StoreError::CorruptSnapshot(msg) => SmritidbError::Corruption { message: msg },
            core::StoreError::Persistence(err) => SmritidbError::from(err),
        }
    }
}

impl From<core::KmfError> for SmritidbError {
    fn from(value: core::KmfError) -> Self {
        SmritidbError::Encoding {
            message: value.to_string(),
        }
    }
}

// ---- Match dictionary, shared by both Store and PersistentStore ----

pub struct Match {
    pub id: String,
    pub similarity: f64,
    pub value: Vec<u8>,
    pub tags: Vec<String>,
    pub access_count: u32,
}

// ---- legacy in-memory Store (kept for backward compatibility) ----
//
// This is a thin toy substrate maintained for parity with the original
// FFI surface. New consumers should prefer `PersistentStore`, which is
// backed by the real `smritidb_core::Store`.

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

// ---- PersistentStore — wraps smritidb_core::Store + a persistence adapter ----

/// Which persistence adapter to back a `PersistentStore` with.
///
/// UniFFI lowers this to a Kotlin `enum class` and a Swift `enum`.
#[derive(Debug, Clone, Copy)]
pub enum AdapterKind {
    Memory,
    File,
    Sqlite,
}

/// Bundled adapter configuration. `path` is required for `File` and
/// `Sqlite` adapters and ignored for `Memory`.
#[derive(Debug, Clone)]
pub struct PersistenceConfig {
    pub kind: AdapterKind,
    pub path: Option<String>,
}

/// Store-level configuration. All fields map to `smritidb_core::StoreConfig`;
/// callers that want defaults can pass `0` for any field and the host shim
/// will substitute the core default.
#[derive(Debug, Clone, Copy)]
pub struct StoreOptions {
    pub dimension: u32,
    pub value_cap_bytes: u32,
    pub default_top_k: u32,
    pub default_min_similarity: f64,
}

impl StoreOptions {
    fn to_core(&self) -> core::StoreConfig {
        let defaults = core::StoreConfig::default();
        core::StoreConfig {
            dimension: if self.dimension == 0 {
                defaults.dimension
            } else {
                self.dimension as usize
            },
            value_cap_bytes: if self.value_cap_bytes == 0 {
                defaults.value_cap_bytes
            } else {
                self.value_cap_bytes as usize
            },
            default_top_k: if self.default_top_k == 0 {
                defaults.default_top_k
            } else {
                self.default_top_k as usize
            },
            default_min_similarity: if self.default_min_similarity == 0.0 {
                defaults.default_min_similarity
            } else {
                self.default_min_similarity
            },
            consolidation: defaults.consolidation,
        }
    }
}

/// Persistent associative store. Wraps `smritidb_core::Store` and a
/// `PersistenceAdapter`; both are guarded behind interior mutexes so the
/// type can be shared across UniFFI's `Arc<Self>` handles.
pub struct PersistentStore {
    inner: Mutex<core::Store>,
    adapter: Arc<dyn core::PersistenceAdapter + Send + Sync>,
}

impl PersistentStore {
    /// SQLite-backed `PersistentStore`. The database is opened (or created)
    /// at `path`; any pre-existing snapshot is loaded and the WAL replayed.
    pub fn open_sqlite(path: String, config: StoreOptions) -> Result<Self, SmritidbError> {
        let adapter: Arc<dyn core::PersistenceAdapter + Send + Sync> =
            Arc::new(core::persist::SqliteAdapter::open(&path)?);
        Self::from_adapter(adapter, config)
    }

    /// Filesystem-backed `PersistentStore`. The snapshot lives at `path`,
    /// the WAL at `<path>.wal`.
    pub fn open_file(path: String, config: StoreOptions) -> Result<Self, SmritidbError> {
        let adapter: Arc<dyn core::PersistenceAdapter + Send + Sync> =
            Arc::new(core::persist::FileSystemAdapter::new(path));
        Self::from_adapter(adapter, config)
    }

    /// In-memory `PersistentStore`, backed by `MemoryAdapter`. Useful for
    /// tests and ephemeral workloads.
    pub fn open_memory(config: StoreOptions) -> Result<Self, SmritidbError> {
        let adapter: Arc<dyn core::PersistenceAdapter + Send + Sync> =
            Arc::new(core::persist::MemoryAdapter::new());
        Self::from_adapter(adapter, config)
    }

    /// Open from a `PersistenceConfig`. Equivalent to the per-adapter
    /// constructors above.
    pub fn open(
        persistence: PersistenceConfig,
        config: StoreOptions,
    ) -> Result<Self, SmritidbError> {
        match persistence.kind {
            AdapterKind::Memory => Self::open_memory(config),
            AdapterKind::File => {
                let path = persistence.path.ok_or(SmritidbError::InvalidConfig)?;
                Self::open_file(path, config)
            }
            AdapterKind::Sqlite => {
                let path = persistence.path.ok_or(SmritidbError::InvalidConfig)?;
                Self::open_sqlite(path, config)
            }
        }
    }

    fn from_adapter(
        adapter: Arc<dyn core::PersistenceAdapter + Send + Sync>,
        config: StoreOptions,
    ) -> Result<Self, SmritidbError> {
        let core_cfg = config.to_core();
        // `open_persistent_store` expects `Arc<dyn PersistenceAdapter>`; the
        // bound `Send + Sync` is enforced by the trait itself.
        let store = core::open_persistent_store(adapter.clone(), core_cfg)?;
        Ok(Self {
            inner: Mutex::new(store),
            adapter,
        })
    }

    pub fn put(
        &self,
        key: String,
        value: Vec<u8>,
        tags: Vec<String>,
        metadata: Option<String>,
    ) -> Result<String, SmritidbError> {
        let metadata_value = match metadata {
            Some(raw) => serde_json::from_str(&raw).map_err(|err| SmritidbError::Encoding {
                message: err.to_string(),
            })?,
            None => serde_json::Value::Null,
        };
        let mut store = self.inner.lock();
        let tag_refs: Vec<&str> = tags.iter().map(|s| s.as_str()).collect();
        let item = store.put_string(&key, &value, &tag_refs, metadata_value)?;
        Ok(item.id)
    }

    pub fn recall(
        &self,
        cue: String,
        top_k: u32,
        min_similarity: f64,
    ) -> Result<Vec<Match>, SmritidbError> {
        let mut store = self.inner.lock();
        let hits = store.recall_string(&cue, top_k as usize, min_similarity);
        Ok(hits
            .into_iter()
            .map(|m| Match {
                id: m.item.id,
                similarity: m.similarity,
                value: m.item.value,
                tags: m.item.tags,
                access_count: m.item.access_count,
            })
            .collect())
    }

    pub fn delete(&self, id: String) -> bool {
        self.inner.lock().delete(&id)
    }

    pub fn get(&self, id: String) -> Result<Match, SmritidbError> {
        let store = self.inner.lock();
        let item = store.get(&id)?;
        Ok(Match {
            id: item.id.clone(),
            similarity: 1.0,
            value: item.value.clone(),
            tags: item.tags.clone(),
            access_count: item.access_count,
        })
    }

    pub fn size(&self) -> Result<u32, SmritidbError> {
        Ok(self.inner.lock().size() as u32)
    }

    pub fn dimension(&self) -> Result<u32, SmritidbError> {
        Ok(self.inner.lock().dimension() as u32)
    }

    pub fn consolidate(&self) -> Result<u32, SmritidbError> {
        let report = self.inner.lock().consolidate(None);
        Ok(report.bits_flipped as u32)
    }

    pub fn persist(&self) -> Result<(), SmritidbError> {
        let store = self.inner.lock();
        core::persist_store(&*store, self.adapter.as_ref())?;
        Ok(())
    }

    pub fn close(&self) -> Result<(), SmritidbError> {
        self.adapter.close()?;
        Ok(())
    }
}
