package leetcode

import "math"

// LeetCode #265 - Paint House II
//
// Paint each house a color plus the cheapest different color used on the previous house.
func minCostII(costs [][]int) int {
	if len(costs) == 0 {
		return 0
	}
	k := len(costs[0])
	prev := append([]int(nil), costs[0]...)
	for _, row := range costs[1:] {
		next := make([]int, k)
		for color := 0; color < k; color++ {
			best := math.MaxInt32
			for other := 0; other < k; other++ {
				if other != color && prev[other] < best {
					best = prev[other]
				}
			}
			next[color] = row[color] + best
		}
		prev = next
	}
	best := prev[0]
	for _, cost := range prev[1:] {
		if cost < best {
			best = cost
		}
	}
	return best
}
