package leetcode

import "testing"

func TestDistributeCandiesExamples(t *testing.T) {
	if got := distributeCandies([]int{1, 1, 2, 2, 3, 3}); got != 3 {
		t.Fatalf("distributeCandies = %d, want 3", got)
	}
	if got := distributeCandies([]int{1, 1, 2, 3}); got != 2 {
		t.Fatalf("distributeCandies = %d, want 2", got)
	}
	if got := distributeCandies([]int{6, 6, 6, 6}); got != 1 {
		t.Fatalf("distributeCandies = %d, want 1", got)
	}
}
