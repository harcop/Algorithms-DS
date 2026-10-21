package leetcode

import "testing"

func TestHammingWeightExampleOne(t *testing.T) {
	if got := hammingWeight(11); got != 3 {
		t.Fatalf("hammingWeight() = %d, want 3", got)
	}
}

func TestHammingWeightExampleTwo(t *testing.T) {
	if got := hammingWeight(128); got != 1 {
		t.Fatalf("hammingWeight() = %d, want 1", got)
	}
}

func TestHammingWeightExampleThree(t *testing.T) {
	if got := hammingWeight(2147483645); got != 30 {
		t.Fatalf("hammingWeight() = %d, want 30", got)
	}
}
