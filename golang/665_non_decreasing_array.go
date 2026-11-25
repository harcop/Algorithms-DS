package leetcode

// LeetCode #665 - Non-decreasing Array
//
// One descent can be repaired by lowering the previous value or raising the current one.
func checkPossibility(nums []int) bool {
	changed := false
	for i := 1; i < len(nums); i++ {
		if nums[i] >= nums[i-1] {
			continue
		}
		if changed {
			return false
		}
		changed = true
		if i == 1 || nums[i] >= nums[i-2] {
			nums[i-1] = nums[i]
		} else {
			nums[i] = nums[i-1]
		}
	}
	return true
}
