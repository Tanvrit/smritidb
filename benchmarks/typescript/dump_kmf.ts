// Dumps raw KMF blob produced by TS for the benchmark workload.
import { writeFileSync } from "node:fs";
import { Smritidb, snapshot } from "@tanvrit/smritidb";

const D = 10_000;
const N = 10_000;

const store = new Smritidb({ dimension: D });
for (let i = 0; i < N; i++) {
  store.put(`item_${i}`, `value for item ${i}`);
}
const blob = snapshot(store);
writeFileSync("/tmp/ts-kmf.bin", blob);
console.log(`TS: KMF blob length = ${blob.length}`);
