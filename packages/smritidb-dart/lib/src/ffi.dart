// High-level, idiomatic Dart API over the raw FFI bindings.
//
// Errors are surfaced as `SmritidbException`. Buffers returned by the C
// library are copied into Dart-owned `Uint8List` / `String` values so the
// underlying heap memory can be released immediately.

import 'dart:ffi';
import 'dart:typed_data';

import 'package:ffi/ffi.dart';

import 'bindings.dart';
import 'library.dart';

class SmritidbException implements Exception {
  final int status;
  final String message;
  SmritidbException(this.status, this.message);
  @override
  String toString() => 'SmritidbException(status=$status): $message';
}

/// A recall hit.
class SmritidbMatch {
  final String id;
  final double similarity;
  final Uint8List value;
  final int accessCount;
  SmritidbMatch(this.id, this.similarity, this.value, this.accessCount);
}

String _lastError(SmritidbBindings b, String fallback) {
  final p = b.lastError();
  if (p == nullptr) return fallback;
  return p.toDartString();
}

class SmritidbStore {
  Pointer<SmritidbStoreNative> _handle;
  final SmritidbBindings _b;
  bool _closed = false;

  SmritidbStore._(this._handle, this._b);

  /// Opens an in-memory persistent store.
  factory SmritidbStore.openMemory({int dimension = 0}) {
    final b = SmritidbBindings.instance;
    final h = b.openMemory(dimension);
    if (h == nullptr) {
      throw SmritidbException(-1, _lastError(b, 'open_memory returned NULL'));
    }
    return SmritidbStore._(h, b);
  }

  /// Opens (or creates) a SQLite-backed persistent store at [path].
  factory SmritidbStore.openSqlite(String path, {int dimension = 0}) {
    final b = SmritidbBindings.instance;
    final cpath = path.toNativeUtf8();
    try {
      final h = b.openSqlite(cpath, dimension);
      if (h == nullptr) {
        throw SmritidbException(
            -1, _lastError(b, 'open_sqlite returned NULL'));
      }
      return SmritidbStore._(h, b);
    } finally {
      calloc.free(cpath);
    }
  }

  /// Number of items currently stored.
  int get size {
    _ensureOpen();
    return _b.size(_handle);
  }

  /// Hypervector dimension of the store.
  int get dimension {
    _ensureOpen();
    return _b.dimension(_handle);
  }

  /// Inserts (or updates) an item keyed by [key]. Returns the item id.
  String put(String key, Uint8List value) {
    _ensureOpen();
    final ckey = key.toNativeUtf8();
    final outId = calloc<Pointer<Utf8>>();
    Pointer<Uint8> valuePtr = nullptr;
    if (value.isNotEmpty) {
      valuePtr = calloc<Uint8>(value.length);
      for (var i = 0; i < value.length; i++) {
        valuePtr[i] = value[i];
      }
    }
    try {
      final rc = _b.put(_handle, ckey, valuePtr, value.length, outId);
      if (rc != statusOk) {
        throw SmritidbException(
            rc, _lastError(_b, 'put failed with status $rc'));
      }
      final idPtr = outId.value;
      final id = idPtr.toDartString();
      _b.freeString(idPtr);
      return id;
    } finally {
      calloc.free(ckey);
      if (valuePtr != nullptr) calloc.free(valuePtr);
      calloc.free(outId);
    }
  }

  /// Returns up to [topK] matches with similarity at least [minSimilarity].
  List<SmritidbMatch> recall(String cue,
      {int topK = 10, double minSimilarity = 0.0}) {
    _ensureOpen();
    final ccue = cue.toNativeUtf8();
    final outMatches = calloc<Pointer<SmritidbMatchNative>>();
    final outCount = calloc<Size>();
    try {
      final rc = _b.recall(
          _handle, ccue, topK, minSimilarity, outMatches, outCount);
      if (rc != statusOk) {
        throw SmritidbException(
            rc, _lastError(_b, 'recall failed with status $rc'));
      }
      final basePtr = outMatches.value;
      final count = outCount.value;
      final results = <SmritidbMatch>[];
      for (var i = 0; i < count; i++) {
        final m = (basePtr + i).ref;
        final id = m.id == nullptr ? '' : m.id.toDartString();
        final len = m.value.len;
        final bytes = Uint8List(len);
        for (var j = 0; j < len; j++) {
          bytes[j] = m.value.data[j];
        }
        results.add(SmritidbMatch(id, m.similarity, bytes, m.accessCount));
      }
      _b.freeMatches(basePtr, count);
      return results;
    } finally {
      calloc.free(ccue);
      calloc.free(outMatches);
      calloc.free(outCount);
    }
  }

  /// Forces any pending state to be flushed to the backing adapter.
  void persist() {
    _ensureOpen();
    final rc = _b.persist(_handle);
    if (rc != statusOk) {
      throw SmritidbException(
          rc, _lastError(_b, 'persist failed with status $rc'));
    }
  }

  /// Closes the store. Idempotent.
  void close() {
    if (_closed) return;
    _b.close(_handle);
    _handle = nullptr;
    _closed = true;
  }

  void _ensureOpen() {
    if (_closed || _handle == nullptr) {
      throw StateError('SmritidbStore is closed');
    }
  }
}
