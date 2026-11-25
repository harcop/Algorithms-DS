package leetcode

import "testing"

func TestConstructArrayExamples(t *testing.T) {
	if got := constructArray(3, 1); !sameInts(got, []int{1, 2, 3}) {
		t.Fatalf("constructArray(3, 1) = %v, want [1 2 3]", got)
	}
	if got := constructArray(3, 2); !sameInts(got, []int{1, 3, 2}) {
		t.Fatalf("constructArray(3, 2) = %v, want [1 3 2]", got)
	}
}
