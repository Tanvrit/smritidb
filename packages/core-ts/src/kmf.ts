// KMF — Smritidb Memory Format — per SPEC.md §8.
//
// Phase 1 reference writer/reader.
// Deliberately compact and uncompressed; zstd compression and chunked
// streaming arrive in Phase 4 alongside the persistence adapters. Until then
// the on-disk layout is faithful to the spec; only the contents of each block
// are uncompressed for ease of review.

import { blake3 } from "@noble/hashes/blake3.js";

const MAGIC = new Uint8Array([0x4b, 0x4d, 0x46, 0x00]); // "KMF\0"
const TRAILER = new Uint8Array([0x46, 0x4d, 0x4b, 0x00]); // "FMK\0"
const SPEC_VERSION_BYTES = encodeAscii("0.1.0\0"); // 6 bytes

export const KMF_SPEC_VERSION = "0.1.0" as const;

export type KmfBlockKind = "hv_block" | "meta_block" | "value_block";

export interface KmfBlockRef {
  readonly kind: KmfBlockKind;
  readonly offset: number;
  readonly length: number;
  readonly blake3: string;
}

export interface KmfHeader {
  readonly spec_version: typeof KMF_SPEC_VERSION;
  readonly dimension: number;
  readonly item_count: number;
  readonly created_at: number;
  readonly index: readonly KmfBlockRef[];
}

export interface KmfItem {
  readonly id: string;
  readonly key: Uint8Array;
  readonly value: Uint8Array;
  readonly tags: readonly string[];
  readonly metadata: Readonly<Record<string, string | number | boolean | null>>;
  readonly createdAt: number;
  readonly accessCount: number;
  readonly lastAccessedAt: number;
}

export interface KmfSnapshot {
  readonly dimension: number;
  readonly createdAt: number;
  readonly items: readonly KmfItem[];
}

// --- writer ---------------------------------------------------------------

export function writeKmf(snapshot: KmfSnapshot): Uint8Array {
  if (snapshot.items.length === 0) {
    return buildPayload({
      magic: MAGIC,
      blocks: new Uint8Array(0),
      header: serialiseHeader({
        spec_version: KMF_SPEC_VERSION,
        dimension: snapshot.dimension,
        item_count: 0,
        created_at: snapshot.createdAt,
        index: [],
      }),
    });
  }

  const D = snapshot.dimension;
  const hvBytes = new Uint8Array(snapshot.items.length * Math.ceil(D / 8));
  packHypervectorBlock(hvBytes, snapshot.items, D);

  const metaJson = encodeAscii(JSON.stringify(snapshot.items.map((it) => ({
    id: it.id,
    tags: it.tags,
    metadata: it.metadata,
    createdAt: it.createdAt,
    accessCount: it.accessCount,
    lastAccessedAt: it.lastAccessedAt,
  }))));

  const valueBlock = serialiseValueBlock(snapshot.items.map((it) => it.value));

  const blocks: { kind: KmfBlockKind; bytes: Uint8Array }[] = [
    { kind: "hv_block", bytes: hvBytes },
    { kind: "meta_block", bytes: metaJson },
    { kind: "value_block", bytes: valueBlock },
  ];

  let cursor = MAGIC.length + SPEC_VERSION_BYTES.length + 8;
  const index: KmfBlockRef[] = [];
  for (const b of blocks) {
    index.push({
      kind: b.kind,
      offset: cursor,
      length: b.bytes.length,
      blake3: toHex(blake3(b.bytes)),
    });
    cursor += b.bytes.length;
  }
  const blocksBytes = concat(blocks.map((b) => b.bytes));

  return buildPayload({
    magic: MAGIC,
    blocks: blocksBytes,
    header: serialiseHeader({
      spec_version: KMF_SPEC_VERSION,
      dimension: D,
      item_count: snapshot.items.length,
      created_at: snapshot.createdAt,
      index,
    }),
  });
}

interface PayloadParts {
  readonly magic: Uint8Array;
  readonly blocks: Uint8Array;
  readonly header: Uint8Array;
}

function buildPayload({ magic, blocks, header }: PayloadParts): Uint8Array {
  const headerOffset = magic.length + SPEC_VERSION_BYTES.length + 8 + blocks.length;
  const total = headerOffset + header.length + TRAILER.length;
  const out = new Uint8Array(total);
  let cursor = 0;
  out.set(magic, cursor); cursor += magic.length;
  out.set(SPEC_VERSION_BYTES, cursor); cursor += SPEC_VERSION_BYTES.length;
  new DataView(out.buffer).setBigUint64(cursor, BigInt(headerOffset), true);
  cursor += 8;
  out.set(blocks, cursor); cursor += blocks.length;
  out.set(header, cursor); cursor += header.length;
  out.set(TRAILER, cursor);
  return out;
}

function serialiseHeader(h: KmfHeader): Uint8Array {
  return encodeAscii(JSON.stringify(h));
}

function packHypervectorBlock(out: Uint8Array, items: readonly KmfItem[], D: number): void {
  const bytesPerHv = Math.ceil(D / 8);
  for (let i = 0; i < items.length; i++) {
    const key = items[i]!.key;
    if (key.length !== D) {
      throw new Error(`item ${items[i]!.id} key length ${key.length} != dimension ${D}`);
    }
    // MSB-first packing per SPEC §1.1.
    for (let bit = 0; bit < D; bit++) {
      if (key[bit] === 1) {
        const byteIndex = i * bytesPerHv + (bit >>> 3);
        out[byteIndex] = (out[byteIndex] ?? 0) | (1 << (7 - (bit & 7)));
      }
    }
  }
}

