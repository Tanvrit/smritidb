// Kanerva — TypeScript reference implementation.
// Phase 1 is not yet started. This file is a placeholder so the workspace
// resolves; the real implementation will land against SPEC.md once Phase 0
// (the validation notebook) is signed off.
//
// See ../../SPEC.md and ../../docs/MANIFESTO.md.

export const SPEC_VERSION = "0.1.0-draft" as const;

export interface KanervaConfig {
  dimension: number;
  backend: "memory";
}

export class Kanerva {
  readonly spec = SPEC_VERSION;
  constructor(public readonly config: KanervaConfig) {
    if (config.dimension < 1024) {
      throw new Error(`dimension < 1024 is rejected by the spec (got ${config.dimension})`);
    }
  }
}
