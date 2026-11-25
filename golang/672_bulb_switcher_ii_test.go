package leetcode

import "testing"

func TestFlipLightsExamples(t *testing.T) {
	if got := flipLights(1, 1); got != 2 {
		t.Fatalf("flipLights(1, 1) = %d, want 2", got)
	}
	if got := flipLights(2, 1); got != 3 {
		t.Fatalf("flipLights(2, 1) = %d, want 3", got)
	}
	if got := flipLights(3, 1); got != 4 {
		t.Fatalf("flipLights(3, 1) = %d, want 4", got)
	}
	if got := flipLights(1, 0); got != 1 {
		t.Fatalf("flipLights(1, 0) = %d, want 1", got)
	}
}
