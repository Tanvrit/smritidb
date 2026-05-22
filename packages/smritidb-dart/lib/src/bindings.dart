// Raw FFI bindings for `libsmritidb_c`. Mirrors `smritidb-c/include/smritidb.h`.
//
// Keep this file in sync with the cbindgen-generated header. The higher-level
// ergonomic API lives in `ffi.dart`.

import 'dart:ffi';
import 'package:ffi/ffi.dart';

// Status codes (mirroring SMRITIDB_STATUS_* in smritidb.h).
const int statusOk = 0;
const int statusInvalidArg = 1;
const int statusDimensionMismatch = 2;
const int statusValueTooLarge = 3;
const int statusNotFound = 4;
const int statusPersistence = 5;
const int statusCorruption = 6;
const int statusInternal = 7;

/// Opaque pointer to a `SmritidbStore` handle.
final class SmritidbStoreNative extends Opaque {}

/// `SmritidbBytes { uint8_t* data; uintptr_t len; uintptr_t cap; }`.
final class SmritidbBytesNative extends Struct {
  external Pointer<Uint8> data;
  @Size()
  external int len;
  @Size()
  external int cap;
}

/// `SmritidbMatch { char* id; double similarity; SmritidbBytes value; uint32_t access_count; }`.
final class SmritidbMatchNative extends Struct {
  external Pointer<Utf8> id;
  @Double()
  external double similarity;
  external SmritidbBytesNative value;
  @Uint32()
  external int accessCount;
}

// ---------------------------------------------------------------------------
// C function signatures (C side / Dart side).
// ---------------------------------------------------------------------------

typedef SmritidbOpenMemoryC = Pointer<SmritidbStoreNative> Function(Uint32);
typedef SmritidbOpenMemoryDart = Pointer<SmritidbStoreNative> Function(int);

typedef SmritidbOpenSqliteC = Pointer<SmritidbStoreNative> Function(
    Pointer<Utf8>, Uint32);
typedef SmritidbOpenSqliteDart = Pointer<SmritidbStoreNative> Function(
    Pointer<Utf8>, int);

typedef SmritidbPutC = Int32 Function(
  Pointer<SmritidbStoreNative>,
  Pointer<Utf8>,
  Pointer<Uint8>,
  Size,
  Pointer<Pointer<Utf8>>,
);
typedef SmritidbPutDart = int Function(
  Pointer<SmritidbStoreNative>,
  Pointer<Utf8>,
  Pointer<Uint8>,
  int,
  Pointer<Pointer<Utf8>>,
);

typedef SmritidbRecallC = Int32 Function(
  Pointer<SmritidbStoreNative>,
  Pointer<Utf8>,
  Uint32,
  Double,
  Pointer<Pointer<SmritidbMatchNative>>,
  Pointer<Size>,
);
typedef SmritidbRecallDart = int Function(
  Pointer<SmritidbStoreNative>,
  Pointer<Utf8>,
  int,
  double,
  Pointer<Pointer<SmritidbMatchNative>>,
  Pointer<Size>,
);

typedef SmritidbSizeC = Size Function(Pointer<SmritidbStoreNative>);
typedef SmritidbSizeDart = int Function(Pointer<SmritidbStoreNative>);

typedef SmritidbDimensionC = Uint32 Function(Pointer<SmritidbStoreNative>);
typedef SmritidbDimensionDart = int Function(Pointer<SmritidbStoreNative>);

typedef SmritidbCloseC = Void Function(Pointer<SmritidbStoreNative>);
typedef SmritidbCloseDart = void Function(Pointer<SmritidbStoreNative>);

typedef SmritidbPersistC = Int32 Function(Pointer<SmritidbStoreNative>);
typedef SmritidbPersistDart = int Function(Pointer<SmritidbStoreNative>);

typedef SmritidbFreeStringC = Void Function(Pointer<Utf8>);
typedef SmritidbFreeStringDart = void Function(Pointer<Utf8>);

typedef SmritidbFreeMatchesC = Void Function(
    Pointer<SmritidbMatchNative>, Size);
typedef SmritidbFreeMatchesDart = void Function(
    Pointer<SmritidbMatchNative>, int);

typedef SmritidbLastErrorC = Pointer<Utf8> Function();
typedef SmritidbLastErrorDart = Pointer<Utf8> Function();

typedef SmritidbSpecVersionC = Pointer<Utf8> Function();
typedef SmritidbSpecVersionDart = Pointer<Utf8> Function();
