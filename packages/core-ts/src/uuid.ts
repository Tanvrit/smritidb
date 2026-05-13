// UUIDv7 generator per RFC 9562.
// Time-ordered, suitable for monotonic item ids.

const HEX = "0123456789abcdef";

function getRandomBytes(n: number): Uint8Array {
  const out = new Uint8Array(n);
  // Node >= 19 and browsers expose globalThis.crypto with getRandomValues.
  const c = (globalThis as { crypto?: { getRandomValues?: (a: Uint8Array) => void } }).crypto;
  if (!c?.getRandomValues) {
    throw new Error("globalThis.crypto.getRandomValues is required");
  }
  c.getRandomValues(out);
  return out;
}

export function uuidv7(now: number = Date.now()): string {
  const bytes = getRandomBytes(16);
  const ms = BigInt(now);

  bytes[0] = Number((ms >> 40n) & 0xffn);
  bytes[1] = Number((ms >> 32n) & 0xffn);
  bytes[2] = Number((ms >> 24n) & 0xffn);
  bytes[3] = Number((ms >> 16n) & 0xffn);
  bytes[4] = Number((ms >> 8n) & 0xffn);
  bytes[5] = Number(ms & 0xffn);

  // Version 7 in the high nibble of byte 6.
  bytes[6] = (bytes[6]! & 0x0f) | 0x70;
  // RFC 4122 variant in the high two bits of byte 8.
  bytes[8] = (bytes[8]! & 0x3f) | 0x80;

  let s = "";
  for (let i = 0; i < 16; i++) {
    const b = bytes[i]!;
    s += HEX[b >>> 4]! + HEX[b & 0xf]!;
    if (i === 3 || i === 5 || i === 7 || i === 9) s += "-";
  }
  return s;
}
