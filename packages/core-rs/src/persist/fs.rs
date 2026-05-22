//! Filesystem adapter — single KMF snapshot file plus an append-only WAL.
//!
//! Mirrors the behaviour of `packages/core-ts/src/adapters/fs.ts` for the
//! snapshot path (write to `<path>.tmp` then `rename` for POSIX-atomic
//! replacement). Adds a sibling `<path>.wal` log file for WAL entries — each
//! entry is preceded by an 8-byte LE length prefix so we can recover entry
//! boundaries on restart.

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use super::{PersistenceAdapter, PersistenceError};

/// Persists the snapshot at `path` and the WAL at `<path>.wal`.
#[derive(Debug)]
pub struct FileSystemAdapter {
    snapshot_path: PathBuf,
    tmp_path: PathBuf,
    wal_path: PathBuf,
    // Serialise WAL appends and snapshot writes so we don't race rename and
    // append from different threads.
    lock: Mutex<()>,
}

impl FileSystemAdapter {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let snapshot_path: PathBuf = path.into();
        let tmp_path = with_extension_suffix(&snapshot_path, "tmp");
        let wal_path = with_extension_suffix(&snapshot_path, "wal");
        Self {
            snapshot_path,
            tmp_path,
            wal_path,
            lock: Mutex::new(()),
        }
    }
}

fn with_extension_suffix(base: &Path, suffix: &str) -> PathBuf {
    // Append a literal `.<suffix>` to the path; we deliberately don't replace
    // existing extensions so `db.kmf` becomes `db.kmf.wal`, mirroring the TS
    // adapter's naming convention.
    let mut out = base.as_os_str().to_owned();
    out.push(".");
    out.push(suffix);
    PathBuf::from(out)
}

impl PersistenceAdapter for FileSystemAdapter {
    fn read_snapshot(&self) -> Result<Option<Vec<u8>>, PersistenceError> {
        let _guard = self.lock.lock().expect("fs mutex poisoned");
        match fs::read(&self.snapshot_path) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    fn write_snapshot(&self, bytes: &[u8]) -> Result<(), PersistenceError> {
        let _guard = self.lock.lock().expect("fs mutex poisoned");
        // Atomic-on-POSIX: write tmp, fsync, rename.
        if let Some(parent) = self.snapshot_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        {
            let mut f = File::create(&self.tmp_path)?;
            f.write_all(bytes)?;
            f.sync_all()?;
        }
        fs::rename(&self.tmp_path, &self.snapshot_path)?;
        Ok(())
    }

    fn append_wal(&self, bytes: &[u8]) -> Result<(), PersistenceError> {
        let _guard = self.lock.lock().expect("fs mutex poisoned");
        if let Some(parent) = self.wal_path.parent() {
            if !parent.as_os_str().is_empty() {
                fs::create_dir_all(parent)?;
            }
        }
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.wal_path)?;
        let len = bytes.len() as u64;
        f.write_all(&len.to_le_bytes())?;
        f.write_all(bytes)?;
        f.sync_all()?;
        Ok(())
    }

    fn read_wal(&self) -> Result<Vec<Vec<u8>>, PersistenceError> {
        let _guard = self.lock.lock().expect("fs mutex poisoned");
        let mut f = match File::open(&self.wal_path) {
            Ok(f) => f,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(err) => return Err(err.into()),
        };
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        decode_wal_blob(&buf)
    }

    fn truncate_wal(&self) -> Result<(), PersistenceError> {
        let _guard = self.lock.lock().expect("fs mutex poisoned");
        match fs::remove_file(&self.wal_path) {
            Ok(_) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    fn close(&self) -> Result<(), PersistenceError> {
        // Files are opened on-demand; nothing to close.
        Ok(())
    }
}

fn decode_wal_blob(buf: &[u8]) -> Result<Vec<Vec<u8>>, PersistenceError> {
    let mut entries = Vec::new();
    let mut cursor = 0usize;
    while cursor < buf.len() {
        if cursor + 8 > buf.len() {
            return Err(PersistenceError::Corruption(
                "WAL truncated mid-length-prefix".into(),
            ));
        }
        let mut len_bytes = [0u8; 8];
        len_bytes.copy_from_slice(&buf[cursor..cursor + 8]);
        let len = u64::from_le_bytes(len_bytes) as usize;
        cursor += 8;
        if cursor + len > buf.len() {
            return Err(PersistenceError::Corruption(
                "WAL truncated mid-entry".into(),
            ));
        }
        entries.push(buf[cursor..cursor + len].to_vec());
        cursor += len;
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn snapshot_round_trip_via_atomic_rename() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.kmf");
        let adapter = FileSystemAdapter::new(&path);

        assert!(adapter.read_snapshot().unwrap().is_none());
        adapter.write_snapshot(b"first snapshot").unwrap();
        assert_eq!(
            adapter.read_snapshot().unwrap().as_deref(),
            Some(&b"first snapshot"[..])
        );
        adapter.write_snapshot(b"second snapshot").unwrap();
        assert_eq!(
            adapter.read_snapshot().unwrap().as_deref(),
            Some(&b"second snapshot"[..])
        );
        assert!(!path.with_extension("kmf.tmp").exists());
    }

    #[test]
    fn wal_round_trip_with_length_prefixes() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.kmf");
        let adapter = FileSystemAdapter::new(&path);

        adapter.append_wal(b"first").unwrap();
        adapter.append_wal(b"second longer entry").unwrap();
        let entries = adapter.read_wal().unwrap();
        assert_eq!(
            entries,
            vec![b"first".to_vec(), b"second longer entry".to_vec()]
        );

        adapter.truncate_wal().unwrap();
        assert!(adapter.read_wal().unwrap().is_empty());
    }

    #[test]
    fn corrupt_wal_is_reported() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("store.kmf");
        let wal_path = dir.path().join("store.kmf.wal");
        std::fs::write(&wal_path, [0, 0, 0]).unwrap();
        let adapter = FileSystemAdapter::new(&path);
        let err = adapter.read_wal().unwrap_err();
        assert!(matches!(err, PersistenceError::Corruption(_)));
    }
}
