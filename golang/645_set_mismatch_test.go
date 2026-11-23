package leetcode

import "testing"

func TestFindErrorNumsExamples(t *testing.T) {
	if got := findErrorNums([]int{1, 2, 2, 4}); !sameInts(got, []int{2, 3}) {
		t.Fatalf("findErrorNums() = %v, want [2 3]", got)
	}
	if got := findErrorNums([]int{1, 1}); !sameInts(got, []int{1, 2}) {
		t.Fatalf("findErrorNums() = %v, want [1 2]", got)
	}
}
