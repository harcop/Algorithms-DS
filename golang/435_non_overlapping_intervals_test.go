package leetcode

import "testing"

func TestEraseOverlapIntervalsExamples(t *testing.T) {
	if got := eraseOverlapIntervals([][]int{{1, 2}, {2, 3}, {3, 4}, {1, 3}}); got != 1 {
		t.Fatalf("eraseOverlapIntervals(chain) = %d, want 1", got)
	}
	if got := eraseOverlapIntervals([][]int{{1, 2}, {1, 2}, {1, 2}}); got != 2 {
		t.Fatalf("eraseOverlapIntervals(duplicates) = %d, want 2", got)
	}
	if got := eraseOverlapIntervals([][]int{{1, 2}, {2, 3}}); got != 0 {
		t.Fatalf("eraseOverlapIntervals(touching) = %d, want 0", got)
	}
}
