package leetcode

import "testing"

func TestTriangleNumberExamples(t *testing.T) {
	if got := triangleNumber([]int{2, 2, 3, 4}); got != 3 {
		t.Fatalf("triangleNumber([2,2,3,4]) = %d, want 3", got)
	}
	if got := triangleNumber([]int{4, 2, 3, 4}); got != 4 {
		t.Fatalf("triangleNumber([4,2,3,4]) = %d, want 4", got)
	}
}
