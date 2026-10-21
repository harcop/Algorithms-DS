package leetcode

import "testing"

func TestNumIslandsExampleOne(t *testing.T) {
	grid := [][]byte{
		[]byte("11110"),
		[]byte("11010"),
		[]byte("11000"),
		[]byte("00000"),
	}
	if got := numIslands(grid); got != 1 {
		t.Fatalf("numIslands() = %d, want 1", got)
	}
}

func TestNumIslandsExampleTwo(t *testing.T) {
	grid := [][]byte{
		[]byte("11000"),
		[]byte("11000"),
		[]byte("00100"),
		[]byte("00011"),
	}
	if got := numIslands(grid); got != 3 {
		t.Fatalf("numIslands() = %d, want 3", got)
	}
}
