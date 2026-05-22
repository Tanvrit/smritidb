package smritidb_test

import (
	"fmt"
	"log"

	smritidb "github.com/kanervalabs/smritidb/packages/smritidb-go"
)

// ExampleStore_Recall demonstrates the basic open/put/recall/close flow
// against an in-memory store. The exact-cue recall returns the matching
// item with a similarity very close to 1.0; we print its stored value
// since the returned `Match.ID` is a generated UUID.
func ExampleStore_Recall() {
	store, err := smritidb.OpenMemory(0)
	if err != nil {
		log.Fatalf("OpenMemory: %v", err)
	}
	defer store.Close()

	if _, err := store.Put("alpha", []byte("alpha-val")); err != nil {
		log.Fatalf("Put: %v", err)
	}
	if _, err := store.Put("beta", []byte("beta-val")); err != nil {
		log.Fatalf("Put: %v", err)
	}

	matches, err := store.Recall("alpha", 1, 0.0)
	if err != nil {
		log.Fatalf("Recall: %v", err)
	}
	fmt.Println(string(matches[0].Value))
	// Output: alpha-val
}
