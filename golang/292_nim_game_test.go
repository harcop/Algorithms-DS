package leetcode

import "testing"

func TestCanWinNimExampleOne(t *testing.T) {
	if canWinNim(4) {
		t.Fatal("canWinNim(4) = true, want false")
	}
}

func TestCanWinNimExampleTwo(t *testing.T) {
	if !canWinNim(1) {
		t.Fatal("canWinNim(1) = false, want true")
	}
}
