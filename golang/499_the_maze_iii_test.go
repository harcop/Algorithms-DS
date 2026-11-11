package leetcode

import "testing"

func TestFindShortestWayExamples(t *testing.T) {
	maze := [][]int{
		{0, 0, 0, 0, 0},
		{1, 1, 0, 0, 1},
		{0, 0, 0, 0, 0},
		{0, 1, 0, 0, 1},
		{0, 1, 0, 0, 0},
	}
	if got := findShortestWay(maze, []int{4, 3}, []int{0, 1}); got != "lul" {
		t.Fatalf("findShortestWay() = %q, want lul", got)
	}
	if got := findShortestWay(maze, []int{4, 3}, []int{3, 0}); got != "impossible" {
		t.Fatalf("findShortestWay() = %q, want impossible", got)
	}
	wider := [][]int{
		{0, 0, 0, 0, 0, 0, 0},
		{0, 0, 1, 0, 0, 1, 0},
		{0, 0, 0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0, 0, 1},
	}
	if got := findShortestWay(wider, []int{0, 4}, []int{3, 5}); got != "dldr" {
		t.Fatalf("findShortestWay() = %q, want dldr", got)
	}
}
