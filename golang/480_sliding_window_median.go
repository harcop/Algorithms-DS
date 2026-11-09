package leetcode

import "sort"

// LeetCode #480 - Sliding Window Median
//
// Sort each window and read the middle value, or the average of the two middle values when the window length is even.
func medianSlidingWindow(nums []int, k int) []float64 {
	out := make([]float64, 0, len(nums)-k+1)
	for i := 0; i+k <= len(nums); i++ {
		window := append([]int(nil), nums[i:i+k]...)
		sort.Ints(window)
		if k%2 == 1 {
			out = append(out, float64(window[k/2]))
			continue
		}
		out = append(out, (float64(window[k/2-1])+float64(window[k/2]))/2)
	}
	return out
}
