//! Ephemeral in-memory adapter. Useful for tests and for ephemeral stores.

use std::sync::Mutex;

use super::{PersistenceAdapter, PersistenceError};

/// Adapter backed by two `Mutex`-guarded buffers — one for the snapshot, one
/// for the WAL entry list.
#[derive(Debug, Default)]
pub struct MemoryAdapter {
    snapshot: Mutex<Option<Vec<u8>>>,
    wal: Mutex<Vec<Vec<u8>>>,
}

impl MemoryAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pre-seed the adapter with a snapshot blob.
    pub fn with_snapshot(bytes: Vec<u8>) -> Self {
        Self {
            snapshot: Mutex::new(Some(bytes)),
            wal: Mutex::new(Vec::new()),
        }
    }
}

impl PersistenceAdapter for MemoryAdapter {
    fn read_snapshot(&self) -> Result<Option<Vec<u8>>, PersistenceError> {
        Ok(self
            .snapshot
            .lock()
            .expect("snapshot mutex poisoned")
            .clone())
    }

    fn write_snapshot(&self, bytes: &[u8]) -> Result<(), PersistenceError> {
        *self.snapshot.lock().expect("snapshot mutex poisoned") = Some(bytes.to_vec());
        Ok(())
    }

    fn append_wal(&self, bytes: &[u8]) -> Result<(), PersistenceError> {
        self.wal
            .lock()
            .expect("wal mutex poisoned")
            .push(bytes.to_vec());
        Ok(())
    }

    fn read_wal(&self) -> Result<Vec<Vec<u8>>, PersistenceError> {
        Ok(self.wal.lock().expect("wal mutex poisoned").clone())
    }

    fn truncate_wal(&self) -> Result<(), PersistenceError> {
        self.wal.lock().expect("wal mutex poisoned").clear();
        Ok(())
    }

    fn close(&self) -> Result<(), PersistenceError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_round_trip() {
        let adapter = MemoryAdapter::new();
        assert!(adapter.read_snapshot().unwrap().is_none());
        adapter.write_snapshot(b"hello").unwrap();
        assert_eq!(
            adapter.read_snapshot().unwrap().as_deref(),
            Some(&b"hello"[..])
        );
    }

    #[test]
    fn wal_round_trip() {
        let adapter = MemoryAdapter::new();
        adapter.append_wal(b"op1").unwrap();
        adapter.append_wal(b"op2").unwrap();
        let entries = adapter.read_wal().unwrap();
        assert_eq!(entries, vec![b"op1".to_vec(), b"op2".to_vec()]);
        adapter.truncate_wal().unwrap();
        assert!(adapter.read_wal().unwrap().is_empty());
    }
}
