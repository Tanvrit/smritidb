//! Text encoders. Same shape as the TypeScript reference; deterministic and
//! bit-exact across both. See `packages/core-ts/src/text.ts` for the rationale.

use crate::encode::encode_string;
use crate::hypervector::{bundle, permute, Hypervector};

#[derive(Clone, Copy, Debug)]
pub struct TextEncodingOptions {
    pub min_word_length: usize,
    pub lowercase: bool,
}

impl Default for TextEncodingOptions {
    fn default() -> Self {
        Self {
            min_word_length: 3,
            lowercase: true,
        }
    }
}

fn normalise_words(text: &str, opts: TextEncodingOptions) -> Vec<String> {
    let cleaned: String = if opts.lowercase {
        text.to_lowercase()
    } else {
        text.to_string()
    };

    cleaned
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .filter(|w| w.len() >= opts.min_word_length)
        .map(|w| w.to_string())
        .collect()
}

/// Bag-of-words encoder: bundle the per-word hypervectors.
pub fn encode_bag_of_words(text: &str, dim: usize, opts: TextEncodingOptions) -> Hypervector {
    let words = normalise_words(text, opts);
    if words.is_empty() {
        return encode_string(text, dim);
    }
    let hvs: Vec<Hypervector> = words
        .iter()
        .map(|w| encode_string(&format!("word:{w}"), dim))
        .collect();
    let refs: Vec<&Hypervector> = hvs.iter().collect();
    bundle(&refs)
}

/// Word n-gram encoder. n >= 2 preserves order via `permute`.
pub fn encode_word_ngrams(text: &str, dim: usize, n: usize, opts: TextEncodingOptions) -> Hypervector {
    assert!(n >= 1, "encode_word_ngrams: n must be >= 1");
    let words = normalise_words(text, opts);
    if words.is_empty() {
        return encode_string(text, dim);
    }
    if n == 1 {
        return encode_bag_of_words(text, dim, opts);
    }
    if words.len() < n {
        return encode_bag_of_words(text, dim, opts);
    }

    let mut ngrams: Vec<Hypervector> = Vec::with_capacity(words.len() - n + 1);
    for i in 0..=words.len() - n {
        let mut acc = encode_string(&format!("word:{}", words[i]), dim);
        for j in 1..n {
            let term = permute(&encode_string(&format!("word:{}", words[i + j]), dim), j as i32);
            for k in 0..acc.len() {
                acc[k] ^= term[k];
            }
        }
        ngrams.push(acc);
    }
    let refs: Vec<&Hypervector> = ngrams.iter().collect();
    bundle(&refs)
}

/// Character n-gram encoder ("shingles"). Typo-tolerant. Default n=3.
pub fn encode_char_ngrams(text: &str, dim: usize, n: usize, lowercase: bool) -> Hypervector {
    assert!(n >= 1, "encode_char_ngrams: n must be >= 1");
    let cleaned: String = if lowercase { text.to_lowercase() } else { text.to_string() };
    let chars: Vec<char> = cleaned.chars().collect();
    if chars.len() < n {
        return encode_string(text, dim);
    }
    let mut grams: Vec<Hypervector> = Vec::with_capacity(chars.len() - n + 1);
    for i in 0..=chars.len() - n {
        let gram: String = chars[i..i + n].iter().collect();
        grams.push(encode_string(&format!("char:{gram}"), dim));
    }
    let refs: Vec<&Hypervector> = grams.iter().collect();
    bundle(&refs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hypervector::similarity;

    #[test]
    fn bag_of_words_is_deterministic_and_shares_under_overlap() {
        let dim = 4096;
        let opts = TextEncodingOptions::default();
        let a = encode_bag_of_words("the cat sat on the mat", dim, opts);
        let b = encode_bag_of_words("cat sat", dim, opts);
        let c = encode_bag_of_words("zebra danced", dim, opts);
        let s_ab = similarity(&a, &b);
        let s_ac = similarity(&a, &c);
        assert!(s_ab > 0.55, "expected shared-word sim > 0.55, got {s_ab}");
        assert!(s_ab > s_ac);
    }

    #[test]
    fn word_ngrams_preserve_order() {
        let dim = 4096;
        let ab = encode_word_ngrams("alpha beta", dim, 2, TextEncodingOptions::default());
        let ba = encode_word_ngrams("beta alpha", dim, 2, TextEncodingOptions::default());
        assert!((similarity(&ab, &ba) - 1.0).abs() > 1e-6);
    }

    #[test]
    fn char_ngrams_tolerate_typos() {
        let dim = 4096;
        let a = encode_char_ngrams("photograph", dim, 3, true);
        let b = encode_char_ngrams("phtograph", dim, 3, true); // missing 'o'
        let s = similarity(&a, &b);
        assert!(s > 0.7, "expected typo-tolerant sim > 0.7, got {s}");
    }
}
