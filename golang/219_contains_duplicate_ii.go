package leetcode

// LeetCode #219 - Contains Duplicate II
//
// Remember the last index of each value and see whether it falls inside the window.
func containsNearbyDuplicate(nums []int, k int) bool {
	last := map[int]int{}
	for i, value := range nums {
		if j, ok := last[value]; ok && i-j <= k {
			return true
		}
		last[value] = i
	}
	return false
}
