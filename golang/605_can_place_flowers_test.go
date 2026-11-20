package leetcode

import "testing"

func TestCanPlaceFlowersExamples(t *testing.T) {
	if !canPlaceFlowers([]int{1, 0, 0, 0, 1}, 1) {
		t.Fatal("canPlaceFlowers([1,0,0,0,1], 1) = false, want true")
	}
	if canPlaceFlowers([]int{1, 0, 0, 0, 1}, 2) {
		t.Fatal("canPlaceFlowers([1,0,0,0,1], 2) = true, want false")
	}
	if !canPlaceFlowers([]int{0, 0, 1, 0, 0}, 2) {
		t.Fatal("canPlaceFlowers([0,0,1,0,0], 2) = false, want true")
	}
}
