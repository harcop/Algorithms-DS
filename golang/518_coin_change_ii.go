package leetcode

// LeetCode #518 - Coin Change II
//
// Each coin extends the combinations already counted for the smaller amounts.
func change(amount int, coins []int) int {
	ways := make([]int, amount+1)
	ways[0] = 1
	for _, coin := range coins {
		for total := coin; total <= amount; total++ {
			ways[total] += ways[total-coin]
		}
	}
	return ways[amount]
}
