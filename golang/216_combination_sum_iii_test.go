package leetcode

import "testing"

func TestCombinationSum3ExampleOne(t *testing.T) {
	got := combinationSum3(3, 7)
	want := [][]int{{1, 2, 4}}
	if !sameIntGroups(got, want) {
		t.Fatalf("combinationSum3() = %v, want %v", got, want)
	}
}

func TestCombinationSum3ExampleTwo(t *testing.T) {
	got := combinationSum3(3, 9)
	want := [][]int{{1, 2, 6}, {1, 3, 5}, {2, 3, 4}}
	if !sameIntGroups(got, want) {
		t.Fatalf("combinationSum3() = %v, want %v", got, want)
	}
}
