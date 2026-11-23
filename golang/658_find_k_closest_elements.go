package leetcode

// LeetCode #658 - Find K Closest Elements
//
// The start of the window moves right only when the element just past it is closer than the element it would drop.
func findClosestElements(arr []int, k int, x int) []int {
	lo, hi := 0, len(arr)-k
	for lo < hi {
		mid := (lo + hi) / 2
		if x-arr[mid] > arr[mid+k]-x {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return arr[lo : lo+k]
}
