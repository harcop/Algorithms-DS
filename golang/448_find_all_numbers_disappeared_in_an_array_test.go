package leetcode

import "testing"

func TestFindDisappearedNumbersExample(t *testing.T) {
	got := findDisappearedNumbers([]int{4, 3, 2, 7, 8, 2, 3, 1})
	want := []int{5, 6}
	if !sameInts(got, want) {
		t.Fatalf("findDisappearedNumbers() = %v, want %v", got, want)
	}
}
