package leetcode

import "testing"

func TestMinAreaExampleOne(t *testing.T) {
	image := [][]byte{
		[]byte("0010"),
		[]byte("0110"),
		[]byte("0110"),
	}
	if got := minArea(image, 0, 2); got != 6 {
		t.Fatalf("minArea() = %d, want 6", got)
	}
}
