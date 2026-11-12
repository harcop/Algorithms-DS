package leetcode

import "testing"

func TestShortestDistanceMazeExamples(t *testing.T) {
	maze := [][]int{
		{0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0},
		{0, 0, 0, 1, 0},
		{1, 1, 0, 1, 1},
		{0, 0, 0, 0, 0},
	}
	if got := shortestDistanceMaze(maze, []int{0, 4}, []int{4, 4}); got != 12 {
		t.Fatalf("shortestDistanceMaze(exit) = %d, want 12", got)
	}
	if got := shortestDistanceMaze(maze, []int{0, 4}, []int{3, 2}); got != -1 {
		t.Fatalf("shortestDistanceMaze(blocked) = %d, want -1", got)
	}
}
