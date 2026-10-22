package leetcode

import "testing"

func TestSummaryRangesExampleOne(t *testing.T) {
	got := summaryRanges([]int{0, 1, 2, 4, 5, 7})
	want := []string{"0->2", "4->5", "7"}
	if len(got) != len(want) {
		t.Fatalf("summaryRanges() = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("summaryRanges() = %v, want %v", got, want)
		}
	}
}

func TestSummaryRangesExampleTwo(t *testing.T) {
	got := summaryRanges([]int{0, 2, 3, 4, 6, 8, 9})
	want := []string{"0", "2->4", "6", "8->9"}
	if len(got) != len(want) {
		t.Fatalf("summaryRanges() = %v, want %v", got, want)
	}
	for i := range want {
		if got[i] != want[i] {
			t.Fatalf("summaryRanges() = %v, want %v", got, want)
		}
	}
}
