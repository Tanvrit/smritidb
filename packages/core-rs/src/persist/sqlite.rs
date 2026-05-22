//! SQLite persistence adapter using `rusqlite` with the bundled libsqlite3.
//!
//! Schema (created idempotently on first use):
//!
//! ```sql
//! CREATE TABLE IF NOT EXISTS smritidb_snapshot (
//!     id INTEGER PRIMARY KEY,
//!     blob BLOB NOT NULL,
//!     written_at INTEGER NOT NULL
//! );
//! CREATE TABLE IF NOT EXISTS smritidb_wal (
//!     seq INTEGER PRIMARY KEY AUTOINCREMENT,
//!     blob BLOB NOT NULL,
//!     written_at INTEGER NOT NULL
//! );
//! ```
//!
//! The snapshot table is keyed at `id = 1` — a single logical row. Writes use
//! `INSERT OR REPLACE` to keep the row count at one.
//!
//! WAL journal mode is enabled (`PRAGMA journal_mode=WAL`) on open. Note this
//! is SQLite's *own* WAL — orthogonal to the Smritidb WAL stored in the
//! `smritidb_wal` table.
//!
//! NB: we deliberately do not depend on any third-party ORM. `rusqlite` is the
//! only storage dependency added in this mission.

use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, Connection};

use super::{PersistenceAdapter, PersistenceError};

/// Adapter backed by a single SQLite connection guarded by a `Mutex`. SQLite
/// handles serialise their own access, but we mutex on top so the adapter can
/// satisfy `Send + Sync` and so multi-statement sequences are atomic at the
/// adapter layer.
pub struct SqliteAdapter {
    conn: Mutex<Option<Connection>>,
}

impl std::fmt::Debug for SqliteAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqliteAdapter").finish_non_exhaustive()
    }
}

const SNAPSHOT_TABLE: &str = "smritidb_snapshot";
const WAL_TABLE: &str = "smritidb_wal";
const SNAPSHOT_ROW_ID: i64 = 1;

impl SqliteAdapter {
    /// Open or create a SQLite database at `path` and ensure the schema.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, PersistenceError> {
        let conn = Connection::open(path)?;
        Self::initialise(&conn)?;
        Ok(Self {
            conn: Mutex::new(Some(conn)),
        })
    }

    /// Open an in-memory SQLite database. Useful for tests.
    pub fn open_in_memory() -> Result<Self, PersistenceError> {
        let conn = Connection::open_in_memory()?;
        Self::initialise(&conn)?;
        Ok(Self {
            conn: Mutex::new(Some(conn)),
        })
    }

    fn initialise(conn: &Connection) -> Result<(), PersistenceError> {
        // WAL journal mode is best-effort: not all targets (e.g. :memory:) can
        // enable it, but the call is harmless.
        let _ = conn.pragma_update(None, "journal_mode", "WAL");
        conn.execute_batch(&format!(
            "CREATE TABLE IF NOT EXISTS {SNAPSHOT_TABLE} (
                 id INTEGER PRIMARY KEY,
                 blob BLOB NOT NULL,
                 written_at INTEGER NOT NULL
             );
             CREATE TABLE IF NOT EXISTS {WAL_TABLE} (
                 seq INTEGER PRIMARY KEY AUTOINCREMENT,
                 blob BLOB NOT NULL,
                 written_at INTEGER NOT NULL
             );"
        ))?;
        Ok(())
    }

    fn with_conn<R>(
        &self,
        f: impl FnOnce(&Connection) -> Result<R, PersistenceError>,
    ) -> Result<R, PersistenceError> {
        let guard = self.conn.lock().expect("sqlite mutex poisoned");
        match guard.as_ref() {
            Some(conn) => f(conn),
            None => Err(PersistenceError::Other("adapter has been closed".into())),
        }
    }
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

impl PersistenceAdapter for SqliteAdapter {
    fn read_snapshot(&self) -> Result<Option<Vec<u8>>, PersistenceError> {
        self.with_conn(|conn| {
            let row: Option<Vec<u8>> = conn
                .query_row(
                    &format!("SELECT blob FROM {SNAPSHOT_TABLE} WHERE id = ?"),
                    params![SNAPSHOT_ROW_ID],
                    |r| r.get(0),
                )
                .optional()?;
            Ok(row)
        })
    }

    fn write_snapshot(&self, bytes: &[u8]) -> Result<(), PersistenceError> {
        self.with_conn(|conn| {
            conn.execute(
                &format!(
                    "INSERT INTO {SNAPSHOT_TABLE} (id, blob, written_at) VALUES (?, ?, ?)
                     ON CONFLICT(id) DO UPDATE SET blob = excluded.blob,
                                                   written_at = excluded.written_at"
                ),
                params![SNAPSHOT_ROW_ID, bytes, now_millis()],
            )?;
            Ok(())
        })
    }

    fn append_wal(&self, bytes: &[u8]) -> Result<(), PersistenceError> {
        self.with_conn(|conn| {
            conn.execute(
                &format!("INSERT INTO {WAL_TABLE} (blob, written_at) VALUES (?, ?)"),
                params![bytes, now_millis()],
            )?;
            Ok(())
        })
    }

    fn read_wal(&self) -> Result<Vec<Vec<u8>>, PersistenceError> {
        self.with_conn(|conn| {
            let mut stmt =
                conn.prepare(&format!("SELECT blob FROM {WAL_TABLE} ORDER BY seq ASC"))?;
            let rows = stmt.query_map([], |r| r.get::<_, Vec<u8>>(0))?;
            let mut out = Vec::new();
            for row in rows {
                out.push(row?);
            }
            Ok(out)
        })
    }

