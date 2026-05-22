//! The Smritidb `Store` — Rust port of `packages/core-ts/src/store.ts`.
//!
//! In-memory by default; persisted via the `PersistenceAdapter` trait in
//! `crate::persist`. Items are addressed by hypervector keys (binary HDC) and
//! retrieved by Hamming similarity.

use std::collections::HashMap;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value as JsonValue;
use thiserror::Error;
use uuid::Uuid;

use crate::consolidate::{
    flag_cold_items, pull_closer, CoactivationTracker, ColdCandidate, ConsolidationConfig,
    ConsolidationReport,
};
use crate::encode::{encode_embedding, encode_string};
use crate::hypervector::{similarity, Hypervector};
use crate::kmf::{read_kmf, write_kmf, KmfError, KmfItem, KmfSnapshot};
use crate::persist::{PersistenceAdapter, PersistenceError};

const DEFAULT_DIMENSION: usize = 10_000;
const DEFAULT_VALUE_CAP: usize = 16 * 1024 * 1024;
const DEFAULT_TOP_K: usize = 10;
const DEFAULT_MIN_SIM: f64 = 0.5;

#[derive(Debug, Clone)]
pub struct StoreConfig {
    pub dimension: usize,
    pub value_cap_bytes: usize,
    pub default_top_k: usize,
    pub default_min_similarity: f64,
    pub consolidation: ConsolidationConfig,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            dimension: DEFAULT_DIMENSION,
            value_cap_bytes: DEFAULT_VALUE_CAP,
            default_top_k: DEFAULT_TOP_K,
            default_min_similarity: DEFAULT_MIN_SIM,
            consolidation: ConsolidationConfig::default(),
        }
    }
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },
    #[error("value too large: {size} bytes exceeds cap of {cap}")]
    ValueTooLarge { size: usize, cap: usize },
    #[error("item {0} not found")]
    NotFound(String),
    #[error("invalid config: {0}")]
    InvalidConfig(String),
    #[error("corrupt snapshot: {0}")]
    CorruptSnapshot(String),
    #[error("persistence error: {0}")]
    Persistence(#[from] PersistenceError),
}

impl From<KmfError> for StoreError {
    fn from(value: KmfError) -> Self {
        StoreError::CorruptSnapshot(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub key: Hypervector,
    pub value: Vec<u8>,
    pub tags: Vec<String>,
    pub metadata: JsonValue,
    pub created_at: i64,
    pub access_count: u32,
    pub last_accessed_at: i64,
    pub cold: bool,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub item: Item,
    pub similarity: f64,
}

#[derive(Debug)]
pub struct Store {
    config: StoreConfig,
    items: HashMap<String, Item>,
    tracker: CoactivationTracker,
    consolidation_generation: u64,
}

#[cfg(not(target_arch = "wasm32"))]
fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// `SystemTime::now()` is unimplemented on `wasm32-unknown-unknown` and
/// panics if called. The wasm build (which always has the `wasm`
/// feature on) reaches into the host JS runtime's `Date.now()` via
/// `js_sys`. On non-wasm builds we use the stdlib clock. The signature
/// stays identical so the rest of the module is target-agnostic.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn now_millis() -> i64 {
    // `Date.now()` returns an `f64` in milliseconds since the UNIX
    // epoch. The cast saturates on overflow (i.e. roughly year 292
    // million AD) — well within tolerances.
    js_sys::Date::now() as i64
}

// Belt-and-braces fallback: if someone tries to build for `wasm32` *without*
// the `wasm` feature, give them a friendlier message than a runtime panic.
#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
fn now_millis() -> i64 {
    // No clock available — return the epoch so the substrate is
    // observable but timestamps are useless.
    0
}

impl Store {
    /// Create a fresh in-memory store.
    pub fn new(config: StoreConfig) -> Result<Self, StoreError> {
        if config.dimension < 1024 {
            return Err(StoreError::InvalidConfig(format!(
                "dimension must be >= 1024 (got {})",
                config.dimension
            )));
        }
        let tracker = CoactivationTracker::new(config.consolidation.window_size);
        Ok(Self {
            config,
            items: HashMap::new(),
            tracker,
            consolidation_generation: 0,
        })
    }

