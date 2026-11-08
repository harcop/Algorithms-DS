package leetcode

import "testing"

func TestPoorPigsExamples(t *testing.T) {
	if got := poorPigs(1000, 15, 60); got != 5 {
		t.Fatalf("poorPigs(1000) = %d, want 5", got)
	}
	if got := poorPigs(4, 15, 15); got != 2 {
		t.Fatalf("poorPigs(4) = %d, want 2", got)
	}
}
