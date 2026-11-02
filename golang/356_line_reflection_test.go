package leetcode

import "testing"

func TestIsReflectedExamples(t *testing.T) {
	if !isReflected([][]int{{1, 1}, {-1, 1}}) {
		t.Fatal("isReflected() = false, want true")
	}
	if isReflected([][]int{{1, 1}, {2, 2}}) {
		t.Fatal("isReflected() = true, want false")
	}
	if !isReflected([][]int{{0, 0}}) {
		t.Fatal("isReflected() = false, want true")
	}
}
