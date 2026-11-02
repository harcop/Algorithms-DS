package leetcode

import "testing"

func TestSummaryRangesMerges(t *testing.T) {
	ranges := NewSummaryRanges()
	for _, value := range []int{1, 3, 7, 2, 6} {
		ranges.AddNum(value)
	}
	got := ranges.GetIntervals()
	want := [][]int{{1, 3}, {6, 7}}
	if !sameMatrix(got, want) {
		t.Fatalf("GetIntervals() = %v, want %v", got, want)
	}
	ranges.AddNum(1)
	if got = ranges.GetIntervals(); !sameMatrix(got, want) {
		t.Fatalf("GetIntervals() after duplicate = %v, want %v", got, want)
	}
}
