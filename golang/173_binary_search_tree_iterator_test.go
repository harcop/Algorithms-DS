package leetcode

import "testing"

func TestBSTIteratorExample(t *testing.T) {
	root := makeTree(7,
		makeTree(3, nil, nil),
		makeTree(15, makeTree(9, nil, nil), makeTree(20, nil, nil)),
	)
	it := NewBSTIterator(root)
	if got := it.Next(); got != 3 {
		t.Fatalf("Next() = %d, want 3", got)
	}
	if got := it.Next(); got != 7 {
		t.Fatalf("Next() = %d, want 7", got)
	}
	if !it.HasNext() {
		t.Fatal("HasNext() = false, want true")
	}
	if got := it.Next(); got != 9 {
		t.Fatalf("Next() = %d, want 9", got)
	}
	if !it.HasNext() {
		t.Fatal("HasNext() = false, want true")
	}
	if got := it.Next(); got != 15 {
		t.Fatalf("Next() = %d, want 15", got)
	}
	if !it.HasNext() {
		t.Fatal("HasNext() = false, want true")
	}
	if got := it.Next(); got != 20 {
		t.Fatalf("Next() = %d, want 20", got)
	}
	if it.HasNext() {
		t.Fatal("HasNext() = true, want false")
	}
}
