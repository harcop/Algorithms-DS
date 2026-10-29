package leetcode

import "math"

// LeetCode #309 - Best Time to Buy and Sell Stock with Cooldown
//
// LeetCode names this maxProfit; that name already belongs to one transaction (#121).
// After a sale the next buy waits one day, so track holding, just sold, and resting.
func maxProfitCooldown(prices []int) int {
	hold := math.MinInt32 / 4
	sold, rest := 0, 0
	for _, price := range prices {
		prevHold, prevSold := hold, sold
		hold = max(hold, rest-price)
		sold = prevHold + price
		rest = max(rest, prevSold)
	}
	return max(sold, rest)
}
