package leetcode

import "testing"

func TestCanJumpExampleOne(t *testing.T) {
	if !canJump([]int{2, 3, 1, 1, 4}) {
		t.Fatal("canJump() = false, want true")
	}
}

func TestCanJumpExampleTwo(t *testing.T) {
	if canJump([]int{3, 2, 1, 0, 4}) {
		t.Fatal("canJump() = true, want false")
	}
}
