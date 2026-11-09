package leetcode

import "testing"

func TestCanIWinExamples(t *testing.T) {
	if canIWin(10, 11) {
		t.Fatal("canIWin(10, 11) = true, want false")
	}
	if !canIWin(10, 0) {
		t.Fatal("canIWin(10, 0) = false, want true")
	}
	if !canIWin(10, 1) {
		t.Fatal("canIWin(10, 1) = false, want true")
	}
}
