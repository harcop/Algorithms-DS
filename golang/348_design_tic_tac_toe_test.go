package leetcode

import "testing"

func TestTicTacToeColumnWin(t *testing.T) {
	game := NewTicTacToe(3)
	if got := game.Move(0, 0, 1); got != 0 {
		t.Fatalf("Move() = %d, want 0", got)
	}
	if got := game.Move(0, 1, 2); got != 0 {
		t.Fatalf("Move() = %d, want 0", got)
	}
	if got := game.Move(1, 0, 1); got != 0 {
		t.Fatalf("Move() = %d, want 0", got)
	}
	if got := game.Move(0, 2, 2); got != 0 {
		t.Fatalf("Move() = %d, want 0", got)
	}
	if got := game.Move(2, 0, 1); got != 1 {
		t.Fatalf("Move() = %d, want 1", got)
	}
}
