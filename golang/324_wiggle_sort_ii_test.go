package leetcode

import "testing"

func TestWiggleSortIIExample(t *testing.T) {
	nums := []int{1, 5, 1, 1, 6, 4}
	original := append([]int(nil), nums...)
	wiggleSortII(nums)
	counts := map[int]int{}
	for _, value := range original {
		counts[value]++
	}
	for _, value := range nums {
		counts[value]--
	}
	for value, count := range counts {
		if count != 0 {
			t.Fatalf("wiggleSortII() = %v, count of %d is off by %d", nums, value, count)
		}
	}
	if !isWiggle(nums) {
		t.Fatalf("wiggleSortII() = %v, which does not wiggle", nums)
	}
}
