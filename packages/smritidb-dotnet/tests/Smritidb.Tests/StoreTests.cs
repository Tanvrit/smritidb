using System;
using System.IO;
using System.Linq;
using System.Text;
using Smritidb;
using Xunit;

namespace Smritidb.Tests;

public class StoreTests
{
    [Fact]
    public void RoundTripMemory()
    {
        using var store = Store.OpenMemory();

        var items = new (string Key, byte[] Value)[]
        {
            ("alpha", new byte[] { 0x61 }),
            ("beta",  new byte[] { 0x62 }),
            ("gamma", new byte[] { 0x63 }),
        };
        foreach (var (key, value) in items)
        {
            var id = store.Put(key, value);
            Assert.False(string.IsNullOrEmpty(id));
        }

        Assert.Equal(3, store.Size);

        var matches = store.Recall("alpha", topK: 5, minSimilarity: 0.0);
        Assert.NotEmpty(matches);
        Assert.InRange(matches[0].Similarity, 0.0, 1.0);
    }

    [Fact]
    public void SpecVersionIsNotEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Store.SpecVersion()));
    }

    [Fact]
    public void DimensionDefaults()
    {
        using var store = Store.OpenMemory();
        // Core default is 10_000; we just assert it's a sane positive number.
        Assert.True(store.Dimension > 0);
    }

    [Fact]
    public void ExactCueRecallHasHighSimilarity()
    {
        using var store = Store.OpenMemory();

        var alphaId = store.Put("alpha", Encoding.UTF8.GetBytes("A"));
        store.Put("beta", Encoding.UTF8.GetBytes("B"));
        store.Put("gamma", Encoding.UTF8.GetBytes("C"));

        var matches = store.Recall("alpha", topK: 3, minSimilarity: 0.0);
        Assert.NotEmpty(matches);

        // The top-1 result for the exact cue we just inserted should be the
        // same item (matched by the id Put returned). Self-similarity must
        // be > 0.99.
        var top = matches[0];
        Assert.Equal(alphaId, top.Id);
        Assert.True(top.Similarity > 0.99,
            $"Expected top match for exact cue to have similarity > 0.99, got {top.Similarity}");
        Assert.Equal((byte)'A', top.Value[0]);
    }

    [Fact]
    public void RecallRespectsMinSimilarity()
    {
        using var store = Store.OpenMemory();
        store.Put("alpha", new byte[] { 1 });
        store.Put("beta", new byte[] { 2 });

        // A threshold > 1.0 must yield zero matches.
        var none = store.Recall("alpha", topK: 5, minSimilarity: 1.5);
        Assert.Empty(none);
    }

    [Fact]
    public void DisposeIsIdempotent()
    {
        var store = Store.OpenMemory();
        store.Dispose();
        store.Dispose(); // must not throw
        Assert.Throws<ObjectDisposedException>(() => _ = store.Size);
    }

    [Fact]
    public void SqliteRoundTripPersistsAcrossReopen()
    {
        var tempPath = Path.Combine(Path.GetTempPath(),
            $"smritidb-dotnet-{Guid.NewGuid():N}.sqlite");
        string alphaId;
        try
        {
            using (var store = Store.OpenSqlite(tempPath))
            {
                Assert.True(store.Dimension > 0);

                alphaId = store.Put("alpha", Encoding.UTF8.GetBytes("A"));
                store.Put("beta", Encoding.UTF8.GetBytes("B"));
                store.Put("gamma", Encoding.UTF8.GetBytes("C"));

                Assert.Equal(3, store.Size);
                store.Persist();
            }

            // Reopen and verify items survive.
            using (var store2 = Store.OpenSqlite(tempPath))
            {
                Assert.Equal(3, store2.Size);

                var matches = store2.Recall("alpha", topK: 3, minSimilarity: 0.0);
                Assert.NotEmpty(matches);

                var alphaMatch = matches.FirstOrDefault(m => m.Id == alphaId);
                Assert.NotNull(alphaMatch);
                Assert.True(alphaMatch!.Similarity > 0.99,
                    $"Expected reopened exact-cue similarity > 0.99, got {alphaMatch.Similarity}");
                Assert.Equal((byte)'A', alphaMatch.Value[0]);
            }
        }
        finally
        {
            try { if (File.Exists(tempPath)) File.Delete(tempPath); } catch { /* ignore */ }
        }
    }

    [Fact]
    public void PutAcceptsEmptyValue()
    {
        using var store = Store.OpenMemory();
        var id = store.Put("zeta", Array.Empty<byte>());
        Assert.False(string.IsNullOrEmpty(id));
        Assert.Equal(1, store.Size);
    }
}
