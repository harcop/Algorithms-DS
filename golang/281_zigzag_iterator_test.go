package leetcode

import "testing"

func TestZigzagIteratorExample(t *testing.T) {
	it := NewZigzagIterator([]int{1, 2}, []int{3, 4, 5, 6})
	var got []int
	for it.HasNext() {
		got = append(got, it.Next())
	}
	want := []int{1, 3, 2, 4, 5, 6}
	if !sameInts(got, want) {
		t.Fatalf("ZigzagIterator = %v, want %v", got, want)
	}
}
