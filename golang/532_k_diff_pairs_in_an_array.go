package leetcode

// LeetCode #532 - K-diff Pairs in an Array
//
// Count how many values have a partner k away, and when k is zero count the values that appear more than once.
func findPairs(nums []int, k int) int {
	if k < 0 {
		return 0
	}
	count := map[int]int{}
	for _, value := range nums {
		count[value]++
	}
	if k == 0 {
		pairs := 0
		for _, times := range count {
			if times > 1 {
				pairs++
			}
		}
		return pairs
	}
	pairs := 0
	for value := range count {
		if count[value+k] > 0 {
			pairs++
		}
	}
	return pairs
}
