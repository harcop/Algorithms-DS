package leetcode

import "testing"

func TestRandomFlipMatrixExample(t *testing.T) {
	matrix := NewRandomFlipMatrix(3, 1)
	seen := map[int]bool{}
	for i := 0; i < 3; i++ {
		point := matrix.Flip()
		if len(point) != 2 || point[0] < 0 || point[0] > 2 || point[1] != 0 {
			t.Fatalf("Flip() = %v, want a row in [0,2] and column 0", point)
		}
		if seen[point[0]] {
			t.Fatalf("Flip() repeated row %d", point[0])
		}
		seen[point[0]] = true
	}
	matrix.Reset()
	point := matrix.Flip()
	if point[0] < 0 || point[0] > 2 || point[1] != 0 {
		t.Fatalf("Flip() after reset = %v, want a valid cell", point)
	}
}
