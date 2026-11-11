package leetcode

import "testing"

func TestFindMinStepExamples(t *testing.T) {
	if got := findMinStep("WRRBBW", "RB"); got != -1 {
		t.Fatalf("findMinStep(WRRBBW) = %d, want -1", got)
	}
	if got := findMinStep("WWRRBBWW", "WRBRW"); got != 2 {
		t.Fatalf("findMinStep(WWRRBBWW) = %d, want 2", got)
	}
	if got := findMinStep("G", "GGGGG"); got != 2 {
		t.Fatalf("findMinStep(G) = %d, want 2", got)
	}
}
