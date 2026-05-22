# smritidb-c

Stable C ABI over `smritidb-core`. Consumed by the Go, Dart/Flutter, and
.NET bindings in sibling packages.

## What it is

A narrow, deliberately minimal C surface — `open_memory` / `open_sqlite`,
`put`, `recall`, `size`, `close` — emitted as `libsmritidb_c.{dylib,so,dll}`
plus a checked-in header at `include/smritidb.h` regenerated via
[cbindgen](https://github.com/mozilla/cbindgen). It is **not** a public
Smritidb surface: apps should depend on the language-specific package
(`smritidb-go`, `smritidb-dart`, `smritidb-dotnet`) which polishes the
ergonomics, instead of linking this directly.

## Build

```sh
cd packages/smritidb-c
cargo build --release
```

This produces:

- `target/release/libsmritidb_c.dylib` (macOS) / `.so` (Linux) / `.dll` (Windows)
- `target/release/libsmritidb_c.a` for fully-static linking

## Regenerate the C header

The checked-in header at `include/smritidb.h` is the source of truth for
downstream bindings. Regenerate it via [cbindgen](https://github.com/mozilla/cbindgen):

```sh
cargo install cbindgen     # one-time
cbindgen --config cbindgen.toml --crate smritidb-c --output include/smritidb.h
```

Bump the `smritidb-c` crate version when the regenerated header changes
shape — downstream bindings pin against it.

## ABI overview

```c
typedef struct SmritidbStore SmritidbStore;

SmritidbStore* smritidb_open_memory(uint32_t dimension);
SmritidbStore* smritidb_open_sqlite(const char* path, uint32_t dimension);

int  smritidb_put(SmritidbStore*, const char* key,
                  const uint8_t* value, size_t value_len,
                  char** out_id);
int  smritidb_recall(SmritidbStore*, const char* cue, uint32_t top_k,
                     double min_similarity,
                     SmritidbMatch** out_matches, size_t* out_count);
size_t smritidb_size(const SmritidbStore*);
void   smritidb_close(SmritidbStore*);

const char* smritidb_last_error(void);   /* thread-local */

void smritidb_free_string(char*);
void smritidb_free_bytes(SmritidbBytes);
void smritidb_free_matches(SmritidbMatch*, size_t);
```

### Memory ownership

- `out_*` parameters allocate and transfer ownership to the caller. Use the
  matching `smritidb_free_*` to release.
- The opaque `SmritidbStore*` handle is freed exclusively via
  `smritidb_close`.
- `smritidb_last_error()` returns a thread-local pointer that is valid only
  until the next library call on the same thread.

### Error model

Functions returning `int` use `SMRITIDB_STATUS_*` codes; `0` is OK. Pointer
returns use `NULL` for failure. Diagnostics are stashed in a thread-local
slot reachable via `smritidb_last_error`.

## Docs

- API reference (rustdoc): [`docs/api/rust/doc/smritidb_c/`](../../docs/api/rust/doc/smritidb_c/index.html)
- Raw header: [`include/smritidb.h`](include/smritidb.h) — the source of truth for downstream bindings.

## Tests

**2 tests passing** (in-crate, exercising the C surface from Rust unsafe blocks) plus a C smoke harness under `tests/smoke.c`. Each language binding (Go, Dart, .NET) additionally ships its own smoke test that links against the built `cdylib`.

```sh
cargo test --release
cc tests/smoke.c -o /tmp/smritidb-smoke -L target/release -lsmritidb_c && /tmp/smritidb-smoke
```

## License

Apache-2.0. See the repo root [`LICENSE`](../../LICENSE) and [`NOTICE`](../../NOTICE).
