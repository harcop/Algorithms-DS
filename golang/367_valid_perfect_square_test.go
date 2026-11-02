package leetcode

import "testing"

func TestIsPerfectSquareExamples(t *testing.T) {
	if !isPerfectSquare(16) {
		t.Fatal("isPerfectSquare(16) = false, want true")
	}
	if isPerfectSquare(14) {
		t.Fatal("isPerfectSquare(14) = true, want false")
	}
	if !isPerfectSquare(1) {
		t.Fatal("isPerfectSquare(1) = false, want true")
	}
	if !isPerfectSquare(2147395600) {
		t.Fatal("isPerfectSquare(2147395600) = false, want true")
	}
}
