package leetcode

// LeetCode #525 - Contiguous Array
//
// Treat zeros as minus one. The longest stretch with equal zeros and ones returns the balance to a value seen before.
func findMaxLength(nums []int) int {
	first := map[int]int{0: -1}
	balance, best := 0, 0
	for i, value := range nums {
		if value == 1 {
			balance++
		} else {
			balance--
		}
		if previous, ok := first[balance]; ok {
			best = max(best, i-previous)
			continue
		}
		first[balance] = i
	}
	return best
}
