package leetcode

// LeetCode #26 - Remove Duplicates from Sorted Array
//
// Compact unique values to the front and return how many remain.
func removeDuplicates(nums []int) int {
	if len(nums) == 0 {
		return 0
	}

	write := 1
	for read := 1; read < len(nums); read++ {
		if nums[read] != nums[read-1] {
			nums[write] = nums[read]
			write++
		}
	}
	return write
}
