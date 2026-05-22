#!/usr/bin/env bash
# Smritidb cross-binding benchmark orchestrator.
#
# Runs each binding's benchmark sequentially on the same machine in a single
# session. Collects results/<lang>.json files and renders RESULTS.md.
#
# Any binding that fails to run is marked "N/A" in the final table — we never
# fabricate numbers.

set -u  # exit on unset; we explicitly do NOT use -e because we want partial
        # results when one binding's toolchain is broken.

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"

RESULTS_DIR="$HERE/results"
mkdir -p "$RESULTS_DIR"

# We don't want a runaway browser update or pip-mirror fetch to perturb the
# benchmark. Best-effort: set the no-network env vars several tools honour.
export PIP_NO_INDEX=1
export HOMEBREW_NO_AUTO_UPDATE=1
export GRADLE_OPTS="${GRADLE_OPTS:-} -Dorg.gradle.welcome=never"

print_section() {
    echo
    echo "===== $1 ====="
    echo
}

binding_status=()

# -- Rust ------------------------------------------------------------------
print_section "Rust"
if command -v cargo >/dev/null 2>&1; then
    pushd "$HERE/rust" >/dev/null
    if cargo build --release --bin smritidb-bench 2>&1 | tail -5; then
        if "$HERE/rust/target/release/smritidb-bench" "$RESULTS_DIR/rust.json"; then
            binding_status+=("rust:ok")
        else
            echo "rust: binary failed at runtime"
            binding_status+=("rust:run-failed")
        fi
    else
        echo "rust: cargo build failed"
        binding_status+=("rust:build-failed")
    fi
    popd >/dev/null
else
    echo "rust: cargo not installed — skipping."
    binding_status+=("rust:no-toolchain")
fi

# -- Python ----------------------------------------------------------------
print_section "Python"
PY="${SMRITIDB_BENCH_PYTHON:-$ROOT/.venv/bin/python}"
if [ -x "$PY" ]; then
    if "$PY" -c "import smritidb" 2>/dev/null; then
        if "$PY" "$HERE/python/run_all.py"; then
            binding_status+=("python:ok")
        else
            echo "python: run_all.py failed"
            binding_status+=("python:run-failed")
        fi
    else
        echo "python: smritidb module not importable from $PY — skipping."
        echo "  Hint: cd $ROOT && pip install -e packages/smritidb-py"
        binding_status+=("python:module-missing")
    fi
else
    echo "python: $PY not found — skipping."
    binding_status+=("python:no-toolchain")
fi

# -- TypeScript ------------------------------------------------------------
print_section "TypeScript"
if command -v node >/dev/null 2>&1; then
    pushd "$HERE/typescript" >/dev/null
    # The tsx import resolver picks up the workspace @tanvrit/smritidb via
    # pnpm's symlink farm; pnpm install must have run at the repo root first.
    # `--expose-gc` lets the memory bench take a heapUsed sample after a
    # synchronous Mark-Sweep, which is much less noisy than rss.
    if node --expose-gc --import tsx "$HERE/typescript/run-all.ts"; then
        binding_status+=("typescript:ok")
    else
        echo "typescript: run-all.ts failed"
        binding_status+=("typescript:run-failed")
    fi
    popd >/dev/null
else
    echo "typescript: node not installed — skipping."
    binding_status+=("typescript:no-toolchain")
fi

# -- Kotlin/JVM ------------------------------------------------------------
print_section "Kotlin/JVM"
KMP_JAR="$ROOT/packages/smritidb-kmp/build/libs/smritidb-kmp-jvm-0.1.0.jar"
if ! command -v gradle >/dev/null 2>&1; then
    echo "kotlin: gradle not installed — skipping."
    binding_status+=("kotlin:no-toolchain")
elif [ ! -f "$KMP_JAR" ]; then
    echo "kotlin: $KMP_JAR not found — skipping."
    echo "  Hint: cd $ROOT/packages/smritidb-kmp && gradle jvmJar"
    binding_status+=("kotlin:no-jar")
else
    pushd "$HERE/kotlin" >/dev/null
    if gradle --no-daemon benchmark --console=plain 2>&1 | tail -40; then
        binding_status+=("kotlin:ok")
    else
        echo "kotlin: gradle benchmark failed"
        binding_status+=("kotlin:run-failed")
    fi
    popd >/dev/null
fi

# -- Aggregate -------------------------------------------------------------
print_section "Aggregating results"
echo "binding statuses: ${binding_status[*]}"
echo
# The renderer is pure-Python with no smritidb dependency, so any python3
# on PATH suffices — even if $PY was empty above because the venv was missing.
RENDER_PY="${PY:-python3}"
if [ ! -x "$RENDER_PY" ]; then RENDER_PY="$(command -v python3)"; fi
"$RENDER_PY" "$HERE/scripts/render_results.py"

echo
echo "===== DONE ====="
echo "Per-binding JSON: $RESULTS_DIR/*.json"
echo "Rendered table:   $HERE/RESULTS.md"
