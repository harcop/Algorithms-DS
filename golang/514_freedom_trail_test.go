package leetcode

import "testing"

func TestFindRotateStepsExamples(t *testing.T) {
	if got := findRotateSteps("godding", "gd"); got != 4 {
		t.Fatalf("findRotateSteps(gd) = %d, want 4", got)
	}
	if got := findRotateSteps("godding", "godding"); got != 13 {
		t.Fatalf("findRotateSteps(godding) = %d, want 13", got)
	}
}
