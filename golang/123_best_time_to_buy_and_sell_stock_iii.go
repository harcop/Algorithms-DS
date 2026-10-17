package leetcode

import "math"

// LeetCode #123 - Best Time to Buy and Sell Stock III
//
// LeetCode names this maxProfit; that name already belongs to a single sale (#121).
// Keep the best one-sale profit, then the best second sale that starts after it.
func maxProfitIII(prices []int) int {
	buy1, buy2 := math.MaxInt, math.MaxInt
	profit1, profit2 := 0, 0
	for _, price := range prices {
		if price < buy1 {
			buy1 = price
		}
		if gain := price - buy1; gain > profit1 {
			profit1 = gain
		}
		if cost := price - profit1; cost < buy2 {
			buy2 = cost
		}
		if gain := price - buy2; gain > profit2 {
			profit2 = gain
		}
	}
	return profit2
}
