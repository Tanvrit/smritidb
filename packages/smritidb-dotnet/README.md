# Smritidb (.NET)

.NET binding for the Smritidb associative store. P/Invoke over the
`libsmritidb_c` shared library produced by `packages/smritidb-c`.

## Status

Verified building and passing the full test suite on:

- macOS (arm64), .NET SDK **8.0.125** (installed via `brew install dotnet@8`).

The project targets `net8.0` and is forward-compatible with .NET 9 and 10
runtimes; only the `net8.0` framework is required to consume the package.

## Build prerequisites

1. Build the C ABI shared library:

   ```sh
   cd packages/smritidb-c
   cargo build --release
   ```

   This produces:
   - macOS:    `target/release/libsmritidb_c.dylib`
   - Linux:    `target/release/libsmritidb_c.so`
   - Windows:  `target/release/smritidb_c.dll`

2. Install .NET 8 SDK or later (https://dotnet.microsoft.com/download).

   On macOS the easiest no-sudo install is the Homebrew formula:

   ```sh
   brew install dotnet@8
   # Per the formula's caveat:
   export DOTNET_ROOT="/opt/homebrew/opt/dotnet@8/libexec"
   export PATH="/opt/homebrew/opt/dotnet@8/bin:$PATH"
   ```

   Note: the `dotnet-sdk` Homebrew **cask** installs a `.pkg` and requires
   `sudo`. The `dotnet@8` **formula** above does not.

## Build / Test

```sh
cd packages/smritidb-dotnet
dotnet restore
dotnet build --configuration Release
dotnet test  --configuration Release
```

## Native library resolution

The package installs a `NativeLibrary.SetDllImportResolver` hook that
searches, in order:

1. `$SMRITIDB_LIB` — absolute path to the library file (highest priority).
2. `$SMRITIDB_C_LIB_DIR` — directory containing the library file.
3. Next to the managed assembly (e.g. when the native binary is co-deployed).
4. `runtimes/native/libsmritidb_c.{dylib,so,dll}` (NuGet RID-style layout).
5. Repo-relative paths walking up 3–6 levels into
   `smritidb-c/target/release/` — covers both `src/Smritidb/bin/...` and
   `tests/Smritidb.Tests/bin/...`.
6. Platform default search (`LD_LIBRARY_PATH`, `DYLD_LIBRARY_PATH`, `PATH`).

### Platform-specific DllImport notes

- **macOS** — the resolver looks for `libsmritidb_c.dylib`. If it can't
  find the dylib via any search rule above, you can prepend it manually
  with:
  ```sh
  DYLD_LIBRARY_PATH=$(pwd)/../smritidb-c/target/release dotnet test
  ```
- **Linux** — file name `libsmritidb_c.so`. For `dotnet run`/`dotnet test`
  inside a workspace the repo-relative search already finds the build
  output; in deployed scenarios either copy the `.so` next to your
  executable or set `LD_LIBRARY_PATH`.
- **Windows** — file name `smritidb_c.dll` (no `lib` prefix). The simplest
  layout is to copy the DLL next to the consuming executable, or set
  `$env:SMRITIDB_C_LIB_DIR` to the directory holding it.

For deployed apps the most portable option is to ship the platform-specific
binary alongside the managed assembly (under `runtimes/<rid>/native/` for
NuGet, or simply next to your executable for self-contained publishes).

## Usage

```csharp
using Smritidb;

// In-memory store (default 10_000-dim hypervectors).
using var store = Store.OpenMemory();

var alphaId = store.Put("alpha", new byte[] { 0x41 });
store.Put("beta",  new byte[] { 0x42 });
store.Put("gamma", new byte[] { 0x43 });

foreach (var m in store.Recall("alpha", topK: 5, minSimilarity: 0.5))
{
    Console.WriteLine($"{m.Id} sim={m.Similarity:F3} bytes={m.Value.Length}");
}

// SQLite-backed persistence.
using var disk = Store.OpenSqlite("/tmp/smritidb.sqlite");
disk.Put("delta", new byte[] { 0x44 });
disk.Persist();
```

## Notes

- The wrapper copies all C-owned buffers (`SmritidbMatch.Value`, ids) into
  managed memory so the native heap is released immediately after each
  call.
- The `id` field on a `SmritidbMatch` is the UUID returned by `Store.Put`,
  not the user-supplied `key` argument (the key is the textual cue used
  to construct the hypervector).
- The C ABI is intentionally narrow; advanced operations (consolidate,
  tags, metadata) are not yet surfaced. Extend `packages/smritidb-c`
  first, then this binding.
