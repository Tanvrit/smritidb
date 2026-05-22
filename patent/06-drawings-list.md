<!-- Round 3 changelog (pinned to SHA 17334f8) - applied 2026-05-20
- Fig. 2 meta_block annotation: "MessagePack" -> "JSON (Phase 1; MessagePack contemplated for Phase 2)" (cross-reconciliation with 07-detailed-description.md §8.3).
- Fig. 2 header annotation: "zstd-compressed JSON" -> "JSON (Phase 1; zstd compression contemplated for Phase 2)" (cross-reconciliation with 07-detailed-description.md §8.5).
- Fig. 2 attic_block annotation: marked "contemplated for Phase 2; not present at SHA 17334f8" (cross-reconciliation with 07-detailed-description.md §8.7).
- Fig. 2 caption: updated parse/decompress wording to reflect Phase 1 JSON / Phase 2 zstd demarcation.
- Fig. 6 step 803: formula corrected from `level = floor((v_i + 1) / 2 * L)` to `level = round((v_i + 1) * (L - 1) / 2)` (cross-reconciliation with 07-detailed-description.md §5.2.1).
- Fig. 6 step 805: phantom "bind levelHV_i with permute(i) of a per-dimension key vector" step deleted (the actual code XORs level hypervectors directly without permutation per `core-ts/src/encode.ts` and `core-rs/src/encode.rs`).
- Fig. 6 step 806: clarified that the d level hypervectors are XOR-accumulated (no tiebreaker invocation, since XOR has no ties).
-->

# List of Figures (Drawings)

The accompanying drawings illustrate exemplary embodiments of the invention. They are not intended to limit the scope of the claims. Reference numerals are consistent across the drawings and the detailed description: 100-series numerals identify encoders; 200-series numerals identify primitives; 300-series numerals identify the in-memory substrate; 400-series numerals identify KMF wire-format fields; 500-series numerals identify the persistence-adapter interface; 600-series numerals identify adapter implementations; 700-series numerals identify conformance bindings; and 800-series numerals identify embedding-encoder steps.

---

## Fig. 1 — System Architecture (Layered)

```
+--------------------------------------------------------------------+
|                       APPLICATION LAYER (90)                       |
|        (semantic notebook, agent memory, RAG, search UI)           |
+--------------------------------------------------------------------+
                                  |
                                  v
+--------------------------------------------------------------------+
|                       ENCODER LAYER (100)                          |
|  +-------------+  +-------------+  +-------------+  +-----------+  |
|  | String      |  | Embedding   |  | Bag-of-     |  | Word /    |  |
|  | encoder     |  | (level/RP)  |  | words text  |  | char      |  |
|  | (110)       |  | encoder     |  | encoder     |  | n-gram    |  |
|  |             |  | (120)       |  | (130)       |  | (140)     |  |
|  +-------------+  +-------------+  +-------------+  +-----------+  |
+--------------------------------------------------------------------+
                                  |
                                  v
+--------------------------------------------------------------------+
|                  PRIMITIVE LAYER (200)                             |
|  +---------+  +-------+  +---------+  +-----------+ +------------+ |
|  | bundle  |  | bind  |  | permute |  | similarity| | randomHV   | |
|  | (210)   |  | (220) |  | (230)   |  | (240)     | | (250)      | |
|  +---------+  +-------+  +---------+  +-----------+ +------------+ |
|       ^                                                            |
|       |                                                            |
|  +----------------+   tiebreaker invoked by bundle (210)           |
|  | tiebreaker     |     BLAKE3(tag || D || i || n)[0] & 1          |
|  | (260)          |                                                |
|  +----------------+                                                |
+--------------------------------------------------------------------+
                                  |
                                  v
+--------------------------------------------------------------------+
|                  IN-MEMORY SUBSTRATE (300)                         |
|     items: Map<UUIDv7, Item{ key, value, tags, metadata,           |
|                              createdAt, accessCount,               |
|                              lastAccessedAt }>                     |
|     cleanup-memory index (310)                                     |
|     co-activation tracker (320)                                    |
+--------------------------------------------------------------------+
                                  |
              recall() <---+      |       +---> consolidate() (330)
                           |      |       |
                           |      v       |
                          read   write   read+write
                                  |
                                  v
+--------------------------------------------------------------------+
|                  KMF SERIALISER (400)                              |
|     packs hv_block (410), meta_block (420), value_block (430),     |
|     attic_block (440); computes per-block BLAKE3 digest (450);     |
|     emits header (460) and trailer magic (470)                     |
+--------------------------------------------------------------------+
                                  |
                                  v
+--------------------------------------------------------------------+
|                  PERSISTENCE ADAPTER (500)                         |
|     read_snapshot (510), write_snapshot (520),                     |
|     append_wal (530), read_wal (540),                              |
|     truncate_wal (550), close (560)                                |
+--------------------------------------------------------------------+
                                  |
       +--------------+-----------+----------+----------------+
       v              v                      v                v
+-----------+   +-----------+         +-----------+    +------------+
| Memory    |   | Filesystem|         | SQLite    |    | IndexedDB  |
| adapter   |   | adapter   |         | adapter   |    | adapter    |
| (610)     |   | (620)     |         | (630)     |    | (640)      |
+-----------+   +-----------+         +-----------+    +------------+
```

