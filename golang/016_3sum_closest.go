package leetcode

import "sort"

// LeetCode #16 - 3Sum Closest
//
// Sort, then for each fixed index scan with two pointers and keep the sum nearest to target.
func threeSumClosest(nums []int, target int) int {
	sort.Ints(nums)
	n := len(nums)
	closest := nums[0] + nums[1] + nums[2]

	for i := 0; i < n-2; i++ {
		left := i + 1
		right := n - 1
		for left < right {
			sum := nums[i] + nums[left] + nums[right]
			if distance(sum, target) < distance(closest, target) {
				closest = sum
			}
			if sum < target {
				left++
			} else if sum > target {
				right--
			} else {
				return target
			}
		}
	}

	return closest
}

func distance(a, b int) int {
	if a < b {
		return b - a
	}
	return a - b
}
