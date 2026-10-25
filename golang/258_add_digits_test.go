package leetcode

import "testing"

func TestAddDigitsExampleOne(t *testing.T) {
	if got := addDigits(38); got != 2 {
		t.Fatalf("addDigits() = %d, want 2", got)
	}
}

func TestAddDigitsExampleTwo(t *testing.T) {
	if got := addDigits(0); got != 0 {
		t.Fatalf("addDigits() = %d, want 0", got)
	}
}
