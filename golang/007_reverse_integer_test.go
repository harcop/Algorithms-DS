package leetcode

import "testing"

func TestReverseExampleOne(t *testing.T) {
	if got := reverse(123); got != 321 {
		t.Fatalf("reverse() = %d, want 321", got)
	}
}

func TestReverseExampleTwo(t *testing.T) {
	if got := reverse(-123); got != -321 {
		t.Fatalf("reverse() = %d, want -321", got)
	}
}

func TestReverseExampleThree(t *testing.T) {
	if got := reverse(120); got != 21 {
		t.Fatalf("reverse() = %d, want 21", got)
	}
}

func TestReverseOverflowReturnsZero(t *testing.T) {
	if got := reverse(1534236469); got != 0 {
		t.Fatalf("reverse() = %d, want 0", got)
	}
}
