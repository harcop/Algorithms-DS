package leetcode

import "testing"

func TestStrobogrammaticInRangeExampleOne(t *testing.T) {
	if got := strobogrammaticInRange("50", "100"); got != 3 {
		t.Fatalf("strobogrammaticInRange() = %d, want 3", got)
	}
}

func TestStrobogrammaticInRangeExampleTwo(t *testing.T) {
	if got := strobogrammaticInRange("0", "0"); got != 1 {
		t.Fatalf("strobogrammaticInRange() = %d, want 1", got)
	}
}
