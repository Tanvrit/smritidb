#!/usr/bin/env bash
# Regenerate every API doc target into docs/api/<lang>/.
#
# Run from the repository root. Continues past per-target failures so a
# missing toolchain doesn't block the others; reports a summary at the end.

set -uo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCS_DIR="$REPO_ROOT/docs"

# Track which targets succeeded so we can summarise at the end.
declare -a OK
declare -a FAIL

run_step() {
    local name="$1"; shift
    echo
    echo "==> $name"
    if "$@"; then
        OK+=("$name")
    else
        FAIL+=("$name")
        echo "    (continuing)"
    fi
}

# --- Rust (core-rs, smritidb-ffi, smritidb-c) -------------------------------
# `cargo doc --target-dir` shares a directory with cargo's normal build
# output, so `debug/` ends up alongside `doc/`. We only want `doc/` checked
# in; strip the build cache after the docs land.
gen_rust() {
    (cd "$REPO_ROOT/packages/core-rs" && \
        cargo doc --no-deps --no-default-features \
            --target-dir "$DOCS_DIR/api/rust") && \
    (cd "$REPO_ROOT/packages/smritidb-ffi" && \
        cargo doc --no-deps --target-dir "$DOCS_DIR/api/rust") && \
    (cd "$REPO_ROOT/packages/smritidb-c" && \
        cargo doc --no-deps --target-dir "$DOCS_DIR/api/rust") && \
    rm -rf "$DOCS_DIR/api/rust/debug" \
           "$DOCS_DIR/api/rust/.rustc_info.json" \
           "$DOCS_DIR/api/rust/.rustdoc_fingerprint.json" \
           "$DOCS_DIR/api/rust/.cargo-lock" \
           "$DOCS_DIR/api/rust/CACHEDIR.TAG" \
           "$DOCS_DIR/api/rust/doc/.lock"
}

# --- Python (smritidb-py) ---------------------------------------------------
gen_python() {
    local pdoc_bin
    if command -v pdoc >/dev/null 2>&1; then
        pdoc_bin="pdoc"
    elif [[ -x "$REPO_ROOT/.venv/bin/pdoc" ]]; then
        pdoc_bin="$REPO_ROOT/.venv/bin/pdoc"
    else
        echo "pdoc not found. Install with: pip install pdoc"
        return 1
    fi
    (cd "$REPO_ROOT/packages/smritidb-py" && \
        "$pdoc_bin" smritidb -o "$DOCS_DIR/api/python")
}

# --- Kotlin (smritidb-kmp) --------------------------------------------------
gen_kotlin() {
    # Dokka 1.9.20 cannot parse the JDK 25 version string. If a JDK 17 is
    # available locally, prefer it; otherwise let the user supply
    # $JAVA_HOME explicitly.
    if [[ -z "${JAVA_HOME:-}" ]]; then
        for candidate in \
            /Library/Java/JavaVirtualMachines/zulu-17.jdk/Contents/Home \
            /Library/Java/JavaVirtualMachines/jdk-17.jdk/Contents/Home \
            /usr/lib/jvm/java-17-openjdk-amd64
        do
            if [[ -d "$candidate" ]]; then
                export JAVA_HOME="$candidate"
                echo "    using JAVA_HOME=$JAVA_HOME"
                break
            fi
        done
    fi
    (cd "$REPO_ROOT/packages/smritidb-kmp" && \
        gradle --init-script "$DOCS_DIR/dokka-init.gradle.kts" \
               dokkaHtml --no-daemon \
               -x wasmPackBuildWeb -x wasmPackBuildNode)
}

# --- TypeScript (core-ts) ---------------------------------------------------
gen_typescript() {
    (cd "$REPO_ROOT/packages/core-ts" && \
        if [[ ! -x node_modules/.bin/typedoc ]]; then
            npm install --save-dev typedoc
        fi && \
        npx typedoc src/index.ts \
            --out "$DOCS_DIR/api/typescript" \
            --skipErrorChecking)
}

run_step "Rust       (core-rs + ffi + c)" gen_rust
run_step "Python     (smritidb-py)"       gen_python
run_step "Kotlin     (smritidb-kmp)"      gen_kotlin
run_step "TypeScript (core-ts)"           gen_typescript

echo
echo "===== Summary ====="
for name in "${OK[@]}";   do echo "  PASS  $name"; done
for name in "${FAIL[@]}"; do echo "  FAIL  $name"; done

if [[ ${#FAIL[@]} -gt 0 ]]; then
    exit 1
fi