    pub fn dimension(&self) -> usize {
        self.config.dimension
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn config(&self) -> &StoreConfig {
        &self.config
    }

    /// Put an item keyed by a precomputed hypervector.
    pub fn put(
        &mut self,
        key: Hypervector,
        value: Vec<u8>,
        tags: Vec<String>,
        metadata: JsonValue,
        id: Option<String>,
    ) -> Result<Item, StoreError> {
        if key.len() != self.config.dimension {
            return Err(StoreError::DimensionMismatch {
                expected: self.config.dimension,
                got: key.len(),
            });
        }
        if value.len() > self.config.value_cap_bytes {
            return Err(StoreError::ValueTooLarge {
                size: value.len(),
                cap: self.config.value_cap_bytes,
            });
        }
        let now = now_millis();
        let resolved_id = id.unwrap_or_else(|| Uuid::now_v7().to_string());
        let existing = self.items.get(&resolved_id);
        let item = Item {
            id: resolved_id.clone(),
            key,
            value,
            tags,
            metadata,
            created_at: existing.map(|e| e.created_at).unwrap_or(now),
            access_count: existing.map(|e| e.access_count).unwrap_or(0),
            last_accessed_at: now,
            cold: false,
        };
        self.items.insert(resolved_id, item.clone());
        Ok(item)
    }

    /// Convenience: put a string-keyed item. The string is encoded per
    /// SPEC §3.5.
    pub fn put_string(
        &mut self,
        key: &str,
        value: &[u8],
        tags: &[&str],
        metadata: JsonValue,
    ) -> Result<Item, StoreError> {
        let hv = encode_string(key, self.config.dimension);
        self.put(
            hv,
            value.to_vec(),
            tags.iter().map(|s| s.to_string()).collect(),
            metadata,
            None,
        )
    }

    /// Convenience: put an embedding-keyed item. The embedding is encoded per
    /// SPEC §3.6 / appendix A.1.
    pub fn put_embedding(
        &mut self,
        embedding: &[f32],
        value: &[u8],
        tags: &[&str],
        metadata: JsonValue,
    ) -> Result<Item, StoreError> {
        let hv = encode_embedding(embedding, self.config.dimension);
        self.put(
            hv,
            value.to_vec(),
            tags.iter().map(|s| s.to_string()).collect(),
            metadata,
            None,
        )
    }

    /// Linear-scan recall — returns top-k items above `min_sim`.
    pub fn recall(&mut self, cue: &Hypervector, top_k: usize, min_sim: f64) -> Vec<Match> {
        assert_eq!(cue.len(), self.config.dimension, "cue dimension mismatch");
        let mut hits: Vec<(String, f64)> = Vec::new();
        for item in self.items.values() {
            let s = similarity(&item.key, cue);
            if s >= min_sim {
                hits.push((item.id.clone(), s));
            }
        }
        // Sort: similarity desc; tie-break id asc, per SPEC §4.2.
        hits.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.0.cmp(&b.0))
        });
        hits.truncate(top_k);

