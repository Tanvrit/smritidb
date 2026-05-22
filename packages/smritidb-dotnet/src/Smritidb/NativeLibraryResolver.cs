// Resolves `libsmritidb_c` at runtime. By default we walk a handful of
// conventional locations relative to the assembly; the SMRITIDB_LIB env var
// (absolute path to the library file) or SMRITIDB_C_LIB_DIR env var
// (directory containing the library) overrides everything.

using System;
using System.IO;
using System.Reflection;
using System.Runtime.InteropServices;

namespace Smritidb;

internal static class NativeLibraryResolver
{
    private static int _installed;

    internal static void EnsureInstalled()
    {
        if (System.Threading.Interlocked.Exchange(ref _installed, 1) != 0)
        {
            return;
        }
        NativeLibrary.SetDllImportResolver(typeof(Store).Assembly, Resolve);
    }

    private static IntPtr Resolve(string name, Assembly assembly, DllImportSearchPath? path)
    {
        if (!string.Equals(name, NativeMethods.LibName, StringComparison.Ordinal))
        {
            return IntPtr.Zero;
        }

        // 1) Absolute file path override.
        var overridePath = Environment.GetEnvironmentVariable("SMRITIDB_LIB");
        if (!string.IsNullOrEmpty(overridePath) && File.Exists(overridePath))
        {
            return NativeLibrary.Load(overridePath);
        }

        // 2) Directory override.
        var dirOverride = Environment.GetEnvironmentVariable("SMRITIDB_C_LIB_DIR");
        if (!string.IsNullOrEmpty(dirOverride))
        {
            var candidate = Path.Combine(dirOverride, PlatformFileName());
            if (File.Exists(candidate))
            {
                return NativeLibrary.Load(candidate);
            }
        }

        foreach (var candidate in EnumerateCandidates())
        {
            if (File.Exists(candidate))
            {
                try
                {
                    return NativeLibrary.Load(candidate);
                }
                catch
                {
                    // try next
                }
            }
        }

        // Fall back to platform default search.
        return NativeLibrary.Load(name);
    }

    private static string PlatformFileName()
    {
        var ext = RuntimeInformation.IsOSPlatform(OSPlatform.OSX) ? ".dylib"
                : RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ? ".dll"
                : ".so";
        var prefix = RuntimeInformation.IsOSPlatform(OSPlatform.Windows) ? "" : "lib";
        return $"{prefix}smritidb_c{ext}";
    }

    private static System.Collections.Generic.IEnumerable<string> EnumerateCandidates()
    {
        var fileName = PlatformFileName();

        var asmDir = Path.GetDirectoryName(typeof(Store).Assembly.Location) ?? AppContext.BaseDirectory;
        yield return Path.Combine(asmDir, fileName);
        yield return Path.Combine(asmDir, "runtimes", "native", fileName);

        // Repo-relative for dev builds. Test assemblies live at
        // tests/Smritidb.Tests/bin/{Config}/{tfm}/, library assemblies at
        // src/Smritidb/bin/{Config}/{tfm}/. Cover both with multiple depths
        // up to the packages/smritidb-dotnet root, then over to smritidb-c.
        for (int up = 3; up <= 6; up++)
        {
            var parts = new System.Collections.Generic.List<string> { asmDir };
            for (int i = 0; i < up; i++) parts.Add("..");
            parts.Add("smritidb-c");
            parts.Add("target");
            parts.Add("release");
            parts.Add(fileName);
            yield return Path.GetFullPath(Path.Combine(parts.ToArray()));
        }
    }
}
