package leetcode

import "sort"

// LeetCode #18 - 4Sum
//
// Sort, fix two numbers, then two-pointer the rest. Sums use int64 to avoid overflow.
func fourSum(nums []int, target int) [][]int {
	sort.Ints(nums)
	n := len(nums)
	if n < 4 {
		return nil
	}

	var result [][]int
	target64 := int64(target)

	for i := 0; i < n-3; i++ {
		if i > 0 && nums[i] == nums[i-1] {
			continue
		}
		for j := i + 1; j < n-2; j++ {
			if j > i+1 && nums[j] == nums[j-1] {
				continue
			}

			left := j + 1
			right := n - 1
			for left < right {
				sum := int64(nums[i]) + int64(nums[j]) + int64(nums[left]) + int64(nums[right])
				if sum < target64 {
					left++
				} else if sum > target64 {
					right--
				} else {
					result = append(result, []int{nums[i], nums[j], nums[left], nums[right]})
					left++
					right--
					for left < right && nums[left] == nums[left-1] {
						left++
					}
					for left < right && nums[right] == nums[right+1] {
						right--
					}
				}
			}
		}
	}

	return result
}
