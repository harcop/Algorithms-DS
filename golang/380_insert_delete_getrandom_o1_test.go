package leetcode

import "testing"

func TestRandomizedSetExample(t *testing.T) {
	set := NewRandomizedSet()
	if !set.Insert(2) {
		t.Fatal("Insert(2) = false, want true")
	}
	if set.Insert(2) {
		t.Fatal("Insert(2) = true, want false")
	}
	if !set.Remove(2) {
		t.Fatal("Remove(2) = false, want true")
	}
	if !set.Insert(3) {
		t.Fatal("Insert(3) = false, want true")
	}
	if got := set.GetRandom(); got != 3 {
		t.Fatalf("GetRandom() = %d, want 3", got)
	}
}
