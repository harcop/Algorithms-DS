package leetcode

import "testing"

func TestSwapPairsExampleOne(t *testing.T) {
	head := vecToList([]int{1, 2, 3, 4})
	got := listToVec(swapPairs(head))
	want := []int{2, 1, 4, 3}
	if !sameInts(got, want) {
		t.Fatalf("swapPairs() = %v, want %v", got, want)
	}
}

func TestSwapPairsExampleTwo(t *testing.T) {
	got := listToVec(swapPairs(nil))
	if len(got) != 0 {
		t.Fatalf("swapPairs() = %v, want empty", got)
	}
}

func TestSwapPairsExampleThree(t *testing.T) {
	head := vecToList([]int{1})
	got := listToVec(swapPairs(head))
	want := []int{1}
	if !sameInts(got, want) {
		t.Fatalf("swapPairs() = %v, want %v", got, want)
	}
}
