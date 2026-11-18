package leetcode

// LeetCode #594 - Longest Harmonious Subsequence
//
// A harmonious pair is two values one apart, and the length is their combined frequencies.
func findLHS(nums []int) int {
	count := map[int]int{}
	for _, value := range nums {
		count[value]++
	}
	best := 0
	for value, freq := range count {
		if next, ok := count[value+1]; ok {
			best = max(best, freq+next)
		}
	}
	return best
}
