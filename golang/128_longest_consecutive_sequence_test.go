package leetcode

import "testing"

func TestLongestConsecutiveExampleOne(t *testing.T) {
	got := longestConsecutive([]int{100, 4, 200, 1, 3, 2})
	if got != 4 {
		t.Fatalf("longestConsecutive() = %d, want 4", got)
	}
}

func TestLongestConsecutiveExampleTwo(t *testing.T) {
	got := longestConsecutive([]int{0, 3, 7, 2, 5, 8, 4, 6, 0, 1})
	if got != 9 {
		t.Fatalf("longestConsecutive() = %d, want 9", got)
	}
}
