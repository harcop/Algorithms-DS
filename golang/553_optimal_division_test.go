package leetcode

import "testing"

func TestOptimalDivisionExample(t *testing.T) {
	got := optimalDivision([]int{1000, 100, 10, 2})
	if got != "1000/(100/10/2)" {
		t.Fatalf("optimalDivision = %q, want 1000/(100/10/2)", got)
	}
}
