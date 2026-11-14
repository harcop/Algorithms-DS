package leetcode

// LeetCode #523 - Continuous Subarray Sum
//
// A repeated remainder means the stretch between those prefixes is a multiple of k, and it must cover at least two elements.
func checkSubarraySum(nums []int, k int) bool {
	seen := map[int]int{0: -1}
	sum := 0
	for i, value := range nums {
		sum += value
		remainder := sum
		if k != 0 {
			mod := k
			if mod < 0 {
				mod = -mod
			}
			remainder %= mod
			if remainder < 0 {
				remainder += mod
			}
		}
		if previous, ok := seen[remainder]; ok {
			if i-previous >= 2 {
				return true
			}
			continue
		}
		seen[remainder] = i
	}
	return false
}
