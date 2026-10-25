package leetcode

import "testing"

func TestGetFactorsExampleOne(t *testing.T) {
	got := getFactors(12)
	want := [][]int{{2, 6}, {2, 2, 3}, {3, 4}}
	if !sameIntGroups(got, want) {
		t.Fatalf("getFactors() = %v, want %v", got, want)
	}
}

func TestGetFactorsExampleTwo(t *testing.T) {
	if len(getFactors(1)) != 0 {
		t.Fatal("getFactors(1) returned combinations, want none")
	}
}
