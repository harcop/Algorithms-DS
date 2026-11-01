package leetcode

import "testing"

func TestDepthSumExample(t *testing.T) {
	nested := []NestedInteger{
		nestedList(nestedInt(1), nestedInt(1)),
		nestedInt(2),
		nestedList(nestedInt(1), nestedInt(1)),
	}
	if got := depthSum(nested); got != 10 {
		t.Fatalf("depthSum() = %d, want 10", got)
	}
}
