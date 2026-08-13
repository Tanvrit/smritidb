//! KMF — Smritidb Memory Format — per SPEC.md §8.
//!
//! Port of `packages/core-ts/src/kmf.ts`. The on-the-wire byte layout MUST be
//! byte-identical to the TypeScript writer for the same logical snapshot:
//! magic + spec_version + header_offset + blocks + JSON header + trailer.
//!
//! Two pieces of the layout depend on stable map ordering:
//!   * the JSON header object orders its fields
//!     `spec_version, dimension, item_count, created_at, index`;
//!   * each `meta_block` entry orders its fields
//!     `id, tags, metadata, createdAt, accessCount, lastAccessedAt`.
//!
//! JavaScript objects iterate in insertion order; we replicate that via the
//! `serde_json/preserve_order` feature plus explicit ordering when we build
//! the JSON values below. User-supplied metadata also preserves insertion
//! order — pass a `serde_json::Value::Object` whose keys are in the order you
//! want them serialised.

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use thiserror::Error;

use crate::hypervector::Hypervector;

const MAGIC: [u8; 4] = [0x4b, 0x4d, 0x46, 0x00];
const TRAILER: [u8; 4] = [0x46, 0x4d, 0x4b, 0x00];
const SPEC_VERSION_BYTES: [u8; 6] = *b"0.1.0\0";

pub const KMF_SPEC_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KmfBlockKind {
    HvBlock,
    MetaBlock,
    ValueBlock,
}

impl KmfBlockKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HvBlock => "hv_block",
            Self::MetaBlock => "meta_block",
            Self::ValueBlock => "value_block",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "hv_block" => Some(Self::HvBlock),
            "meta_block" => Some(Self::MetaBlock),
            "value_block" => Some(Self::ValueBlock),
            _ => None,
        }
    }
}

/// A single item in a KMF snapshot.
#[derive(Debug, Clone, PartialEq)]
pub struct KmfItem {
    pub id: String,
    pub key: Hypervector,
    pub value: Vec<u8>,
    pub tags: Vec<String>,
    /// Metadata. SPEC §1.4: a `map<string, scalar>` whose scalar values are
    /// `string | number | boolean | null`. On the wire we always emit a JSON
    /// object — any non-object value (e.g. `JsonValue::Null`) is normalised
    /// to `{}` by `serialise_meta_block` so cross-impl byte-identity holds.
    pub metadata: JsonValue,
    pub created_at: i64,
    pub access_count: u32,
    pub last_accessed_at: i64,
}

#[derive(Debug, Clone)]
pub struct KmfSnapshot {
    pub dimension: usize,
    pub created_at: i64,
    pub items: Vec<KmfItem>,
}

