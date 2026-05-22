import 'dart:typed_data';

import 'package:smritidb/smritidb.dart';
import 'package:test/test.dart';

void main() {
  group('SmritidbStore (in-memory)', () {
    test('round-trip put/recall/size', () {
      final store = SmritidbStore.openMemory();
      try {
        final items = <String, List<int>>{
          'alpha': [0x61],
          'beta': [0x62],
          'gamma': [0x63],
        };
        for (final entry in items.entries) {
          final id = store.put(entry.key, Uint8List.fromList(entry.value));
          expect(id, isNotEmpty);
        }
        expect(store.size, equals(3));

        final matches = store.recall('alpha', topK: 5, minSimilarity: 0.0);
        expect(matches, isNotEmpty);
        expect(matches.first.similarity, greaterThan(0.0));
        expect(matches.first.similarity, lessThanOrEqualTo(1.0));
      } finally {
        store.close();
      }
    });

    test('spec version is non-empty', () {
      expect(specVersion(), isNotEmpty);
    });
  });
}
