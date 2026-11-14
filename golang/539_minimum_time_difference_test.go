package leetcode

import "testing"

func TestFindMinDifferenceExample(t *testing.T) {
	if got := findMinDifference([]string{"23:59", "00:00"}); got != 1 {
		t.Fatalf("findMinDifference() = %d, want 1", got)
	}
}
