// Typed errors per SPEC.md §3.7.
// Each binding surfaces these same error kinds via its native error type.

export type SmritidbErrorKind =
  | "DimensionMismatch"
  | "ValueTooLarge"
  | "NotFound"
  | "CorruptSnapshot"
  | "UnsupportedSpecVersion"
  | "InvalidConfig";

export class SmritidbError extends Error {
  override readonly name = "SmritidbError";
  constructor(
    readonly kind: SmritidbErrorKind,
    message: string,
  ) {
    super(`[${kind}] ${message}`);
  }
}

export const dimensionMismatch = (a: number, b: number): SmritidbError =>
  new SmritidbError("DimensionMismatch", `expected matching dimensions, got ${a} and ${b}`);

export const valueTooLarge = (size: number, cap: number): SmritidbError =>
  new SmritidbError("ValueTooLarge", `value of ${size} bytes exceeds cap of ${cap} bytes`);

export const notFound = (id: string): SmritidbError =>
  new SmritidbError("NotFound", `item ${id} not found`);

export const invalidConfig = (reason: string): SmritidbError =>
  new SmritidbError("InvalidConfig", reason);
