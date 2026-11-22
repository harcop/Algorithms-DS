package leetcode

import "math"

// LeetCode #628 - Maximum Product of Three Numbers
//
// The product is the three largest values, or the two smallest values with the largest.
func maximumProduct(nums []int) int {
	min1, min2 := math.MaxInt, math.MaxInt
	max1, max2, max3 := math.MinInt, math.MinInt, math.MinInt
	for _, num := range nums {
		if num < min1 {
			min1, min2 = num, min1
		} else if num < min2 {
			min2 = num
		}
		if num > max1 {
			max1, max2, max3 = num, max1, max2
		} else if num > max2 {
			max2, max3 = num, max2
		} else if num > max3 {
			max3 = num
		}
	}
	high := max1 * max2 * max3
	lowPair := min1 * min2 * max1
	if lowPair > high {
		return lowPair
	}
	return high
}
