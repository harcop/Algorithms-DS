package leetcode

import "testing"

func TestPeekingIteratorExample(t *testing.T) {
	it := NewPeekingIterator([]int{1, 2, 3})
	if got := it.Peek(); got != 1 {
		t.Fatalf("Peek() = %d, want 1", got)
	}
	if got := it.Next(); got != 1 {
		t.Fatalf("Next() = %d, want 1", got)
	}
	if got := it.Peek(); got != 2 {
		t.Fatalf("Peek() = %d, want 2", got)
	}
	if got := it.Next(); got != 2 {
		t.Fatalf("Next() = %d, want 2", got)
	}
	if !it.HasNext() {
		t.Fatal("HasNext() = false, want true")
	}
	if got := it.Next(); got != 3 {
		t.Fatalf("Next() = %d, want 3", got)
	}
	if it.HasNext() {
		t.Fatal("HasNext() = true, want false")
	}
}
