package leetcode

import "testing"

func TestCleanRoomExamples(t *testing.T) {
	grid := [][]int{
		{1, 1, 1, 1, 1, 0, 1, 1},
		{1, 1, 1, 1, 1, 0, 1, 1},
		{1, 0, 1, 1, 1, 1, 1, 1},
		{0, 0, 0, 1, 0, 0, 0, 0},
		{1, 1, 1, 1, 1, 1, 1, 1},
	}
	if got := cleanRoom(grid, 1, 3); got != 30 {
		t.Fatalf("cleanRoom() = %d, want 30", got)
	}
	if got := cleanRoom([][]int{{1}}, 0, 0); got != 1 {
		t.Fatalf("cleanRoom() = %d, want 1", got)
	}
}
