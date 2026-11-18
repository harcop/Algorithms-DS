package leetcode

import "testing"

func TestMinDeleteDistanceExamples(t *testing.T) {
	if got := minDeleteDistance("sea", "eat"); got != 2 {
		t.Fatalf("minDeleteDistance = %d, want 2", got)
	}
	if got := minDeleteDistance("leetcode", "etco"); got != 4 {
		t.Fatalf("minDeleteDistance = %d, want 4", got)
	}
}
