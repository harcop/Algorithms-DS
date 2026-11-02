package leetcode

import (
	"math"
	"sort"
)

// LeetCode #363 - Max Sum of Rectangle No Larger Than K
//
// Fix the left and right columns, then find the best row-range sum that stays at most k.
func maxSumSubmatrix(matrix [][]int, k int) int {
	rows, cols := len(matrix), len(matrix[0])
	best := math.MinInt
	colSum := make([]int, rows)
	for left := 0; left < cols; left++ {
		for i := range colSum {
			colSum[i] = 0
		}
		for right := left; right < cols; right++ {
			for r := 0; r < rows; r++ {
				colSum[r] += matrix[r][right]
			}
			seen := []int{0}
			prefix := 0
			for _, value := range colSum {
				prefix += value
				at := sort.SearchInts(seen, prefix-k)
				if at < len(seen) {
					sum := prefix - seen[at]
					if sum > best {
						best = sum
						if best == k {
							return k
						}
					}
				}
				seen = insertSorted(seen, prefix)
			}
		}
	}
	return best
}

func insertSorted(values []int, value int) []int {
	at := sort.SearchInts(values, value)
	if at < len(values) && values[at] == value {
		return values
	}
	values = append(values, 0)
	copy(values[at+1:], values[at:])
	values[at] = value
	return values
}
