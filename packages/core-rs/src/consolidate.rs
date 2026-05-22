//! Hebbian consolidation per SPEC.md §5. Port of
//! `packages/core-ts/src/consolidate.ts`.
//!
//! All operations are deterministic given identical inputs — same `salt`,
//! same hypervectors -> same bit choices. This makes consolidation replayable
//! from a KMF snapshot + access log.

use std::collections::HashMap;

use crate::hypervector::Hypervector;

#[derive(Debug, Clone, Copy)]
pub struct ConsolidationConfig {
    pub window_size: usize,
    pub pull_threshold: u32,
    pub max_sim_delta: f64,
    pub cold_days: u32,
    pub cold_min_access: u32,
}

impl Default for ConsolidationConfig {
    fn default() -> Self {
        Self {
            window_size: 1000,
            pull_threshold: 32,
            max_sim_delta: 0.02,
            cold_days: 30,
            cold_min_access: 3,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ConsolidationReport {
    pub pairs_pulled: usize,
    pub bits_flipped: usize,
    pub cold_items_flagged: usize,
}

/// Tracks pairwise co-activation within a sliding window of recall batches.
#[derive(Debug, Default)]
pub struct CoactivationTracker {
    window: std::collections::VecDeque<Vec<String>>,
    pairs: HashMap<(String, String), u32>,
    size: usize,
}

impl CoactivationTracker {
    pub fn new(size: usize) -> Self {
        Self {
            window: std::collections::VecDeque::new(),
            pairs: HashMap::new(),
            size,
        }
    }

    pub fn record(&mut self, ids_in_batch: &[String]) {
        if ids_in_batch.len() < 2 {
            self.window.push_back(ids_in_batch.to_vec());
            self.trim();
            return;
        }
        let mut sorted = ids_in_batch.to_vec();
        sorted.sort();
        for i in 0..sorted.len() {
            for j in (i + 1)..sorted.len() {
                let key = (sorted[i].clone(), sorted[j].clone());
                *self.pairs.entry(key).or_insert(0) += 1;
            }
        }
        self.window.push_back(sorted);
        self.trim();
    }

    fn trim(&mut self) {
        while self.window.len() > self.size {
            let expired = self.window.pop_front().expect("window non-empty");
            if expired.len() < 2 {
                continue;
            }
            for i in 0..expired.len() {
                for j in (i + 1)..expired.len() {
                    let key = (expired[i].clone(), expired[j].clone());
                    if let Some(count) = self.pairs.get_mut(&key) {
                        if *count <= 1 {
                            self.pairs.remove(&key);
                        } else {
                            *count -= 1;
                        }
                    }
                }
            }
        }
    }

    /// Returns pairs with count `>= threshold`, sorted by descending count,
    /// then by ascending `a`, then by ascending `b` — same ordering as the TS
    /// reference.
    pub fn pairs_at_or_above(&self, threshold: u32) -> Vec<(String, String, u32)> {
        let mut out: Vec<(String, String, u32)> = self
            .pairs
            .iter()
            .filter(|(_, c)| **c >= threshold)
            .map(|((a, b), c)| (a.clone(), b.clone(), *c))
            .collect();
        out.sort_by(|x, y| {
            y.2.cmp(&x.2)
                .then_with(|| x.0.cmp(&y.0))
                .then_with(|| x.1.cmp(&y.1))
        });
        out
    }

    pub fn reset(&mut self) {
        self.window.clear();
        self.pairs.clear();
    }
}

/// Move two hypervectors closer in Hamming space by flipping a bounded number
/// of disagreeing bits. Deterministic given `(a, b, salt)`.
pub fn pull_closer(
    a: &Hypervector,
    b: &Hypervector,
    max_sim_delta: f64,
    salt: u64,
) -> (Hypervector, Hypervector, usize) {
    let d = a.len();
    assert_eq!(b.len(), d, "dimension mismatch in pull_closer");

    let mut disagree: Vec<usize> = (0..d).filter(|&i| a[i] != b[i]).collect();
    let max_flips = std::cmp::max(1, (max_sim_delta * d as f64).floor() as usize);
    let to_flip = std::cmp::min(disagree.len(), max_flips);

    // Derive a deterministic ordering over disagreeing bits — BLAKE3-XOF over
    // an 8-byte LE salt, dkLen = max(64, to_flip * 4). We don't strictly need
    // the buffer to grow with `to_flip` (indices are hashed modulo the buffer
    // length), but matching the TS code keeps the implementations aligned.
    let mut salt_buf = [0u8; 8];
    salt_buf.copy_from_slice(&salt.to_le_bytes());
    let dk_len = std::cmp::max(64usize, to_flip.saturating_mul(4));
    let mut digest = vec![0u8; dk_len];
    let mut hasher = blake3::Hasher::new();
    hasher.update(&salt_buf);
    hasher.finalize_xof().fill(&mut digest);

    disagree.sort_by(|x, y| {
        let dx = digest[x % digest.len()];
        let dy = digest[y % digest.len()];
        dx.cmp(&dy).then(x.cmp(y))
    });

    let mut a_out = a.clone();
    let mut b_out = b.clone();
    for (i, &bit) in disagree.iter().take(to_flip).enumerate() {
        if i % 2 == 0 {
            a_out[bit] = b[bit];
        } else {
            b_out[bit] = a[bit];
        }
    }
    (a_out, b_out, to_flip)
}

#[derive(Debug)]
pub struct ColdCandidate {
    pub id: String,
    pub access_count: u32,
    pub last_accessed_at: i64,
}

/// Flag items that are stale (older than `cold_days`) and rarely accessed
/// (fewer than `cold_min_access` hits).
pub fn flag_cold_items(
    items: &[ColdCandidate],
    config: &ConsolidationConfig,
    now: i64,
) -> Vec<String> {
    let cutoff_ms = (config.cold_days as i64) * 24 * 60 * 60 * 1000;
    let cutoff = now - cutoff_ms;
    items
        .iter()
        .filter(|it| it.access_count < config.cold_min_access && it.last_accessed_at < cutoff)
        .map(|it| it.id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hypervector::{random_hv, similarity};

    #[test]
    fn tracker_increments_pairs_and_decays() {
        let mut t = CoactivationTracker::new(2);
        t.record(&["a".into(), "b".into()]);
        t.record(&["a".into(), "b".into()]);
        let pairs = t.pairs_at_or_above(2);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], ("a".to_string(), "b".to_string(), 2));

        // Push two more batches to evict the originals.
        t.record(&["c".into(), "d".into()]);
        t.record(&["c".into(), "d".into()]);
        // (a, b) should now have count 0 and be gone.
        assert!(t
            .pairs_at_or_above(1)
            .iter()
            .all(|(a, b, _)| !(a == "a" && b == "b")));
    }

    #[test]
    fn pull_closer_reduces_distance() {
        let dim = 4096;
        let a = random_hv(b"alpha", dim);
        let b = random_hv(b"beta", dim);
        let before = similarity(&a, &b);
        let (a2, b2, flipped) = pull_closer(&a, &b, 0.02, 1);
        let after = similarity(&a2, &b2);
        assert!(
            after >= before,
            "similarity should not decrease: {before} -> {after}"
        );
        assert!(flipped > 0);
    }

    #[test]
    fn pull_closer_is_deterministic() {
        let dim = 1024;
        let a = random_hv(b"x", dim);
        let b = random_hv(b"y", dim);
        let (a1, b1, _) = pull_closer(&a, &b, 0.02, 7);
        let (a2, b2, _) = pull_closer(&a, &b, 0.02, 7);
        assert_eq!(a1, a2);
        assert_eq!(b1, b2);
    }

    #[test]
    fn flag_cold_items_uses_cutoff() {
        let cfg = ConsolidationConfig::default();
        let now = 100 * 24 * 60 * 60 * 1000;
        let items = vec![
            ColdCandidate {
                id: "old".into(),
                access_count: 0,
                last_accessed_at: 0,
            },
            ColdCandidate {
                id: "recent".into(),
                access_count: 0,
                last_accessed_at: now - 1,
            },
            ColdCandidate {
                id: "popular".into(),
                access_count: 100,
                last_accessed_at: 0,
            },
        ];
        let flagged = flag_cold_items(&items, &cfg, now);
        assert_eq!(flagged, vec!["old".to_string()]);
    }
}
