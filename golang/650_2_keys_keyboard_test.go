package leetcode

import "testing"

func TestMinStepsExamples(t *testing.T) {
	if got := minSteps(3); got != 3 {
		t.Fatalf("minSteps(3) = %d, want 3", got)
	}
	if got := minSteps(1); got != 0 {
		t.Fatalf("minSteps(1) = %d, want 0", got)
	}
	if got := minSteps(6); got != 5 {
		t.Fatalf("minSteps(6) = %d, want 5", got)
	}
}
