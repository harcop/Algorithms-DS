package leetcode

// LeetCode #283 - Move Zeroes
//
// Swap each nonzero into the next open slot so the zeros collect at the end.
func moveZeroes(nums []int) {
	write := 0
	for read := 0; read < len(nums); read++ {
		if nums[read] != 0 {
			nums[write], nums[read] = nums[read], nums[write]
			write++
		}
	}
}
