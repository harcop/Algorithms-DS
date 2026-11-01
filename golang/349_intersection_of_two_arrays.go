package leetcode

// LeetCode #349 - Intersection of Two Arrays
//
// A value is kept once when it appears in both arrays.
func intersection(nums1, nums2 []int) []int {
	seen := map[int]struct{}{}
	for _, value := range nums1 {
		seen[value] = struct{}{}
	}
	var out []int
	for _, value := range nums2 {
		if _, ok := seen[value]; ok {
			out = append(out, value)
			delete(seen, value)
		}
	}
	return out
}
