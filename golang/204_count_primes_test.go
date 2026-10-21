package leetcode

import "testing"

func TestCountPrimesExampleOne(t *testing.T) {
	if got := countPrimes(10); got != 4 {
		t.Fatalf("countPrimes() = %d, want 4", got)
	}
}

func TestCountPrimesExampleTwo(t *testing.T) {
	if got := countPrimes(0); got != 0 {
		t.Fatalf("countPrimes() = %d, want 0", got)
	}
}

func TestCountPrimesExampleThree(t *testing.T) {
	if got := countPrimes(1); got != 0 {
		t.Fatalf("countPrimes() = %d, want 0", got)
	}
}
