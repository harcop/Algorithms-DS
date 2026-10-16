package leetcode

import "testing"

func TestMinDistanceExampleOne(t *testing.T) {
	got := minDistance("horse", "ros")
	if got != 3 {
		t.Fatalf("minDistance() = %d, want 3", got)
	}
}

func TestMinDistanceExampleTwo(t *testing.T) {
	got := minDistance("intention", "execution")
	if got != 5 {
		t.Fatalf("minDistance() = %d, want 5", got)
	}
}
