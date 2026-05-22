// Raw P/Invoke bindings for libsmritidb_c. Mirrors
// packages/smritidb-c/include/smritidb.h.
//
// The runtime loads `libsmritidb_c` from the platform's library search path
// (or via a NativeLibrary.SetDllImportResolver hook installed in
// `Store.cs`).

using System;
using System.Runtime.InteropServices;

namespace Smritidb;

internal static class NativeMethods
{
    internal const string LibName = "smritidb_c";

    internal const int StatusOk = 0;
    internal const int StatusInvalidArg = 1;
    internal const int StatusDimensionMismatch = 2;
    internal const int StatusValueTooLarge = 3;
    internal const int StatusNotFound = 4;
    internal const int StatusPersistence = 5;
    internal const int StatusCorruption = 6;
    internal const int StatusInternal = 7;

    [StructLayout(LayoutKind.Sequential)]
    internal struct SmritidbBytes
    {
        public IntPtr Data;
        public UIntPtr Len;
        public UIntPtr Cap;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct SmritidbMatch
    {
        public IntPtr Id;        // char*
        public double Similarity;
        public SmritidbBytes Value;
        public uint AccessCount;
    }

    [DllImport(LibName, EntryPoint = "smritidb_last_error")]
    internal static extern IntPtr LastError();

    [DllImport(LibName, EntryPoint = "smritidb_open_memory")]
    internal static extern IntPtr OpenMemory(uint dimension);

    [DllImport(LibName, EntryPoint = "smritidb_open_sqlite", CharSet = CharSet.Ansi)]
    internal static extern IntPtr OpenSqlite([MarshalAs(UnmanagedType.LPUTF8Str)] string path, uint dimension);

    [DllImport(LibName, EntryPoint = "smritidb_put", CharSet = CharSet.Ansi)]
    internal static extern int Put(
        IntPtr store,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string key,
        IntPtr value,
        UIntPtr valueLen,
        out IntPtr outId);

    [DllImport(LibName, EntryPoint = "smritidb_recall", CharSet = CharSet.Ansi)]
    internal static extern int Recall(
        IntPtr store,
        [MarshalAs(UnmanagedType.LPUTF8Str)] string cue,
        uint topK,
        double minSimilarity,
        out IntPtr outMatches,
        out UIntPtr outCount);

    [DllImport(LibName, EntryPoint = "smritidb_size")]
    internal static extern UIntPtr Size(IntPtr store);

    [DllImport(LibName, EntryPoint = "smritidb_dimension")]
    internal static extern uint Dimension(IntPtr store);

    [DllImport(LibName, EntryPoint = "smritidb_persist")]
    internal static extern int Persist(IntPtr store);

    [DllImport(LibName, EntryPoint = "smritidb_close")]
    internal static extern void Close(IntPtr store);

    [DllImport(LibName, EntryPoint = "smritidb_free_string")]
    internal static extern void FreeString(IntPtr s);

    [DllImport(LibName, EntryPoint = "smritidb_free_matches")]
    internal static extern void FreeMatches(IntPtr matches, UIntPtr count);

    [DllImport(LibName, EntryPoint = "smritidb_spec_version")]
    internal static extern IntPtr SpecVersion();
}
