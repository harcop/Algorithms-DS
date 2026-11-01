package leetcode

// LeetCode #325 - Maximum Size Subarray Sum Equals k
//
// The first time each prefix appears gives the longest span that sums to k.
func maxSubArrayLen(nums []int, k int) int {
	best := 0
	prefix := 0
	first := map[int]int{0: -1}
	for i, value := range nums {
		prefix += value
		if left, ok := first[prefix-k]; ok && i-left > best {
			best = i - left
		}
		if _, seen := first[prefix]; !seen {
			first[prefix] = i
		}
	}
	return best
}
