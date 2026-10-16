package leetcode

import "testing"

func TestMaximalRectangleExampleOne(t *testing.T) {
	matrix := [][]byte{
		[]byte("10100"),
		[]byte("10111"),
		[]byte("11111"),
		[]byte("10010"),
	}
	got := maximalRectangle(matrix)
	if got != 6 {
		t.Fatalf("maximalRectangle() = %d, want 6", got)
	}
}

func TestMaximalRectangleExampleTwo(t *testing.T) {
	got := maximalRectangle([][]byte{[]byte("0")})
	if got != 0 {
		t.Fatalf("maximalRectangle() = %d, want 0", got)
	}
}
