package leetcode

import "testing"

func TestGameOfLifeExampleOne(t *testing.T) {
	board := [][]int{{0, 1, 0}, {0, 0, 1}, {1, 1, 1}, {0, 0, 0}}
	gameOfLife(board)
	want := [][]int{{0, 0, 0}, {1, 0, 1}, {0, 1, 1}, {0, 1, 0}}
	if !sameMatrix(board, want) {
		t.Fatalf("gameOfLife() = %v, want %v", board, want)
	}
}
