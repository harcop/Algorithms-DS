package leetcode

// LeetCode #74 - Search a 2D Matrix
//
// Treat the sorted rows as one increasing sequence and binary-search that sequence.
func searchMatrix(matrix [][]int, target int) bool {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return false
	}
	n := len(matrix[0])
	lo, hi := 0, len(matrix)*n
	for lo < hi {
		mid := lo + (hi-lo)/2
		val := matrix[mid/n][mid%n]
		if val < target {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo < len(matrix)*n && matrix[lo/n][lo%n] == target
}
