package leetcode

import "testing"

func TestIsPowerOfThreeExamples(t *testing.T) {
	if !isPowerOfThree(27) {
		t.Fatal("isPowerOfThree(27) = false, want true")
	}
	if isPowerOfThree(0) {
		t.Fatal("isPowerOfThree(0) = true, want false")
	}
}
