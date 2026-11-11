package leetcode

import "testing"

func TestFindPoisonedDurationExamples(t *testing.T) {
	if got := findPoisonedDuration([]int{1, 4}, 2); got != 4 {
		t.Fatalf("findPoisonedDuration() = %d, want 4", got)
	}
	if got := findPoisonedDuration([]int{1, 2}, 2); got != 3 {
		t.Fatalf("findPoisonedDuration() = %d, want 3", got)
	}
}
