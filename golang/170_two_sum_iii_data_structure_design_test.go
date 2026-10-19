package leetcode

import "testing"

func TestTwoSumDesignExample(t *testing.T) {
	store := NewTwoSum()
	store.Add(1)
	store.Add(3)
	store.Add(5)
	if !store.Find(4) {
		t.Fatal("Find(4) = false, want true")
	}
	if store.Find(7) {
		t.Fatal("Find(7) = true, want false")
	}
}
