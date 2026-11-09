package leetcode

import "testing"

func TestFindComplementExamples(t *testing.T) {
	if got := findComplement(5); got != 2 {
		t.Fatalf("findComplement(5) = %d, want 2", got)
	}
	if got := findComplement(1); got != 0 {
		t.Fatalf("findComplement(1) = %d, want 0", got)
	}
}