        let now = now_millis();
        let mut out = Vec::with_capacity(hits.len());
        let mut activated: Vec<String> = Vec::with_capacity(hits.len());
        for (id, s) in hits {
            if let Some(item) = self.items.get_mut(&id) {
                item.access_count = item.access_count.saturating_add(1);
                item.last_accessed_at = now;
                item.cold = false;
                activated.push(item.id.clone());
                out.push(Match {
                    item: item.clone(),
                    similarity: s,
                });
            }
        }
        if out.len() > 1 {
            self.tracker.record(&activated);
        }
        out
    }

    /// Convenience: recall by string cue.
    pub fn recall_string(&mut self, cue: &str, top_k: usize, min_sim: f64) -> Vec<Match> {
        let hv = encode_string(cue, self.config.dimension);
        self.recall(&hv, top_k, min_sim)
    }

    pub fn get(&self, id: &str) -> Result<&Item, StoreError> {
        self.items
            .get(id)
            .ok_or_else(|| StoreError::NotFound(id.to_string()))
    }

    pub fn delete(&mut self, id: &str) -> bool {
        self.items.remove(id).is_some()
    }

    /// Run a consolidation pass. Returns a report; idempotent on a substrate
    /// with no recent activity.
    pub fn consolidate(&mut self, now: Option<i64>) -> ConsolidationReport {
        let now = now.unwrap_or_else(now_millis);
        let pairs = self
            .tracker
            .pairs_at_or_above(self.config.consolidation.pull_threshold);

        let mut bits_flipped = 0usize;
        let mut pairs_pulled = 0usize;
        for (a, b, _count) in pairs {
            // Fetch both keys before mutating either to avoid double-borrow.
            let (key_a, key_b) = match (self.items.get(&a), self.items.get(&b)) {
                (Some(ia), Some(ib)) => (ia.key.clone(), ib.key.clone()),
                _ => continue,
            };
            self.consolidation_generation += 1;
            let (a_new, b_new, flipped) = pull_closer(
                &key_a,
                &key_b,
                self.config.consolidation.max_sim_delta,
                self.consolidation_generation,
            );
            if let Some(item) = self.items.get_mut(&a) {
                item.key = a_new;
            }
            if let Some(item) = self.items.get_mut(&b) {
                item.key = b_new;
            }
            bits_flipped += flipped;
            pairs_pulled += 1;
        }

        let candidates: Vec<ColdCandidate> = self
            .items
            .values()
            .map(|it| ColdCandidate {
                id: it.id.clone(),
                access_count: it.access_count,
                last_accessed_at: it.last_accessed_at,
            })
            .collect();
        let cold_ids = flag_cold_items(&candidates, &self.config.consolidation, now);
        for id in &cold_ids {
            if let Some(item) = self.items.get_mut(id) {
                item.cold = true;
            }
        }

        ConsolidationReport {
            pairs_pulled,
            bits_flipped,
            cold_items_flagged: cold_ids.len(),
        }
    }

    /// Serialise the current substrate to a KMF byte vector.
    pub fn snapshot(&self) -> Result<Vec<u8>, StoreError> {
        let mut items: Vec<&Item> = self.items.values().collect();
        // Stable ordering by id keeps repeated snapshots byte-identical
        // (important for diffing and content-addressing).
        items.sort_by(|a, b| a.id.cmp(&b.id));
        let kmf_items: Vec<KmfItem> = items
            .into_iter()
            .map(|it| KmfItem {
                id: it.id.clone(),
                key: it.key.clone(),
                value: it.value.clone(),
                tags: it.tags.clone(),
                metadata: it.metadata.clone(),
                created_at: it.created_at,
                access_count: it.access_count,
                last_accessed_at: it.last_accessed_at,
            })
            .collect();
        let snap = KmfSnapshot {
            dimension: self.config.dimension,
            created_at: now_millis(),
            items: kmf_items,
        };
        Ok(write_kmf(&snap)?)
    }

    /// Replace the current substrate from a KMF byte vector. `dimension` in
    /// the snapshot takes precedence over the configured dimension.
    pub fn restore(&mut self, bytes: &[u8]) -> Result<(), StoreError> {
        let snap = read_kmf(bytes)?;
        self.items.clear();
        for it in snap.items {
            let item = Item {
                id: it.id,
                key: it.key,
                value: it.value,
                tags: it.tags,
                metadata: it.metadata,
                created_at: it.created_at,
                access_count: it.access_count,
                last_accessed_at: it.last_accessed_at,
                cold: false,
            };
            self.items.insert(item.id.clone(), item);
        }
        self.config.dimension = snap.dimension;
        Ok(())
    }
}

// --- persistence helpers -------------------------------------------------

/// Open a store backed by an adapter. Loads the most recent snapshot, then
/// replays any WAL entries appended after it. Returns an in-memory `Store`;
/// callers persist explicitly via `persist_store`.
pub fn open_persistent_store(
    adapter: Arc<dyn PersistenceAdapter>,
    config: StoreConfig,
) -> Result<Store, StoreError> {
    let mut store = Store::new(config)?;
    if let Some(bytes) = adapter.read_snapshot()? {
        store.restore(&bytes)?;
    }
    // Each WAL entry is itself a KMF blob holding the *delta* substrate;
    // applying it just unions its items into the live store (last write
    // wins on id collision).
    let wal = adapter.read_wal()?;
    for blob in wal {
        let snap = read_kmf(&blob)?;
        for it in snap.items {
            store.items.insert(
                it.id.clone(),
                Item {
                    id: it.id,
                    key: it.key,
                    value: it.value,
                    tags: it.tags,
                    metadata: it.metadata,
                    created_at: it.created_at,
                    access_count: it.access_count,
                    last_accessed_at: it.last_accessed_at,
                    cold: false,
                },
            );
        }
    }
    Ok(store)
}

