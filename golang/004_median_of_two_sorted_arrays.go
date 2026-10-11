package leetcode

import "math"

// LeetCode #4 - Median of Two Sorted Arrays
//
// Given two sorted arrays nums1 and nums2 of size m and n respectively,
// return the median of the two sorted arrays. Overall run time complexity should be O(log (m+n)).
func findMedianSortedArrays(nums1 []int, nums2 []int) float64 {
	a, b := nums1, nums2
	if len(a) > len(b) {
		a, b = b, a
	}

	m := len(a)
	n := len(b)
	left := 0
	right := m

	for left <= right {
		partitionA := (left + right) / 2
		partitionB := (m+n+1)/2 - partitionA

		maxLeftA := math.MinInt32
		if partitionA > 0 {
			maxLeftA = a[partitionA-1]
		}
		minRightA := math.MaxInt32
		if partitionA < m {
			minRightA = a[partitionA]
		}

		maxLeftB := math.MinInt32
		if partitionB > 0 {
			maxLeftB = b[partitionB-1]
		}
		minRightB := math.MaxInt32
		if partitionB < n {
			minRightB = b[partitionB]
		}

		if maxLeftA <= minRightB && maxLeftB <= minRightA {
			if (m+n)%2 == 0 {
				leftMax := max(maxLeftA, maxLeftB)
				rightMin := min(minRightA, minRightB)
				return (float64(leftMax) + float64(rightMin)) / 2
			}
			return float64(max(maxLeftA, maxLeftB))
		}

		if maxLeftA > minRightB {
			right = partitionA - 1
		} else {
			left = partitionA + 1
		}
	}

	return 0
}
