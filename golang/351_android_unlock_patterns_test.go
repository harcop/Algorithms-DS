package leetcode

import "testing"

func TestNumberOfPatternsSmall(t *testing.T) {
	if got := numberOfPatterns(1, 1); got != 9 {
		t.Fatalf("numberOfPatterns(1, 1) = %d, want 9", got)
	}
	if got := numberOfPatterns(1, 2); got != 65 {
		t.Fatalf("numberOfPatterns(1, 2) = %d, want 65", got)
	}
	if got := numberOfPatterns(3, 3); got != 320 {
		t.Fatalf("numberOfPatterns(3, 3) = %d, want 320", got)
	}
}