*Data flow arrows are downward on the write path and upward on the read path. Encoders (100) consume application inputs; primitives (200) operate on hypervectors; the in-memory substrate (300) tracks items, the cleanup-memory index, and the co-activation counters; the KMF serialiser (400) emits a wire-format byte stream verified by per-block BLAKE3 digests (450); and the persistence adapter (500) abstracts the backing store.*

---

## Fig. 2 — KMF File Layout

```
offset
  0 +--------------------------------------------------------------+
    |  Magic header (410):  "KMF\x00"                4 bytes       |
    +--------------------------------------------------------------+
  4 |  Spec-version string (411):  "0.1.0"           6 bytes       |
    +--------------------------------------------------------------+
 10 |  Header offset (412):  u64 LE                  8 bytes       |
    +--------------------------------------------------------------+
 18 |                                                              |
    |  hv_block #0 (420):                                          |
    |     n_0 hypervectors, each ceil(D/8) bytes,                  |
    |     MSB-first packed.                                        |
    |     BLAKE3 (450) recorded in header (460), not inline.       |
    |                                                              |
    +--------------------------------------------------------------+
    |  meta_block #0 (430):                                        |
    |     n_0 rows of { id, tags, metadata, createdAt,             |
    |     accessCount, lastAccessedAt } in JSON (Phase 1;          |
    |     MessagePack contemplated for Phase 2).                   |
    +--------------------------------------------------------------+
    |  value_block #0 (440):                                       |
    |     n_0 value payloads, each length-prefixed by u32 LE.      |
    +--------------------------------------------------------------+
    |  ... further hv_block / meta_block / value_block triples,    |
    |  one triple per snapshot chunk ...                           |
    +--------------------------------------------------------------+
    |  attic_block (445):  cold-summary entries                    |
    |     produced by §5.3 consolidation                           |
    |     (contemplated for Phase 2; not present at SHA 17334f8).  |
    +--------------------------------------------------------------+
H = +--------------------------------------------------------------+
    |  Header (460), JSON (Phase 1; zstd compression of the        |
    |  said JSON contemplated for Phase 2):                        |
    |    {                                                         |
    |      "dimension":  10000,                                    |
    |      "item_count": <u32>,                                    |
    |      "created_at": <u64 unix-millis>,                        |
    |      "spec_version": "0.1.0",                                |
    |      "index": [                                              |
    |         { "kind": "hv_block",                                |
    |           "offset": 18,                                      |
    |           "length": <u64>,                                   |
    |           "blake3": <32-byte hex> },                         |
    |         ...                                                  |
    |      ]                                                       |
    |    }                                                         |
    +--------------------------------------------------------------+
E = +--------------------------------------------------------------+
    |  Trailer magic (470):  "FMK\x00"               4 bytes       |
    +--------------------------------------------------------------+
                                                          (EOF = E+4)
```

*The reader first seeks to `EOF − 4`, verifies the trailer magic (470), then reads the header-offset field (412) at byte 10, seeks to the header (460), parses the JSON (in Phase 2, additionally decompresses zstd), and verifies each block's BLAKE3 digest (450) before deserialising the block. The trailer magic must be checked before any offset is trusted.*

---

## Fig. 3 — Deterministic Tiebreaker Flowchart

