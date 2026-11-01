package leetcode

import "testing"

func TestCountBitsExampleOne(t *testing.T) {
	got := countBits(2)
	want := []int{0, 1, 1}
	if !sameInts(got, want) {
		t.Fatalf("countBits() = %v, want %v", got, want)
	}
}
