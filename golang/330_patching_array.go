package leetcode

// LeetCode #330 - Patching Array
//
// miss is the smallest sum not yet formable; a gap is patched by adding miss itself.
func minPatches(nums []int, n int) int {
	limit := int64(n)
	patched := 0
	miss := int64(1)
	i := 0
	for miss <= limit {
		if i < len(nums) && int64(nums[i]) <= miss {
			miss += int64(nums[i])
			i++
		} else {
			patched++
			miss += miss
		}
	}
	return patched
}
