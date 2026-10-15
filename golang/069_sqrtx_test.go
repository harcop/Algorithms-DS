package leetcode

import "testing"

func TestMySqrtExampleOne(t *testing.T) {
	got := mySqrt(4)
	if got != 2 {
		t.Fatalf("mySqrt() = %d, want 2", got)
	}
}

func TestMySqrtExampleTwo(t *testing.T) {
	got := mySqrt(8)
	if got != 2 {
		t.Fatalf("mySqrt() = %d, want 2", got)
	}
}
