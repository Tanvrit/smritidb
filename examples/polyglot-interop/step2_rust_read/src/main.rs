//! Step 2 — Rust reader for the polyglot interop demo.
//!
//! Opens the SQLite file written by Step 1 (Python), restores the store via
//! `open_persistent_store`, asserts the item count and runs a recall.

use std::process::ExitCode;
use std::sync::Arc;

use smritidb_core::persist::{PersistenceAdapter, SqliteAdapter};
use smritidb_core::store::{open_persistent_store, StoreConfig};

const PATH: &str = "/tmp/smritidb-polyglot.db";
const EXPECTED_ITEMS: usize = 100;

fn main() -> ExitCode {
    let adapter: Arc<dyn PersistenceAdapter> = match SqliteAdapter::open(PATH) {
        Ok(a) => Arc::new(a),
        Err(e) => {
            eprintln!("FAIL: cannot open {PATH}: {e}");
            return ExitCode::from(1);
        }
    };

    let mut store = match open_persistent_store(adapter, StoreConfig::default()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("FAIL: cannot restore store: {e}");
            return ExitCode::from(1);
        }
    };

    if store.size() != EXPECTED_ITEMS {
        eprintln!(
            "FAIL: expected {EXPECTED_ITEMS} items, got {}",
            store.size()
        );
        return ExitCode::from(1);
    }

    // Recall the middle key. The Python writer inserted concept_50 -> "item value 50".
    let hits = store.recall_string("concept_50", 5, 0.5);
    if hits.is_empty() {
        eprintln!("FAIL: recall returned no hits");
        return ExitCode::from(1);
    }
    let top = &hits[0];
    if top.item.value != b"item value 50" {
        eprintln!(
            "FAIL: top hit value mismatch: got {:?}",
            String::from_utf8_lossy(&top.item.value)
        );
        return ExitCode::from(1);
    }
    if top.similarity < 0.99 {
        eprintln!("FAIL: top similarity {} < 0.99", top.similarity);
        return ExitCode::from(1);
    }

    println!(
        "PASS: Rust read {} items from {} (top-1 = '{}' @ sim={:.6})",
        store.size(),
        PATH,
        String::from_utf8_lossy(&top.item.value),
        top.similarity,
    );
    ExitCode::SUCCESS
}
