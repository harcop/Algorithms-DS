package leetcode

import "testing"

func TestLinkedListRandomExample(t *testing.T) {
	picker := NewLinkedListRandom(vecToList([]int{1, 2, 3}))
	got := picker.GetRandom()
	if got < 1 || got > 3 {
		t.Fatalf("GetRandom() = %d, want 1, 2, or 3", got)
	}
	single := NewLinkedListRandom(vecToList([]int{7}))
	if got = single.GetRandom(); got != 7 {
		t.Fatalf("GetRandom() = %d, want 7", got)
	}
}
