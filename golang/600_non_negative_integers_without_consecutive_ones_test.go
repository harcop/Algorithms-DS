package leetcode

import "testing"

func TestFindIntegersExamples(t *testing.T) {
	if got := findIntegers(5); got != 5 {
		t.Fatalf("findIntegers(5) = %d, want 5", got)
	}
	if got := findIntegers(1); got != 2 {
		t.Fatalf("findIntegers(1) = %d, want 2", got)
	}
	if got := findIntegers(2); got != 3 {
		t.Fatalf("findIntegers(2) = %d, want 3", got)
	}
}
