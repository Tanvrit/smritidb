// Run every TypeScript benchmark, write results/typescript.json.

import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import os from "node:os";

import { SPEC_VERSION } from "@tanvrit/smritidb";

import { runPrimitives } from "./bench-primitives.js";
import { runRecall } from "./bench-recall.js";
import { runMemory, runPersistence } from "./bench-persistence.js";

const here = dirname(fileURLToPath(import.meta.url));

async function main(): Promise<number> {
  console.log("Smritidb benchmark — TypeScript binding");
  console.log(`  Node ${process.version} on ${os.platform()}/${os.arch()}`);

  const primitives = runPrimitives();
  const recall = runRecall();
  const persistence = await runPersistence();
  const memory = runMemory();

  const output = {
    binding: "typescript",
    version: "0.1.0",
    spec_version: SPEC_VERSION,
    host: { os: os.platform(), arch: os.arch() },
    primitives,
    recall,
    persistence,
    memory,
  };

  const resultsDir = resolve(here, "..", "results");
  mkdirSync(resultsDir, { recursive: true });
  const outPath = join(resultsDir, "typescript.json");
  writeFileSync(outPath, JSON.stringify(output, null, 2));
  console.log(`wrote ${outPath}`);
  return 0;
}

main().then((code) => process.exit(code));
