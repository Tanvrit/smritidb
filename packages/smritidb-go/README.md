# smritidb-go

Go binding for the Smritidb associative store. CGo over the
`packages/smritidb-c` ABI.

Confirmed working with **go 1.26.3** (`darwin/arm64`) against the
`smritidb-c` shared library produced by `cargo build --release`.

## Build prerequisites

1. Build the C ABI shared library:

   ```sh
   cd ../smritidb-c
   cargo build --release
   ```

   This produces `target/release/libsmritidb_c.{dylib,so}` and the header at
   `include/smritidb.h`. The Go binding references both via `#cgo` directives
   that key off `${SRCDIR}`, so the layout is intentionally fixed:

   ```
   packages/
     smritidb-c/include/smritidb.h
     smritidb-c/target/release/libsmritidb_c.{dylib,so}
     smritidb-go/...
   ```

2. Ensure Go 1.21+ and a working C toolchain (`xcode-select --install` on
   macOS, `build-essential` on Debian/Ubuntu).

## Usage

```go
package main

import (
    "fmt"
    "log"

    smritidb "github.com/kanervalabs/smritidb/packages/smritidb-go"
)

func main() {
    store, err := smritidb.OpenMemory(0) // 0 = core default (10_000)
    if err != nil { log.Fatal(err) }
    defer store.Close()

    if _, err := store.Put("alpha", []byte("alpha-val")); err != nil { log.Fatal(err) }
    matches, _ := store.Recall("alpha", 5, 0.5)
    for _, m := range matches {
        fmt.Println(m.ID, m.Similarity, string(m.Value))
    }
}
```

Swap `OpenMemory` for `OpenSqlite("/path/to/store.sqlite", 0)` to get a
durable, file-backed store with the same surface area.

## Test

```sh
cd packages/smritidb-go
go test -v ./...
```

The package's `#cgo` directives already point at
`${SRCDIR}/../smritidb-c/target/release` for `-L` and `-Wl,-rpath`, so no
extra environment is required when the in-tree layout is preserved.

## Consuming the package outside this tree

If you vendor or `go get` this package into another module, CGo can no
longer resolve the sibling C ABI through `${SRCDIR}` and the build will
fail with `ld: library 'smritidb_c' not found`. Build the C ABI yourself
and point Go at it explicitly:

```sh
# Build the C ABI (any checkout of smritidb works).
cd /path/to/smritidb/packages/smritidb-c
cargo build --release

# Then, in your own Go module, point CGo at the C ABI:
export CGO_CFLAGS="-I/path/to/smritidb/packages/smritidb-c/include"
export CGO_LDFLAGS="-L/path/to/smritidb/packages/smritidb-c/target/release -lsmritidb_c"

# At runtime (macOS), tell the dynamic loader where to find the dylib:
export DYLD_LIBRARY_PATH=/path/to/smritidb/packages/smritidb-c/target/release
# Linux equivalent:
export LD_LIBRARY_PATH=/path/to/smritidb/packages/smritidb-c/target/release

go build ./...
```

The header (`smritidb.h`) is the stable C ABI; any layout that exposes it
plus the matching `libsmritidb_c.{dylib,so}` works.

## Notes

- The binding uses `runtime.LockOSThread` around calls that touch the
  thread-local `smritidb_last_error` buffer.
- The `Store` carries a finalizer that calls `Close`; explicit `Close` is
  preferred for deterministic teardown.
- `Match.ID` is the item's generated UUID, not the UTF-8 key supplied to
  `Put`. Capture the id returned by `Put` if you need to correlate keys
  with subsequent recalls.
- The C ABI is intentionally narrow; advanced operations (consolidate,
  metadata, tags) are not yet surfaced here. Add them by extending
  `packages/smritidb-c` first.
