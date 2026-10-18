package leetcode

import "testing"

func TestSingleNumberExampleOne(t *testing.T) {
	got := singleNumber([]int{2, 2, 1})
	if got != 1 {
		t.Fatalf("singleNumber() = %d, want 1", got)
	}
}

func TestSingleNumberExampleTwo(t *testing.T) {
	got := singleNumber([]int{4, 1, 2, 1, 2})
	if got != 4 {
		t.Fatalf("singleNumber() = %d, want 4", got)
	}
}

func TestSingleNumberExampleThree(t *testing.T) {
	got := singleNumber([]int{1})
	if got != 1 {
		t.Fatalf("singleNumber() = %d, want 1", got)
	}
}
