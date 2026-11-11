package leetcode

import "testing"

func TestConstructRectangleExamples(t *testing.T) {
	if got := constructRectangle(4); !sameInts(got, []int{2, 2}) {
		t.Fatalf("constructRectangle(4) = %v, want [2 2]", got)
	}
	if got := constructRectangle(37); !sameInts(got, []int{37, 1}) {
		t.Fatalf("constructRectangle(37) = %v, want [37 1]", got)
	}
}
