// High-level Smritidb store API for .NET. Wraps the unsafe P/Invoke calls
// in `NativeMethods` and surfaces a familiar IDisposable interface.

using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace Smritidb;

public sealed class SmritidbException : Exception
{
    public int Status { get; }
    public SmritidbException(int status, string message) : base(message)
    {
        Status = status;
    }
}

public sealed record SmritidbMatch(string Id, double Similarity, byte[] Value, uint AccessCount);

/// <summary>
/// A persistent Smritidb store.
/// </summary>
public sealed class Store : IDisposable
{
    private IntPtr _handle;

    static Store()
    {
        NativeLibraryResolver.EnsureInstalled();
    }

    private Store(IntPtr handle)
    {
        _handle = handle;
    }

    /// <summary>Opens an in-memory store. Pass 0 to use the core default dimension.</summary>
    public static Store OpenMemory(uint dimension = 0)
    {
        NativeLibraryResolver.EnsureInstalled();
        var h = NativeMethods.OpenMemory(dimension);
        if (h == IntPtr.Zero)
        {
            throw new SmritidbException(-1, LastError("open_memory returned NULL"));
        }
        return new Store(h);
    }

    /// <summary>Opens (or creates) a SQLite-backed store at <paramref name="path"/>.</summary>
    public static Store OpenSqlite(string path, uint dimension = 0)
    {
        NativeLibraryResolver.EnsureInstalled();
        var h = NativeMethods.OpenSqlite(path, dimension);
        if (h == IntPtr.Zero)
        {
            throw new SmritidbException(-1, LastError("open_sqlite returned NULL"));
        }
        return new Store(h);
    }

    public int Size
    {
        get
        {
            EnsureOpen();
            var n = NativeMethods.Size(_handle);
            return checked((int)(ulong)n);
        }
    }

    public uint Dimension
    {
        get
        {
            EnsureOpen();
            return NativeMethods.Dimension(_handle);
        }
    }

    public string Put(string key, byte[] value)
    {
        EnsureOpen();
        ArgumentNullException.ThrowIfNull(key);
        value ??= Array.Empty<byte>();

        IntPtr buffer = IntPtr.Zero;
        try
        {
            if (value.Length > 0)
            {
                buffer = Marshal.AllocHGlobal(value.Length);
                Marshal.Copy(value, 0, buffer, value.Length);
            }
            var rc = NativeMethods.Put(_handle, key, buffer, (UIntPtr)value.Length, out var outId);
            if (rc != NativeMethods.StatusOk)
            {
                throw new SmritidbException(rc, LastError($"put failed with status {rc}"));
            }
            try
            {
                return Marshal.PtrToStringUTF8(outId) ?? string.Empty;
            }
            finally
            {
                NativeMethods.FreeString(outId);
            }
        }
        finally
        {
            if (buffer != IntPtr.Zero) Marshal.FreeHGlobal(buffer);
        }
    }

    public IReadOnlyList<SmritidbMatch> Recall(string cue, uint topK = 10, double minSimilarity = 0.0)
    {
        EnsureOpen();
        ArgumentNullException.ThrowIfNull(cue);
        var rc = NativeMethods.Recall(_handle, cue, topK, minSimilarity, out var matchesPtr, out var countPtr);
        if (rc != NativeMethods.StatusOk)
        {
            throw new SmritidbException(rc, LastError($"recall failed with status {rc}"));
        }
        var count = (int)(ulong)countPtr;
        var results = new List<SmritidbMatch>(count);
        try
        {
            var stride = Marshal.SizeOf<NativeMethods.SmritidbMatch>();
            for (var i = 0; i < count; i++)
            {
                var entryPtr = IntPtr.Add(matchesPtr, i * stride);
                var m = Marshal.PtrToStructure<NativeMethods.SmritidbMatch>(entryPtr);
                var id = m.Id != IntPtr.Zero ? (Marshal.PtrToStringUTF8(m.Id) ?? string.Empty) : string.Empty;
                var len = (int)(ulong)m.Value.Len;
                var bytes = new byte[len];
                if (len > 0 && m.Value.Data != IntPtr.Zero)
                {
                    Marshal.Copy(m.Value.Data, bytes, 0, len);
                }
                results.Add(new SmritidbMatch(id, m.Similarity, bytes, m.AccessCount));
            }
        }
        finally
        {
            NativeMethods.FreeMatches(matchesPtr, countPtr);
        }
        return results;
    }

    public void Persist()
    {
        EnsureOpen();
        var rc = NativeMethods.Persist(_handle);
        if (rc != NativeMethods.StatusOk)
        {
            throw new SmritidbException(rc, LastError($"persist failed with status {rc}"));
        }
    }

    public void Dispose()
    {
        if (_handle != IntPtr.Zero)
        {
            NativeMethods.Close(_handle);
            _handle = IntPtr.Zero;
        }
    }

    public static string SpecVersion()
    {
        NativeLibraryResolver.EnsureInstalled();
        var p = NativeMethods.SpecVersion();
        return p == IntPtr.Zero ? string.Empty : Marshal.PtrToStringUTF8(p) ?? string.Empty;
    }

    private void EnsureOpen()
    {
        if (_handle == IntPtr.Zero)
        {
            throw new ObjectDisposedException(nameof(Store));
        }
    }

    private static string LastError(string fallback)
    {
        var p = NativeMethods.LastError();
        return p == IntPtr.Zero ? fallback : Marshal.PtrToStringUTF8(p) ?? fallback;
    }
}
