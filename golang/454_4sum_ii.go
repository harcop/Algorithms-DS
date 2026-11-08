package leetcode

// LeetCode #454 - 4Sum II
//
// Store every sum from the first two arrays, then count how often the last two arrays negate it.
func fourSumCount(nums1 []int, nums2 []int, nums3 []int, nums4 []int) int {
	sums := map[int]int{}
	for _, a := range nums1 {
		for _, b := range nums2 {
			sums[a+b]++
		}
	}
	count := 0
	for _, c := range nums3 {
		for _, d := range nums4 {
			count += sums[-(c + d)]
		}
	}
	return count
}
