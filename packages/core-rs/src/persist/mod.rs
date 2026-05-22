//! Persistence adapters for the Rust core.
//!
//! Per SPEC.md §6, a Smritidb implementation may persist its substrate via a
//! snapshot + write-ahead-log model. The TypeScript reference (`packages/core-ts/`)
//! ships a simpler snapshot-only contract (every write re-serialises the full
//! KMF blob); this Rust core extends that contract with an explicit WAL so
//! native consumers can amortise writes.
//!
//! All adapters MUST be safe to share across threads and MUST treat the
//! snapshot blob as opaque KMF bytes (see `crate::kmf`). The WAL stores
//! per-operation deltas in an implementation-defined encoding — for v0.1 the
//! deltas are themselves small KMF-shaped blobs, but the trait is agnostic.

use thiserror::Error;

pub mod fs;
pub mod memory;
#[cfg(feature = "persist-sqlite")]
pub mod sqlite;

pub use fs::FileSystemAdapter;
pub use memory::MemoryAdapter;
#[cfg(feature = "persist-sqlite")]
pub use sqlite::SqliteAdapter;

/// Errors surfaced by a `PersistenceAdapter`.
#[derive(Debug, Error)]
pub enum PersistenceError {
    /// Underlying I/O failed (file system, etc.).
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    /// Underlying database error (only present with the `persist-sqlite`
    /// feature). Stored as a string so non-sqlite builds still compile.
    #[error("database error: {0}")]
    Database(String),

    /// On-disk state was unreadable or violated invariants.
    #[error("corrupt snapshot: {0}")]
    Corruption(String),

    /// Catch-all for adapter-specific failures.
    #[error("adapter error: {0}")]
    Other(String),
}

#[cfg(feature = "persist-sqlite")]
impl From<rusqlite::Error> for PersistenceError {
    fn from(value: rusqlite::Error) -> Self {
        PersistenceError::Database(value.to_string())
    }
}

/// The persistence interface every adapter implements.
///
/// All methods take `&self` so adapters can be wrapped in `Arc` and shared.
/// Adapters that need interior state use `Mutex` or equivalent.
pub trait PersistenceAdapter: Send + Sync + std::fmt::Debug {
    /// Read the most recently written snapshot, or `None` if the adapter has
    /// never been written to.
    fn read_snapshot(&self) -> Result<Option<Vec<u8>>, PersistenceError>;

    /// Write a snapshot atomically. After this returns, any subsequent
    /// `read_snapshot` MUST yield exactly `bytes`.
    fn write_snapshot(&self, bytes: &[u8]) -> Result<(), PersistenceError>;

    /// Append a WAL entry. Order is preserved across calls.
    fn append_wal(&self, bytes: &[u8]) -> Result<(), PersistenceError>;

    /// Read the WAL entries in append order. Each entry is a single blob as
    /// passed to `append_wal`.
    fn read_wal(&self) -> Result<Vec<Vec<u8>>, PersistenceError>;

    /// Discard the WAL. Called after a successful snapshot has been written.
    fn truncate_wal(&self) -> Result<(), PersistenceError>;

    /// Release any underlying resources (close file handles, drop the SQLite
    /// connection, etc.). Idempotent.
    fn close(&self) -> Result<(), PersistenceError>;
}
