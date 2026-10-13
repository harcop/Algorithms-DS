package leetcode

// LeetCode #27 - Remove Element
//
// Overwrite values equal to val and return the count of values that remain.
func removeElement(nums []int, val int) int {
	write := 0
	for _, n := range nums {
		if n != val {
			nums[write] = n
			write++
		}
	}
	return write
}
