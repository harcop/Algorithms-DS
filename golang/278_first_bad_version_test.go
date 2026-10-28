package leetcode

import "testing"

func TestFirstBadVersionExampleOne(t *testing.T) {
	badVersionCutoff = 4
	if got := firstBadVersion(5); got != 4 {
		t.Fatalf("firstBadVersion() = %d, want 4", got)
	}
}

func TestFirstBadVersionExampleTwo(t *testing.T) {
	badVersionCutoff = 1
	if got := firstBadVersion(1); got != 1 {
		t.Fatalf("firstBadVersion() = %d, want 1", got)
	}
}
