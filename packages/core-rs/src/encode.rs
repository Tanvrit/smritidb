//! Semantic key encoders. Per SPEC.md §3.5 / §3.6 / appendix A.1.

use crate::hypervector::{bind, random_hv, Hypervector};

const STRING_DOMAIN: &str = "str:";
const LEVEL_DOMAIN: &str = "lvl:";
const LEVELS: usize = 100;

/// Map a UTF-8 string to a hypervector via BLAKE3-derived randomness.
pub fn encode_string(s: &str, dim: usize) -> Hypervector {
    let mut seed = Vec::with_capacity(STRING_DOMAIN.len() + s.len());
    seed.extend_from_slice(STRING_DOMAIN.as_bytes());
    seed.extend_from_slice(s.as_bytes());
    let digest = blake3::hash(&seed);
    random_hv(digest.as_bytes(), dim)
}

/// Binary thermometer + random projection: map a `[-1, 1]` embedding to a
/// hypervector that preserves cosine similarity approximately.
///
/// See SPEC.md appendix A.1.
pub fn encode_embedding(embedding: &[f32], dim: usize) -> Hypervector {
    let mut acc: Hypervector = vec![0u8; dim];
    for (i, &raw) in embedding.iter().enumerate() {
        let clamped = raw.clamp(-1.0, 1.0);
        let level = (((clamped + 1.0) * ((LEVELS - 1) as f32) / 2.0).round() as i32)
            .clamp(0, (LEVELS - 1) as i32) as usize;
        let mut seed = Vec::with_capacity(LEVEL_DOMAIN.len() + 32);
        seed.extend_from_slice(LEVEL_DOMAIN.as_bytes());
        seed.extend_from_slice(i.to_string().as_bytes());
        seed.push(b':');
        seed.extend_from_slice(level.to_string().as_bytes());
        let digest = blake3::hash(&seed);
        let hv = random_hv(digest.as_bytes(), dim);
        acc = bind(&acc, &hv);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hypervector::similarity;

    #[test]
    fn encode_string_is_deterministic() {
        let a = encode_string("hello", 4096);
        let b = encode_string("hello", 4096);
        let c = encode_string("world", 4096);
        assert_eq!(a, b);
        assert!(similarity(&a, &c) < 0.55);
    }

    #[test]
    fn embedding_encoding_round_trips_through_levels() {
        let v: Vec<f32> = (0..16).map(|i| (i as f32) / 16.0 - 0.5).collect();
        let a = encode_embedding(&v, 4096);
        let b = encode_embedding(&v, 4096);
        assert_eq!(a, b);
    }
}
