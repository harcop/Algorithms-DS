package leetcode

import "testing"

func TestNthUglyNumberExampleOne(t *testing.T) {
	if got := nthUglyNumber(10); got != 12 {
		t.Fatalf("nthUglyNumber() = %d, want 12", got)
	}
}

func TestNthUglyNumberExampleTwo(t *testing.T) {
	if got := nthUglyNumber(1); got != 1 {
		t.Fatalf("nthUglyNumber() = %d, want 1", got)
	}
}
