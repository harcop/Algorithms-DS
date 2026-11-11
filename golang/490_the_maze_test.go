package leetcode

import "testing"

func TestHasPathExamples(t *testing.T) {
	maze := [][]int{
		{0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0},
		{0, 0, 0, 1, 0},
		{1, 1, 0, 1, 1},
		{0, 0, 0, 0, 0},
	}
	if !hasPath(maze, []int{0, 4}, []int{4, 4}) {
		t.Fatal("hasPath(destination) = false, want true")
	}
	if hasPath(maze, []int{0, 4}, []int{3, 2}) {
		t.Fatal("hasPath(wall stop) = true, want false")
	}
	blocked := [][]int{
		{0, 0, 0, 0, 0},
		{1, 1, 0, 0, 1},
		{0, 0, 0, 0, 0},
		{0, 1, 0, 0, 1},
		{0, 1, 0, 0, 0},
	}
	if hasPath(blocked, []int{4, 3}, []int{0, 1}) {
		t.Fatal("hasPath(blocked) = true, want false")
	}
}