/// Snapshot the store and write the bytes through the adapter. Truncates the
/// WAL since the snapshot supersedes it.
pub fn persist_store(store: &Store, adapter: &dyn PersistenceAdapter) -> Result<(), StoreError> {
    let bytes = store.snapshot()?;
    adapter.write_snapshot(&bytes)?;
    adapter.truncate_wal()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persist::MemoryAdapter;

    #[test]
    fn put_and_recall_string() {
        let mut store = Store::new(StoreConfig::default()).unwrap();
        store
            .put_string("alpha", b"alpha-val", &["greek"], JsonValue::Null)
            .unwrap();
        store
            .put_string("beta", b"beta-val", &["greek"], JsonValue::Null)
            .unwrap();
        store
            .put_string("one", b"one-val", &["english"], JsonValue::Null)
            .unwrap();

        let hits = store.recall_string("alpha", 1, 0.9);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].item.value, b"alpha-val");
        assert!((hits[0].similarity - 1.0).abs() < 1e-9);
    }

    #[test]
    fn snapshot_round_trip_via_store() {
        let mut store = Store::new(StoreConfig {
            dimension: 2048,
            ..Default::default()
        })
        .unwrap();
        store
            .put_string("alpha", b"alpha", &["greek"], JsonValue::Null)
            .unwrap();
        store
            .put_string("beta", b"beta", &["greek"], JsonValue::Null)
            .unwrap();
        let bytes = store.snapshot().unwrap();
        let mut restored = Store::new(StoreConfig::default()).unwrap();
        restored.restore(&bytes).unwrap();
        assert_eq!(restored.size(), 2);
        let hits = restored.recall_string("alpha", 1, 0.9);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].item.value, b"alpha");
    }

    #[test]
    fn memory_adapter_persists_and_replays_wal() {
        let adapter: Arc<dyn PersistenceAdapter> = Arc::new(MemoryAdapter::new());
        let mut store = open_persistent_store(adapter.clone(), StoreConfig::default()).unwrap();
        let alpha = store
            .put_string("alpha", b"a", &[], JsonValue::Null)
            .unwrap();
        persist_store(&store, adapter.as_ref()).unwrap();

        // Add an item to the live store and replicate it into the WAL.
        let beta = store
            .put_string("beta", b"b", &[], JsonValue::Null)
            .unwrap();
        let wal_blob = {
            let mut tmp_store = Store::new(StoreConfig::default()).unwrap();
            tmp_store
                .put(
                    beta.key.clone(),
                    b"b".to_vec(),
                    vec![],
                    JsonValue::Null,
                    Some(beta.id.clone()),
                )
                .unwrap();
            tmp_store.snapshot().unwrap()
        };
        adapter.append_wal(&wal_blob).unwrap();

        let restored = open_persistent_store(adapter.clone(), StoreConfig::default()).unwrap();
        assert_eq!(restored.size(), 2);
        assert!(restored.get(&alpha.id).is_ok());
        assert!(restored.get(&beta.id).is_ok());
    }

    #[test]
    fn delete_and_get() {
        let mut store = Store::new(StoreConfig::default()).unwrap();
        let item = store
            .put_string("alpha", b"a", &[], JsonValue::Null)
            .unwrap();
        assert!(store.get(&item.id).is_ok());
        assert!(store.delete(&item.id));
        assert!(store.get(&item.id).is_err());
        assert!(!store.delete(&item.id));
    }

    #[test]
    fn rejects_undersized_dimension() {
        let err = Store::new(StoreConfig {
            dimension: 256,
            ..Default::default()
        })
        .unwrap_err();
        assert!(matches!(err, StoreError::InvalidConfig(_)));
    }

    #[test]
    fn rejects_oversized_value() {
        let mut store = Store::new(StoreConfig {
            value_cap_bytes: 16,
            ..Default::default()
        })
        .unwrap();
        let big = vec![0u8; 17];
        let err = store
            .put_string("k", &big, &[], JsonValue::Null)
            .unwrap_err();
        assert!(matches!(err, StoreError::ValueTooLarge { .. }));
    }
}
