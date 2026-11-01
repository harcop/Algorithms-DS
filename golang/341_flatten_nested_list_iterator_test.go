package leetcode

import "testing"

func TestNestedIteratorExample(t *testing.T) {
	nested := []NestedInteger{
		nestedList(nestedInt(1), nestedInt(1)),
		nestedInt(2),
		nestedList(nestedInt(1), nestedInt(1)),
	}
	it := NewNestedIterator(nested)
	var got []int
	for it.HasNext() {
		got = append(got, it.Next())
	}
	want := []int{1, 1, 2, 1, 1}
	if !sameInts(got, want) {
		t.Fatalf("NestedIterator = %v, want %v", got, want)
	}
}
