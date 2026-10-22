package leetcode

import "testing"

func TestGetSkylineExampleOne(t *testing.T) {
	buildings := [][]int{{2, 9, 10}, {3, 7, 15}, {5, 12, 12}, {15, 20, 10}, {19, 24, 8}}
	got := getSkyline(buildings)
	want := [][]int{{2, 10}, {3, 15}, {7, 12}, {12, 0}, {15, 10}, {20, 8}, {24, 0}}
	if !sameMatrix(got, want) {
		t.Fatalf("getSkyline() = %v, want %v", got, want)
	}
}
