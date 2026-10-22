package leetcode

import "testing"

func TestRobIIExampleOne(t *testing.T) {
	if got := robII([]int{2, 3, 2}); got != 3 {
		t.Fatalf("robII() = %d, want 3", got)
	}
}

func TestRobIIExampleTwo(t *testing.T) {
	if got := robII([]int{1, 2, 3, 1}); got != 4 {
		t.Fatalf("robII() = %d, want 4", got)
	}
}

func TestRobIIExampleThree(t *testing.T) {
	if got := robII([]int{1, 2, 3}); got != 3 {
		t.Fatalf("robII() = %d, want 3", got)
	}
}
