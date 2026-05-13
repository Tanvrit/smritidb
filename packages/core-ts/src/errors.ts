// Typed errors per SPEC.md §3.7.
// Each binding surfaces these same error kinds via its native error type.

export type KanervaErrorKind =
  | "DimensionMismatch"
  | "ValueTooLarge"
  | "NotFound"
  | "CorruptSnapshot"
  | "UnsupportedSpecVersion"
  | "InvalidConfig";

export class KanervaError extends Error {
  override readonly name = "KanervaError";
  constructor(
    readonly kind: KanervaErrorKind,
    message: string,
  ) {
    super(`[${kind}] ${message}`);
  }
}

export const dimensionMismatch = (a: number, b: number): KanervaError =>
  new KanervaError("DimensionMismatch", `expected matching dimensions, got ${a} and ${b}`);

export const valueTooLarge = (size: number, cap: number): KanervaError =>
  new KanervaError("ValueTooLarge", `value of ${size} bytes exceeds cap of ${cap} bytes`);

export const notFound = (id: string): KanervaError =>
  new KanervaError("NotFound", `item ${id} not found`);

export const invalidConfig = (reason: string): KanervaError =>
  new KanervaError("InvalidConfig", reason);
