package leetcode

import "testing"

func TestFirstMissingPositiveExampleOne(t *testing.T) {
	got := firstMissingPositive([]int{1, 2, 0})
	if got != 3 {
		t.Fatalf("firstMissingPositive() = %d, want 3", got)
	}
}

func TestFirstMissingPositiveExampleTwo(t *testing.T) {
	got := firstMissingPositive([]int{3, 4, -1, 1})
	if got != 2 {
		t.Fatalf("firstMissingPositive() = %d, want 2", got)
	}
}

func TestFirstMissingPositiveExampleThree(t *testing.T) {
	got := firstMissingPositive([]int{7, 8, 9, 11, 12})
	if got != 1 {
		t.Fatalf("firstMissingPositive() = %d, want 1", got)
	}
}