```
                +----------------------------------+
                |  Input: (D, bit_index i,         |
                |          multiplicity n)         |
                |              (step 301)          |
                +----------------------------------+
                                 |
                                 v
                +----------------------------------+
                |  Build input byte string (302):  |
                |    tag || D_le_u32 || i_le_u32   |
                |        || n_le_u32               |
                |  where tag = "smritidb/tiebreak" |
                +----------------------------------+
                                 |
                                 v
                +----------------------------------+
                |  Compute BLAKE3 digest (303)     |
                |    H = BLAKE3(input)             |
                +----------------------------------+
                                 |
                                 v
                +----------------------------------+
                |  Take first byte H[0] (304)      |
                +----------------------------------+
                                 |
                                 v
                +----------------------------------+
                |  Mask LSB: bit = H[0] & 1 (305)  |
                +----------------------------------+
                                 |
                                 v
                +----------------------------------+
                |  Output bit (306)                |
                +----------------------------------+

Pseudocode equivalent:
    fn tiebreak(D: u32, i: u32, n: u32) -> u8 {
        let mut buf = Vec::new();
        buf.extend_from_slice(b"smritidb/tiebreak");
        buf.extend_from_slice(&D.to_le_bytes());
        buf.extend_from_slice(&i.to_le_bytes());
        buf.extend_from_slice(&n.to_le_bytes());
        let h = blake3::hash(&buf);
        h.as_bytes()[0] & 1
    }
```

*The tiebreaker (260) is invoked by the bundle primitive (210) only when, at a given bit position, the count of 1-bits across the input multiset is exactly equal to half the multiplicity. The output bit is the least-significant bit of the first byte of the BLAKE3 digest of the tuple `(D, i, n)` with the fixed domain-separation tag.*

---

## Fig. 4 — Replayable Consolidation Flow

```
+-----------------+        +-----------------+
| substrate items |        | recall access   |
|     (300)       |        |   log (320)     |
+--------+--------+        +--------+--------+
         |                          |
         v                          v
   +-----------------------------------+
   | co-activation tracker             |
   |    c(a, b) sliding-window count   |
   |                                   |
   |  step 401: for each recall hit r, |
   |    for each pair (r, r') in       |
   |    window W, c(r, r') += 1        |
   +-----------------+-----------------+
                     |
                     v
   +-----------------------------------+
   | step 402: enumerate pairs (a, b)  |
   | with c(a, b) > pull_threshold     |
   +-----------------+-----------------+
                     |
                     v
   +-----------------------------------+
   | step 403: for each such (a, b):   |
   |   disagree = { i : a.key[i] !=    |
   |                   b.key[i] }      |
   +-----------------+-----------------+
                     |
                     v
   +-----------------------------------+
   | step 404: sort indices in         |
   |   disagree by BLAKE3(salt || i)   |
   |   ascending                       |
   +-----------------+-----------------+
                     |
                     v
   +-----------------------------------+
   | step 405: take first k positions  |
   |   where k = min(                  |
   |     floor(drift_cap * D),         |
   |     |disagree|)                   |
   +-----------------+-----------------+
                     |
                     v
   +-----------------------------------+
   | step 406: alternate-flip:         |
   |   even-indexed positions:         |
   |     flip a.key[i] toward b.key[i] |
   |   odd-indexed positions:          |
   |     flip b.key[i] toward a.key[i] |
   +-----------------+-----------------+
                     |
                     v
   +-----------------------------------+
   | step 407: emit ConsolidationReport|
   |   { pairs_processed,              |
   |     bits_flipped,                 |
   |     max_drift_observed }          |
   +-----------------------------------+
```

*The salt parameter to step 404 is supplied as part of the substrate configuration and is recorded in the KMF header so that the consolidation step is replayable from a snapshot together with the access log.*

---

## Fig. 5 — Conformance Verification Loop

