package leetcode

import "testing"

func TestNextClosestTimeExamples(t *testing.T) {
	if got := nextClosestTime("19:34"); got != "19:39" {
		t.Fatalf("nextClosestTime(19:34) = %q, want 19:39", got)
	}
	if got := nextClosestTime("23:59"); got != "22:22" {
		t.Fatalf("nextClosestTime(23:59) = %q, want 22:22", got)
	}
}