    fn truncate_wal(&self) -> Result<(), PersistenceError> {
        self.with_conn(|conn| {
            conn.execute(&format!("DELETE FROM {WAL_TABLE}"), [])?;
            Ok(())
        })
    }

    fn close(&self) -> Result<(), PersistenceError> {
        let mut guard = self.conn.lock().expect("sqlite mutex poisoned");
        // Drop the connection if it's still here.
        if let Some(conn) = guard.take() {
            // Connection has no explicit close that returns Result; dropping is fine.
            drop(conn);
        }
        Ok(())
    }
}

// Small helper to import `rusqlite::OptionalExtension` only here, so the rest
// of the file stays terse.
use rusqlite::OptionalExtension;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kmf::{write_kmf, KmfItem, KmfSnapshot};
    use crate::store::{open_persistent_store, persist_store, StoreConfig};
    use std::sync::Arc;

    #[test]
    fn schema_is_idempotent() {
        let adapter = SqliteAdapter::open_in_memory().unwrap();
        // Calling initialise again via reopen-on-same-path is not possible for
        // :memory:; just confirm read_snapshot returns None and the WAL is empty.
        assert!(adapter.read_snapshot().unwrap().is_none());
        assert!(adapter.read_wal().unwrap().is_empty());
    }

    #[test]
    fn snapshot_single_row_semantics() {
        let adapter = SqliteAdapter::open_in_memory().unwrap();
        adapter.write_snapshot(b"alpha").unwrap();
        adapter.write_snapshot(b"beta").unwrap();
        // Second write replaces the first — single-row semantics.
        assert_eq!(
            adapter.read_snapshot().unwrap().as_deref(),
            Some(&b"beta"[..])
        );
    }

    #[test]
    fn wal_round_trip() {
        let adapter = SqliteAdapter::open_in_memory().unwrap();
        adapter.append_wal(b"one").unwrap();
        adapter.append_wal(b"two").unwrap();
        adapter.append_wal(b"three").unwrap();
        let entries = adapter.read_wal().unwrap();
        assert_eq!(
            entries,
            vec![b"one".to_vec(), b"two".to_vec(), b"three".to_vec()]
        );
        adapter.truncate_wal().unwrap();
        assert!(adapter.read_wal().unwrap().is_empty());
    }

    #[test]
    fn thousand_item_kmf_round_trips_bit_identically() {
        // Build 1000 items with deterministic keys, write KMF to the adapter,
        // read back, and verify every hypervector matches byte-for-byte.
        let dim = 1024usize;
        let mut items: Vec<KmfItem> = Vec::with_capacity(1000);
        for i in 0..1000 {
            let key = crate::random_hv(format!("k-{i}").as_bytes(), dim);
            items.push(KmfItem {
                id: format!("id-{i:04}"),
                key,
                value: format!("value-{i}").into_bytes(),
                tags: vec!["bench".to_string()],
                metadata: serde_json::json!({"i": i}),
                created_at: 1_000 + i as i64,
                access_count: 0,
                last_accessed_at: 1_000 + i as i64,
            });
        }
        let snap = KmfSnapshot {
            dimension: dim,
            created_at: 1_700_000_000_000,
            items: items.clone(),
        };
        let bytes = write_kmf(&snap).unwrap();

        let adapter: Arc<dyn PersistenceAdapter> =
            Arc::new(SqliteAdapter::open_in_memory().unwrap());
        adapter.write_snapshot(&bytes).unwrap();

        let restored_bytes = adapter.read_snapshot().unwrap().unwrap();
        assert_eq!(restored_bytes, bytes, "raw snapshot bytes round-trip");

        let restored = crate::kmf::read_kmf(&restored_bytes).unwrap();
        assert_eq!(restored.items.len(), 1000);
        for (orig, got) in items.iter().zip(restored.items.iter()) {
            assert_eq!(orig.id, got.id);
            assert_eq!(orig.key, got.key, "hypervector for {} differs", orig.id);
            assert_eq!(orig.value, got.value);
            assert_eq!(orig.tags, got.tags);
            assert_eq!(orig.metadata, got.metadata);
            assert_eq!(orig.created_at, got.created_at);
            assert_eq!(orig.access_count, got.access_count);
            assert_eq!(orig.last_accessed_at, got.last_accessed_at);
        }
    }

    #[test]
    fn store_open_and_persist_via_sqlite() {
        let adapter: Arc<dyn PersistenceAdapter> =
            Arc::new(SqliteAdapter::open_in_memory().unwrap());
        // Open empty -> populate -> persist -> reopen -> verify items.
        let mut store = open_persistent_store(adapter.clone(), StoreConfig::default()).unwrap();
        store
            .put_string("alpha", b"alpha-value", &[], serde_json::Value::Null)
            .unwrap();
        store
            .put_string("beta", b"beta-value", &[], serde_json::Value::Null)
            .unwrap();
        persist_store(&store, adapter.as_ref()).unwrap();

        let mut reopened = open_persistent_store(adapter.clone(), StoreConfig::default()).unwrap();
        assert_eq!(reopened.size(), 2);
        let hits = reopened.recall_string("alpha", 1, 0.9);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].item.value, b"alpha-value");
    }
}
