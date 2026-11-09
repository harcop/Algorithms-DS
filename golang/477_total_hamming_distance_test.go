package leetcode

import "testing"

func TestTotalHammingDistanceExample(t *testing.T) {
	if got := totalHammingDistance([]int{4, 14, 2}); got != 6 {
		t.Fatalf("totalHammingDistance() = %d, want 6", got)
	}
}
