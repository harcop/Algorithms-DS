package leetcode

import "testing"

func TestCanWinExampleOne(t *testing.T) {
	if !canWin("++++") {
		t.Fatal("canWin() = false, want true")
	}
}
