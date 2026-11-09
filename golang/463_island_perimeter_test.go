package leetcode

import "testing"

func TestIslandPerimeterExample(t *testing.T) {
	grid := [][]int{
		{0, 1, 0, 0},
		{1, 1, 1, 0},
		{0, 1, 0, 0},
		{1, 1, 0, 0},
	}
	if got := islandPerimeter(grid); got != 16 {
		t.Fatalf("islandPerimeter() = %d, want 16", got)
	}
}
