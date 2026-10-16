package leetcode

import "testing"

func TestSearchIIExampleOne(t *testing.T) {
	if !searchII([]int{2, 5, 6, 0, 0, 1, 2}, 0) {
		t.Fatal("searchII() = false, want true")
	}
}

func TestSearchIIExampleTwo(t *testing.T) {
	if searchII([]int{2, 5, 6, 0, 0, 1, 2}, 3) {
		t.Fatal("searchII() = true, want false")
	}
}

func TestSearchIIExampleThree(t *testing.T) {
	if !searchII([]int{1, 0, 1, 1, 1}, 0) {
		t.Fatal("searchII() = false, want true")
	}
}
