package leetcode

import "testing"

func TestNumDistinctExampleOne(t *testing.T) {
	got := numDistinct("rabbbit", "rabbit")
	if got != 3 {
		t.Fatalf("numDistinct() = %d, want 3", got)
	}
}

func TestNumDistinctExampleTwo(t *testing.T) {
	got := numDistinct("babgbag", "bag")
	if got != 5 {
		t.Fatalf("numDistinct() = %d, want 5", got)
	}
}
