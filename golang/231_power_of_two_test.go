package leetcode

import "testing"

func TestIsPowerOfTwoExampleOne(t *testing.T) {
	if !isPowerOfTwo(1) {
		t.Fatal("isPowerOfTwo(1) = false, want true")
	}
}

func TestIsPowerOfTwoExampleTwo(t *testing.T) {
	if !isPowerOfTwo(16) {
		t.Fatal("isPowerOfTwo(16) = false, want true")
	}
}

func TestIsPowerOfTwoExampleThree(t *testing.T) {
	if isPowerOfTwo(3) {
		t.Fatal("isPowerOfTwo(3) = true, want false")
	}
}
