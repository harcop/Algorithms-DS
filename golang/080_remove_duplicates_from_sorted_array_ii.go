package leetcode

// LeetCode #80 - Remove Duplicates from Sorted Array II
//
// LeetCode names this removeDuplicates; that name already belongs to the single-copy version (#26).
// Keep a value when it differs from the element two places behind the write index.
func removeDuplicatesII(nums []int) int {
	if len(nums) <= 2 {
		return len(nums)
	}
	write := 2
	for read := 2; read < len(nums); read++ {
		if nums[read] != nums[write-2] {
			nums[write] = nums[read]
			write++
		}
	}
	return write
}
