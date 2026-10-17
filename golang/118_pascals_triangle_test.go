package leetcode

import "testing"

func TestGenerateExampleOne(t *testing.T) {
	got := generate(5)
	want := [][]int{{1}, {1, 1}, {1, 2, 1}, {1, 3, 3, 1}, {1, 4, 6, 4, 1}}
	if !sameMatrix(got, want) {
		t.Fatalf("generate() = %v, want %v", got, want)
	}
}

func TestGenerateExampleTwo(t *testing.T) {
	got := generate(1)
	want := [][]int{{1}}
	if !sameMatrix(got, want) {
		t.Fatalf("generate() = %v, want %v", got, want)
	}
}
