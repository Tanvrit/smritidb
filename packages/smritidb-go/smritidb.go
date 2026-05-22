// Package smritidb provides a Go binding for the Smritidb associative store.
//
// The binding talks to a CGo-loaded `libsmritidb_c` shared library produced
// by the sibling `packages/smritidb-c` crate. Build the cdylib first:
//
//	cd ../smritidb-c && cargo build --release
//
// then run `go test ./...` here.
package smritidb

/*
#cgo CFLAGS: -I${SRCDIR}/../smritidb-c/include
#cgo LDFLAGS: -L${SRCDIR}/../smritidb-c/target/release -lsmritidb_c
#cgo darwin LDFLAGS: -Wl,-rpath,${SRCDIR}/../smritidb-c/target/release
#cgo linux  LDFLAGS: -Wl,-rpath,${SRCDIR}/../smritidb-c/target/release -lm -ldl -lpthread

#include <stdlib.h>
#include <string.h>
#include "smritidb.h"
*/
import "C"

import (
	"errors"
	"fmt"
	"runtime"
	"unsafe"
)

// Match is the Go-side projection of `SmritidbMatch`.
type Match struct {
	ID          string
	Similarity  float64
	Value       []byte
	AccessCount uint32
}

// Store is a handle to a persistent Smritidb store.
type Store struct {
	c      *C.SmritidbStore
	closed bool
}

// lastError reads the thread-local error buffer from the C ABI.
//
// The C ABI documents the pointer as valid until the next library call on
// the same thread, so we copy it into a Go string while we still hold the
// goroutine-thread association.
func lastError(fallback string) error {
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	p := C.smritidb_last_error()
	if p == nil {
		if fallback == "" {
			return errors.New("smritidb: unknown error")
		}
		return errors.New(fallback)
	}
	return errors.New(C.GoString(p))
}

// OpenMemory opens an in-memory persistent store with the given hypervector
// dimension. Pass 0 to use the core default (10_000).
func OpenMemory(dimension uint32) (*Store, error) {
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	h := C.smritidb_open_memory(C.uint32_t(dimension))
	if h == nil {
		return nil, lastError("smritidb_open_memory returned NULL")
	}
	s := &Store{c: h}
	runtime.SetFinalizer(s, (*Store).Close)
	return s, nil
}

// OpenSqlite opens (or creates) a SQLite-backed store at `path`.
func OpenSqlite(path string, dimension uint32) (*Store, error) {
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	cpath := C.CString(path)
	defer C.free(unsafe.Pointer(cpath))
	h := C.smritidb_open_sqlite(cpath, C.uint32_t(dimension))
	if h == nil {
		return nil, lastError("smritidb_open_sqlite returned NULL")
	}
	s := &Store{c: h}
	runtime.SetFinalizer(s, (*Store).Close)
	return s, nil
}

// Put inserts (or updates) an item under the UTF-8 string `key`.
func (s *Store) Put(key string, value []byte) (string, error) {
	if s == nil || s.c == nil || s.closed {
		return "", errors.New("smritidb: store is closed")
	}
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	ckey := C.CString(key)
	defer C.free(unsafe.Pointer(ckey))

	var valPtr *C.uint8_t
	var valLen C.uintptr_t
	if len(value) > 0 {
		valPtr = (*C.uint8_t)(unsafe.Pointer(&value[0]))
		valLen = C.uintptr_t(len(value))
	}

	var outID *C.char
	rc := C.smritidb_put(s.c, ckey, valPtr, valLen, &outID)
	if rc != C.SMRITIDB_STATUS_OK {
		return "", lastError(fmt.Sprintf("smritidb_put: status %d", int(rc)))
	}
	defer C.smritidb_free_string(outID)
	return C.GoString(outID), nil
}

// Recall returns the top-K matches for the given UTF-8 cue.
func (s *Store) Recall(cue string, topK uint32, minSimilarity float64) ([]Match, error) {
	if s == nil || s.c == nil || s.closed {
		return nil, errors.New("smritidb: store is closed")
	}
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()
	ccue := C.CString(cue)
	defer C.free(unsafe.Pointer(ccue))

	var matches *C.SmritidbMatch
	var count C.uintptr_t
	rc := C.smritidb_recall(s.c, ccue, C.uint32_t(topK), C.double(minSimilarity), &matches, &count)
	if rc != C.SMRITIDB_STATUS_OK {
		return nil, lastError(fmt.Sprintf("smritidb_recall: status %d", int(rc)))
	}
	defer C.smritidb_free_matches(matches, count)

	n := int(count)
	out := make([]Match, n)
	if n == 0 {
		return out, nil
	}
	// Treat the C array as a Go slice for indexing. cgo's "view" trick.
	view := unsafe.Slice(matches, n)
	for i := 0; i < n; i++ {
		m := view[i]
		out[i].ID = C.GoString(m.id)
		out[i].Similarity = float64(m.similarity)
		out[i].AccessCount = uint32(m.access_count)
		if m.value.data != nil && m.value.len > 0 {
			out[i].Value = C.GoBytes(unsafe.Pointer(m.value.data), C.int(m.value.len))
		} else {
			out[i].Value = []byte{}
		}
	}
	return out, nil
}

// Size returns the number of items currently in the store.
func (s *Store) Size() int {
	if s == nil || s.c == nil || s.closed {
		return 0
	}
	return int(C.smritidb_size(s.c))
}

// Dimension returns the hypervector dimension this store was opened with.
func (s *Store) Dimension() uint32 {
	if s == nil || s.c == nil || s.closed {
		return 0
	}
	return uint32(C.smritidb_dimension(s.c))
}

// Persist forces any pending in-memory state to be flushed to the backing
// adapter. No-op for in-memory stores.
func (s *Store) Persist() error {
	if s == nil || s.c == nil || s.closed {
		return errors.New("smritidb: store is closed")
	}
	rc := C.smritidb_persist(s.c)
	if rc != C.SMRITIDB_STATUS_OK {
		return lastError(fmt.Sprintf("smritidb_persist: status %d", int(rc)))
	}
	return nil
}

// Close releases the underlying store. Idempotent.
func (s *Store) Close() error {
	if s == nil || s.c == nil || s.closed {
		return nil
	}
	runtime.SetFinalizer(s, nil)
	C.smritidb_close(s.c)
	s.c = nil
	s.closed = true
	return nil
}

// SpecVersion returns the SPEC.md version string the underlying core
// implements.
func SpecVersion() string {
	p := C.smritidb_spec_version()
	if p == nil {
		return ""
	}
	return C.GoString(p)
}
