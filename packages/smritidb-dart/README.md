# smritidb (Dart / Flutter)

Dart / Flutter binding for the Smritidb associative store.

## What it is

A `dart:ffi` wrapper over the stable C ABI (`libsmritidb_c`) produced by `packages/smritidb-c`. Same bit-exact semantics as every other Smritidb binding — a SQLite file written by the Python or Rust binding opens identically here, and vice versa. The C ABI is intentionally narrow; consolidate / tag / metadata operations land here as the underlying ABI surface grows.

## Build prerequisites

1. Build the C ABI shared library:

   ```sh
   cd ../smritidb-c
   cargo build --release
   ```

   This produces `target/release/libsmritidb_c.{dylib,so}`.

2. Install dependencies:

   ```sh
   cd packages/smritidb-dart
   dart pub get
   ```

## Library resolution

By default the binding looks for the shared library in:

- `../smritidb-c/target/release/libsmritidb_c.{dylib,so,dll}`
- `packages/smritidb-c/target/release/libsmritidb_c.{dylib,so,dll}`
- the platform's default search path

Override with the `SMRITIDB_LIB` env var (absolute path).

## Usage

```dart
import 'dart:typed_data';
import 'package:smritidb/smritidb.dart';

final store = SmritidbStore.openMemory();
store.put('alpha', Uint8List.fromList([0x61]));
final matches = store.recall('alpha', topK: 5, minSimilarity: 0.5);
for (final m in matches) {
  print('${m.id} sim=${m.similarity}');
}
store.close();
```

## Test

```sh
dart pub get
dart test
```

Or analyse only (no runtime needed for the C lib):

```sh
dart analyze
```

## Flutter

The same package works under Flutter on macOS, Linux, and Android. You will
need to bundle `libsmritidb_c.{dylib,so}` with your app and set
`SMRITIDB_LIB` (or place it on the platform's library search path) so it
can be located at runtime. iOS support requires building a static library
and linking it directly into the Flutter app via Xcode.

## Notes

- The wrapper copies all C-owned buffers into Dart heap objects, so it is
  safe to drop the underlying `libsmritidb_c` pointer immediately.
- The C ABI is intentionally narrow; advanced operations (consolidate,
  tags, metadata) are not yet surfaced. Extend `packages/smritidb-c`
  first, then this binding.

## Docs

- Persistence architecture: [`docs/guides/persistence.md`](../../docs/guides/persistence.md)
- Cross-language interop: [`docs/guides/cross-language-interop.md`](../../docs/guides/cross-language-interop.md)

## Tests

Scaffolded — runs against the built `libsmritidb_c` cdylib. Run with `dart pub get && dart test`.

## License

Apache-2.0. See the repo root [`LICENSE`](../../../LICENSE) and [`NOTICE`](../../../NOTICE).
