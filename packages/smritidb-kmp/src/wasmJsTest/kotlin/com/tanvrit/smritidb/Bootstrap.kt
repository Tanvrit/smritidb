package com.tanvrit.smritidb

/**
 * One-time setup that loads the wasm-bindgen Node bundle and stashes it
 * on `globalThis.SmritidbWasm`. The `@JsFun` bridges in
 * `PersistentStoreWasmJs.kt` and `SmritidbWasmJs.kt` read from there.
 *
 * Kotlin/Wasm emits ESM (`.mjs`) test bundles, so plain `require()` and
 * `__dirname` are not in scope inside a `@JsFun` closure. We synthesise
 * a CommonJS `require` on first run via `node:module`'s
 * `createRequire(...)`, accessed through `process.getBuiltinModule`
 * (Node ≥ 22).
 *
 * The closure returns a status string instead of throwing — Kotlin/Wasm
 * cannot directly catch a JS exception thrown across the `@JsFun`
 * boundary; the empty string signals success.
 */

private var bootstrapped = false

internal fun ensureWasmBootstrapped() {
    if (bootstrapped) return
    val err = bootstrapPkgNodeReturningError()
    if (err.isNotEmpty()) {
        throw IllegalStateException(
            "wasm bootstrap failed (rebuild via `gradle :wasmPackBuild`?): $err",
        )
    }
    bootstrapped = true
}

/**
 * Side-effect import of `fake-indexeddb/auto` so the Kotlin/Wasm
 * IndexedDB adapter can run under `wasmJsNodeTest` (Node has no native
 * `indexedDB`). Same idempotent pattern as the K/JS adapter — the
 * Node-side polyfill installs `globalThis.indexedDB` on first import,
 * and a fresh `IDBFactory` is constructed between tests to reset state.
 *
 * `fake-indexeddb` isn't currently in the wasmJsTest npm dependencies —
 * we resolve it via the same `createRequire` shim as the wasm-bindgen
 * module load. Adding it to the `wasmJsTest` source set's npm deps would
 * also work, but reusing the test runner's existing node_modules tree
 * (which inherits from `jsTest` since both use the same Yarn workspace
 * during the test runner build) avoids a second download.
 */
internal fun installFakeIndexedDbAutoWasm() {
    val err = installFakeIndexedDbReturningError()
    if (err.isNotEmpty()) {
        throw IllegalStateException("fake-indexeddb install failed: $err")
    }
}

internal fun resetIndexedDbWasm() {
    val err = resetIndexedDbReturningError()
    if (err.isNotEmpty()) {
        throw IllegalStateException("fake-indexeddb reset failed: $err")
    }
}

@JsFun(
    """() => {
        try {
            if (typeof process === 'undefined') {
                return 'process is undefined; fake-indexeddb only supports Node';
            }
            if (typeof process.getBuiltinModule !== 'function') {
                return 'process.getBuiltinModule is missing; need Node 22.1+';
            }
            const Module = process.getBuiltinModule('node:module');
            const req = Module.createRequire(process.cwd() + '/');
            // The wasmJsTest's bundle does not have fake-indexeddb in
            // its npm deps (the kotlin-js-store pulls jsTest's tree).
            // Try a few well-known locations.
            const path = req('path');
            const fs = req('fs');
            const candidates = [];
            // fake-indexeddb v6 ships an `auto/` subfolder, not an
            // `auto.js` file. Walk upward from cwd and try every layout
            // the Kotlin test runner / IDE / a direct Gradle invocation
            // might land us in.
            const suffixes = [
                'node_modules/fake-indexeddb/auto/index.js',
                'build/js/node_modules/fake-indexeddb/auto/index.js',
                'packages/smritidb-kmp/build/js/node_modules/fake-indexeddb/auto/index.js',
            ];
            let dir = process.cwd();
            for (let i = 0; i < 12; i++) {
                for (const suf of suffixes) candidates.push(path.resolve(dir, suf));
                const parent = path.dirname(dir);
                if (parent === dir) break;
                dir = parent;
            }
            const errors = [];
            for (const c of candidates) {
                if (!fs.existsSync(c)) continue;
                try {
                    req(c);
                    return '';
                } catch (e) {
                    errors.push(c + ': ' + (e && e.message));
                }
            }
            return 'no fake-indexeddb found; tried ' + (errors.join(' | ') || '(none existed)');
        } catch (e) {
            return 'fake-indexeddb install threw: ' + ((e && e.message) || String(e));
        }
    }""",
)
internal external fun installFakeIndexedDbReturningError(): String

