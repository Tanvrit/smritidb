import 'package:ffi/ffi.dart';

import 'library.dart';

/// SPEC.md version string the underlying core implements.
String specVersion() {
  final p = SmritidbBindings.instance.specVersion();
  if (p.address == 0) return '';
  return p.toDartString();
}
