package leetcode

import "testing"

func TestFindMaxAverageExample(t *testing.T) {
	got := findMaxAverage([]int{1, 12, -5, -6, 50, 3}, 4)
	if got != 12.75 {
		t.Fatalf("findMaxAverage() = %v, want 12.75", got)
	}
	if got := findMaxAverage([]int{5}, 1); got != 5 {
		t.Fatalf("findMaxAverage() = %v, want 5", got)
	}
}
