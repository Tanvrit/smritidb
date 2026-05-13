// Text encoders. Bridge between raw strings and hypervectors.
//
// `encodeString` (in encode.ts) is whole-string-hash: any change to the input
// produces a near-orthogonal hypervector. That's the right primitive for
// exact-match keys, but it's the wrong primitive for fuzzy text recall —
// queries that share words / substrings should land near the originals.
//
// The encoders here cover the common middle ground:
//
//   - `encodeBagOfWords`   — bundle the per-word hypervectors. Cheap, no
//                            order info; good baseline for prose recall.
//   - `encodeWordNgrams`   — bundle per-n-gram hypervectors with a permute
//                            on the second word; preserves some order.
//   - `encodeCharNgrams`   — bundle character-level shingles; tolerates
//                            typos, partial matches, and unknown words.
//
// All three are deterministic, bit-exact across implementations (they only
// call `encodeString`, `bundle`, and `permute` from the SPEC §1 primitives),
// and have no internal vocabulary — any string in, hypervector out.

import { bundle, permute, type Hypervector } from "./hypervector.js";
import { encodeString } from "./encode.js";

const WORD_BREAK = /[^a-z0-9]+/g;

export interface TextEncodingOptions {
  /**
   * Skip words shorter than this. 0 includes single-character "words" too.
   * Default 3 — drops "a", "is", "to", "of" etc. that flood the bundle
   * without carrying much information.
   */
  readonly minWordLength?: number;
  /** Lowercase before tokenising. Default true. */
  readonly lowercase?: boolean;
}

const DEFAULT_OPTS: Required<TextEncodingOptions> = {
  minWordLength: 3,
  lowercase: true,
};

function normaliseWords(s: string, opts: Required<TextEncodingOptions>): string[] {
  const cleaned = opts.lowercase ? s.toLowerCase() : s;
  return cleaned
    .replace(WORD_BREAK, " ")
    .split(/\s+/)
    .filter((w) => w.length >= opts.minWordLength);
}

/**
 * Bag-of-words encoder. Bundles `encodeString("word:<w>")` for each word.
 * Order is discarded. Two strings sharing many words will have similar
 * encodings; recall is partial-cue friendly.
 *
 * Empty input returns `encodeString("", dim)` as a degenerate fallback so the
 * function never returns a zero-length vector.
 */
export function encodeBagOfWords(
  text: string,
  dim: number,
  options: TextEncodingOptions = {},
): Hypervector {
  const opts = { ...DEFAULT_OPTS, ...options };
  const words = normaliseWords(text, opts);
  if (words.length === 0) return encodeString(text, dim);
  return bundle(words.map((w) => encodeString(`word:${w}`, dim)));
}

/**
 * Word-level n-gram encoder. Each n-gram is encoded as
 *   bind(encodeString("word:w0"), permute(encodeString("word:w1"), 1), permute(..., 2), ...)
 * preserving order. The bundle of n-grams is the final hypervector.
 *
 * For `n = 1` this reduces to `encodeBagOfWords`.
 */
export function encodeWordNgrams(
  text: string,
  dim: number,
  n: number,
  options: TextEncodingOptions = {},
): Hypervector {
  if (n < 1) throw new Error("encodeWordNgrams: n must be >= 1");
  const opts = { ...DEFAULT_OPTS, ...options };
  const words = normaliseWords(text, opts);
  if (words.length === 0) return encodeString(text, dim);
  if (n === 1) return encodeBagOfWords(text, dim, options);

  const ngrams: Hypervector[] = [];
  for (let i = 0; i <= words.length - n; i++) {
    let acc = encodeString(`word:${words[i]}`, dim);
    for (let j = 1; j < n; j++) {
      const term = permute(encodeString(`word:${words[i + j]}`, dim), j);
      acc = xor(acc, term);
    }
    ngrams.push(acc);
  }
  // If the text is shorter than n words, fall back to bag-of-words so
  // recall remains useful.
  if (ngrams.length === 0) return encodeBagOfWords(text, dim, options);
  return bundle(ngrams);
}

/**
 * Character n-gram encoder ("shingles"). Tolerant of typos and partial
 * matches. Default `n = 3` is the common "trigram" pattern that gives good
 * fuzzy-string-match behaviour for short queries.
 */
export function encodeCharNgrams(
  text: string,
  dim: number,
  n: number = 3,
  options: { lowercase?: boolean } = {},
): Hypervector {
  if (n < 1) throw new Error("encodeCharNgrams: n must be >= 1");
  const cleaned = (options.lowercase ?? true) ? text.toLowerCase() : text;
  if (cleaned.length < n) return encodeString(text, dim);
  const grams: Hypervector[] = [];
  for (let i = 0; i <= cleaned.length - n; i++) {
    grams.push(encodeString(`char:${cleaned.slice(i, i + n)}`, dim));
  }
  return bundle(grams);
}

function xor(a: Hypervector, b: Hypervector): Hypervector {
  const out = new Uint8Array(a.length);
  for (let i = 0; i < a.length; i++) out[i] = a[i]! ^ b[i]!;
  return out;
}
