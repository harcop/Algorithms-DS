package leetcode

import "testing"

func TestIsOneEditDistanceExampleOne(t *testing.T) {
	if !isOneEditDistance("ab", "acb") {
		t.Fatal("isOneEditDistance() = false, want true")
	}
}

func TestIsOneEditDistanceExampleTwo(t *testing.T) {
	if isOneEditDistance("cab", "ad") {
		t.Fatal("isOneEditDistance() = true, want false")
	}
}
