# Getting Started

Pick the binding for your stack. Every binding talks to the same Rust core
(except the TypeScript reference, which is a pure-TS reimplementation that
matches the Rust core byte-for-byte through the conformance corpus).

For the full method-by-method surface, follow the link at the end of each
section into the auto-generated API reference.

---

## Rust (`smritidb-core`)

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
smritidb-core = "0.1"
```

Hello world:

```rust
use smritidb_core::Store;

fn main() {
    let mut store = Store::new(10_000);
    let id = store.put("the quick brown fox", b"jumped over the lazy dog");

    for hit in store.recall("quick brown", 5, 0.5) {
        println!("{:.3}\t{}", hit.similarity, String::from_utf8_lossy(&hit.value));
    }

    let _ = id;
}
```

Open a SQLite-backed persistent store:

```rust
use smritidb_core::{open_persistent_store, SqliteAdapter};

let adapter = SqliteAdapter::open("/tmp/notes.smritidb")?;
let mut store = open_persistent_store(adapter, 10_000)?;
store.put("welcome", b"hello, persistent world");
store.persist()?;
```

API reference: [`api/rust/doc/smritidb_core/index.html`](../api/rust/doc/smritidb_core/index.html).

---

## C / C++ (`smritidb-c`)

Build the static / dynamic library:

```sh
cd packages/smritidb-c
cargo build --release
# -> target/release/libsmritidb_c.{a,dylib,so,dll}
```

Hello world (link against the dylib):

```c
#include "smritidb.h"
#include <stdio.h>

int main(void) {
    SmritidbStore* s = smritidb_open_memory(10000);
    char* id = NULL;
    smritidb_put(s, "the quick brown fox",
                 (const uint8_t*)"jumped over the lazy dog", 24, &id);

    SmritidbMatch* matches = NULL;
    uintptr_t n = 0;
    smritidb_recall(s, "quick brown", 5, 0.5, &matches, &n);
    for (uintptr_t i = 0; i < n; i++) {
        printf("%.3f\t%.*s\n", matches[i].similarity,
               (int)matches[i].value.len, matches[i].value.data);
    }

    smritidb_free_string(id);
    smritidb_free_matches(matches, n);
    smritidb_close(s);
    return 0;
}
```

The full C ABI is in
[`packages/smritidb-c/include/smritidb.h`](../../packages/smritidb-c/include/smritidb.h).
The rustdoc view is at
[`api/rust/doc/smritidb_c/index.html`](../api/rust/doc/smritidb_c/index.html).

---

## Python (`smritidb-py`)

Install (once published):

```sh
pip install smritidb
```

For development against this checkout:

```sh
python -m venv .venv && source .venv/bin/activate
pip install maturin
cd packages/smritidb-py && maturin develop --release
```

Hello world:

```python
from smritidb import Store, PersistentStore

# In-memory.
store = Store(dimension=10_000)
store.put("the quick brown fox", b"jumped over the lazy dog")
for hit in store.recall("quick brown", top_k=5, min_similarity=0.5):
    print(f"{hit['similarity']:.3f}\t{hit['value'].decode()}")

# SQLite-backed persistent.
with PersistentStore.open_sqlite("/tmp/notes.smritidb") as p:
    p.put("welcome", b"hello, persistent world")
    p.persist()
```

API reference: [`api/python/smritidb.html`](../api/python/smritidb.html).

---

## Kotlin (`smritidb-kmp`)

Add the dependency in your `build.gradle.kts`:

```kotlin
repositories {
    maven {
        url = uri("https://maven.pkg.github.com/Tanvrit/smritidb")
        credentials { /* GitHub token */ }
    }
}

dependencies {
    implementation("com.tanvrit:smritidb-kmp:0.1.0")
}
```

Hello world (JVM target):

```kotlin
import com.tanvrit.smritidb.Smritidb
import com.tanvrit.smritidb.PersistentStore

fun main() {
    val store = Smritidb(dimension = 10_000)
    store.put("the quick brown fox", "jumped over the lazy dog".toByteArray())
    for (hit in store.recall("quick brown", topK = 5, minSimilarity = 0.5)) {
        println("%.3f\t%s".format(hit.similarity, String(hit.value)))
    }

    PersistentStore.openSqlite("/tmp/notes.smritidb").use { p ->
        p.put("welcome", "hello, persistent world".toByteArray())
        p.persist()
    }
}
```

API reference: [`api/kotlin/index.html`](../api/kotlin/index.html).

---

## TypeScript (`core-ts`)

Install:

```sh
npm install @tanvrit/smritidb
```

Hello world:

```ts
import { Smritidb, openPersistentStore, sqliteAdapter } from "@tanvrit/smritidb";

const store = new Smritidb({ dimension: 10_000 });
store.put("the quick brown fox", new TextEncoder().encode("jumped over the lazy dog"));
for (const hit of store.recall("quick brown", { topK: 5, minSimilarity: 0.5 })) {
  console.log(hit.similarity.toFixed(3), new TextDecoder().decode(hit.value));
}

const persistent = await openPersistentStore({
  adapter: sqliteAdapter({ path: "/tmp/notes.smritidb" }),
  dimension: 10_000,
});
persistent.put("welcome", new TextEncoder().encode("hello, persistent world"));
await persistent.persist();
```

API reference: [`api/typescript/index.html`](../api/typescript/index.html).

---

## Next steps

- Read the [Cross-Language Interop](cross-language-interop.md) guide to see
  one SQLite file open in four languages.
- Read the [Persistence](persistence.md) guide for adapter selection and
  the unified SQLite schema story.
- If you are adding a new binding, run the
  [Conformance Corpus](conformance-corpus.md) to prove bit-equivalence.
