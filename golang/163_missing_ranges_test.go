package leetcode

import "testing"

func TestFindMissingRangesExampleOne(t *testing.T) {
	got := findMissingRanges([]int{0, 1, 3, 50, 75}, 0, 99)
	want := []string{"2", "4->49", "51->74", "76->99"}
	if len(got) != len(want) {
		t.Fatalf("findMissingRanges() = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("findMissingRanges() = %v, want %v", got, want)
		}
	}
}

func TestFindMissingRangesExampleTwo(t *testing.T) {
	got := findMissingRanges(nil, 1, 1)
	if len(got) != 1 || got[0] != "1" {
		t.Fatalf("findMissingRanges() = %v, want [1]", got)
	}
}
