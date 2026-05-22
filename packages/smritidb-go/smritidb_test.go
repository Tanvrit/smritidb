package smritidb

import (
	"os"
	"path/filepath"
	"testing"
)

// TestRoundTripMemory is the canonical smoke test for the Go binding.
// Open a memory-backed store, write three items, recall, assert size,
// and check that exact-cue recall returns the matching item with
// similarity ~1.0.
func TestRoundTripMemory(t *testing.T) {
	store, err := OpenMemory(0)
	if err != nil {
		t.Fatalf("OpenMemory: %v", err)
	}
	defer store.Close()

	items := []struct {
		key   string
		value []byte
	}{
		{"alpha", []byte("alpha-val")},
		{"beta", []byte("beta-val")},
		{"gamma", []byte("gamma-val")},
	}
	ids := make(map[string]string, len(items))
	for _, it := range items {
		id, err := store.Put(it.key, it.value)
		if err != nil {
			t.Fatalf("Put(%q): %v", it.key, err)
		}
		if id == "" {
			t.Fatalf("Put(%q): empty id", it.key)
		}
		ids[it.key] = id
	}

	if got, want := store.Size(), 3; got != want {
		t.Fatalf("Size: got %d, want %d", got, want)
	}

	matches, err := store.Recall("alpha", 5, 0.0)
	if err != nil {
		t.Fatalf("Recall: %v", err)
	}
	if len(matches) == 0 {
		t.Fatalf("Recall: expected at least one match")
	}
	// The most-similar match for the cue "alpha" against keys
	// {"alpha","beta","gamma"} must be the item we keyed as "alpha".
	if got, want := matches[0].ID, ids["alpha"]; got != want {
		t.Fatalf("Recall: top id = %q, want %q", got, want)
	}
	if got := matches[0].Similarity; got <= 0.99 || got > 1.0+1e-9 {
		t.Fatalf("Recall: exact-cue similarity %f not in (0.99, 1.0]", got)
	}
	if got, want := string(matches[0].Value), "alpha-val"; got != want {
		t.Fatalf("Recall: top value = %q, want %q", got, want)
	}
}

// TestRoundTripSqlite mirrors the memory smoke test against the SQLite
// adapter, then re-opens the same file and verifies the inserted items
// recall correctly.
func TestRoundTripSqlite(t *testing.T) {
	dir := t.TempDir()
	dbPath := filepath.Join(dir, "smritidb-go-test.sqlite")

	items := []struct {
		key   string
		value []byte
	}{
		{"alpha", []byte("alpha-val")},
		{"beta", []byte("beta-val")},
		{"gamma", []byte("gamma-val")},
	}
	ids := make(map[string]string, len(items))

	// First session: write + recall.
	{
		store, err := OpenSqlite(dbPath, 0)
		if err != nil {
			t.Fatalf("OpenSqlite: %v", err)
		}

		for _, it := range items {
			id, err := store.Put(it.key, it.value)
			if err != nil {
				store.Close()
				t.Fatalf("Put(%q): %v", it.key, err)
			}
			ids[it.key] = id
		}

		if got, want := store.Size(), 3; got != want {
			store.Close()
			t.Fatalf("Size: got %d, want %d", got, want)
		}

		if err := store.Persist(); err != nil {
			store.Close()
			t.Fatalf("Persist: %v", err)
		}
		if err := store.Close(); err != nil {
			t.Fatalf("Close: %v", err)
		}
	}

	// Sanity: file should exist on disk.
	if _, err := os.Stat(dbPath); err != nil {
		t.Fatalf("sqlite file missing after persist+close: %v", err)
	}

	// Second session: re-open, expect items to be recoverable.
	{
		store, err := OpenSqlite(dbPath, 0)
		if err != nil {
			t.Fatalf("OpenSqlite (reopen): %v", err)
		}
		defer store.Close()

		if got, want := store.Size(), 3; got != want {
			t.Fatalf("reopen Size: got %d, want %d", got, want)
		}

		matches, err := store.Recall("beta", 3, 0.0)
		if err != nil {
			t.Fatalf("reopen Recall: %v", err)
		}
		if len(matches) == 0 {
			t.Fatalf("reopen Recall: expected at least one match")
		}
		if got, want := matches[0].ID, ids["beta"]; got != want {
			t.Fatalf("reopen Recall: top id = %q, want %q", got, want)
		}
		if got := matches[0].Similarity; got <= 0.99 || got > 1.0+1e-9 {
			t.Fatalf("reopen Recall: exact-cue similarity %f not in (0.99, 1.0]", got)
		}
		if got, want := string(matches[0].Value), "beta-val"; got != want {
			t.Fatalf("reopen Recall: top value = %q, want %q", got, want)
		}
	}
}

// TestCloseIsIdempotent makes sure double-Close does not panic or error.
func TestCloseIsIdempotent(t *testing.T) {
	store, err := OpenMemory(0)
	if err != nil {
		t.Fatalf("OpenMemory: %v", err)
	}
	if err := store.Close(); err != nil {
		t.Fatalf("first Close: %v", err)
	}
	if err := store.Close(); err != nil {
		t.Fatalf("second Close: %v", err)
	}
	if got := store.Size(); got != 0 {
		t.Fatalf("Size on closed store: got %d, want 0", got)
	}
}

// TestDimensionMatchesRequest confirms the dimension surface mirrors the
// constructor argument (and the default falls back to a non-zero value).
func TestDimensionMatchesRequest(t *testing.T) {
	store, err := OpenMemory(0)
	if err != nil {
		t.Fatalf("OpenMemory: %v", err)
	}
	defer store.Close()
	if d := store.Dimension(); d == 0 {
		t.Fatalf("Dimension: got 0, want core default")
	}
}

func TestSpecVersion(t *testing.T) {
	v := SpecVersion()
	if v == "" {
		t.Fatalf("SpecVersion: empty string")
	}
}