```
                  +-----------------------------+
                  | Conformance corpus (660)    |
                  | tests/conformance/          |
                  |   golden.json               |
                  +-------------+---------------+
                                |
       +------------------------+------------------------+
       |              |              |              |    |
       v              v              v              v    v
  +---------+   +---------+   +---------+   +--------+ +--------+
  | TS      |   | Rust    |   | Python  |   | Kotlin | | Swift  |
  | binding |   | binding |   | binding |   | binding| | binding|
  |  (710)  |   |  (720)  |   |  (730)  |   |  (740) | |  (750) |
  +----+----+   +----+----+   +----+----+   +---+----+ +---+----+
       |             |             |            |          |
       v             v             v            v          v
   produce       produce       produce       produce    produce
   output        output        output        output     output
   bytes         bytes         bytes         bytes      bytes
       |             |             |            |          |
       v             v             v            v          v
   +------+      +------+      +------+      +------+   +------+
   |SHA256|      |SHA256|      |SHA256|      |SHA256|   |SHA256|
   +--+---+      +--+---+      +--+---+      +--+---+   +--+---+
      |             |             |             |          |
      +-------------+-------------+-------------+----------+
                                |
                                v
                  +------------------------------+
                  | compare to expected SHA-256  |
                  | in golden.json (step 770)    |
                  +-------------+----------------+
                                |
                       +--------+--------+
                       |                 |
                       v                 v
                   +-------+          +-------+
                   | PASS  |          | FAIL  |
                   | (780) |          | (790) |
                   +-------+          +-------+
```

*Each candidate binding is executed independently against the same corpus inputs and required to produce a binary output whose SHA-256 digest matches the digest pre-computed by the reference TypeScript implementation. A FAIL outcome is a defect in the candidate binding, not in the specification.*

---

## Fig. 6 — Thermometer + Random Projection Encoder

```
+---------------------------------------------------+
| Input: f32 vector v of length d                   |
| (step 801)                                        |
+---------------------+-----------------------------+
                      |
                      v
+---------------------------------------------------+
| Clamp each component to [-1, 1]                   |
| (step 802)                                        |
+---------------------+-----------------------------+
                      |
                      v
+---------------------------------------------------+
| Quantise: level l_i = round((v_i + 1) *           |
|                              (L - 1) / 2)         |
| where L = 100 (configurable)                      |
| (step 803)                                        |
+---------------------+-----------------------------+
                      |
                      v
+---------------------------------------------------+
| For each (dimension i, level l_i):                |
|   seed_i = "lvl:" || i_le_u32 || ":" ||           |
|            l_i_le_u32                             |
|   levelHV_i = randomHV( BLAKE3(seed_i) )          |
| (step 804)                                        |
+---------------------+-----------------------------+
                      |
                      v
+---------------------------------------------------+
| XOR-accumulate the d level hypervectors           |
| into a single binary hypervector                  |
| (step 806)                                        |
+---------------------+-----------------------------+
                      |
                      v
+---------------------------------------------------+
| Output: binary hypervector of dimension D         |
| (step 807)                                        |
+---------------------------------------------------+
```

*Property: cosine similarity of two input vectors is approximately preserved as Hamming similarity of the output hypervectors, within a bounded error that decreases monotonically in `L` and in `D`.*

---

## Fig. 7 — Persistence Adapter Trait Surface

```
+--------------------------------------------------------+
|              <<interface>>                             |
|              PersistenceAdapter (500)                  |
+--------------------------------------------------------+
| + read_snapshot()   -> Option<KmfBytes>          (510) |
| + write_snapshot(b: KmfBytes) -> Result<(), Err> (520) |
| + append_wal(op: WalOp) -> Result<(), Err>       (530) |
| + read_wal()        -> Vec<WalOp>                (540) |
| + truncate_wal()    -> Result<(), Err>           (550) |
| + close()           -> Result<(), Err>           (560) |
+--------------------------------------------------------+
                          ^
                          | implements
       +------------------+------------------+----------------+
       |                  |                  |                |
+--------------+  +---------------+  +---------------+  +---------------+
| MemoryAdapter|  | FileSystem    |  | Sqlite        |  | IndexedDb     |
|     (610)    |  | Adapter (620) |  | Adapter (630) |  | Adapter (640) |
+--------------+  +---------------+  +---------------+  +---------------+
| RAM only;    |  | snapshot.kmf  |  | single-row    |  | snapshot in   |
| no I/O;      |  | + wal.log     |  | blob schema;  |  | one object    |
| zero-cost    |  | files on a    |  | WAL journal   |  | store; WAL in |
| for tests    |  | local POSIX-  |  | mode;         |  | a second      |
| and          |  | compatible    |  | better-       |  | object store. |
| ephemeral    |  | filesystem.   |  | sqlite3 peer  |  |               |
| use.         |  |               |  | dependency.   |  |               |
+--------------+  +---------------+  +---------------+  +---------------+
```

*Additional adapters may be supplied by third parties by implementing the said interface; the substrate is, by design, agnostic to the choice of adapter.*
