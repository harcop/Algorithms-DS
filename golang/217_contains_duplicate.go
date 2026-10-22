package leetcode

// LeetCode #217 - Contains Duplicate
//
// A value seen twice is a duplicate.
func containsDuplicate(nums []int) bool {
	seen := map[int]struct{}{}
	for _, value := range nums {
		if _, ok := seen[value]; ok {
			return true
		}
		seen[value] = struct{}{}
	}
	return false
}
