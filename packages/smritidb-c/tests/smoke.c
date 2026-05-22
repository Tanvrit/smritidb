/*
 * Standalone C smoke test for the smritidb-c ABI.
 *
 * Build (after `cargo build --release` in the parent crate):
 *
 *   clang -I../include -L../target/release -lsmritidb_c -Wl,-rpath,../target/release \
 *     tests/smoke.c -o tests/smoke && tests/smoke
 *
 * Exits 0 on success; non-zero otherwise with a diagnostic on stderr.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "smritidb.h"

static int fail(const char *what)
{
    const char *msg = smritidb_last_error();
    fprintf(stderr, "FAIL: %s: %s\n", what, msg ? msg : "(no error message)");
    return 1;
}

int main(void)
{
    SmritidbStore *store = smritidb_open_memory(0);
    if (!store)
        return fail("smritidb_open_memory");

    const char *keys[] = {"alpha", "beta", "gamma"};
    const char *vals[] = {"a", "b", "c"};
    for (int i = 0; i < 3; i++) {
        char *id = NULL;
        int rc = smritidb_put(store, keys[i], (const uint8_t *)vals[i], strlen(vals[i]), &id);
        if (rc != SMRITIDB_STATUS_OK) {
            smritidb_close(store);
            return fail("smritidb_put");
        }
        if (!id) {
            smritidb_close(store);
            fprintf(stderr, "FAIL: smritidb_put produced NULL id\n");
            return 1;
        }
        smritidb_free_string(id);
    }

    size_t n = smritidb_size(store);
    if (n != 3) {
        smritidb_close(store);
        fprintf(stderr, "FAIL: smritidb_size = %zu, expected 3\n", n);
        return 1;
    }

    SmritidbMatch *matches = NULL;
    size_t count = 0;
    int rc = smritidb_recall(store, "alpha", 5, 0.0, &matches, &count);
    if (rc != SMRITIDB_STATUS_OK) {
        smritidb_close(store);
        return fail("smritidb_recall");
    }
    if (count == 0) {
        smritidb_free_matches(matches, count);
        smritidb_close(store);
        fprintf(stderr, "FAIL: smritidb_recall returned zero matches\n");
        return 1;
    }
    printf("OK: top match id=%s similarity=%.4f size=%zu count=%zu spec=%s\n",
           matches[0].id, matches[0].similarity, n, count, smritidb_spec_version());

    smritidb_free_matches(matches, count);
    smritidb_close(store);
    return 0;
}
