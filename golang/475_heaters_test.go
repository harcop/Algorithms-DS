package leetcode

import "testing"

func TestFindRadiusExamples(t *testing.T) {
	if got := findRadius([]int{1, 2, 3}, []int{2}); got != 1 {
		t.Fatalf("findRadius() = %d, want 1", got)
	}
	if got := findRadius([]int{1, 2, 3, 4}, []int{1, 4}); got != 1 {
		t.Fatalf("findRadius() = %d, want 1", got)
	}
	if got := findRadius([]int{1, 5}, []int{2}); got != 3 {
		t.Fatalf("findRadius() = %d, want 3", got)
	}
}
