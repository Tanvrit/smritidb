// Smritidb — TypeScript reference implementation against SPEC.md v0.1.0-draft.
//
// This is the NODE entry. The portable surface lives in `./index.browser.ts`
// (which the package's `browser` export condition points at); this file adds
// the Node-only storage adapters on top.

export * from "./index.browser.js";

export { fsAdapter, sqliteAdapter } from "./adapters/index.js";
export type { SqliteAdapterOptions, SqliteDatabase } from "./adapters/index.js";
