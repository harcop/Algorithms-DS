package leetcode

import "testing"

func TestLeastIntervalExamples(t *testing.T) {
	if got := leastInterval([]byte{'A', 'A', 'A', 'B', 'B', 'B'}, 2); got != 8 {
		t.Fatalf("leastInterval() = %d, want 8", got)
	}
	if got := leastInterval([]byte{'A', 'C', 'A', 'B', 'D', 'B'}, 1); got != 6 {
		t.Fatalf("leastInterval() = %d, want 6", got)
	}
	if got := leastInterval([]byte{'A', 'A', 'A', 'B', 'B', 'B'}, 0); got != 6 {
		t.Fatalf("leastInterval() = %d, want 6", got)
	}
}
