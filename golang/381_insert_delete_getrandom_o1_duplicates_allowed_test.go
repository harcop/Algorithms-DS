package leetcode

import "testing"

func TestRandomizedCollectionExample(t *testing.T) {
	collection := NewRandomizedCollection()
	if !collection.Insert(4) {
		t.Fatal("Insert(4) = false, want true")
	}
	if collection.Insert(4) {
		t.Fatal("Insert(4) = true, want false")
	}
	if collection.Remove(3) {
		t.Fatal("Remove(3) = true, want false")
	}
	if !collection.Remove(4) {
		t.Fatal("Remove(4) = false, want true")
	}
	if got := collection.GetRandom(); got != 4 {
		t.Fatalf("GetRandom() = %d, want 4", got)
	}
}
