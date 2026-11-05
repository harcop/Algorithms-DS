package leetcode

// LeetCode #413 - Arithmetic Slices
//
// A slice that keeps the same difference grows by one new subarray for every extra element.
func numberOfArithmeticSlices(nums []int) int {
	cur := 0
	total := 0
	for i := 2; i < len(nums); i++ {
		if nums[i]-nums[i-1] == nums[i-1]-nums[i-2] {
			cur++
			total += cur
			continue
		}
		cur = 0
	}
	return total
}
