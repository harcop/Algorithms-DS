package leetcode

import "testing"

func TestCalPointsExamples(t *testing.T) {
	if got := calPoints([]string{"5", "2", "C", "D", "+"}); got != 30 {
		t.Fatalf("calPoints() = %d, want 30", got)
	}
	if got := calPoints([]string{"5", "-2", "4", "C", "D", "9", "+", "+"}); got != 27 {
		t.Fatalf("calPoints() = %d, want 27", got)
	}
}