@JsFun(
    """() => {
        try {
            if (typeof process === 'undefined') return '';
            const Module = process.getBuiltinModule('node:module');
            const req = Module.createRequire(process.cwd() + '/');
            const path = req('path');
            const fs = req('fs');
            // fake-indexeddb v6's main entry is build/cjs/fakeIndexedDB.js
            // and exposes the `FDBFactory` constructor at
            // build/cjs/FDBFactory.js. We can re-import the latter and
            // assign a fresh instance to `globalThis.indexedDB`.
            const suffixes = [
                'node_modules/fake-indexeddb/build/cjs/FDBFactory.js',
                'build/js/node_modules/fake-indexeddb/build/cjs/FDBFactory.js',
                'packages/smritidb-kmp/build/js/node_modules/fake-indexeddb/build/cjs/FDBFactory.js',
            ];
            let dir = process.cwd();
            for (let i = 0; i < 12; i++) {
                for (const suf of suffixes) {
                    const c = path.resolve(dir, suf);
                    if (!fs.existsSync(c)) continue;
                    try {
                        const mod = req(c);
                        const Factory = mod.default || mod.FDBFactory || mod;
                        globalThis.indexedDB = new Factory();
                        return '';
                    } catch (e) {
                        // try next candidate
                    }
                }
                const parent = path.dirname(dir);
                if (parent === dir) break;
                dir = parent;
            }
            return 'could not reset fake-indexeddb';
        } catch (e) {
            return 'reset threw: ' + ((e && e.message) || String(e));
        }
    }""",
)
internal external fun resetIndexedDbReturningError(): String

@JsFun(
    """() => {
        try {
            if (typeof process === 'undefined') {
                return 'process is undefined; bootstrap only supports Node';
            }
            // `process.getBuiltinModule('node:module')` lands the
            // builtin synchronously (Node ≥ 22). Older Nodes would need
            // a top-level await of `import('node:module')`, which
            // Kotlin/Wasm's `@JsFun` can't wait on.
            if (typeof process.getBuiltinModule !== 'function') {
                return 'process.getBuiltinModule is missing; need Node 22+ (have ' + process.version + ')';
            }
            const Module = process.getBuiltinModule('node:module');
            if (!Module || typeof Module.createRequire !== 'function') {
                return 'node:module did not expose createRequire';
            }
            const req = Module.createRequire(process.cwd() + '/');
            const path = req('path');
            const fs = req('fs');
            // Walk upward from cwd looking for either `core-rs/pkg-node/...`
            // (the layout when cwd is inside `packages/`) or
            // `packages/core-rs/pkg-node/...` (cwd at repo root). The
            // K/Wasm test runner picks cwd =
            // `build/js/packages/smritidb-kmp-wasm-js-test` which is four
            // levels deep into `packages/smritidb-kmp/build/...`, so a
            // generous upward walk (8 levels) covers every layout that
            // Gradle, Yarn, and IDE invocations produce.
            const candidates = [];
            let dir = process.cwd();
            for (let i = 0; i < 10; i++) {
                candidates.push(path.resolve(dir, 'core-rs/pkg-node/smritidb_core.js'));
                candidates.push(path.resolve(dir, 'packages/core-rs/pkg-node/smritidb_core.js'));
                const parent = path.dirname(dir);
                if (parent === dir) break;
                dir = parent;
            }
            const errors = [];
            for (const c of candidates) {
                if (!fs.existsSync(c)) continue;
                try {
                    globalThis.SmritidbWasm = req(c);
                    return '';
                } catch (e) {
                    errors.push(c + ': ' + (e && e.message));
                }
            }
            return 'no wasm bundle found; cwd=' + process.cwd() + '; tried ' + (errors.join(' | ') || '(none of ' + candidates.length + ' candidates existed)');
        } catch (e) {
            return 'bootstrap threw: ' + ((e && e.message) || String(e));
        }
    }""",
)
internal external fun bootstrapPkgNodeReturningError(): String
