package leetcode

import "testing"

func TestTrailingZeroesExampleOne(t *testing.T) {
	if got := trailingZeroes(3); got != 0 {
		t.Fatalf("trailingZeroes() = %d, want 0", got)
	}
}

func TestTrailingZeroesExampleTwo(t *testing.T) {
	if got := trailingZeroes(5); got != 1 {
		t.Fatalf("trailingZeroes() = %d, want 1", got)
	}
}

func TestTrailingZeroesExampleThree(t *testing.T) {
	if got := trailingZeroes(0); got != 0 {
		t.Fatalf("trailingZeroes() = %d, want 0", got)
	}
}
