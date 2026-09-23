// Guards the browser/Node entry split (package.json `exports["."].browser`).
//
// `web/` bundles this package for the browser. When the root entry pulled in
// `adapters/fs.ts`, `next build` failed outright with
// `UnhandledSchemeError: Reading from "node:fs/promises"`. These tests fail
// first, and name the file, if a Node-only module leaks back into the browser
// graph or the two entries drift apart.

import { strict as assert } from "node:assert";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { describe, it } from "node:test";
import { fileURLToPath } from "node:url";

import ts from "typescript";

import * as browserEntry from "./index.browser.js";
import * as nodeEntry from "./index.js";

const SRC = dirname(fileURLToPath(import.meta.url));

/** Exports that exist only on the Node entry. */
const NODE_ONLY_EXPORTS = ["fsAdapter", "sqliteAdapter"];

/** Source files that must never be reachable from the browser entry. */
const NODE_ONLY_FILES = ["adapters/fs.ts", "adapters/sqlite.ts", "adapters/index.ts", "index.ts"];

/**
 * Every module statically reachable from `entry`, following relative imports
 * (value and type-only alike, which over-approximates the runtime graph), plus
 * every bare specifier seen on the way.
 */
function reachableFrom(entry: string): { files: Set<string>; bare: Set<string> } {
  const files = new Set<string>();
  const bare = new Set<string>();
  const visit = (file: string) => {
    if (files.has(file)) return;
    files.add(file);
    const { importedFiles } = ts.preProcessFile(readFileSync(file, "utf8"), true, true);
    for (const { fileName } of importedFiles) {
      if (fileName.startsWith(".")) {
        visit(resolve(dirname(file), fileName.replace(/\.js$/, ".ts")));
      } else {
        bare.add(fileName);
      }
    }
  };
  visit(join(SRC, entry));
  return { files, bare };
}

describe("browser entry (index.browser.ts)", () => {
  it("exports the Node entry's surface minus exactly the Node-only adapters", () => {
    const expected = Object.keys(nodeEntry)
      .filter((name) => !NODE_ONLY_EXPORTS.includes(name))
      .sort();
    assert.deepEqual(Object.keys(browserEntry).sort(), expected);
    for (const name of NODE_ONLY_EXPORTS) {
      assert.ok(name in nodeEntry, `Node entry lost ${name}`);
    }
  });

  it("statically reaches no Node-only module", () => {
    const { files, bare } = reachableFrom("index.browser.ts");
    const reached = [...files].map((f) => relative(SRC, f));
    assert.deepEqual(
      reached.filter((f) => NODE_ONLY_FILES.includes(f)),
      [],
      "browser graph reaches a Node-only source file",
    );
    assert.deepEqual(
      [...bare].filter((s) => s.startsWith("node:") || s === "better-sqlite3"),
      [],
      "browser graph imports a Node-only package",
    );
    // Sanity: the walk actually followed the graph rather than stopping at the entry.
    assert.ok(reached.includes("adapters/core.ts"), `walk stopped early: ${reached.join(", ")}`);
  });
});