#[derive(Debug, Error)]
pub enum KmfError {
    #[error("KMF: file too small to be valid")]
    TooSmall,
    #[error("KMF: invalid magic bytes")]
    BadMagic,
    #[error("KMF: invalid trailer (file truncated?)")]
    BadTrailer,
    #[error("KMF: unsupported spec_version {found}; this reader speaks {expected}")]
    UnsupportedVersion { found: String, expected: String },
    #[error("KMF: BLAKE3 mismatch on {0} block")]
    Blake3Mismatch(&'static str),
    #[error("KMF: malformed header: {0}")]
    BadHeader(String),
    #[error("KMF: malformed meta block: {0}")]
    BadMeta(String),
    #[error("KMF: malformed value block: {0}")]
    BadValueBlock(String),
    #[error("KMF: item {id} key length {got} does not match dimension {dim}")]
    DimensionMismatch { id: String, got: usize, dim: usize },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BlockRefRaw {
    kind: String,
    offset: u64,
    length: u64,
    blake3: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct KmfHeaderRaw {
    spec_version: String,
    dimension: usize,
    item_count: usize,
    created_at: i64,
    index: Vec<BlockRefRaw>,
}

// --- writer ---------------------------------------------------------------

pub fn write_kmf(snapshot: &KmfSnapshot) -> Result<Vec<u8>, KmfError> {
    if snapshot.items.is_empty() {
        let header = KmfHeaderRaw {
            spec_version: KMF_SPEC_VERSION.to_string(),
            dimension: snapshot.dimension,
            item_count: 0,
            created_at: snapshot.created_at,
            index: Vec::new(),
        };
        let header_bytes =
            serde_json::to_vec(&header).map_err(|e| KmfError::BadHeader(e.to_string()))?;
        return Ok(build_payload(&[], &header_bytes));
    }

    let d = snapshot.dimension;
    let bytes_per_hv = d.div_ceil(8);
    let mut hv_bytes = vec![0u8; snapshot.items.len() * bytes_per_hv];
    pack_hypervector_block(&mut hv_bytes, &snapshot.items, d)?;

    let meta_json = serialise_meta_block(&snapshot.items)?;
    let value_block = serialise_value_block(&snapshot.items);

    // Cursor starts after magic (4) + spec (6) + header_offset (8) = 18.
    let mut cursor = (MAGIC.len() + SPEC_VERSION_BYTES.len() + 8) as u64;
    let blocks: [(&str, &[u8]); 3] = [
        ("hv_block", &hv_bytes),
        ("meta_block", &meta_json),
        ("value_block", &value_block),
    ];
    let mut index: Vec<BlockRefRaw> = Vec::with_capacity(3);
    let mut blocks_concat: Vec<u8> =
        Vec::with_capacity(hv_bytes.len() + meta_json.len() + value_block.len());
    for (kind, bytes) in blocks.iter() {
        index.push(BlockRefRaw {
            kind: (*kind).to_string(),
            offset: cursor,
            length: bytes.len() as u64,
            blake3: blake3_hex(bytes),
        });
        cursor += bytes.len() as u64;
        blocks_concat.extend_from_slice(bytes);
    }

    let header = KmfHeaderRaw {
        spec_version: KMF_SPEC_VERSION.to_string(),
        dimension: d,
        item_count: snapshot.items.len(),
        created_at: snapshot.created_at,
        index,
    };
    let header_bytes =
        serde_json::to_vec(&header).map_err(|e| KmfError::BadHeader(e.to_string()))?;

    Ok(build_payload(&blocks_concat, &header_bytes))
}

fn build_payload(blocks: &[u8], header: &[u8]) -> Vec<u8> {
    let header_offset = (MAGIC.len() + SPEC_VERSION_BYTES.len() + 8 + blocks.len()) as u64;
    let total = header_offset as usize + header.len() + TRAILER.len();
    let mut out = Vec::with_capacity(total);
    out.extend_from_slice(&MAGIC);
    out.extend_from_slice(&SPEC_VERSION_BYTES);
    out.extend_from_slice(&header_offset.to_le_bytes());
    out.extend_from_slice(blocks);
    out.extend_from_slice(header);
    out.extend_from_slice(&TRAILER);
    out
}

fn pack_hypervector_block(out: &mut [u8], items: &[KmfItem], d: usize) -> Result<(), KmfError> {
    let bytes_per_hv = d.div_ceil(8);
    for (i, item) in items.iter().enumerate() {
        if item.key.len() != d {
            return Err(KmfError::DimensionMismatch {
                id: item.id.clone(),
                got: item.key.len(),
                dim: d,
            });
        }
        // MSB-first packing per SPEC §1.1.
        for bit in 0..d {
            if item.key[bit] == 1 {
                let byte_index = i * bytes_per_hv + (bit >> 3);
                out[byte_index] |= 1 << (7 - (bit & 7));
            }
        }
    }
    Ok(())
}

fn serialise_meta_block(items: &[KmfItem]) -> Result<Vec<u8>, KmfError> {
    // We construct each entry as an ordered JSON object so the field order
    // matches the TS writer exactly: id, tags, metadata, createdAt,
    // accessCount, lastAccessedAt.
    let mut arr: Vec<JsonValue> = Vec::with_capacity(items.len());
    for it in items {
        let mut obj = serde_json::Map::new();
        obj.insert("id".into(), JsonValue::String(it.id.clone()));
        obj.insert(
            "tags".into(),
            JsonValue::Array(it.tags.iter().cloned().map(JsonValue::String).collect()),
        );
        // Per SPEC §1.4 / §8.1, `metadata` is `map<string, scalar>` — the
        // canonical empty value is the empty object `{}`, not `null`. The TS
        // reference always emits `{}` because its `metadata` field is typed as
        // `Record<...>` and defaults to `{}`. Callers that pass
        // `JsonValue::Null` (e.g. the Python binding's default, or any Rust
        // caller convenience-shorthand) MUST not produce wire bytes that
        // diverge from the canonical empty-object form, otherwise cross-impl
        // byte-identity breaks (see `tests/cross_impl_write_parity.rs`).
        // Normalise any non-object metadata to `{}` at the encoder boundary
        // so the on-the-wire layout matches the TS reference byte-for-byte.
        let normalised_meta = if it.metadata.is_object() {
            it.metadata.clone()
        } else {
            JsonValue::Object(serde_json::Map::new())
        };
        obj.insert("metadata".into(), normalised_meta);
        obj.insert("createdAt".into(), JsonValue::from(it.created_at));
        obj.insert("accessCount".into(), JsonValue::from(it.access_count));
        obj.insert(
            "lastAccessedAt".into(),
            JsonValue::from(it.last_accessed_at),
        );
        arr.push(JsonValue::Object(obj));
    }
    serde_json::to_vec(&JsonValue::Array(arr)).map_err(|e| KmfError::BadMeta(e.to_string()))
}

fn serialise_value_block(items: &[KmfItem]) -> Vec<u8> {
    let total: usize = items.iter().map(|it| 4 + it.value.len()).sum();
    let mut out = Vec::with_capacity(total);
    for it in items {
        let len = it.value.len() as u32;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&it.value);
    }
    out
}

fn blake3_hex(bytes: &[u8]) -> String {
    let hash = blake3::hash(bytes);
    let mut s = String::with_capacity(64);
    for b in hash.as_bytes() {
        use std::fmt::Write;
        let _ = write!(&mut s, "{b:02x}");
    }
    s
}

// --- reader ---------------------------------------------------------------

pub fn read_kmf(bytes: &[u8]) -> Result<KmfSnapshot, KmfError> {
    let prefix_len = MAGIC.len() + SPEC_VERSION_BYTES.len() + 8;
    if bytes.len() < prefix_len + TRAILER.len() {
        return Err(KmfError::TooSmall);
    }
    if bytes[..MAGIC.len()] != MAGIC {
        return Err(KmfError::BadMagic);
    }
    if bytes[bytes.len() - TRAILER.len()..] != TRAILER {
        return Err(KmfError::BadTrailer);
    }

    let spec_bytes = &bytes[MAGIC.len()..MAGIC.len() + SPEC_VERSION_BYTES.len()];
    let spec = trim_nul(std::str::from_utf8(spec_bytes).map_err(|_| KmfError::TooSmall)?);
    if spec != KMF_SPEC_VERSION {
        return Err(KmfError::UnsupportedVersion {
            found: spec.to_string(),
            expected: KMF_SPEC_VERSION.to_string(),
        });
    }

    let header_offset_bytes: [u8; 8] = bytes
        [MAGIC.len() + SPEC_VERSION_BYTES.len()..MAGIC.len() + SPEC_VERSION_BYTES.len() + 8]
        .try_into()
        .map_err(|_| KmfError::TooSmall)?;
    let header_offset = u64::from_le_bytes(header_offset_bytes) as usize;
    let header_end = bytes.len() - TRAILER.len();
    if header_offset > header_end {
        return Err(KmfError::BadHeader("header offset past trailer".into()));
    }

    let header: KmfHeaderRaw = serde_json::from_slice(&bytes[header_offset..header_end])
        .map_err(|e| KmfError::BadHeader(e.to_string()))?;

    if header.item_count == 0 {
        return Ok(KmfSnapshot {
            dimension: header.dimension,
            created_at: header.created_at,
            items: Vec::new(),
        });
    }

    let d = header.dimension;
    let bytes_per_hv = d.div_ceil(8);

    let hv_ref = header
        .index
        .iter()
        .find(|b| KmfBlockKind::parse(&b.kind) == Some(KmfBlockKind::HvBlock))
        .ok_or_else(|| KmfError::BadHeader("missing hv_block in index".into()))?;
    let meta_ref = header
        .index
        .iter()
        .find(|b| KmfBlockKind::parse(&b.kind) == Some(KmfBlockKind::MetaBlock))
        .ok_or_else(|| KmfError::BadHeader("missing meta_block in index".into()))?;
    let value_ref = header
        .index
        .iter()
        .find(|b| KmfBlockKind::parse(&b.kind) == Some(KmfBlockKind::ValueBlock))
        .ok_or_else(|| KmfError::BadHeader("missing value_block in index".into()))?;

    let hv_block = slice_and_verify(bytes, hv_ref, "hv_block")?;
    let meta_block = slice_and_verify(bytes, meta_ref, "meta_block")?;
    let value_block = slice_and_verify(bytes, value_ref, "value_block")?;

    let meta: Vec<MetaEntry> =
        serde_json::from_slice(meta_block).map_err(|e| KmfError::BadMeta(e.to_string()))?;
    if meta.len() != header.item_count {
        return Err(KmfError::BadMeta(format!(
            "item_count {} != meta entries {}",
            header.item_count,
            meta.len()
        )));
    }

    let values = read_value_block(value_block, meta.len())?;

    let mut items = Vec::with_capacity(meta.len());
    for (i, m) in meta.into_iter().enumerate() {
        let key = unpack_hv(hv_block, i, bytes_per_hv, d);
        items.push(KmfItem {
            id: m.id,
            key,
            value: values[i].clone(),
            tags: m.tags,
            metadata: m.metadata,
            created_at: m.created_at,
            access_count: m.access_count,
            last_accessed_at: m.last_accessed_at,
        });
    }

    Ok(KmfSnapshot {
        dimension: d,
        created_at: header.created_at,
        items,
    })
}

#[derive(Debug, Deserialize)]
struct MetaEntry {
    id: String,
    tags: Vec<String>,
    metadata: JsonValue,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "accessCount")]
    access_count: u32,
    #[serde(rename = "lastAccessedAt")]
    last_accessed_at: i64,
}

