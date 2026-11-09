package leetcode

// LeetCode #477 - Total Hamming Distance
//
// On each bit, every 1 pairs with every 0, and those pairs are the distances for that bit.
func totalHammingDistance(nums []int) int {
	total := 0
	for bit := 0; bit < 31; bit++ {
		ones := 0
		for _, value := range nums {
			ones += (value >> bit) & 1
		}
		total += ones * (len(nums) - ones)
	}
	return total
}
