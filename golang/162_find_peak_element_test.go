package leetcode

import "testing"

func isPeak(nums []int, index int) bool {
	if index < 0 || index >= len(nums) {
		return false
	}
	if index > 0 && nums[index] <= nums[index-1] {
		return false
	}
	if index+1 < len(nums) && nums[index] <= nums[index+1] {
		return false
	}
	return true
}

func TestFindPeakElementExampleOne(t *testing.T) {
	nums := []int{1, 2, 3, 1}
	if got := findPeakElement(nums); !isPeak(nums, got) {
		t.Fatalf("findPeakElement() = %d, which is not a peak", got)
	}
}

func TestFindPeakElementExampleTwo(t *testing.T) {
	nums := []int{1, 2, 1, 3, 5, 6, 4}
	if got := findPeakElement(nums); !isPeak(nums, got) {
		t.Fatalf("findPeakElement() = %d, which is not a peak", got)
	}
}
