package leetcode

import "testing"

func TestDepthSumInverseDeep(t *testing.T) {
	nested := []NestedInteger{
		nestedInt(1),
		nestedList(nestedInt(4), nestedList(nestedInt(6))),
	}
	if got := depthSumInverse(nested); got != 17 {
		t.Fatalf("depthSumInverse() = %d, want 17", got)
	}
}

func TestDepthSumInverseWide(t *testing.T) {
	nested := []NestedInteger{
		nestedList(nestedInt(1), nestedInt(1)),
		nestedInt(2),
		nestedList(nestedInt(1), nestedInt(1)),
	}
	if got := depthSumInverse(nested); got != 8 {
		t.Fatalf("depthSumInverse() = %d, want 8", got)
	}
}
