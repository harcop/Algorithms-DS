package leetcode

import "math"

// LeetCode #121 - Best Time to Buy and Sell Stock
//
// Track the cheapest price so far and the best later sale.
func maxProfit(prices []int) int {
	minPrice := math.MaxInt
	best := 0
	for _, price := range prices {
		if price < minPrice {
			minPrice = price
		}
		if gain := price - minPrice; gain > best {
			best = gain
		}
	}
	return best
}
