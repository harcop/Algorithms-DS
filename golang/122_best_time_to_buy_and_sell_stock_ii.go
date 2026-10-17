package leetcode

// LeetCode #122 - Best Time to Buy and Sell Stock II
//
// LeetCode names this maxProfit; that name already belongs to a single sale (#121).
// Add every upward step, which is the same as selling before each drop.
func maxProfitII(prices []int) int {
	total := 0
	for i := 1; i < len(prices); i++ {
		if prices[i] > prices[i-1] {
			total += prices[i] - prices[i-1]
		}
	}
	return total
}
