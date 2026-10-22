package leetcode

import "testing"

func TestMaximalSquareExampleOne(t *testing.T) {
	matrix := [][]byte{
		[]byte("10100"),
		[]byte("10111"),
		[]byte("11111"),
		[]byte("10010"),
	}
	if got := maximalSquare(matrix); got != 4 {
		t.Fatalf("maximalSquare() = %d, want 4", got)
	}
}

func TestMaximalSquareExampleTwo(t *testing.T) {
	if got := maximalSquare([][]byte{[]byte("0")}); got != 0 {
		t.Fatalf("maximalSquare() = %d, want 0", got)
	}
}

func TestMaximalSquareExampleThree(t *testing.T) {
	if got := maximalSquare([][]byte{[]byte("1")}); got != 1 {
		t.Fatalf("maximalSquare() = %d, want 1", got)
	}
}
