//! Binary hypervector primitives.
//!
//! In-memory representation: a `Vec<u8>` where each byte is 0 or 1 (unpacked).
//! This matches the TypeScript reference and keeps `bundle`/`similarity`
//! straight-line over bytes. The packed bit-array form lives in the KMF
//! wire format only.
//!
//! Future work (Phase 4): SIMD popcount over packed `u64` words for
//! `similarity`; SIMD vertical adds for `bundle`. The reference layout
//! optimises for clarity and cross-impl bit-exactness rather than throughput.

const TIEBREAKER_DOMAIN: &[u8] = b"smritidb/tiebreak";

/// A binary hypervector — `Vec<u8>` where each byte is 0 or 1.
pub type Hypervector = Vec<u8>;

/// Deterministic random binary hypervector from a seed.
///
/// Uses BLAKE3 in XOF mode to expand the seed to `ceil(dim/8)` bytes, then
/// unpacks MSB-first within each byte (per SPEC §1.1).
pub fn random_hv(seed: &[u8], dim: usize) -> Hypervector {
    let bytes = (dim + 7) / 8;
    let mut buf = vec![0u8; bytes];
    let mut hasher = blake3::Hasher::new();
    hasher.update(seed);
    hasher.finalize_xof().fill(&mut buf);

    let mut out = vec![0u8; dim];
    for i in 0..dim {
        let byte = buf[i >> 3];
        out[i] = (byte >> (7 - (i & 7))) & 1;
    }
    out
}

/// `1 - hamming(a, b) / D`, in `[0, 1]`.
///
/// Panics if `a.len() != b.len()`.
pub fn similarity(a: &[u8], b: &[u8]) -> f64 {
    assert_eq!(a.len(), b.len(), "dimension mismatch in similarity");
    let mut mismatches = 0usize;
    for (&x, &y) in a.iter().zip(b.iter()) {
        if x != y {
            mismatches += 1;
        }
    }
    1.0 - (mismatches as f64) / (a.len() as f64)
}

/// Element-wise XOR. Self-inverse: `bind(bind(a, b), b) == a`.
pub fn bind(a: &[u8], b: &[u8]) -> Hypervector {
    assert_eq!(a.len(), b.len(), "dimension mismatch in bind");
    a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
}

/// Element-wise XOR. Same op as `bind`; the name signals intent.
pub fn unbind(a: &[u8], b: &[u8]) -> Hypervector {
    bind(a, b)
}

/// Element-wise majority over a slice of hypervectors. Ties resolved
/// deterministically per SPEC §1.3 using BLAKE3 over `(dim, index, count)`.
pub fn bundle(hvs: &[&Hypervector]) -> Hypervector {
    assert!(!hvs.is_empty(), "bundle requires at least one hypervector");
    let dim = hvs[0].len();
    for hv in hvs.iter() {
        assert_eq!(hv.len(), dim, "dimension mismatch in bundle");
    }
    let n = hvs.len();
    let half = (n as f64) / 2.0;

    let mut sums = vec![0u32; dim];
    for hv in hvs {
        for (i, &b) in hv.iter().enumerate() {
            sums[i] += b as u32;
        }
    }

    let mut out = vec![0u8; dim];
    for i in 0..dim {
        let s = sums[i] as f64;
        if s > half {
            out[i] = 1;
        } else if s < half {
            out[i] = 0;
        } else {
            out[i] = deterministic_tiebreak(dim as u32, i as u32, n as u32);
        }
    }
    out
}

/// Cyclic bit rotation by `k`. Positive `k` rotates forward; negative rotates
/// backward. Used to encode order and protect bindings from interference.
pub fn permute(hv: &[u8], k: i32) -> Hypervector {
    let dim = hv.len();
    let shift = ((k % dim as i32) + dim as i32) as usize % dim;
    let mut out = vec![0u8; dim];
    for i in 0..dim {
        out[(i + shift) % dim] = hv[i];
    }
    out
}

fn deterministic_tiebreak(dim: u32, index: u32, count: u32) -> u8 {
    let mut buf = Vec::with_capacity(TIEBREAKER_DOMAIN.len() + 12);
    buf.extend_from_slice(TIEBREAKER_DOMAIN);
    buf.extend_from_slice(&dim.to_le_bytes());
    buf.extend_from_slice(&index.to_le_bytes());
    buf.extend_from_slice(&count.to_le_bytes());
    blake3::hash(&buf).as_bytes()[0] & 1
}
