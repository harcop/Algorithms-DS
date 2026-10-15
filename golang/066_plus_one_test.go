package leetcode

import "testing"

func TestPlusOneExampleOne(t *testing.T) {
	got := plusOne([]int{1, 2, 3})
	want := []int{1, 2, 4}
	if !sameInts(got, want) {
		t.Fatalf("plusOne() = %v, want %v", got, want)
	}
}

func TestPlusOneExampleTwo(t *testing.T) {
	got := plusOne([]int{4, 3, 2, 1})
	want := []int{4, 3, 2, 2}
	if !sameInts(got, want) {
		t.Fatalf("plusOne() = %v, want %v", got, want)
	}
}

func TestPlusOneExampleThree(t *testing.T) {
	got := plusOne([]int{9})
	want := []int{1, 0}
	if !sameInts(got, want) {
		t.Fatalf("plusOne() = %v, want %v", got, want)
	}
}