function serialiseValueBlock(values: readonly Uint8Array[]): Uint8Array {
  let total = 0;
  for (const v of values) total += 4 + v.length;
  const out = new Uint8Array(total);
  const dv = new DataView(out.buffer);
  let cursor = 0;
  for (const v of values) {
    dv.setUint32(cursor, v.length, true);
    cursor += 4;
    out.set(v, cursor);
    cursor += v.length;
  }
  return out;
}

// --- reader ---------------------------------------------------------------

export function readKmf(bytes: Uint8Array): KmfSnapshot {
  if (bytes.length < MAGIC.length + SPEC_VERSION_BYTES.length + 8 + TRAILER.length) {
    throw new Error("KMF: file too small to be valid");
  }
  if (!startsWith(bytes, MAGIC)) {
    throw new Error("KMF: invalid magic bytes");
  }
  if (!endsWith(bytes, TRAILER)) {
    throw new Error("KMF: invalid trailer (file truncated?)");
  }

  const specBytes = bytes.subarray(MAGIC.length, MAGIC.length + SPEC_VERSION_BYTES.length);
  const spec = trimNul(decodeAscii(specBytes));
  if (spec !== KMF_SPEC_VERSION) {
    throw new Error(`KMF: unsupported spec_version ${spec}; this reader speaks ${KMF_SPEC_VERSION}`);
  }

  const headerOffset = Number(
    new DataView(bytes.buffer, bytes.byteOffset + MAGIC.length + SPEC_VERSION_BYTES.length, 8).getBigUint64(0, true),
  );
  const headerEnd = bytes.length - TRAILER.length;
  const headerBytes = bytes.subarray(headerOffset, headerEnd);
  const header = JSON.parse(decodeAscii(headerBytes)) as KmfHeader;

  if (header.item_count === 0) {
    return { dimension: header.dimension, createdAt: header.created_at, items: [] };
  }

  const D = header.dimension;
  const bytesPerHv = Math.ceil(D / 8);

  const hvRef = header.index.find((b) => b.kind === "hv_block")!;
  const metaRef = header.index.find((b) => b.kind === "meta_block")!;
  const valueRef = header.index.find((b) => b.kind === "value_block")!;

  const hvBytes = sliceAndVerify(bytes, hvRef);
  const metaBytes = sliceAndVerify(bytes, metaRef);
  const valueBytes = sliceAndVerify(bytes, valueRef);

  const meta = JSON.parse(decodeAscii(metaBytes)) as Array<{
    id: string;
    tags: string[];
    metadata: Record<string, string | number | boolean | null>;
    createdAt: number;
    accessCount: number;
    lastAccessedAt: number;
  }>;

  const values = readValueBlock(valueBytes, meta.length);
  const items: KmfItem[] = [];
  for (let i = 0; i < meta.length; i++) {
    const key = unpackHv(hvBytes, i, bytesPerHv, D);
    items.push({
      id: meta[i]!.id,
      key,
      value: values[i]!,
      tags: meta[i]!.tags,
      metadata: meta[i]!.metadata,
      createdAt: meta[i]!.createdAt,
      accessCount: meta[i]!.accessCount,
      lastAccessedAt: meta[i]!.lastAccessedAt,
    });
  }

  return {
    dimension: D,
    createdAt: header.created_at,
    items,
  };
}

function sliceAndVerify(bytes: Uint8Array, ref: KmfBlockRef): Uint8Array {
  const view = bytes.subarray(ref.offset, ref.offset + ref.length);
  const actual = toHex(blake3(view));
  if (actual !== ref.blake3) {
    throw new Error(`KMF: BLAKE3 mismatch on ${ref.kind} block`);
  }
  return view;
}

function unpackHv(block: Uint8Array, index: number, bytesPerHv: number, D: number): Uint8Array {
  const out = new Uint8Array(D);
  const base = index * bytesPerHv;
  for (let bit = 0; bit < D; bit++) {
    const byte = block[base + (bit >>> 3)]!;
    out[bit] = (byte >>> (7 - (bit & 7))) & 1;
  }
  return out;
}

function readValueBlock(block: Uint8Array, count: number): Uint8Array[] {
  const out: Uint8Array[] = [];
  const dv = new DataView(block.buffer, block.byteOffset, block.byteLength);
  let cursor = 0;
  for (let i = 0; i < count; i++) {
    const length = dv.getUint32(cursor, true);
    cursor += 4;
    out.push(block.subarray(cursor, cursor + length));
    cursor += length;
  }
  return out;
}

// --- utilities ------------------------------------------------------------

function startsWith(haystack: Uint8Array, needle: Uint8Array): boolean {
  if (haystack.length < needle.length) return false;
  for (let i = 0; i < needle.length; i++) {
    if (haystack[i] !== needle[i]) return false;
  }
  return true;
}

function endsWith(haystack: Uint8Array, needle: Uint8Array): boolean {
  if (haystack.length < needle.length) return false;
  const offset = haystack.length - needle.length;
  for (let i = 0; i < needle.length; i++) {
    if (haystack[offset + i] !== needle[i]) return false;
  }
  return true;
}

function encodeAscii(s: string): Uint8Array {
  return new TextEncoder().encode(s);
}

function decodeAscii(bytes: Uint8Array): string {
  return new TextDecoder().decode(bytes);
}

function trimNul(s: string): string {
  const idx = s.indexOf("\0");
  return idx === -1 ? s : s.slice(0, idx);
}

function toHex(bytes: Uint8Array): string {
  let s = "";
  for (const b of bytes) {
    s += b.toString(16).padStart(2, "0");
  }
  return s;
}

function concat(parts: readonly Uint8Array[]): Uint8Array {
  let total = 0;
  for (const p of parts) total += p.length;
  const out = new Uint8Array(total);
  let cursor = 0;
  for (const p of parts) {
    out.set(p, cursor);
    cursor += p.length;
  }
  return out;
}
