package leetcode

import "testing"

func TestMinCutExampleOne(t *testing.T) {
	got := minCut("aab")
	if got != 1 {
		t.Fatalf("minCut() = %d, want 1", got)
	}
}

func TestMinCutExampleTwo(t *testing.T) {
	got := minCut("a")
	if got != 0 {
		t.Fatalf("minCut() = %d, want 0", got)
	}
}

func TestMinCutExampleThree(t *testing.T) {
	got := minCut("ab")
	if got != 1 {
		t.Fatalf("minCut() = %d, want 1", got)
	}
}
