package leetcode

import "testing"

func TestGrayCodeExampleOne(t *testing.T) {
	got := grayCode(2)
	want := []int{0, 1, 3, 2}
	if !sameInts(got, want) {
		t.Fatalf("grayCode() = %v, want %v", got, want)
	}
}

func TestGrayCodeExampleTwo(t *testing.T) {
	got := grayCode(1)
	want := []int{0, 1}
	if !sameInts(got, want) {
		t.Fatalf("grayCode() = %v, want %v", got, want)
	}
}
