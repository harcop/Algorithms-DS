package leetcode

import (
	"math"
	"testing"
)

func TestWallsAndGatesExampleOne(t *testing.T) {
	inf := math.MaxInt32
	rooms := [][]int{
		{inf, -1, 0, inf},
		{inf, inf, inf, -1},
		{inf, -1, inf, -1},
		{0, -1, inf, inf},
	}
	wallsAndGates(rooms)
	want := [][]int{
		{3, -1, 0, 1},
		{2, 2, 1, -1},
		{1, -1, 2, -1},
		{0, -1, 3, 4},
	}
	if !sameMatrix(rooms, want) {
		t.Fatalf("wallsAndGates() = %v, want %v", rooms, want)
	}
}
