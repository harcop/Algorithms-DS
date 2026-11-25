package leetcode

import "testing"

func TestKthSmallestInTableExamples(t *testing.T) {
	if got := kthSmallestInTable(3, 3, 5); got != 3 {
		t.Fatalf("kthSmallestInTable(3, 3, 5) = %d, want 3", got)
	}
	if got := kthSmallestInTable(2, 3, 6); got != 6 {
		t.Fatalf("kthSmallestInTable(2, 3, 6) = %d, want 6", got)
	}
}
