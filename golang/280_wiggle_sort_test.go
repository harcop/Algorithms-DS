package leetcode

import "testing"

func isWiggle(nums []int) bool {
	for i := 0; i+1 < len(nums); i++ {
		if i%2 == 0 && nums[i] >= nums[i+1] {
			return false
		}
		if i%2 == 1 && nums[i] <= nums[i+1] {
			return false
		}
	}
	return true
}

func TestWiggleSortExampleOne(t *testing.T) {
	nums := []int{3, 5, 2, 1, 6, 4}
	wiggleSort(nums)
	if !isWiggle(nums) {
		t.Fatalf("wiggleSort() = %v, which does not wiggle", nums)
	}
}
