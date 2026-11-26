package leetcode

// LeetCode #689 - Maximum Sum of 3 Non-Overlapping Subarrays
//
// The best window to the left and right of each middle window gives the strongest triple, ties keeping the smaller index.
func maxSumOfThreeSubarrays(nums []int, k int) []int {
	n := len(nums)
	window := make([]int, n-k+1)
	sum := 0
	for i := 0; i < k; i++ {
		sum += nums[i]
	}
	window[0] = sum
	for i := k; i < n; i++ {
		sum += nums[i] - nums[i-k]
		window[i-k+1] = sum
	}
	left := make([]int, len(window))
	best := 0
	for i := range window {
		if window[i] > window[best] {
			best = i
		}
		left[i] = best
	}
	right := make([]int, len(window))
	best = len(window) - 1
	for i := len(window) - 1; i >= 0; i-- {
		if window[i] >= window[best] {
			best = i
		}
		right[i] = best
	}
	answer := []int{0, k, 2 * k}
	bestSum := -1
	for mid := k; mid+2*k <= n; mid++ {
		l, r := left[mid-k], right[mid+k]
		total := window[l] + window[mid] + window[r]
		if total > bestSum {
			bestSum = total
			answer = []int{l, mid, r}
		}
	}
	return answer
}
