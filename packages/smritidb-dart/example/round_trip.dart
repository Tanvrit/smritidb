import 'dart:typed_data';

import 'package:smritidb/smritidb.dart';

void main() {
  final store = SmritidbStore.openMemory();
  try {
    store.put('alpha', Uint8List.fromList([0x61]));
    store.put('beta', Uint8List.fromList([0x62]));
    store.put('gamma', Uint8List.fromList([0x63]));
    print('size = ${store.size}');
    final matches = store.recall('alpha', topK: 5, minSimilarity: 0.5);
    for (final m in matches) {
      print('${m.id}  sim=${m.similarity.toStringAsFixed(3)}  '
          'value=${m.value}');
    }
  } finally {
    store.close();
  }
}
