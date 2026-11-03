package leetcode

// LeetCode #378 - Kth Smallest Element in a Sorted Matrix
//
// Binary search the value, counting how many entries are no larger than the middle.
func kthSmallestMatrix(matrix [][]int, k int) int {
	n := len(matrix)
	lo, hi := matrix[0][0], matrix[n-1][n-1]
	for lo < hi {
		mid := lo + (hi-lo)/2
		if countAtMost(matrix, mid) < k {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo
}

func countAtMost(matrix [][]int, target int) int {
	n := len(matrix)
	count := 0
	row, col := n-1, 0
	for row >= 0 && col < n {
		if matrix[row][col] <= target {
			count += row + 1
			col++
		} else {
			row--
		}
	}
	return count
}
