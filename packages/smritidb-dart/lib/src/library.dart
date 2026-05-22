// Library loading. Resolves `libsmritidb_c.{dylib,so,dll}` from the sibling
// `packages/smritidb-c/target/release` directory by default; override with
// `SMRITIDB_LIB` env var (absolute path) for non-default layouts.

import 'dart:ffi';
import 'dart:io';

import 'bindings.dart';

class SmritidbBindings {
  final DynamicLibrary lib;
  late final SmritidbOpenMemoryDart openMemory;
  late final SmritidbOpenSqliteDart openSqlite;
  late final SmritidbPutDart put;
  late final SmritidbRecallDart recall;
  late final SmritidbSizeDart size;
  late final SmritidbDimensionDart dimension;
  late final SmritidbCloseDart close;
  late final SmritidbPersistDart persist;
  late final SmritidbFreeStringDart freeString;
  late final SmritidbFreeMatchesDart freeMatches;
  late final SmritidbLastErrorDart lastError;
  late final SmritidbSpecVersionDart specVersion;

  SmritidbBindings._(this.lib) {
    openMemory = lib
        .lookupFunction<SmritidbOpenMemoryC, SmritidbOpenMemoryDart>(
            'smritidb_open_memory');
    openSqlite = lib
        .lookupFunction<SmritidbOpenSqliteC, SmritidbOpenSqliteDart>(
            'smritidb_open_sqlite');
    put = lib.lookupFunction<SmritidbPutC, SmritidbPutDart>('smritidb_put');
    recall = lib
        .lookupFunction<SmritidbRecallC, SmritidbRecallDart>('smritidb_recall');
    size = lib.lookupFunction<SmritidbSizeC, SmritidbSizeDart>('smritidb_size');
    dimension = lib.lookupFunction<SmritidbDimensionC, SmritidbDimensionDart>(
        'smritidb_dimension');
    close = lib
        .lookupFunction<SmritidbCloseC, SmritidbCloseDart>('smritidb_close');
    persist = lib.lookupFunction<SmritidbPersistC, SmritidbPersistDart>(
        'smritidb_persist');
    freeString = lib
        .lookupFunction<SmritidbFreeStringC, SmritidbFreeStringDart>(
            'smritidb_free_string');
    freeMatches = lib
        .lookupFunction<SmritidbFreeMatchesC, SmritidbFreeMatchesDart>(
            'smritidb_free_matches');
    lastError = lib
        .lookupFunction<SmritidbLastErrorC, SmritidbLastErrorDart>(
            'smritidb_last_error');
    specVersion = lib
        .lookupFunction<SmritidbSpecVersionC, SmritidbSpecVersionDart>(
            'smritidb_spec_version');
  }

  static SmritidbBindings? _cached;

  static SmritidbBindings get instance => _cached ??= SmritidbBindings._(_open());

  static DynamicLibrary _open() {
    final override = Platform.environment['SMRITIDB_LIB'];
    if (override != null && override.isNotEmpty) {
      return DynamicLibrary.open(override);
    }
    // Default layout: walk up from this file's source location at runtime —
    // we can't introspect that under `dart:ffi`, so use the conventional
    // repo-relative path from cwd. Callers may set SMRITIDB_LIB instead.
    final ext = _platformExt();
    final candidates = <String>[
      '../smritidb-c/target/release/libsmritidb_c$ext',
      'packages/smritidb-c/target/release/libsmritidb_c$ext',
      // Fall back to system search.
      'libsmritidb_c$ext',
      'smritidb_c$ext',
    ];
    for (final c in candidates) {
      try {
        return DynamicLibrary.open(c);
      } catch (_) {
        continue;
      }
    }
    throw StateError(
      'Could not locate libsmritidb_c. Set the SMRITIDB_LIB env var to the '
      'absolute path of the shared library.',
    );
  }

  static String _platformExt() {
    if (Platform.isMacOS) return '.dylib';
    if (Platform.isWindows) return '.dll';
    return '.so';
  }
}
