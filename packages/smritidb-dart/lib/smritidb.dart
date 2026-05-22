/// Smritidb Dart binding.
///
/// Talks to the `libsmritidb_c` shared library produced by
/// `packages/smritidb-c` via `dart:ffi`.
library;

export 'src/ffi.dart' show SmritidbStore, SmritidbMatch, SmritidbException;
export 'src/spec.dart' show specVersion;
