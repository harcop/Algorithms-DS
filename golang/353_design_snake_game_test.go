package leetcode

import "testing"

func TestSnakeGameExample(t *testing.T) {
	game := NewSnakeGame(3, 2, [][]int{{1, 2}, {0, 1}})
	moves := []struct {
		dir  string
		want int
	}{
		{"R", 0},
		{"D", 0},
		{"R", 1},
		{"U", 1},
		{"L", 2},
		{"U", -1},
	}
	for _, move := range moves {
		if got := game.Move(move.dir); got != move.want {
			t.Fatalf("Move(%s) = %d, want %d", move.dir, got, move.want)
		}
	}
}
