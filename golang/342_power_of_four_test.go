package leetcode

import "testing"

func TestIsPowerOfFourExampleOne(t *testing.T) {
	if !isPowerOfFour(16) {
		t.Fatal("isPowerOfFour(16) = false, want true")
	}
}

func TestIsPowerOfFourExampleTwo(t *testing.T) {
	if isPowerOfFour(8) {
		t.Fatal("isPowerOfFour(8) = true, want false")
	}
}
