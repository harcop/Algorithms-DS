package leetcode

import "testing"

func TestFindRelativeRanksExample(t *testing.T) {
	got := findRelativeRanks([]int{5, 4, 3, 2, 1})
	want := []string{"Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"}
	if len(got) != len(want) {
		t.Fatalf("findRelativeRanks() = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("findRelativeRanks() = %v, want %v", got, want)
		}
	}
}
