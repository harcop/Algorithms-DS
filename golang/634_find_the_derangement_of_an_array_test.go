package leetcode

import "testing"

func TestFindDerangementExamples(t *testing.T) {
	if got := findDerangement(1); got != 0 {
		t.Fatalf("findDerangement(1) = %d, want 0", got)
	}
	if got := findDerangement(2); got != 1 {
		t.Fatalf("findDerangement(2) = %d, want 1", got)
	}
	if got := findDerangement(3); got != 2 {
		t.Fatalf("findDerangement(3) = %d, want 2", got)
	}
	if got := findDerangement(4); got != 9 {
		t.Fatalf("findDerangement(4) = %d, want 9", got)
	}
}
