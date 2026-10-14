package leetcode

import "testing"

func TestJumpExampleOne(t *testing.T) {
	got := jump([]int{2, 3, 1, 1, 4})
	if got != 2 {
		t.Fatalf("jump() = %d, want 2", got)
	}
}

func TestJumpExampleTwo(t *testing.T) {
	got := jump([]int{2, 3, 0, 1, 4})
	if got != 2 {
		t.Fatalf("jump() = %d, want 2", got)
	}
}
