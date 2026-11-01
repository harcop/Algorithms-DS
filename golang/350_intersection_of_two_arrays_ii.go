package leetcode

// LeetCode #350 - Intersection of Two Arrays II
//
// Each shared value is kept as many times as it appears in both arrays.
func intersect(nums1, nums2 []int) []int {
	counts := map[int]int{}
	for _, value := range nums1 {
		counts[value]++
	}
	var out []int
	for _, value := range nums2 {
		if counts[value] == 0 {
			continue
		}
		out = append(out, value)
		counts[value]--
	}
	return out
}
