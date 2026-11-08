package leetcode

import "testing"

func TestAddTwoNumbersIIExample(t *testing.T) {
	got := listToVec(addTwoNumbersII(vecToList([]int{7, 2, 4, 3}), vecToList([]int{5, 6, 4})))
	want := []int{7, 8, 0, 7}
	if !sameInts(got, want) {
		t.Fatalf("addTwoNumbersII() = %v, want %v", got, want)
	}
}