fn slice_and_verify<'a>(
    bytes: &'a [u8],
    r: &BlockRefRaw,
    label: &'static str,
) -> Result<&'a [u8], KmfError> {
    let start = r.offset as usize;
    let end = start
        .checked_add(r.length as usize)
        .ok_or(KmfError::Blake3Mismatch(label))?;
    if end > bytes.len() {
        return Err(KmfError::Blake3Mismatch(label));
    }
    let view = &bytes[start..end];
    let actual = blake3_hex(view);
    if actual != r.blake3 {
        return Err(KmfError::Blake3Mismatch(label));
    }
    Ok(view)
}

fn unpack_hv(block: &[u8], index: usize, bytes_per_hv: usize, d: usize) -> Hypervector {
    let mut out = vec![0u8; d];
    let base = index * bytes_per_hv;
    for bit in 0..d {
        let byte = block[base + (bit >> 3)];
        out[bit] = (byte >> (7 - (bit & 7))) & 1;
    }
    out
}

fn read_value_block(block: &[u8], count: usize) -> Result<Vec<Vec<u8>>, KmfError> {
    let mut out = Vec::with_capacity(count);
    let mut cursor = 0usize;
    for _ in 0..count {
        if cursor + 4 > block.len() {
            return Err(KmfError::BadValueBlock(
                "value block truncated mid-length-prefix".into(),
            ));
        }
        let mut len_bytes = [0u8; 4];
        len_bytes.copy_from_slice(&block[cursor..cursor + 4]);
        let len = u32::from_le_bytes(len_bytes) as usize;
        cursor += 4;
        if cursor + len > block.len() {
            return Err(KmfError::BadValueBlock(
                "value block truncated mid-value".into(),
            ));
        }
        out.push(block[cursor..cursor + len].to_vec());
        cursor += len;
    }
    Ok(out)
}

