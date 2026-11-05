package leetcode

import "testing"

func TestCountBattleshipsExample(t *testing.T) {
	board := [][]byte{
		{'X', '.', '.', 'X'},
		{'.', '.', '.', 'X'},
		{'.', '.', '.', 'X'},
	}
	if got := countBattleships(board); got != 2 {
		t.Fatalf("countBattleships() = %d, want 2", got)
	}
}
