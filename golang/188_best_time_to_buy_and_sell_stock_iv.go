package leetcode

import "math"

// LeetCode #188 - Best Time to Buy and Sell Stock IV
//
// LeetCode names this maxProfit; that name already belongs to one transaction (#121).
// At most k transactions; past n/2 trades the limit no longer matters.
func maxProfitIV(k int, prices []int) int {
	n := len(prices)
	if n == 0 || k == 0 {
		return 0
	}
	if k >= n/2 {
		profit := 0
		for i := 1; i < n; i++ {
			if prices[i] > prices[i-1] {
				profit += prices[i] - prices[i-1]
			}
		}
		return profit
	}
	buy := make([]int, k+1)
	sell := make([]int, k+1)
	for i := range buy {
		buy[i] = math.MinInt32 / 4
	}
	for _, price := range prices {
		for t := 1; t <= k; t++ {
			buy[t] = max(buy[t], sell[t-1]-price)
			sell[t] = max(sell[t], buy[t]+price)
		}
	}
	return sell[k]
}