fn trim_nul(s: &str) -> &str {
    match s.find('\0') {
        Some(idx) => &s[..idx],
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(id: &str, dim: usize, seed: &[u8]) -> KmfItem {
        KmfItem {
            id: id.to_string(),
            key: crate::random_hv(seed, dim),
            value: format!("value-for-{id}").into_bytes(),
            tags: vec!["t1".into(), "t2".into()],
            metadata: serde_json::json!({"rank": 1, "kind": id}),
            created_at: 1_700_000_000_000,
            access_count: 0,
            last_accessed_at: 1_700_000_000_000,
        }
    }

    #[test]
    fn magic_and_version_at_known_offsets() {
        let snap = KmfSnapshot {
            dimension: 1024,
            created_at: 0,
            items: vec![],
        };
        let bytes = write_kmf(&snap).unwrap();
        assert_eq!(&bytes[..4], &[0x4b, 0x4d, 0x46, 0x00]);
        assert_eq!(&bytes[4..10], b"0.1.0\0");
        assert_eq!(&bytes[bytes.len() - 4..], &[0x46, 0x4d, 0x4b, 0x00]);
    }

    #[test]
    fn round_trip_preserves_items() {
        let dim = 2048;
        let items = vec![
            make_item("id-a", dim, b"alpha"),
            make_item("id-b", dim, b"beta"),
            make_item("id-c", dim, b"gamma"),
        ];
        let snap = KmfSnapshot {
            dimension: dim,
            created_at: 42,
            items: items.clone(),
        };
        let bytes = write_kmf(&snap).unwrap();
        let back = read_kmf(&bytes).unwrap();
        assert_eq!(back.dimension, dim);
        assert_eq!(back.created_at, 42);
        assert_eq!(back.items.len(), 3);
        for (a, b) in items.iter().zip(back.items.iter()) {
            assert_eq!(a.id, b.id);
            assert_eq!(a.key, b.key);
            assert_eq!(a.value, b.value);
            assert_eq!(a.tags, b.tags);
            assert_eq!(a.metadata, b.metadata);
            assert_eq!(a.created_at, b.created_at);
            assert_eq!(a.access_count, b.access_count);
            assert_eq!(a.last_accessed_at, b.last_accessed_at);
        }
    }

    #[test]
    fn rejects_truncated_files() {
        let snap = KmfSnapshot {
            dimension: 1024,
            created_at: 0,
            items: vec![make_item("x", 1024, b"x")],
        };
        let bytes = write_kmf(&snap).unwrap();
        let truncated = &bytes[..bytes.len() - 1];
        let err = read_kmf(truncated).unwrap_err();
        assert!(matches!(err, KmfError::BadTrailer));
    }

    #[test]
    fn rejects_blake3_corruption() {
        let snap = KmfSnapshot {
            dimension: 1024,
            created_at: 0,
            items: vec![
                make_item("alpha", 1024, b"alpha"),
                make_item("beta", 1024, b"beta"),
            ],
        };
        let mut bytes = write_kmf(&snap).unwrap();
        bytes[64] ^= 0xff;
        let err = read_kmf(&bytes).unwrap_err();
        assert!(matches!(err, KmfError::Blake3Mismatch(_)));
    }

    #[test]
    fn rejects_unsupported_version() {
        let snap = KmfSnapshot {
            dimension: 1024,
            created_at: 0,
            items: vec![make_item("x", 1024, b"x")],
        };
        let mut bytes = write_kmf(&snap).unwrap();
        bytes[4..10].copy_from_slice(b"9.0.0\0");
        let err = read_kmf(&bytes).unwrap_err();
        assert!(matches!(err, KmfError::UnsupportedVersion { .. }));
    }

    #[test]
    fn pack_unpack_is_msb_first() {
        // Set bit 0 and bit 7 in a fresh hv; verify the packed byte is 0x81
        // (MSB-first: bit 0 -> 0x80, bit 7 -> 0x01).
        let dim = 8;
        let mut hv = vec![0u8; dim];
        hv[0] = 1;
        hv[7] = 1;
        let item = KmfItem {
            id: "p".to_string(),
            key: hv.clone(),
            value: vec![],
            tags: vec![],
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            created_at: 0,
            access_count: 0,
            last_accessed_at: 0,
        };
        let snap = KmfSnapshot {
            dimension: dim,
            created_at: 0,
            items: vec![item],
        };
        let bytes = write_kmf(&snap).unwrap();
        let back = read_kmf(&bytes).unwrap();
        assert_eq!(back.items[0].key, hv);
    }

    #[test]
    fn empty_snapshot_round_trips() {
        let snap = KmfSnapshot {
            dimension: 1024,
            created_at: 99,
            items: vec![],
        };
        let bytes = write_kmf(&snap).unwrap();
        let back = read_kmf(&bytes).unwrap();
        assert_eq!(back.dimension, 1024);
        assert_eq!(back.created_at, 99);
        assert!(back.items.is_empty());
    }
}
