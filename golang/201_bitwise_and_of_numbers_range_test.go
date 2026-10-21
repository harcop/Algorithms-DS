package leetcode

import "testing"

func TestRangeBitwiseAndExampleOne(t *testing.T) {
	if got := rangeBitwiseAnd(5, 7); got != 4 {
		t.Fatalf("rangeBitwiseAnd() = %d, want 4", got)
	}
}

func TestRangeBitwiseAndExampleTwo(t *testing.T) {
	if got := rangeBitwiseAnd(0, 0); got != 0 {
		t.Fatalf("rangeBitwiseAnd() = %d, want 0", got)
	}
}

func TestRangeBitwiseAndExampleThree(t *testing.T) {
	if got := rangeBitwiseAnd(1, 2147483647); got != 0 {
		t.Fatalf("rangeBitwiseAnd() = %d, want 0", got)
	}
}
