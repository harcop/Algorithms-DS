package leetcode

import "testing"

func TestHasAlternatingBitsExamples(t *testing.T) {
	if !hasAlternatingBits(5) {
		t.Fatal("hasAlternatingBits(5) = false, want true")
	}
	if hasAlternatingBits(7) {
		t.Fatal("hasAlternatingBits(7) = true, want false")
	}
	if hasAlternatingBits(11) {
		t.Fatal("hasAlternatingBits(11) = true, want false")
	}
}
